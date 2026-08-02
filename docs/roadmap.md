# Roadmap

Each phase is narrow enough to produce evidence about the next architectural decision.
Later phases may change as earlier work reveals better boundaries, but a phase should not be called complete without its stated criterion.

## Phase 0 - Repository and semantic design

- **User-visible capability:** Contributors can navigate a compiling workspace and understand the intended product, package boundaries, and first workflow.
- **Core concepts learned:** The project records its semantic goals, dependency direction, representation constraints, and initial architectural decisions.
- **Explicit exclusions:** No language, IR behavior, simulation, inference, binding, code generator, or user interface is implemented.
- **Completion criterion:** Exactly four empty-boundary crates pass formatting, linting, tests, and checks, and the required architecture documents agree about their responsibilities.

## Phase 1 - One valid model through parsing and checking

- **User-visible capability:** A user can submit one narrowly defined model and receive either a validated result or clear source diagnostics.
- **Core concepts learned:** The phase establishes the smallest syntax, source-span model, name resolution, physical-dimension rules, and semantic-checking boundary.
- **Explicit exclusions:** There is no equation execution, time integration, parameter inference, broad syntax, or stable public API.
- **Completion criterion:** One valid fixture and a focused set of invalid fixtures pass end-to-end frontend tests with accepted golden diagnostics.

## Phase 2 - Normalized equation IR and interpreted evaluation

- **User-visible capability:** A validated model can be lowered and evaluated at a supplied state and parameter set.
- **Core concepts learned:** The phase establishes typed IDs, IR ownership, normalization invariants, provenance, and interpreter semantics.
- **Explicit exclusions:** There is no time integrator, trajectory output, optimizer, autodiff system, or native code generation.
- **Completion criterion:** Checked source lowers to inspected IR, and interpreted equation values agree with independent expected values for the first model.

## Phase 3 - RK4 simulation and trajectory output

- **User-visible capability:** A user can simulate the first dynamical system over time and inspect a trajectory.
- **Core concepts learned:** The phase establishes runtime state, solver configuration, RK4 stepping, trajectory representation, and numerical error checks.
- **Explicit exclusions:** There is no adaptive solver, stiffness support, parameter fitting, GPU execution, or solver framework abstraction.
- **Completion criterion:** The trajectory matches an analytical solution within a stated tolerance and demonstrates the expected RK4 convergence order.

## Phase 4 - Observations and one-parameter inference

- **User-visible capability:** A user can fit one unknown parameter from observation data and compare simulated and observed trajectories.
- **Core concepts learned:** The phase establishes observation semantics, loss calculation, parameter selection, sensitivity validation, and fit reporting.
- **Explicit exclusions:** There is no general optimizer catalog, Bayesian inference, large parameter set, automatic experiment design, or opaque fit claim.
- **Completion criterion:** An end-to-end calibration test recovers a known synthetic parameter within a justified tolerance and reports the trajectory mismatch.

## Phase 5 - Interactive or Python-facing product surface

- **User-visible capability:** A user can explore the validated simulation and calibration workflow through one selected higher-level surface.
- **Core concepts learned:** The phase establishes the real integration boundary, data-transfer needs, iteration loop, and result presentation requirements.
- **Explicit exclusions:** The project will not build native, Python, and browser surfaces simultaneously, and it will not duplicate engine semantics outside Rust.
- **Completion criterion:** One selected surface completes the Phase 4 workflow with tested packaging and user-visible error handling.

## Phase 6 - Optimization, autodiff improvements, and code generation

- **User-visible capability:** Proven models can execute or calibrate faster while preserving established semantics and diagnostics.
- **Core concepts learned:** The phase measures optimization value, differentiation tradeoffs, backend equivalence, and deployment constraints.
- **Explicit exclusions:** There is no unmeasured rewrite system, backend proliferation, GPU promise, or removal of the interpreter as reference behavior.
- **Completion criterion:** Benchmarks show a material improvement on representative models, and differential tests demonstrate agreement with the interpreter and reference gradients.
