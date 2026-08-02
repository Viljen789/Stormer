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

This repository is an architectural scaffold only.
It contains workspace configuration, crate boundaries, decision records, and planning documents.
It does not yet parse, check, simulate, fit, compile, or render a physical model.

Illustrative future commands include:

```console
stormer check pendulum.storm
stormer simulate pendulum.storm
stormer fit pendulum.storm observations.csv
stormer compile pendulum.storm --target wasm
```

These commands are product direction only and are not implemented.

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
- A type checker, unit checker, equation IR, or optimizer.
- Numerical solvers, automatic differentiation, sensitivity analysis, or parameter inference.
- Native code generation, Python bindings, WebAssembly bindings, or a user interface.
- Parser, linear-algebra, CLI, error, unit, autodiff, e-graph, or code-generation libraries.
- C, C++, CMake, LLVM, MLIR, Eigen, SUNDIALS, or other FFI infrastructure.

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

See [the product direction](docs/product.md), [the architecture](docs/architecture.md), and [the roadmap](docs/roadmap.md) for the decisions this scaffold records.
