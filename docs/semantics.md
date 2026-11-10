# Evolving semantics

## Purpose and authority

This document records the smallest semantic promises needed by the active learning path.
It is not a complete grammar, a stable API specification, or a description of behavior that has already been implemented.

Statements are classified as:

- **Accepted:** a current project contract that implementations and tests should preserve.
- **Provisional:** the working contract for the next milestone, subject to evidence from implementation.
- **Open:** a decision the learner must make explicitly before dependent behavior is considered complete.

When an accepted contract changes, update the relevant tests and record a decision when the change affects more than one milestone or crate.

## Numerical foundation

- **Accepted:** Initial numerical values use `f64`.
- **Accepted:** The first runtime operates on first-order ordinary differential equations.
- **Accepted:** A second-order source equation must eventually lower to explicit first-order states before numerical execution.
- **Accepted:** Runtime execution must not depend on source tokens, parser recovery state, or terminal diagnostics.
- **Accepted:** The interpreter remains the reference execution behavior before any code generator is introduced.

## Parameters

- **Accepted:** A parameter is a named scalar quantity whose value remains constant during one simulation.
- **Accepted:** A parameter has a physical dimension.
- **Accepted:** A parameter value used for calibration is still a model parameter, not mutable state.
- **Provisional:** Known and calibratable parameters share one declaration form plus explicit calibration metadata later.
- **Open:** The source syntax for marking a parameter as calibratable.
- **Open:** Whether parameter bounds are model semantics, calibration configuration, or both.

## States and initial conditions

- **Accepted:** A state is a scalar quantity whose value evolves over simulation time.
- **Accepted:** Every state has a physical dimension.
- **Accepted:** Every executable state has exactly one derivative equation.
- **Accepted:** Every executable state has one explicit, dimensionally compatible initial value.
- **Accepted:** Missing or duplicate derivative equations are semantic errors and cannot reach runtime execution.
- **Accepted:** Missing, conflicting, or invalid initial conditions are reported rather than replaced with implicit defaults.
- **Provisional:** Runtime state is stored in a contiguous vector with one stable slot per validated state.
- **Open:** Which IR structure owns the final mapping from typed state IDs to runtime slots.

## Derivatives

- **Accepted:** A derivative equation associates one state with an expression for its time derivative.
- **Accepted:** The derivative of a state with dimension `D` has dimension `D / T`.
- **Accepted:** Runtime derivative evaluation reads the current time, state, and fixed parameters without mutating the input state.
- **Accepted:** One derivative evaluation produces exactly one scalar derivative per runtime state slot.
- **Accepted:** `DynamicalSystem` declares a state dimension and returns one derivative vector from borrowed time-state input.
- **Accepted:** Checked derivative evaluation validates system, input, and output shape and finite-value contracts through `RuntimeError`.
- **Provisional:** The first hard-coded runtime model exposes its derivative calculation without depending on `stormer-frontend` or `stormer-ir` values.

## Expressions

- **Accepted:** The current structural exercise represents scalar constants, parameter references, state references, negation, addition, subtraction, multiplication, and division.
- **Accepted:** Every expression reference in finalized IR resolves to an earlier available node in the same expression arena (`operand.index() < expression.index()`), guaranteeing topological DAG order.
- **Accepted:** Runtime evaluation performs no name lookup.
- **Accepted:** Invalid source expressions do not reach runtime evaluation.
- **Provisional:** Algebraic intermediate expressions may exist in source and lower into the same typed expression graph as derivative roots.
- **Open:** The eventual normalized evaluation order and common-subexpression policy.

## Physical dimensions and units

- **Accepted:** Physical dimensions are part of language semantics rather than display metadata.
- **Accepted:** A dimension is represented conceptually by signed exponents over the seven SI base dimensions.
- **Accepted:** Multiplication and division compose dimensions according to exponent laws.
- **Accepted:** Addition and subtraction require compatible dimensions.
- **Accepted:** A dimension does not by itself record a named unit, prefix, scale factor, or offset.
- **Accepted:** Invalid dimension combinations are rejected before runtime execution.
- **Open:** The first source notation for units.
- **Open:** Which unit conversions are implicit.
- **Open:** How scaled and affine units are represented and diagnosed.

## Names and scopes

- **Accepted:** Every symbolic reference is resolved before entering executable IR.
- **Accepted:** Typed IDs keep parameters, states, and expressions in distinct identity domains.
- **Accepted:** Runtime execution uses resolved IDs or slots and never source names for lookup.
- **Open:** Whether parameters and states share one source namespace.
- **Open:** The duplicate-name policy for declarations.
- **Open:** Whether local algebraic bindings exist in the first source language.

## Runtime stepping and trajectories

- **Provisional:** The first numerical milestone begins with a hard-coded damped oscillator so numerical behavior can be tested without parser or lowering failures.
- **Provisional:** Explicit Euler is implemented before RK4 so their accuracy and stability behavior can be compared.
- **Accepted:** A step operation must not mutate the caller's input state.
- **Accepted:** State and derivative vector lengths must agree with the system dimension.
- **Accepted:** A trajectory records time together with a complete state snapshot in stable slot order.
- **Accepted:** The initial condition appears as the first trajectory sample.
- **Accepted:** Non-finite configuration or state values must not be accepted silently.
- **Accepted:** The milestone exposes only concrete Euler and classical RK4 choices rather than a generic solver framework.
- **Accepted:** Runtime precondition failures use the structural `RuntimeError` boundary prepared by the milestone scaffold.
- **Accepted:** When a simulation interval `(end_time - start_time)` is not an exact multiple of the requested step size, `integrate` takes full steps of `step_size` and one shorter clamped final step so the trajectory terminates at `end_time`.
- **Open:** Which additional metadata beyond ordered time-state samples is required for reproducibility.

## Calibration

- **Accepted:** Calibration compares explicit simulated observables with measurements using a stated loss.
- **Accepted:** The first calibration target is the damping parameter of the canonical oscillator with synthetic position observations.
- **Accepted:** Measurements retain time, dimension, and observable identity.
- **Accepted:** A fitted value is not trustworthy without reporting the loss and the assumptions used to obtain it.
- **Provisional:** Finite differences provide the first independent sensitivity reference.
- **Open:** The first loss function, optimizer, stopping condition, and parameter constraints.

## Error ownership

- **Accepted:** Parsing errors report malformed source structure.
- **Accepted:** Name-resolution errors report missing or ambiguous declarations.
- **Accepted:** Semantic errors report invalid types, dimensions, derivatives, or initial conditions.
- **Accepted:** Lowering errors indicate a violated compiler invariant and must not be disguised as ordinary runtime input errors.
- **Accepted:** Runtime errors report invalid execution configuration or failures encountered while evaluating an already validated model.
- **Accepted:** User-facing diagnostics belong to `stormer-frontend` or a coordinating product surface, not `stormer-ir`.

## Executable IR invariants

Before a model reaches interpreted execution:

1. Every symbol reference is resolved.
2. Every expression has a known scalar type and physical dimension.
3. Every state has one valid initial value and one derivative expression.
4. Every derivative expression has the required state-per-time dimension.
5. Every typed ID resolves in its owning collection.
6. The runtime can evaluate without source-level name lookup.
7. Source syntax and parser recovery nodes are absent from the runtime contract.
8. Invalid unit or dimension expressions cannot reach the interpreter.

The prepared `stormer-ir` exercise currently specifies only a subset of these structural invariants.
It does not yet prove typed expressions, dimensional inference, lowering, evaluation, or runtime readiness.
