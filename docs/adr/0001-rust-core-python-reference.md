# ADR 0001: Rust core with Python reference work

- **Status:** Accepted for the initial architecture.

## Context

Størmer needs one implementation language for compiler invariants, numerical runtime behavior, portable deployment, and a future command-line product.
It also needs an independent environment for mathematical experiments, trajectory comparisons, gradient checks, and notebooks.

## Decision

The compiler and runtime core will be implemented in stable Rust.
Python will later be used for reference implementations and experiments, but it is not a production dependency and no Python binding is created by this decision.

## Consequences

- The initial semantic and runtime contracts have one production implementation language.
- Rust can support native and future WebAssembly deployment from the same core.
- Python comparisons can provide independent numerical evidence instead of sharing the same implementation path.
- Cross-language packaging and bindings remain future decisions driven by a concrete product workflow.

## Alternatives considered

- A Python-first production core would speed up some experiments but would make portable runtime and deployment constraints a later rewrite risk.
- A mixed Rust and C++ core would add build and ownership boundaries before a selected library justifies them.
- Rust-only experiments would reduce tooling variety but would weaken the independence of numerical reference checks.

This decision can be revisited if measured workflows show that the boundary fails its intended purpose.
