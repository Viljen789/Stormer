# Rolling learning roadmap

## How to use this roadmap

Milestone 0 establishes the charter and contracts.
Six implementation milestones then build the execution spine, the language spine, their joined workflow, and one calibration result.

Only the active implementation milestone is detailed at task level.
Later milestones record direction, ownership, invariants, exit evidence, and reflection prompts without freezing interfaces that earlier work may invalidate.

An exercise guide number records when a scaffold was prepared, not the order in which its TODOs must be completed.
The existing guide `0001-dimensions-and-model-ir.md` was prepared before the runtime-first sequence was chosen.
It remains valid, but it is parked until Milestones 2 and 3 rather than being renumbered or silently completed.

## Milestone overview

| Milestone | Main concepts | Visible result |
| --- | --- | --- |
| 0. Charter and contracts | Scope, semantics, invariants, decomposition, learning ownership | One canonical model and a bounded learning programme |
| 1. Numerical runtime | First-order ODEs, state vectors, Euler, RK4, numerical error | A hard-coded damped oscillator produces a trajectory |
| 2. Expressions and dimensions | Expression graphs, dimensional analysis, recursive reasoning | Manually constructed expressions can be checked for physical consistency |
| 3. Names and typed IR | Syntax trees, symbol tables, resolution, semantic analysis, lowering | A manually constructed source tree lowers to inspected executable IR |
| 4. Minimal parser | Tokens, grammar, precedence, spans, diagnostics | A tiny source program produces the same checked IR |
| 5. End-to-end simulation | Interpreter architecture and subsystem boundaries | Source text produces the oscillator trajectory |
| 6. Parameter calibration | Losses, finite differences, optimization, identifiability | Synthetic observations recover a damping estimate |

## Milestone 0 - Charter and contracts

- **Concepts:** Product scope, learning ownership, semantic promises, package boundaries, and evidence standards.
- **Behavioral result:** A contributor can identify the canonical damped oscillator, the two learning spines, the current checkpoint, and the work reserved for the learner.
- **Invariants:** The four-crate dependency direction stays acyclic, Rust owns production semantics, Python remains reference-only, units belong to language semantics, and interpretation precedes code generation.
- **Learner owns:** Accepting or changing consequential semantic choices.
- **Codex may scaffold:** Documentation, status, repository instructions, decision templates, and non-executable test plans.
- **Exit evidence:** `docs/learning-charter.md`, this roadmap, `docs/semantics.md`, `STATUS.md`, `AGENTS.md`, and existing ADRs agree about scope and ownership.
- **Reflection:** Why are the execution and language spines separated, and which failure ambiguity does that remove?

## Milestone 1 - Numerical runtime

### Goal

Simulate a hard-coded damped harmonic oscillator without involving source text, name resolution, typed IR lowering, or parameter fitting.
This establishes numerical behavior independently from compiler behavior.

### Concepts to learn

- Ordinary differential equations as rules for state change.
- Conversion of a second-order equation into a first-order state system.
- Meaning and ordering of a runtime state vector.
- Pure derivative evaluation over time, state, and fixed parameters.
- Euler's geometric approximation.
- Local error, accumulated global error, and stability.
- RK4 as a higher-order comparison rather than a magical accuracy switch.
- Numerical evidence from analytical and qualitative behavior.

### Behavioral result

A programmatically constructed system with mass `m`, damping `c`, stiffness `k`, position `x`, and velocity `v` produces an inspectable trajectory over a configured time interval.
The same system can be stepped with Euler and RK4 so their error behavior can be compared.

### Required contracts and invariants

- Runtime values use `f64`.
- The oscillator is represented as two first-order states with a stable documented order.
- Fixed parameters do not mutate during a simulation.
- Derivative evaluation does not mutate the input state.
- Every state evaluation produces exactly one derivative value for every state slot.
- A step does not mutate the caller's input state.
- Invalid vector shapes return an explicit error rather than panicking or truncating.
- Non-finite configuration and state values are rejected or reported explicitly.
- A trajectory begins with the initial condition and pairs every time with one complete state snapshot.
- Numerical tests state parameters, interval, step size, tolerance, and the reason for the expected result.

