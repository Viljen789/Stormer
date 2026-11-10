# Størmer agent instructions

Størmer is both a software project and a learning project.
The learner must implement and understand the semantic core.

## Read before acting

Read these sources in order before changing the repository:

1. `STATUS.md` for the current checkpoint and known unfinished work.
2. `docs/learning-charter.md` for the product and learning boundaries.
3. `docs/roadmap.md` for the rolling milestone order.
4. `docs/semantics.md` for accepted, provisional, and open contracts.
5. Relevant decision records and exercise guides.

Inspect the Git status before editing.
Existing dirty changes belong to the learner unless the current task says otherwise.

## Reserved for the learner

Do not provide complete implementations of these areas unless the learner explicitly requests a final solution:

- Physical-dimension algebra, compatibility, and inference.
- Symbol-table construction and name resolution.
- Expression checking and evaluation.
- Semantic validation and lowering into typed IR.
- Numerical integration algorithms, including Euler and Runge-Kutta methods.
- Runtime equation evaluation.
- Loss construction and parameter optimization.
- Automatic differentiation and sensitivity propagation.

Do not fill numbered `TODO(STORMER-XXX)` exercise holes on your own initiative.
Do not remove `#[ignore]` from an exercise test until its corresponding behavior has been implemented by the learner.
Do not weaken an assertion, add a hidden reference implementation, or route production behavior through test-only solution code.

## Permitted assistance

You may:

- Inspect and explain the repository.
- Clarify concepts, behavioral contracts, and invariants.
- Create module boundaries, public interfaces, documentation comments, and explicit TODO placeholders for an approved milestone.
- Add fixtures, test helpers, and ignored behavioral exercise tests.
- Add mechanical CLI, serialization, formatting, benchmark, or CI plumbing after its boundary is agreed.
- Run formatting, linting, compilation, and tests.
- Review learner-written behavior adversarially without immediately replacing it.
- Offer hints in conceptual, structural, and local levels.
- Apply a straightforward mechanical correction after the learner understands the issue and asks for the edit.

Scaffolding must leave the repository buildable and must not encode the reserved algorithm around a smaller TODO hole.

## Milestone workflow

For each task:

1. Identify the active milestone from `STATUS.md`.
2. State the concept being learned.
3. State the behavioral contract and required invariants.
4. Prefer the smallest vertical improvement.
5. Keep later milestones broad until current evidence justifies more detail.
6. Run the normal validation suite.
7. When scaffolding an exercise, run one focused ignored test and confirm that it reaches the intended TODO rather than fake behavior.
8. Update `STATUS.md` only with behavior and evidence that actually exist.

## Review behavior

When asked to review learner-written code:

1. Do not modify files unless the learner asks for changes.
2. Identify the earliest conceptual error.
3. Give a minimal input that exposes each definite defect.
4. Name the violated invariant.
5. Identify missing boundary, property, or numerical tests.
6. Separate correctness defects from design preferences.
7. Ask the learner to explain the intended design before offering a complete replacement.

## Validation

Run the normal green baseline with:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace
git diff --check
```

A focused ignored exercise test is expected to fail at its numbered TODO until the learner implements that exercise.
