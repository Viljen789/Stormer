# Contributing to Størmer

Størmer is currently an architectural scaffold, so contributions should preserve narrow boundaries and avoid presenting planned behavior as implemented behavior.

## Scope discipline

- Keep the workspace at four crates until a separate dependency or deployment boundary is demonstrated.
- Prefer a module in an existing crate over a new crate.
- Keep public APIs minimal and driven by a tested vertical slice.
- Do not add speculative traits, placeholder domain types, or broad abstractions.
- Keep the runtime independent of source-language and terminal concerns.
- Keep unsafe Rust forbidden unless a later decision record defines a small, documented boundary.
- Record product milestones in the roadmap instead of scattering future-work comments through source code.

## Making a change

1. Read the relevant architecture and decision records before changing a boundary.
2. Keep the change focused on one demonstrable capability or documentation decision.
3. Add tests that exercise the real behavior once behavior exists.
4. Update documentation when a recorded architectural decision changes.
5. Run the repository validation commands before requesting review.

## Validation

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace
```

The language, IR, runtime, and CLI APIs are not stable, so proposed interfaces should be justified by the current vertical slice rather than compatibility expectations.
