# Intermediate representation direction

The equation intermediate representation will sit between source-language meaning and numerical execution.
This document records the reason for each planned level without specifying concrete Rust data structures.

## Planned representation sequence

```text
source syntax
    -> syntax tree
    -> resolved and typed representation
    -> normalized equation IR
    -> future optimized IR
    -> runtime execution plan
```

## Why each level exists

### Source syntax

Source syntax is the user's written model and remains the authority for diagnostics and editing.
The exact grammar is intentionally undecided.

### Syntax tree

The syntax tree preserves written structure, tokens, and source ranges closely enough to report parse errors and support later tooling.
It may include incomplete or erroneous constructs that must never reach the runtime.

### Resolved and typed representation

The resolved and typed representation associates names with declarations and records validated scalar, state, and physical-dimension information.
It separates semantic checking from later equation normalization while retaining enough provenance to explain errors.

### Normalized equation IR

The normalized equation IR will express a valid model in a source-independent form suitable for evaluation and transformation.
It should make equations, states, parameters, observations, and model metadata explicit without carrying parser recovery states.

### Future optimized IR

An optimized IR may later record semantics-preserving rewrites that reduce repeated work or expose a better execution order.
It will not be introduced until the normalized IR and interpreter provide stable reference behavior.

### Runtime execution plan

A runtime execution plan may later arrange validated equations and storage for a selected execution strategy.
It should be derived from IR rather than from frontend syntax types.

## Diagnostic provenance

Source file identity, spans, declaration names, and useful origin relationships should survive semantic lowering where they can improve diagnostics and result explanations.
Provenance should be explicit metadata rather than a web of references back into frontend-owned syntax nodes.
The runtime should be able to execute valid IR without requiring a frontend source tree.

## Lowering higher-order equations

Second-order equations may later lower into first-order state systems by introducing explicit state variables for the original quantity and its derivative.
Many numerical integration methods operate on first-order systems, so this normalization can give multiple source forms one execution meaning.
The exact lowering rules must be defined and tested with the language semantics before they become an IR contract.

## Stable IDs and ownership

Graph nodes and values should eventually use stable typed IDs with arena-like ownership.
Stable IDs make it possible to reorder, rewrite, index, serialize, and annotate a graph without relying on long-lived Rust references between nodes.
They also make package boundaries and diagnostic provenance easier to state explicitly.

## Exercise 0001 scaffold

The first prepared guided exercise gives `stormer-ir` this module structure:

```text
dimension.rs   physical-dimension value shape and seven base categories
ids.rs         ParameterId, StateId, and ExprId newtypes
expression.rs  expression node variants and owned arena boundary
model.rs       declarations, derivatives, Model, and ModelBuilder
error.rs       structural lookup and model-construction errors
lib.rs         module declarations and deliberate public re-exports
```

The public structure includes `Dimension`, `BaseDimension`, the three typed IDs, `Expression`, `ExpressionArena`, `Parameter`, `State`, `DerivativeEquation`, `ModelBuilder`, `Model`, and `ModelError`.
The expression enum is deliberately limited to constants, declaration references, negation, addition, subtraction, multiplication, and division.
The model and declaration types expose stored data through read-only accessors.

Meaningful behavior remains unfinished behind `TODO(STORMER-001)` through `TODO(STORMER-013)`.
Those exercises cover dimension construction and composition, canonical formatting, arena allocation and lookup, expression validation, declaration allocation, derivative checks, model finalization, and programmatic harmonic-oscillator construction.
The associated behavioral tests compile but remain ignored during normal validation.

No complete model can be constructed without implementing the exercises.
The scaffold does not perform dimensional inference, parsing, source diagnostics, runtime evaluation, or simulation.

The policy for expression forward references remains deliberately unresolved.
The project owner must decide whether a valid arena may refer to a node inserted later and protect that decision with a focused test.
The exact long-term IR should continue to be informed by completed vertical slices rather than by the scaffold alone.

## Decisions still open

The current node types and owned vectors are a learning surface for the first model, not a final normalized-IR specification.
Long-term normalization rules, serialization format, optimization stages, execution-plan structures, and extension boundaries remain undecided.
No part of this scaffold should be treated as a stable public IR specification.