The remaining provisional and open runtime choices are recorded in `docs/semantics.md` and `STATUS.md`.

### Learner-owned implementation

The learner implements and explains:

- The hard-coded oscillator derivative.
- State and derivative shape checks that embody the runtime contract.
- One Euler step.
- Repeated stepping into a trajectory.
- One RK4 step.
- The comparison between observed error and expected convergence behavior.

The learner must be able to explain what each state slot means, why the derivative has the stated form, how each step uses derivative information, and which step sizes expose instability.

### Codex-approved scaffolding

The prepared scaffold contains only:

- Small public type and trait signatures needed by this milestone.
- Documentation comments with explicit preconditions and invariants.
- Test fixtures and helpers that do not contain the integration algorithm.
- Ignored behavioral tests with numbered TODO labels.
- Mechanical trajectory display or example wiring.

The scaffold must compile with normal tests green.
It must not implement oscillator derivatives, Euler, RK4, repeated stepping, or a hidden equivalent of those operations.

### Planned stages

#### Stage 1A - Runtime state contract

Specify the state ordering, system dimension, derivative input and output shapes, parameter ownership, and error boundary.
Use ignored tests for shape mismatches, input immutability, and one derivative evaluation.

#### Stage 1B - Euler stepping

Add one ignored test for a simple system with an independently calculable step.
Then exercise the oscillator over a short interval and record how changing step size changes the result.

#### Stage 1C - Trajectory and qualitative evidence

Record time and state snapshots from the initial condition through the requested interval.
Check sustained motion without damping, decreasing amplitude with positive damping, and the instability of an intentionally excessive Euler step.

#### Stage 1D - RK4 and convergence evidence

Add RK4 behind the smallest interface justified by the Euler work.
Compare error at successively smaller step sizes against an analytical or independently generated reference.
Do not introduce a solver framework until both implementations show what must actually vary.

### Exit criteria

Milestone 1 is complete only when:

1. A hard-coded damped oscillator produces a trajectory with documented state ordering.
2. Euler and RK4 behavior pass focused tests against independently justified expectations.
3. Invalid shapes and non-finite inputs follow the documented error contract.
4. Numerical tests state their tolerances and show the expected effect of reducing step size.
5. The learner can explain the oscillator conversion, both stepping methods, local versus global error, and an observed stability failure.
6. Formatting, linting, normal tests, and workspace checks pass.

### Reflection questions

- Why does a second-order oscillator require two first-order runtime states?
- What does each state slot mean, and which component owns that mapping?
- Which assumption makes one Euler step only a local approximation?
- Why can a smaller step improve accuracy yet cost more work?
- What did the excessive-step experiment reveal about numerical stability?
- Why is RK4 still not evidence that every step size is safe?
- Which runtime invariant should already be guaranteed before interpreted IR reaches the solver?

### Explicit exclusions

This milestone does not add parser types, source diagnostics, name resolution, dimension inference, IR evaluation, adaptive solvers, calibration, automatic differentiation, code generation, bindings, or a generic solver framework.

## Milestone 2 - Expressions and dimensions

- **Concepts:** Dimension exponent vectors, compatibility, typed expression IDs, arena ownership, and structural graph validation.
- **Behavioral result:** A manually constructed oscillator expression graph can be inspected and checked for physical consistency without parsing source text.
- **Invariants:** Typed IDs remain distinct and stable, every graph edge resolves, multiplication and division compose dimensions, and additive operands are compatible.
- **Learner owns:** `TODO(STORMER-001)` through the relevant dimension and expression exercises, plus dimensional inference not yet scaffolded.
- **Codex may scaffold:** Data shapes, ignored behavioral tests, fixtures, diagnostics expectations, and documentation.
- **Exit evidence:** Valid expressions receive the expected dimensions, invalid combinations produce precise errors, and the learner explains units versus dimensions.
- **Reflection:** Which invalid physical expression is structurally valid, and why must structure and dimensional meaning be checked at different boundaries?

