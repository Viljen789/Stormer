# Architecture

## Dependency graph

An arrow means that the package on the left depends on the package on the right.

```text
stormer-cli -> stormer-frontend -> stormer-ir
stormer-cli -> stormer-runtime  -> stormer-ir
```

The graph is intentionally acyclic and has one shared semantic boundary in `stormer-ir`.

## Package responsibilities

### `stormer-ir`

`stormer-ir` will eventually own language-independent and source-independent model structures.
Its scope includes stable node and value identifiers, physical dimension representations, scalar and state types, typed expressions, equations, parameters, state-variable declarations, observation declarations, model metadata, and future execution-plan structures.
It must not own parsing or terminal-facing diagnostics.

### `stormer-frontend`

`stormer-frontend` will eventually own source files and spans, lexing, parsing, syntax trees, diagnostic production, name resolution, unit and dimensional analysis, type checking, semantic validation, and lowering into `stormer-ir`.
It is the only initial package that understands how a model was written in source text.

### `stormer-runtime`

`stormer-runtime` will eventually own model instantiation, runtime state, equation evaluation, time integration, simulation trajectories, solver configuration, runtime errors, and future sensitivity and differentiation hooks.
It accepts validated IR and must not know how the source syntax was written.

### `stormer-cli`

`stormer-cli` will eventually read model and observation files, invoke the frontend, render diagnostics, invoke the runtime, select simulation actions, and write textual or structured output.
It must contain no compiler or solver logic.
The package builds the future `stormer` executable, which is currently a no-op.

## Dependency-direction rules

1. `stormer-ir` depends on no other workspace package.
2. `stormer-frontend` may depend on `stormer-ir`.
3. `stormer-runtime` may depend on `stormer-ir` but must not depend on `stormer-frontend`.
4. `stormer-cli` may coordinate `stormer-frontend` and `stormer-runtime`.
5. Compiler behavior belongs in the frontend, numerical behavior belongs in the runtime, and neither belongs in the CLI.
6. A new dependency edge requires a clear ownership reason and must preserve the acyclic graph.

## Planned data flow

The planned flow from source text to a trajectory is:

```text
model source text
    -> frontend source map and syntax tree
    -> resolved, dimension-checked, and typed representation
    -> normalized equation IR
    -> runtime model instantiation
    -> interpreted equation evaluation
    -> time integration
    -> simulation trajectory
    -> CLI or future product surface
```

Observation data will enter through the coordinating product surface and be connected to declared observables only after the model has been validated.

## Representation constraints

### Stable graph identity

IR graphs should use stable typed IDs and arena-like ownership rather than long-lived Rust references between nodes.
This keeps graph rewrites, indexing, serialization, diagnostics, and future execution planning possible without making object lifetimes the public model.

### Units are language semantics

Physical units belong to the model language's semantic type system.
The repository will not try to encode the entire unit system through complex Rust generic types before actual language requirements exist.

### Concrete numerical scalar

Initial numerical execution will use `f64`.
The architecture will not introduce a speculative scalar abstraction for `f32`, dual numbers, symbolic values, GPU values, or arbitrary precision before a concrete requirement demonstrates the necessary operations.

### Interpret before code generation

The initial backend will interpret the equation IR.
Native code generation will be considered only after the interpreted semantics are tested and stable.

### Safe boundaries

Unsafe Rust is forbidden across the initial workspace.
Any future unsafe code must be isolated behind a small, documented boundary with a concrete need and focused tests.

### Minimal public APIs

Public APIs should remain minimal until the first vertical slice shows which concepts must cross package boundaries.
Internal representations should not be made public merely to anticipate integrations.

### Modules before crates

New behavior should begin as a module within an existing package.
A module should become a crate only when it has a clear independent dependency boundary or deployment requirement.

## Why the runtime consumes IR instead of frontend types

Frontend types carry concerns such as tokens, source spans, recovery states, and syntax choices that are useful while explaining a model but are not the runtime's execution contract.
The IR boundary gives the runtime a validated, source-independent representation whose invariants can be tested directly.
This separation lets another frontend or a stored executable model eventually feed the same runtime without importing parser internals.
It also keeps numerical execution stable when the source language evolves.

Source information needed for user-facing explanations can survive through explicit provenance metadata without making the runtime depend on frontend data structures.

## Planned extension points

- IR normalization and execution planning can grow inside `stormer-ir` once one checked model establishes the required invariants.
- Solver implementations and sensitivity hooks can grow inside `stormer-runtime` behind concrete runtime interfaces.
- Reference experiments can live under `experiments/` without becoming production dependencies.
- A Python boundary can be added only when a real workflow defines ownership, data transfer, and packaging needs.
- A WebAssembly boundary can be added only when Størmer Studio has a concrete deployment requirement.
- Native code generation can be added behind the tested IR semantics without replacing the interpreter as the reference behavior.
- C or C++ integration can be isolated behind a narrow boundary if a selected mature library justifies it.

## Boundaries deliberately not created yet

The workspace does not contain separate parser, diagnostics, type-system, unit-system, optimizer, solver, autodiff, inference, code-generation, Python, WebAssembly, UI, or FFI crates.
Those names describe possible modules or future capabilities, not proven package boundaries.
The workspace also avoids framework choices for parsing, linear algebra, units, errors, command-line handling, autodiff, e-graphs, and code generation.

Keeping these boundaries absent makes the first vertical slice responsible for revealing the smallest useful interfaces.
