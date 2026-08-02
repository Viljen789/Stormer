# Product direction

## Target user problem

Scientists, engineers, students, and technical product teams often express a physical model in one tool, simulate it in another, fit measurements with custom scripts, and then rebuild parts of the workflow for deployment.
That fragmentation makes units, assumptions, parameter provenance, and numerical behavior difficult to inspect together.

Størmer aims to let a user describe a dynamical system close to its governing equations, validate the model semantically, compare its behavior with measurements, and carry the same validated model toward deployment.
The product should make the path from equations to evidence understandable without hiding the numerical choices that affect the result.

## Model-calibration workflow

The first complete product workflow is planned as:

```text
equations + measurement data
    -> validate dimensions and model structure
    -> run a forward simulation
    -> compare predicted and observed trajectories
    -> estimate one or more unknown physical parameters
    -> inspect the fit and its assumptions
```

This workflow is the organizing constraint for early language, IR, runtime, and interface decisions.
It is not implemented by the current scaffold.

## Forward simulation

Forward simulation starts from a validated model, known parameters, initial conditions, and a requested time range.
The runtime will eventually integrate the state equations and produce a trajectory with enough metadata to explain how it was computed.
Early work will favor explicit and testable numerical semantics over a wide solver catalog.

## Parameter inference

Parameter inference will treat selected physical parameters as unknowns and compare simulated observables with measurement data.
The early product target is a narrow calibration problem whose result can be checked against known synthetic or analytical cases.
General Bayesian inference, large optimizer catalogs, and unconstrained black-box fitting are outside the first slice.

## Sensitivity analysis

Sensitivity analysis will eventually explain how simulated outputs and fit quality change with model parameters.
It is important both for gradient-based fitting and for helping users understand which parameters measurements can actually identify.
The implementation strategy remains open until interpreted runtime semantics and finite-difference reference checks are reliable.

## Future deployment targets

The Størmer Engine is intended to support several product surfaces without duplicating model semantics:

- A native `stormer` command-line interface for checking, simulating, fitting, and compiling models.
- Python-facing experiments and later integrations for reference analysis and scientific workflows.
- WebAssembly delivery for a future browser-based Størmer Studio.
- Shareable Størmer Models that package validated, executable physical models for another application.

No Python binding, WebAssembly crate, browser application, or native code generator exists in this scaffold.

## Why this is a product

A compiler alone can translate notation, but the user problem continues through diagnostics, trustworthy numerical execution, calibration against evidence, inspection of results, and deployment.
Størmer therefore treats the language and compiler as part of a coherent modeling product rather than as the final outcome.
Størmer Studio and Størmer Models are possible product surfaces around the same validated engine, not separate semantic implementations.

## Initial non-goals

The initial product is not intended to provide:

- Rigid-body collision simulation or a game-physics engine.
- Partial differential equations or spatial discretization.
- GPU execution or distributed simulation.
- A general-purpose programming language.
- Arbitrary I/O, concurrency, an object system, or general mutation in model files.
- Every integration method, optimization algorithm, or inference technique.
- A stable language, library API, or deployment format before the first vertical slice is tested.
