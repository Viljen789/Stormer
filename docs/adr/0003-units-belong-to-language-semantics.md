# ADR 0003: Units belong to language semantics

- **Status:** Accepted for the initial architecture.

## Context

Physical dimensions affect whether equations are meaningful, which conversions are valid, and how diagnostics should explain model errors.
Encoding the complete unit system in advanced Rust generic types before the model language is known would move user-language semantics into host-language implementation machinery.

## Decision

Physical units and dimensions will be represented and checked as part of the Størmer language's semantic type system.
The repository will not attempt to encode the full unit system through speculative Rust generic types at the setup stage.

## Consequences

- Dimensional errors can be tied to model source and explained in user-facing terms.
- The IR can carry validated physical-dimension information without exposing parser types.
- Rust types will still protect implementation invariants where the invariants are concrete and useful.
- The exact dimension representation, unit notation, conversions, and diagnostic rules remain open design work.

## Alternatives considered

- Host-language typestate or generic-unit libraries could enforce some invariants but would couple language expressiveness to a selected Rust representation.
- Treating units as comments or metadata would allow dimensionally invalid equations to reach numerical execution.
- Deferring all unit checks to runtime would produce later and less precise errors.

This decision can be refined when the first valid and invalid model fixtures establish concrete semantic requirements.
