#![forbid(unsafe_code)]
//! Command-line interface for Størmer (`check`, `simulate`, and `fit`).

use std::env;
use std::fs;
use std::process::ExitCode;

use stormer_frontend::{Model, compile_str};
use stormer_runtime::{
    CalibrationConfig, IntegrationMethod, IrSystem, Observation, calibrate_parameter,
    simulate_model,
};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        print_usage();
        return ExitCode::SUCCESS;
    }

    let result = match args[0].as_str() {
        "check" => run_check(&args[1..]),
        "simulate" => run_simulate(&args[1..]),
        "fit" => run_fit(&args[1..]),
        other => Err(format!("unknown subcommand `{other}`\n\n{}", usage_text())),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn usage_text() -> &'static str {
    "Usage:
  stormer check <model.storm>
  stormer simulate <model.storm> [--method rk4|euler] [--start <f64>] [--end <f64>] [--dt <f64>] [--csv]
  stormer fit <model.storm> <observations.csv> [--param <name>] [--guess <f64>] [--method rk4|euler] [--dt <f64>]"
}

fn print_usage() {
    println!("STØRMER — Programmable Physics Engine\n\n{}", usage_text());
}

fn load_and_compile(path: &str) -> Result<Model, String> {
    let source =
        fs::read_to_string(path).map_err(|err| format!("failed to read `{path}`: {err}"))?;
    compile_str(&source).map_err(|diagnostics| {
        let mut rendered = String::new();
        for diag in diagnostics {
            rendered.push_str(&diag.render(&source));
        }
        rendered
    })
}

fn run_check(args: &[String]) -> Result<(), String> {
    let Some(path) = args.first() else {
        return Err(format!("missing model file path\n\n{}", usage_text()));
    };
    let model = load_and_compile(path)?;

    println!("Model `{path}` is valid.");
    println!("Parameters ({}):", model.parameters().len());
    for param in model.parameters() {
        println!(
            "  - {}: [{}] = {}",
            param.name(),
            param.dimension(),
            param.initial_value()
        );
    }
    println!("States ({}):", model.states().len());
    for state in model.states() {
        println!(
            "  - {}: [{}] (initial = {})",
            state.name(),
            state.dimension(),
            state.initial_value()
        );
    }
    println!("Derivative equations ({}):", model.derivatives().len());
    for eq in model.derivatives() {
        let state = &model.states()[eq.state().index()];
        let dim = model
            .infer_expression_dimension(eq.expression())
            .map_err(|e| format!("dimension error: {e:?}"))?;
        println!("  - d({})/dt : [{}]", state.name(), dim);
    }
    Ok(())
}

fn run_simulate(args: &[String]) -> Result<(), String> {
    let Some(path) = args.first() else {
        return Err(format!("missing model file path\n\n{}", usage_text()));
    };
    let mut method = IntegrationMethod::RungeKutta4;
    let mut start_time = 0.0_f64;
    let mut end_time = 5.0_f64;
    let mut step_size = 0.01_f64;
    let mut csv = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--csv" => {
                csv = true;
                i += 1;
            }
            "--method" => {
                let val = args
                    .get(i + 1)
                    .ok_or_else(|| "missing value after `--method`".to_owned())?;
                method = parse_method(val)?;
                i += 2;
            }
            "--start" => {
                let val = args
                    .get(i + 1)
                    .ok_or_else(|| "missing value after `--start`".to_owned())?;
                start_time = val
                    .parse::<f64>()
                    .map_err(|_| format!("invalid `--start` number `{val}`"))?;
                i += 2;
            }
            "--end" => {
                let val = args
                    .get(i + 1)
                    .ok_or_else(|| "missing value after `--end`".to_owned())?;
                end_time = val
                    .parse::<f64>()
                    .map_err(|_| format!("invalid `--end` number `{val}`"))?;
                i += 2;
            }
            "--dt" => {
                let val = args
                    .get(i + 1)
                    .ok_or_else(|| "missing value after `--dt`".to_owned())?;
                step_size = val
                    .parse::<f64>()
                    .map_err(|_| format!("invalid `--dt` number `{val}`"))?;
                i += 2;
            }
            other => return Err(format!("unknown flag `{other}`\n\n{}", usage_text())),
        }
    }

    let model = load_and_compile(path)?;
    let trajectory = simulate_model(&model, method, start_time, end_time, step_size)
        .map_err(|err| format!("runtime error: {err:?}"))?;

    if csv {
        let state_names: Vec<&str> = model.states().iter().map(|s| s.name()).collect();
        println!("time,{}", state_names.join(","));
        for sample in trajectory.samples() {
            let vals: Vec<String> = sample.state().iter().map(|v| format!("{v:.8}")).collect();
            println!("{:.6},{}", sample.time(), vals.join(","));
        }
    } else {
        println!(
            "Simulated `{path}` with {:?} over [{start_time}, {end_time}] (dt = {step_size}, {} samples)",
            method,
            trajectory.len()
        );
        let stride = (trajectory.len() / 10).max(1);
        for (idx, sample) in trajectory.samples().iter().enumerate() {
            if idx % stride == 0 || idx + 1 == trajectory.len() {
                let parts: Vec<String> = model
                    .states()
                    .iter()
                    .zip(sample.state().iter())
                    .map(|(st, val)| format!("{} = {:+.6}", st.name(), val))
                    .collect();
                println!("  t = {:>6.3} s | {}", sample.time(), parts.join(", "));
            }
        }
    }
    Ok(())
}

