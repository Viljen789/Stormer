# Testing strategy

The current repository contains no completed domain behavior.
It does contain ignored behavioral specifications for the prepared dimensions-and-model-IR exercise.
Normal workspace tests compile those specifications without running them, so a green baseline does not claim that the exercises work.
As learner-owned behavior arrives, tests should progress from structural contracts to numerical evidence and complete user workflows.

## Guided exercise behavior

- Exercise tests compile as ordinary integration tests but begin with `#[ignore = "TODO(STORMER-XXX): ..."]`.
- Every ignored test states observable behavior and reaches a corresponding explicit TODO while the exercise is unfinished.
- A learner runs one named ignored test while implementing its concept.
- The ignore marker is removed only after the behavior is complete and the assertion remains intact.
- Normal green validation and an expected focused TODO failure are different evidence and should be reported separately.

The prepared IR scaffold has 26 ignored tests covering dimension values, expression storage, model construction, invalid references, and exact harmonic-oscillator topology.
The active runtime scaffold has 25 ignored tests covering checked derivative evaluation, finite-value and shape contracts, Euler, trajectory construction, RK4, and the damped-oscillator capstone.
Their guides are indexed in `docs/exercises/README.md`.

## Frontend behavior

- Parser fixtures will pair representative model text with expected acceptance or syntax structure.
- Golden diagnostic tests will compare complete, user-visible diagnostics for invalid source files.
- Unit-checking tests will cover valid conversions, invalid equations, and diagnostic source locations.
- Property-based dimensional-analysis tests will generate valid and invalid combinations to check algebraic invariants.

Fixture directories are reserved under `tests/fixtures/`, and rendered expectations are reserved under `tests/golden/`.
No language examples should become normative accidentally before the semantics are accepted.

## Numerical behavior

- The runtime milestone will begin with ignored behavioral specifications rather than a hidden reference solver.
- Analytical-solution comparisons will measure simulated trajectories against systems with known solutions.
- Convergence-order tests will verify that numerical error decreases at the expected rate when step size changes.
- Python and JAX reference comparisons will check Rust results against intentionally independent mathematical experiments.
- Finite-difference gradient checking will validate future sensitivities and autodiff results at well-conditioned points.
- Interpreter-versus-codegen differential tests will require future backends to agree with the tested interpreter semantics.

Numerical tests should state tolerances, time ranges, parameter scales, and why each comparison is meaningful.
Passing one reference case must not be treated as proof for every solver regime.

## End-to-end behavior

End-to-end model-calibration tests will start from model source and observations, then check diagnostics, simulation, parameter estimates, and trajectory comparison as a single workflow.
The first such test should use a small system with known synthetic parameters so that the expected result is independently defensible.

## Test ownership

Package-local tests should protect internal invariants near the behavior they exercise.
Repository-level fixtures and golden files should protect contracts that cross the frontend, IR, runtime, and product surface.
Reference notebooks should remain reproducible evidence rather than production dependencies.

No parser, property-testing, numerical, snapshot, or Python dependency is selected by this scaffold.
