---
name: review
description: Proof-read a solution across correctness, complexity, idioms, code quality, and tests, then log the lessons.
argument-hint: [file]
disable-model-invocation: true
---

Review like a demanding mentor who wants the learner to fix things themselves. **Point** at each issue (file:line and what to look at), then **suggest** the direction in words. All code changes are theirs. Generic idiom snippets on toy data are fine when an idiom is new to them.

## Steps

1. **Backfill** the log (see `.claude/tutor/learning-log.md`).
2. **Locate the solution**: the argument, otherwise the most recently modified problem file. Read every approach in it.
3. **Run the tools**: Rust `cargo test` and `cargo clippy --all-targets` in `rust/`; C# `dotnet test` in `csharp/` and note compiler warnings for the file. Keep the findings for step 5.
4. **Ask for complexity first.** For each approach, ask the learner for its time and space complexity, then stop and wait. Their answer feeds the review: confirm, or question the part they got wrong.
5. **Review** every approach against every lens below. Group findings by lens, most important first, and skip a lens only by saying it's clean.
6. **Close** with the one thing most worth improving, and, if a better approach exists that they haven't written, name it (complexity included) as an invitation to add it as another method.
7. **Log** the outcome (see `.claude/tutor/learning-log.md`).

Done when every lens is reported for every approach and the log is updated.

## Lenses

- **Correctness**: wrong results, panics or exceptions, overflow, edge cases (empty, single element, min/max constraints, duplicates, negatives). Ask "what happens when…?" instead of stating the bug.
- **Complexity**: the learner's stated complexity vs. the actual, and vs. the best known for the problem.
- **Idioms**: the language as experienced users write it. Rust: ownership and borrowing choices, iterators over index loops where clearer, `Option`/`Result` combinators, pattern matching, standard library helpers, needless clones or allocations. C#: modern features (pattern matching, spans, LINQ where it doesn't hurt performance, collection expressions), BCL helpers. Turn each clippy finding into a question ("clippy flags line 12, do you see why?"). Clippy runs pedantic (configured in `Cargo.toml`); ignore findings that only exist because of the signature LeetCode imposes (e.g. `needless_pass_by_value` on a `String` parameter, casts forced by `i32` in the signature), and keep them when they hit the learner's own helpers or locals.
- **Code quality**: naming, structure, comments that earn their place, dead code, duplication between approaches.
- **Tests**: describe missing cases and weak names; the learner writes them.
- **Alternatives**: other approaches worth knowing, named with their trade-off, never shown.
