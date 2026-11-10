# Contributing to Størmer

Størmer is currently a guided learning scaffold, so contributions should preserve narrow boundaries, protect learner-owned exercises, and avoid presenting planned behavior as implemented behavior.

Read `STATUS.md`, `docs/learning-charter.md`, and `AGENTS.md` before preparing a change.

## Scope discipline

- Keep the workspace at four crates until a separate dependency or deployment boundary is demonstrated.
- Prefer a module in an existing crate over a new crate.
- Keep public APIs minimal and driven by a tested vertical slice.
- Add interfaces and TODO placeholders only for the active approved milestone.
- Do not implement numbered exercise TODOs unless the learner explicitly asks for a final solution.
- Do not remove an ignored exercise test until its behavior is implemented.
- Do not add speculative traits, placeholder domain types, or broad abstractions outside the active milestone.
- Keep the runtime independent of source-language and terminal concerns.
- Keep unsafe Rust forbidden unless a later decision record defines a small, documented boundary.
- Record product milestones in the roadmap instead of scattering future-work comments through source code.

## Making a change

1. Read the relevant architecture and decision records before changing a boundary.
2. Keep the change focused on one demonstrable capability or documentation decision.
3. For a learning scaffold, add ignored behavioral specifications that reach an explicit numbered TODO.
4. For implemented behavior, unignore the relevant tests only after they pass without weakening their contract.
5. Update documentation when a recorded architectural decision changes.
6. Run the repository validation commands before requesting review.

## Validation

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace
git diff --check
```

The language, IR, runtime, and CLI APIs are not stable, so proposed interfaces should be justified by the current vertical slice rather than compatibility expectations.
