# Størmer status

Snapshot date: 2026-11-09.

## Current checkpoint

Milestones 0 through 6 of the initial vertical slice are implemented and verified:

- **Milestone 0 — Charter and contracts:** Established across `docs/`.
- **Milestone 1 — Numerical runtime (`stormer-runtime`):** Checked derivative evaluation, explicit Euler, classical RK4, trajectory integration with clamped final steps, and damped-oscillator convergence and stability verification.
- **Milestones 2 & 3 — Dimensions, expressions, and checked model IR (`stormer-ir`):** All `TODO(STORMER-001)` through `TODO(STORMER-013)` exercises, topological DAG validation, expression dimensional inference, and model-wide dimensional consistency checking (`Model::validate_dimensions`).
- **Milestone 4 — Source language frontend (`stormer-frontend`):** Span-aware `.storm` parser, SI unit expression parser (`kg`, `m`, `s`, `N`, `J`, `W`, `Pa`, `Hz`, powers, products, quotients), symbol resolution, dimensional checking, diagnostics rendering, and IR lowering (`compile_str`).
- **Milestone 5 — End-to-end interpreted simulation (`stormer-runtime` + `stormer-cli`):** `IrSystem` and `simulate_model` execute validated `stormer_ir::Model` instances in topological order without runtime name lookup, matching the hard-coded oscillator bit-for-bit.
- **Milestone 6 — Parameter calibration (`stormer-runtime` + `stormer-cli`):** Residual and MSE loss evaluation, central finite-difference sensitivities, and damped Gauss-Newton / Levenberg-Marquardt parameter estimation (`calibrate_parameter`), recovering oscillator damping from synthetic observations.

## Working

- `stormer-ir`: 28 active integration tests pass (`dimensions`, `expression_arena`, `model_construction`, `harmonic_oscillator`).
- `stormer-frontend`: 5 active integration tests pass covering valid `.storm` compilation and diagnostics for dimensional mismatches, invalid derivative dimensions, undefined symbols, missing derivatives, duplicate declarations, and unknown units.
- `stormer-runtime`: 31 active integration tests pass (`runtime_contract`, `euler`, `trajectory`, `rk4`, `damped_oscillator`, `ir_and_calibration`).
- `stormer-cli`: 3 end-to-end CLI integration tests pass (`stormer check`, `stormer simulate`, `stormer fit`) using canonical fixtures in `examples/models/` and `examples/observations/`.
- Zero ignored exercise holes remain; all 67 workspace tests run and pass under normal validation.

## Validation baseline

The expected green baseline is:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace
git diff --check
```

## Example CLI commands

```sh
cargo run -p stormer-cli -- check examples/models/damped_oscillator.storm
cargo run -p stormer-cli -- simulate examples/models/damped_oscillator.storm --end 5.0 --dt 0.01
cargo run -p stormer-cli -- fit examples/models/damped_oscillator.storm examples/observations/damped_oscillator_position.csv --param damping --guess 1.8
```
