# Størmer learning charter

## Purpose

Størmer is a programmable physics project for describing physical models, validating their meaning, simulating their behavior, and eventually calibrating unknown parameters against measurements.
It is also a structured way to learn compiler construction, numerical methods, dimensional analysis, Rust systems design, optimization, and automatic differentiation.

The project must serve both goals.
A technically ambitious feature is not progress if it hides the concept the learner is trying to understand.
A learning exercise is not enough if it never contributes evidence toward one coherent product workflow.

## Initial user

The initial user is a technically capable student, engineer, or scientist who can state an ordinary differential-equation model and wants a transparent path from equations to a reproducible trajectory.
The user should be able to inspect model assumptions, physical dimensions, numerical choices, and calibration results instead of treating the engine as a black box.

The first version is not designed for a broad no-code audience.
It is designed to make one small modeling workflow correct, explainable, and extensible.

## Canonical demonstration

The canonical model is a one-dimensional damped harmonic oscillator:

```text
m x'' + c x' + k x = 0
```

It becomes a first-order state system with position `x` and velocity `v`:

```text
x' = v
v' = -(c / m) v - (k / m) x
```

The complete demonstration will eventually:

1. Represent mass, damping, stiffness, position, velocity, and their physical dimensions.
2. Simulate a hard-coded oscillator and produce a trajectory.
3. Express the same model through a minimal source language.
4. Parse, resolve, check, and lower that source into typed IR.
5. Interpret the IR through the tested runtime.
6. Estimate the damping parameter from synthetic position observations.

The undamped oscillator in the prepared IR exercise is a smaller structural precursor.
It does not replace this end-to-end demonstration.

## Two learning spines

The project develops two relatively independent spines before joining them.

```text
execution spine: hard-coded model -> runtime representation -> numerical integration

language spine: source text -> syntax tree -> semantic analysis -> typed IR

joined system: source text -> typed IR -> interpreted runtime -> trajectory
```

This order makes failures easier to localize.
A wrong hard-coded trajectory points to model or numerical behavior, while a mismatch between source and a known runtime model points to frontend semantics.

## Initial scope

Version 0 includes only the capabilities needed to prove the canonical demonstration:

- A stable Rust core split across the existing four crates.
- `f64` numerical values.
- A hard-coded first-order ODE runtime before source-language execution.
- Explicit Euler as the first learning integrator and classical RK4 as the first accuracy comparison.
- Manually constructed expressions and typed IDs.
- Physical dimensions as semantic information.
- A minimal source language for parameters, states, initial conditions, expressions, and derivatives.
- Name resolution, semantic checks, lowering, and interpreted evaluation.
- Trajectory output with explicit time and state ordering.
- One loss function and one basic method for recovering damping from synthetic data.
- Independent numerical checks where they provide useful evidence.

The roadmap may refine interfaces as each milestone produces evidence.
It must not silently broaden this scope.

## Explicit non-goals

The initial programme does not include:

- Partial differential equations or spatial discretization.
- Rigid-body collision simulation or game physics.
- Adaptive, implicit, symplectic, or stiff solver families.
- A general optimizer or inference library.
- Production automatic differentiation.
- Native code generation, GPU execution, or distributed execution.
- Python bindings, WebAssembly bindings, or a visual studio.
- A general-purpose programming language.
- A stable public language, API, or serialized model format.

These can become later questions only after the interpreted vertical slice is correct and understood.

## Responsibility split

### Learner ownership

The learner designs, implements, and explains the logic that embodies the subject being studied:

- Dimension algebra and compatibility rules.
- Symbol tables and name resolution.
- Expression checking and typed lowering.
- Numerical stepping and runtime evaluation.
- Loss construction and optimization.
- Later differentiation logic.

Passing tests is necessary but not sufficient.
The learner should be able to explain the representation, algorithm, invariants, failure cases, and rejected alternatives.

### ChatGPT project role

The conceptual tutor may teach the mathematics and compiler concepts, compare designs, define behavioral contracts, challenge assumptions, and guide a milestone debrief.
It should use graduated hints before giving a complete solution.

### Codex role

The repository agent may inspect real files, prepare approved interfaces and TODO scaffolds, add ignored behavioral specifications, run checks, perform mechanical work, and review learner-written changes adversarially.
It must follow `AGENTS.md` and must not complete reserved logic without an explicit request.

## Evidence standard

Each milestone needs two forms of completion evidence:

1. The required behavior passes tests whose expected result is independently defensible.
2. The learner can explain why the implementation satisfies the contract and where it can fail.

For numerical work, tests must state the modeled system, parameter scale, time interval, step size, tolerance, and reason for the expected result.
A single plausible plot is not correctness evidence.

## Meaning of differentiable

In Størmer, differentiable will eventually mean that the effect of selected model parameters on a stated simulation result or loss can be computed and checked against an independent reference.
It does not currently promise that every language construct, solver, event, or backend supports gradients.

Finite differences should provide an early reference before automatic differentiation is trusted.
The project should document which inputs and outputs participate in differentiation, how numerical integration affects the result, and where gradients are undefined or unreliable.

## Working principle

Use project conversations to decide what the system must mean.
Use Codex to operate on the repository and protect the contracts.
Use learner-owned implementation to turn those meanings into understanding.
