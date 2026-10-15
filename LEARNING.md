# Learning log

Progress through LeetCode practice, maintained with the help of the tutor setup in `.claude/`. Hint levels: L1 question, L2 pattern, L3 key insight, L4 pseudocode, L5 spoiled.

## Patterns

Backbone: the NeetCode 150 categories.

| Pattern | Status | Problems |
|---|---|---|
| Arrays & Hashing | practising | 0001, 0383, 1480, 1672 |
| Two Pointers | practising | 0876 |
| Sliding Window | not started | |
| Stack | not started | |
| Binary Search | not started | |
| Linked List | practising | 0876 |
| Trees | not started | |
| Heap / Priority Queue | not started | |
| Backtracking | not started | |
| Tries | not started | |
| Graphs | not started | |
| Advanced Graphs | not started | |
| 1-D Dynamic Programming | practising | 0070 |
| 2-D Dynamic Programming | not started | |
| Greedy | not started | |
| Intervals | not started | |
| Math & Geometry | practising | 0412, 1342 |
| Bit Manipulation | not started | |

## Problems

| # | Problem | Lang | Date | Status | Hints | Notes |
|---|---|---|---|---|---|---|
| 0001 | Two Sum | Rust | 2026-10-15 | solved | L2 | the inner loop searches for `target - x`; a value→index hash map makes that search O(1), so one pass gives O(n) time for O(n) space (check before insert so an element never pairs with itself) |
| 0070 | Climbing Stairs | C# | 2026-10-06 | solved (independent) | - | no tutor data; four approaches from naive recursion to rolling window |
| 0383 | Ransom Note | Rust | 2026-10-06 | solved (independent) | - | no tutor data; HashMap and fixed 26-array versions |
| 0412 | Fizz Buzz | Rust | 2026-10-06 | solved (independent) | - | no tutor data |
| 0876 | Middle of the Linked List | Rust | 2026-10-06 | solved (independent) | - | no tutor data; fast/slow pointers |
| 1342 | Number of Steps to Reduce a Number to Zero | Rust | 2026-10-06 | solved (independent) | - | no tutor data |
| 1480 | Running Sum of 1d Array | Rust | 2026-10-06 | solved (independent) | - | no tutor data |
| 1672 | Richest Customer Wealth | Rust | 2026-10-06 | solved (independent) | - | no tutor data |

## Recurring feedback

_None yet._

### Resolved

_None yet._

## Idioms learned

### Rust

- Integer conversions: `From`/`into` for lossless widening, `T::try_from(x)` plus `.expect("why it can't fail")` for narrowing or sign changes; `as` only when wrapping or truncation is wanted (0001, 0412)
- `HashMap::get` returns `Option<&V>`: `if let Some(&v) = map.get(&k)` looks up and copies in one step, instead of `contains_key` then indexing (0001)
- Destructure references in loop patterns: `for (i, &x) in v.iter().enumerate()` yields owned `Copy` values (0001)
- `#[expect(lint, reason = "...")]` over `#[allow]`: it warns when the suppression goes stale (0001)
- `with_capacity(n)` on `HashMap`/`Vec` when the final size is known (0001, 0412)

### C#

_None yet._
