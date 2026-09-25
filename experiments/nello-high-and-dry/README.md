# Nel-O: what does a dry 6-5 cost?

**EXPLORATORY** — below every evidentiary tier; sampled, heuristic policies,
cited by nothing. Rules: the doubles-suit Nel-O of PR #90
(`walt/math/SUIT_ALGEBRA_PURE.md` §§3–6 on `codex/nello-player`): doubles are
their own unpowered suit, a mixed lead is its high pip, highest follower wins,
declarer leads, partner sits out with 7 hidden tiles, first trick won = set.
The rules are re-implemented independently here (`cargo test` pins them); the Walt
player is not used.

Why the 6-5 hurts: doubles are their own suit, so the 6-5 is the **top six and
the top five**. With no other six or five, any six or five that is led before you
can discard the 6-5 (you need to be void in some other led suit) wins you the trick.

## Results (2026-09-25)

Each flawed hand is compared with its twin: the same hand holding the **5-0**
instead of the 6-5.

| Hand | Defense: perfect (double dummy, 20k deals) | Blind defense (1M games) | Random defense (1M games) |
|---|---|---|---|
| **6-5** 0-0 1-1 1-0 2-0 2-1 3-0 | 31/20000 = 0.2% | 29.5% | 34.3% |
| 5-0 0-0 1-1 1-0 2-0 2-1 3-0 | 20000/20000 = 100% | 99.9% | 99.9% |
| **6-5** 0-0 1-0 2-0 3-0 4-0 2-1 | 8/20000 = 0.04% | 23.8% | 27.7% |
| 5-0 0-0 1-0 2-0 3-0 4-0 2-1 | 20000/20000 = 100% | 99.99% | 99.99% |
| **6-5** 1-0 2-0 2-1 3-0 3-1 3-2 | 369/20000 = 1.8% | 50.4% | 52.5% |
| 5-0 1-0 2-0 2-1 3-0 3-1 3-2 | 20000/20000 = 100% | 99.9% | 99.9% |

Models:
- **Double dummy**: every hand visible, defenders and declarer perfect; deals
  sampled uniformly (seeded), each solved exactly.
- **Blind**: defenders can't see the declarer's hand, so their leads are uniformly
  random, but they follow correctly (never overtake a winning declarer tile, else
  play low).
- **Random**: defenders play a uniformly random legal tile (the random arm of the
  PR #90 defense screen).
- The declarer in the blind and random models sees only its own hand: it ducks
  with its highest safe tile, plays its lowest follower if it can't duck, and
  discards its biggest tile (the 6-5 first) when void. The best of the seven
  opening leads is reported (common random numbers, so the choice is lightly
  optimistic).

Run as `results/*.txt`. Reproduce:

```sh
cargo test --release
cargo run --release -- ddsample 6-5,0-0,1-1,1-0,2-0,2-1,3-0 20000
cargo run --release -- blind    6-5,0-0,1-1,1-0,2-0,2-1,3-0 1000000
```