fn run_fit(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err(format!(
            "expected `<model.storm> <observations.csv>`\n\n{}",
            usage_text()
        ));
    }
    let model_path = &args[0];
    let obs_path = &args[1];

    let mut param_name: Option<String> = None;
    let mut initial_guess: Option<f64> = None;
    let mut config = CalibrationConfig::default();

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--param" => {
                let val = args
                    .get(i + 1)
                    .ok_or_else(|| "missing value after `--param`".to_owned())?;
                param_name = Some(val.clone());
                i += 2;
            }
            "--guess" => {
                let val = args
                    .get(i + 1)
                    .ok_or_else(|| "missing value after `--guess`".to_owned())?;
                initial_guess = Some(
                    val.parse::<f64>()
                        .map_err(|_| format!("invalid `--guess` value `{val}`"))?,
                );
                i += 2;
            }
            "--method" => {
                let val = args
                    .get(i + 1)
                    .ok_or_else(|| "missing value after `--method`".to_owned())?;
                config.method = parse_method(val)?;
                i += 2;
            }
            "--dt" => {
                let val = args
                    .get(i + 1)
                    .ok_or_else(|| "missing value after `--dt`".to_owned())?;
                config.step_size = val
                    .parse::<f64>()
                    .map_err(|_| format!("invalid `--dt` value `{val}`"))?;
                i += 2;
            }
            other => return Err(format!("unknown flag `{other}`\n\n{}", usage_text())),
        }
    }

    let model = load_and_compile(model_path)?;
    let system = IrSystem::new(&model).map_err(|e| format!("runtime error: {e:?}"))?;

    let (observable_name, observations) = parse_observations_csv(obs_path)?;
    let observable_state = system.find_state(&observable_name).ok_or_else(|| {
        format!("observable column `{observable_name}` is not a state in `{model_path}`")
    })?;

    let target_param_name = param_name.unwrap_or_else(|| {
        if system.find_parameter("damping").is_some() {
            "damping".to_owned()
        } else {
            model
                .parameters()
                .first()
                .map(|p| p.name().to_owned())
                .unwrap_or_default()
        }
    });
    let param_id = system
        .find_parameter(&target_param_name)
        .ok_or_else(|| format!("parameter `{target_param_name}` not found in `{model_path}`"))?;
    let guess =
        initial_guess.unwrap_or_else(|| model.parameters()[param_id.index()].initial_value());

    let result = calibrate_parameter(
        &model,
        param_id,
        guess,
        observable_state,
        &observations,
        config,
    )
    .map_err(|e| format!("calibration failed: {e:?}"))?;

    let param_decl = &model.parameters()[param_id.index()];
    println!("Calibration Report for `{model_path}` against `{obs_path}`:");
    println!(
        "  Target parameter   : {} [{}]",
        param_decl.name(),
        param_decl.dimension()
    );
    println!(
        "  Observable state   : {observable_name} ({} samples)",
        observations.len()
    );
    println!("  Initial guess      : {:.6}", result.initial_guess);
    println!("  Estimated value    : {:.6}", result.estimated_value);
    println!("  Initial MSE loss   : {:.6e}", result.initial_loss);
    println!("  Final MSE loss     : {:.6e}", result.final_loss);
    println!("  Final sensitivity  : {:+.6e}", result.final_sensitivity);
    println!(
        "  Iterations         : {} (converged = {})",
        result.iterations, result.converged
    );
    Ok(())
}

fn parse_method(text: &str) -> Result<IntegrationMethod, String> {
    match text.to_ascii_lowercase().as_str() {
        "rk4" | "runge-kutta-4" => Ok(IntegrationMethod::RungeKutta4),
        "euler" => Ok(IntegrationMethod::Euler),
        other => Err(format!(
            "unknown integration method `{other}` (expected `rk4` or `euler`)"
        )),
    }
}

fn parse_observations_csv(path: &str) -> Result<(String, Vec<Observation>), String> {
    let content =
        fs::read_to_string(path).map_err(|err| format!("failed to read `{path}`: {err}"))?;
    let mut lines = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'));

    let header = lines
        .next()
        .ok_or_else(|| format!("observation file `{path}` is empty"))?;
    let mut header_cols = header.split(',').map(str::trim);
    let first_col = header_cols.next().unwrap_or("");
    let second_col = header_cols
        .next()
        .ok_or_else(|| format!("expected `time,<state_name>` header in `{path}`"))?;
    if first_col != "time" {
        return Err(format!(
            "expected first CSV column to be `time`, found `{first_col}`"
        ));
    }

    let mut observations = Vec::new();
    for (idx, line) in lines.enumerate() {
        let mut parts = line.split(',').map(str::trim);
        let t_str = parts
            .next()
            .ok_or_else(|| format!("missing time on row {}", idx + 2))?;
        let v_str = parts
            .next()
            .ok_or_else(|| format!("missing value on row {}", idx + 2))?;
        let time = t_str
            .parse::<f64>()
            .map_err(|_| format!("invalid time `{t_str}` on row {}", idx + 2))?;
        let value = v_str
            .parse::<f64>()
            .map_err(|_| format!("invalid value `{v_str}` on row {}", idx + 2))?;
        observations.push(Observation::new(time, value));
    }

    Ok((second_col.to_owned(), observations))
}
