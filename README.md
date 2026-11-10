# STØRMER

> From equations to motion.

Størmer is a planned programmable physics product that will turn declarative equations and measurement data into validated simulations, fitted physical parameters, and deployable executable models.

## Planned product family

| Name | Intended purpose |
| --- | --- |
| **Størmer** | The overall project and product. |
| **Størmer Engine** | The compiler and differentiable simulation runtime. |
| **Størmer Studio** | An interactive model-building and parameter-fitting application. |
| **Størmer Models** | Shareable executable physics models. |
| `stormer` | The ASCII repository name and future command-line interface. |
| `stormer-ir` | The equation intermediate representation package. |
| `stormer-runtime` | The numerical runtime package. |
| `stormer-frontend` | The parser, unit checker, and compiler frontend package. |

These names describe the intended product family, not functionality available in the current repository.

## Product direction

The long-term compilation and execution pipeline is:

```text
physics source language
    -> parser and diagnostics
    -> unit and type checker
    -> typed equation IR
    -> equation optimization
    -> differentiable numerical runtime
    -> parameter inference and design optimization
    -> native, Python, and WebAssembly deployment
```

The first planned product workflow is deliberately narrow:

```text
equations + measurement data
    -> simulate a dynamical system
    -> estimate unknown physical parameters
    -> compare simulated and observed trajectories
```

This workflow will be implemented incrementally after the semantic foundations are understood.

## Current status

Størmer implements the complete Version 0 vertical slice across its four workspace crates:

- **`stormer-ir`**: SI physical dimensions (`Dimension`, `BaseDimension`), typed identifiers (`ParameterId`, `StateId`, `ExprId`), topological expression arenas (`ExpressionArena`), checked model construction (`ModelBuilder`, `Model`), and dimensional inference and validation (`Model::infer_expression_dimension`, `Model::validate_dimensions`).
- **`stormer-frontend`**: Span-aware `.storm` parser, SI unit notation parser, symbol resolution, dimensional consistency checking, structured diagnostics (`Diagnostic`), and lowering to validated `stormer_ir::Model` (`compile_str`).
- **`stormer-runtime`**: Checked derivative evaluation (`evaluate_derivative`), explicit Euler (`euler_step`), classical fourth-order Runge-Kutta (`rk4_step`), trajectory integration (`integrate`), zero-lookup IR interpretation (`IrSystem`, `simulate_model`), finite-difference sensitivity analysis (`finite_difference_sensitivity`), and Levenberg-Marquardt parameter calibration (`calibrate_parameter`).
- **`stormer-cli`**: Working command-line interface supporting `check`, `simulate`, and `fit`.

Try the canonical workflow right away:

```console
cargo run -p stormer-cli -- check examples/models/damped_oscillator.storm
cargo run -p stormer-cli -- simulate examples/models/damped_oscillator.storm --end 5.0 --dt 0.01
cargo run -p stormer-cli -- fit examples/models/damped_oscillator.storm examples/observations/damped_oscillator_position.csv --param damping --guess 1.8
```

## Workspace

| Package | Intended boundary |
| --- | --- |
| `stormer-ir` | Source-independent, language-independent representations of validated physical models. |
| `stormer-frontend` | Source files, parsing, diagnostics, semantic checking, and lowering to IR. |
| `stormer-runtime` | Execution of validated IR, numerical state, integration, and trajectories. |
| `stormer-cli` | File and terminal orchestration around the frontend and runtime. |

The dependency direction is:

```text
stormer-cli -> stormer-frontend -> stormer-ir
stormer-cli -> stormer-runtime  -> stormer-ir
```

The runtime does not depend on the frontend, and the IR does not depend on any other workspace package.

## Scaffold non-goals

The scaffold intentionally does not choose or implement:

- A model-language syntax, parser, lexer, or diagnostics framework.
- A type checker, dimension-inference pass, executable normalized IR, or optimizer.
- Completed numerical solvers, automatic differentiation, sensitivity analysis, or parameter inference.
- Native code generation, Python bindings, WebAssembly bindings, or a user interface.
- Parser, linear-algebra, CLI, error, unit, autodiff, e-graph, or code-generation libraries.
- C, C++, CMake, LLVM, MLIR, Eigen, SUNDIALS, or other FFI infrastructure.

## Learning workflow

Start with [the current status](STATUS.md), [the learning charter](docs/learning-charter.md), [the rolling roadmap](docs/roadmap.md), and [the evolving semantics](docs/semantics.md).
Repository agents must also follow [the learner-protection instructions](AGENTS.md).

Prepared exercises live under [the guided exercise index](docs/exercises/README.md).
Their ignored tests are executable specifications, not evidence that the behavior is implemented.

## Validation

Run the same focused checks as continuous integration:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace
```

## Stability and licensing

No language syntax or public API is stable yet.
Licensing has not yet been selected, so no license is granted by this repository at this stage.

See [the product direction](docs/product.md), [the architecture](docs/architecture.md), and [the decision records](docs/adr/) for the longer-term direction this scaffold records.
