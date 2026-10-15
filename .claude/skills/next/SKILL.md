---
name: next
description: Recommend what to solve next, based on the learning log and the NeetCode 150 path.
argument-hint: [pattern or constraint, e.g. "rust" or "dp"]
disable-model-invocation: true
---

Recommend the next problems so practice follows a coherent path instead of random picks.

## Steps

1. **Backfill** the log (see `.claude/tutor/learning-log.md`), then read `LEARNING.md`.
2. **Choose candidates** in this priority order, honouring any argument as a filter:
   1. A `spoiled` problem at least a few days old, for a re-attempt.
   2. A pattern marked `practising` that still needs L3+ hints, or appears in Recurring feedback.
   3. The next unsolved NeetCode 150 problem, following its category order (the Patterns table order), easy before medium before hard.
   When the NeetCode list has no fitting problem, fall back to a well-known problem for the pattern.
3. **Verify** each pick's number and title with `node .claude/skills/new/fetch.mjs <id>`.
4. **Present** 1–3 options: number, title, difficulty, suggested language, and one line on *why this one now* (which gap it fills). State the pattern only for re-attempts and explicitly requested patterns; otherwise recognising the pattern is part of the exercise. Suggest `/new <id>`.

Done when every option is verified and carries its reason.
