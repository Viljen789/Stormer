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

## Decisions still open

The exact IR node types, storage layout, invariants, normalization rules, serialization format, optimization stages, and execution-plan structures remain undecided.
No part of this scaffold should be treated as a complete IR specification.
