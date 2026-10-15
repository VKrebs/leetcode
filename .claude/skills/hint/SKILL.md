---
name: hint
description: Get one graduated hint on the current problem, climbing a ladder from a guiding question to pseudocode.
argument-hint: [what you're stuck on]
disable-model-invocation: true
---

Give exactly **one** rung of help on the learner's current **sticking point**: the specific thing blocking them right now (defining the state, a base case, an off-by-one), as opposed to the problem as a whole.

## The ladder

| Level | Hint |
|---|---|
| L1 | A guiding question that points their attention somewhere useful. |
| L2 | The pattern or technique family ("this is a sliding window"). |
| L3 | The key insight or invariant that makes the pattern apply here. |
| L4 | Pseudocode for the hard part only, in prose-like steps, never in the target language. |

Past L4 the next step is `/spoil`; tell the learner that it exists and that it's their call.

## Steps

1. **Locate the problem**: from the argument, otherwise the most recently modified problem file (`git status`, then recent commits). Read the learner's current code.
2. **Find out what they've tried.** If neither the argument nor the conversation says, ask and stop there. This question replaces the hint for this turn.
3. **Name the sticking point** in one short phrase.
4. **Pick the level.** Same sticking point as the previous `/hint` in this conversation → one level up. Different sticking point, or first hint → L1, announced as "New sticking point: <phrase>. Back to L1." The learner can override either way ("same point", "new point").
5. **Give the hint** at that level, grounded in their code where possible ("look at what your loop does when `i == 0`"). End with a question that hands the thinking back.
6. **Log** the level reached (see `.claude/tutor/learning-log.md`).

Done when one rung is delivered, ends on a question, and is logged.
