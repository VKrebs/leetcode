---
name: spoil
description: Reveal the full solution to the current problem, compare it with your attempt, and log it for a re-attempt.
argument-hint: [problem]
disable-model-invocation: true
---

The learner chose to see the answer. Make it count as a **lesson**: the goal is that they could re-derive it next week.

## Steps

1. **Locate the problem**: the argument, otherwise the most recently modified problem file. Read their attempt, if any.
2. **Reveal the key insight first**, in one or two sentences: the idea everything else follows from.
3. **Show the solution** in the file's language, idiomatic, in chat only; their file stays theirs to edit. State its time and space complexity and walk through why it works on one example.
4. **Compare with their attempt**: where it was right, the exact point where it diverged, and what signal in the problem could have led them to the insight. With no attempt, say what to look for next time.
5. **Log** it as spoiled (see `.claude/tutor/learning-log.md`), noting the missing insight.
6. **Suggest** they write it themselves from memory later as a re-attempt (it will come back via `/next`).

Done when insight, solution, comparison and log entry are all delivered.
