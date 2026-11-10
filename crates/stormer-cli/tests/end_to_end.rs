use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

#[test]
fn cli_check_succeeds_for_canonical_damped_oscillator() {
    let bin = env!("CARGO_BIN_EXE_stormer");
    let model = workspace_root().join("examples/models/damped_oscillator.storm");

    let output = Command::new(bin)
        .arg("check")
        .arg(&model)
        .output()
        .expect("stormer binary should execute");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Parameters (3):"));
    assert!(stdout.contains("d(position)/dt : [L T^-1]"));
    assert!(stdout.contains("d(velocity)/dt : [L T^-2]"));
}

#[test]
fn cli_simulate_produces_csv_trajectory() {
    let bin = env!("CARGO_BIN_EXE_stormer");
    let model = workspace_root().join("examples/models/damped_oscillator.storm");

    let output = Command::new(bin)
        .args(["simulate"])
        .arg(&model)
        .args(["--end", "1.0", "--dt", "0.5", "--csv"])
        .output()
        .expect("stormer binary should execute");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines[0], "time,position,velocity");
    assert_eq!(lines.len(), 4);
}

#[test]
fn cli_fit_recovers_damping_parameter_from_observations() {
    let bin = env!("CARGO_BIN_EXE_stormer");
    let model = workspace_root().join("examples/models/damped_oscillator.storm");
    let obs = workspace_root().join("examples/observations/damped_oscillator_position.csv");

    let output = Command::new(bin)
        .arg("fit")
        .arg(&model)
        .arg(&obs)
        .args(["--param", "damping", "--guess", "1.5"])
        .output()
        .expect("stormer binary should execute");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Estimated value    : 0.500000"));
    assert!(stdout.contains("converged = true"));
}
