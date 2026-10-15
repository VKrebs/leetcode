---
name: new
description: Scaffold a new LeetCode problem file (stub, statement, example tests) and log it as started.
argument-hint: <problem-id | slug> [rs|cs]
disable-model-invocation: true
---

Scaffold the problem so the learner can start solving. You write boilerplate only; the method body stays empty.

## Steps

1. **Fetch.** Run `node .claude/skills/new/fetch.mjs <id-or-slug>` from the repo root. If it fails or the problem is paid-only, ask the learner to paste the statement and signature, and continue from that.
2. **Pick the language** from the second argument: `rs` (default) or `cs`. If the file already exists in that language, stop and say so; the learner may want a re-attempt in a new method instead.
3. **Write the file** following the conventions in `CLAUDE.md`:
   - Top comment: problem number, title, URL, difficulty, then the statement converted from HTML to plain text (examples and constraints included).
   - The starter signature from `snippets.rust` / `snippets.csharp`, with a body that compiles but fails tests: `todo!()` in Rust, `throw new NotImplementedException();` in C#.
   - Support types the signature needs (e.g. `ListNode`, `TreeNode`) and test helpers that only convert data (e.g. vec to list).
   - One test per LeetCode example, named `example1`, `example2`… (Rust) or a `[Theory]` with one `[InlineData]` per example (C#). Only the examples, no extra cases.
4. **Register** a Rust file in `src/problems/mod.rs`, keeping the list sorted.
5. **Verify** it compiles: `cargo test --no-run` in `rust/` or `dotnet build` in `csharp/`.
6. **Log** a `started` row in `LEARNING.md` (see `.claude/tutor/learning-log.md`).
7. **Hand over:** give the file path and invite the learner to read the problem and say how they'd approach it. Mention only what's already in the statement; leave patterns and hints unspoken.

Done when the file compiles, its example tests fail on the stub, and the log row exists.
