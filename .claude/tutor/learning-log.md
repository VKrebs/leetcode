# Learning log rules

How skills maintain `LEARNING.md`. The file's sections are the schema; keep their shape.

## Who writes what

| Event | Update |
|---|---|
| `/new` | Add a Problems row: status `started`, today's date, language, hints `-`. |
| `/hint` | Set the row's Hints to the highest ladder level reached (`L1`–`L4`), keep the higher value. |
| `/review` | Set status `solved`; fill Notes with one line (the key insight or the main lesson). Link the problem under each pattern it exercises and update pattern status. Add or bump Recurring feedback. Add newly learned idioms. |
| `/spoil` | Set status `spoiled` and Hints `L5`; Notes say what the missing insight was. The row stays a re-attempt candidate for `/next`. |
| `/learn` | Set the pattern to `practising` (add it if missing). |
| Re-attempt solved without spoil | Status `solved (re-attempt)`, keep the old Notes and append the new one. |

## Pattern status

- `not started`: no problem solved with it.
- `practising`: at least one problem, or still needing hints at L3+.
- `comfortable`: three or more problems solved with hints at L2 or below, the latest one with none.

## Recurring feedback

A remark goes here only when it shows up in a second, different problem. Format: `- <remark> (seen: NNNN, NNNN)`. Append problem IDs as it recurs; move it to a `Resolved` sub-list once three consecutive reviews no longer trigger it.

## Backfill

Before reading the log, compare it with the repo: list problem files under `rust/src/problems/` and `csharp/Problems/` (and any future language folder). Every file without a matching row gets one with status `solved (independent)`, its date from `git log --diff-filter=A --format=%as -- <file>` (today if uncommitted), Hints `-`, Notes `no tutor data`.

## Privacy

The log is published. Record skills, problems and mistakes only, never names, employers, or personal circumstances.
