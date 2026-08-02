# ADR 0002: Interpret before native code generation

- **Status:** Accepted for the initial architecture.

## Context

Equation semantics, normalization rules, runtime errors, and numerical behavior must be understood before multiple execution backends can be compared meaningfully.
A code generator introduced too early could hide semantic ambiguity behind backend complexity.

## Decision

The first runtime backend will interpret normalized equation IR using `f64` values.
Native code generation will be considered only after interpreter semantics are tested and stable.

## Consequences

- The interpreter becomes a readable reference for equation execution.
- Early tests can focus on semantic correctness rather than compiler-backend details.
- Initial performance may be lower than generated native code.
- A future backend must demonstrate agreement through interpreter-versus-codegen differential tests.

## Alternatives considered

- Generating native code first could improve peak performance but would combine semantic design with backend and toolchain decisions.
- Targeting WebAssembly first could support an early browser demo but would prematurely create a deployment boundary.
- Building an interpreter and code generator together would multiply the behavior that must be debugged before a reference exists.

This decision should be revisited when representative models and measurements expose an actual execution bottleneck.
