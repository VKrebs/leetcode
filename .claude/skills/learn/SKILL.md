---
name: learn
description: Learn an algorithmic concept or pattern through a Socratic lesson, then get practice problems.
argument-hint: <topic>
disable-model-invocation: true
---

Teach the topic as a **lesson**, built up through dialogue, and end with practice. The learner reasons, you guide.

## Steps

1. **Probe the starting point**: ask what they already know about the topic, and check `LEARNING.md` for related patterns. Wait for the answer.
2. **Calibrate depth.** New to it → start from the motivating problem and first principles, with more explanation per step. Familiar but rusty → move fast to the core idea and its variants.
3. **Build up Socratically**, one idea per turn: pose a small question on a toy example (not a LeetCode problem), let them answer, correct or confirm, then build the next idea on theirs. Cover the core idea, when to recognise it, the standard variants, and typical complexity. Explain directly whenever they ask for it or are clearly lost; dialogue is the default, not a rule.
4. **Check understanding**: ask them to explain the concept back in their own words, or to sketch how it would apply to a new toy case.
5. **Assign practice**: 3–5 problems from easiest to hardest, preferring NeetCode 150 problems for the pattern and skipping ones already in `LEARNING.md`. Give for each the number, title, difficulty, and one line on what it trains, with no hint at the solution. Verify each number and title with `node .claude/skills/new/fetch.mjs <id>` before listing it. Suggest `/new <id>` to start.
6. **Log** the pattern as `practising` (see `.claude/tutor/learning-log.md`).

Done when understanding has been checked, the practice list is verified, and the log is updated.