## Milestone 3 - Names and typed IR

- **Concepts:** Source-level declarations, syntax trees, symbol tables, scopes, resolution, semantic checking, lowering, and IR invariants.
- **Behavioral result:** A manually constructed source tree for the oscillator lowers to source-independent IR that the runtime can evaluate.
- **Invariants:** Names resolve before IR, every expression has a scalar type and dimension, every state has an initial value and derivative, and runtime execution performs no name lookup.
- **Learner owns:** Symbol tables, resolution, validation, typed lowering, and remaining model-construction exercises.
- **Codex may scaffold:** AST and diagnostic shapes, fixtures, ignored tests, provenance plumbing, and repetitive accessors.
- **Exit evidence:** The manually constructed source tree lowers to inspected IR, invalid trees fail in the correct semantic phase, and evaluated derivatives agree with the hard-coded runtime model.
- **Reflection:** Which facts belong to source syntax, checked semantics, IR, and runtime respectively?

## Milestone 4 - Minimal parser

- **Concepts:** Tokenization, grammar, precedence, associativity, source spans, recovery, and diagnostics.
- **Behavioral result:** A tiny source program for the oscillator produces the same checked IR as the manually constructed tree.
- **Invariants:** Parsing preserves source provenance, syntax errors do not masquerade as semantic errors, and parsing does not perform numerical execution.
- **Learner owns:** Grammar decisions, parser control flow, precedence handling, and the boundary between recovery and rejection.
- **Codex may scaffold:** Source fixtures, token and span types, golden-test plumbing, and CLI file-loading mechanics.
- **Exit evidence:** One canonical source fixture and focused invalid fixtures produce accepted syntax or complete diagnostics.
- **Reflection:** Which malformed input can be recovered for more diagnostics, and which recovery would invent misleading meaning?

## Milestone 5 - End-to-end simulation

- **Concepts:** Interpreter design, subsystem boundaries, provenance through lowering, and integration testing.
- **Behavioral result:** Source text for the damped oscillator produces the same trajectory as the hard-coded reference model.
- **Invariants:** The runtime consumes validated IR only, interpreted expressions preserve checked semantics, and equivalent hard-coded and source-defined models agree within stated tolerances.
- **Learner owns:** Expression evaluation, runtime model instantiation, and semantic integration across the compiler and solver.
- **Codex may scaffold:** CLI orchestration, fixture loading, output formatting, differential-test harnesses, and error presentation.
- **Exit evidence:** Valid source reaches a trajectory, invalid source stops before runtime, and differential tests agree with the Milestone 1 reference.
- **Reflection:** When a trajectory differs, which evidence separates parser, lowering, interpreter, and solver defects?

## Milestone 6 - Parameter calibration

- **Concepts:** Observations, residuals, loss functions, finite differences, optimization, parameter scaling, and identifiability.
- **Behavioral result:** Synthetic position observations recover an estimate of oscillator damping and report the remaining mismatch.
- **Invariants:** Observations map to explicit modeled quantities, loss and stopping rules are stated, parameters remain separate from dynamic state, and fitted results include assumptions and residual evidence.
- **Learner owns:** Loss construction, sensitivity reference, optimization logic, and interpretation of identifiability.
- **Codex may scaffold:** Observation fixtures, CSV plumbing, reporting, benchmark harnesses, and ignored edge-case tests.
- **Exit evidence:** A known synthetic damping value is recovered within a justified tolerance and the learner explains when the same procedure would be unreliable.
- **Reflection:** What makes damping identifiable from the chosen observations, and which different parameter combinations could produce similar trajectories?

## Later possibilities

Automatic differentiation, native code generation, GPU execution, Python bindings, advanced solvers, partial differential equations, and a visual studio remain later possibilities.
They are not current requirements.

A later feature must be justified by evidence from the completed interpreted calibration slice rather than by architectural possibility alone.

Candidate milestones beyond this slice are collected in `docs/next-level.md`.
They are proposals and are not accepted scope until promoted into this roadmap.
