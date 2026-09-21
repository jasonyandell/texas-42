# Scenario comparison

Selected 30 → 27; 3 added, 6 removed, 24 retained.

Among 206 coordinates valued in both runs, 87 changed action values and 35 changed the set of best actions. 25 changed the helpful/harmful/tied relationship of the query.

These are conditional continuation values. Full support is exact for the frozen players; sampled support is an estimate. Neither is a general game-strength measurement.

| Coordinate | Before | After | Best actions before → after | Selected before → after |
|---|---|---|---|---|
| 04a6a6c59bdffd126196 | harmful | tied | 3-3 → 2-0, 3-3, 5-0 | False → False |
| 145b1cf63a8e25fec162 | helpful | harmful | 5-5 → 3-0 | False → False |
| 1f1e45a0cf8d173d23bc | tied | harmful | 1-1, 5-0, 6-0 → 1-1 | False → False |
| 200b6fb4941015bee378 | harmful | harmful | 3-3 → 0-0 | False → False |
| 276db3a63b465835b3be | harmful | tied | 1-1 → 1-1, 5-1, 5-5 | False → False |
| 3439620274f17d0f8c77 | helpful | tied | 4-1 → 4-1, 5-1, 5-5 | True → False |
| 455fea5ff3390c7cff21 | helpful | harmful | 5-0 → 5-3 | True → False |
| 45cf4f207158b3beea0c | harmful | tied | 5-1 → 4-1, 5-1, 5-5 | False → False |
| 4e622be9d95aab30ddb5 | harmful | harmful | 4-1 → 2-2, 4-1 | False → False |
| 5205829a5516f9e189d5 | harmful | harmful | 5-2 → 5-2, 6-6 | False → False |
| 58370567ecdecc49590b | harmful | harmful | 5-4, 6-5 → 5-4 | False → False |
| 5d94a343252b3d621691 | tied | harmful | 5-0, 6-2 → 6-2 | False → False |
| 648a5c735799a908b433 | harmful | harmful | 5-1 → 2-0 | False → False |
| 6ab1b9e33f28eac0c84d | harmful | helpful | 6-0 → 6-4 | False → True |
| 7233a7eb833c0818d6c0 | tied | harmful | 3-0, 3-2 → 2-2 | False → False |
| 72de903a82cc8846b2e0 | tied | helpful | 3-2, 6-3 → 3-2 | False → True |
| 74923af0be9db8f936af | tied | harmful | 6-2, 6-4 → 6-2 | False → False |
| 85bf49d4a066732092c8 | harmful | harmful | 1-0 → 3-3 | False → False |
| 8f15e49aff34c6ece47d | helpful | harmful | 3-2 → 2-0 | True → False |
| 91da83ab9625e461add3 | harmful | harmful | 3-0 → 0-0 | False → False |
| 92678c873174b8fcc352 | helpful | tied | 6-4 → 6-4, 6-5 | False → False |
| 9cb073ff4702802a69dc | helpful | harmful | 6-4 → 4-0 | True → False |
| 9ee04a84ec113255fbd6 | tied | helpful | 3-2, 6-2 → 3-2 | False → False |
| a5a4cc385d9152df1c08 | harmful | harmful | 3-0 → 0-0, 3-0 | False → False |
| a730058a7c8eccb1c53b | harmful | tied | 5-4 → 5-1, 5-4, 5-5 | False → False |
| be5bc06fe19a7be618aa | harmful | harmful | 1-1, 2-0 → 2-0 | False → False |
| d26ed09ae23d9d482f34 | tied | helpful | 3-2, 6-0 → 3-2 | False → True |
| e936c0348679be661b3a | helpful | harmful | 6-4 → 6-1 | True → False |
| ec2da87edef9cc62cef1 | harmful | harmful | 2-2 → 2-2, 5-3 | False → False |
| ec85e1fc54aae89d8bea | tied | harmful | 1-1, 4-4, 6-4 → 4-4 | False → False |
| effadcf3da60a30ddada | tied | harmful | 5-3, 5-5 → 6-5 | False → False |
| f32b8d34e3a17ee76ea2 | helpful | harmful | 6-4 → 6-0 | True → False |
| f65f3e8ba7d881687624 | tied | harmful | 5-3, 6-4 → 5-3, 5-4 | False → False |
| fa50fe35c2a0660069b4 | harmful | tied | 5-1 → 4-1, 5-1, 5-5 | False → False |
| fee22b837128a76acdcf | helpful | tied | 6-4 → 6-4, 6-5 | False → False |

## Paired witness: advantage-11 (6ab1b9e33f28eac0c84d)

harmful → helpful. Preferred 6-4, comparison 6-0; 3 saved worlds, 1 lost worlds.

Saved world, remaining hands: 5-0 5-1 / 4-2 5-5 / 5-4 6-0 6-4 / 2-1 4-1 5-2.
- preferred: 2:6-4 3:2-1 0:5-1 1:5-5 2:5-4 3:5-2 1:4-2 2:6-0 3:4-1 0:5-0; final team points [31, 11]; success True.
- comparison: 2:6-0 3:5-2 0:5-1 1:5-5 2:5-4 3:4-1 1:4-2 2:6-4 3:2-1 0:5-0; final team points [26, 16]; success False.

Lost world, remaining hands: 5-0 5-2 / 2-1 5-5 / 5-4 6-0 6-4 / 4-1 4-2 5-1.
- preferred: 2:6-4 3:5-1 0:5-0 1:5-5 2:5-4 3:4-1 1:2-1 2:6-0 3:4-2 0:5-2; final team points [21, 21]; success False.
- comparison: 2:6-0 3:4-1 0:5-2 1:5-5 2:5-4 3:5-1 1:2-1 2:6-4 3:4-2 0:5-0; final team points [31, 11]; success True.

## Paired witness: advantage-12 (72de903a82cc8846b2e0)

tied → helpful. Preferred 3-2, comparison 6-3; 4 saved worlds, 1 lost worlds.

Saved world, remaining hands: 3-2 6-1 6-3 / 2-2 4-1 6-2 / 5-0 6-6 / 6-0 6-4.
- preferred: 0:3-2 1:6-2 2:6-6 3:6-4 0:6-1 1:2-2 2:5-0 3:6-0 0:6-3 1:4-1; final team points [31, 11]; success True.
- comparison: 0:6-3 1:6-2 2:6-6 3:6-4 0:6-1 1:2-2 2:5-0 3:6-0 0:3-2 1:4-1; final team points [26, 16]; success False.

Lost world, remaining hands: 3-2 6-1 6-3 / 2-2 4-1 6-6 / 6-0 6-2 / 5-0 6-4.
- preferred: 0:3-2 1:6-6 2:6-2 3:6-4 0:6-1 1:4-1 3:5-0 0:6-3 1:2-2 2:6-0; final team points [26, 16]; success False.
- comparison: 0:6-3 1:2-2 2:6-2 3:6-4 0:6-1 1:6-6 1:4-1 2:6-0 3:5-0 0:3-2; final team points [31, 11]; success True.

## Paired witness: advantage-22 (d26ed09ae23d9d482f34)

tied → helpful. Preferred 3-2, comparison 6-0; 33 saved worlds, 30 lost worlds.

Saved world, remaining hands: 2-2 4-3 / 5-0 6-4 / 3-2 5-5 6-0 / 5-2 5-3 5-4.
- preferred: 2:3-2 3:5-3 0:2-2 1:5-0 2:5-5 3:5-2 0:4-3 1:6-4 2:6-0 3:5-4; final team points [31, 11]; success True.
- comparison: 2:6-0 3:5-2 0:2-2 1:5-0 2:3-2 3:5-3 0:4-3 1:6-4 2:5-5 3:5-4; final team points [21, 21]; success False.

Lost world, remaining hands: 2-2 5-0 / 5-2 5-4 / 3-2 5-5 6-0 / 4-3 5-3 6-4.
- preferred: 2:3-2 3:5-3 0:2-2 1:5-2 2:5-5 3:4-3 0:5-0 1:5-4 2:6-0 3:6-4; final team points [26, 16]; success False.
- comparison: 2:6-0 3:5-3 0:2-2 1:5-2 2:3-2 3:4-3 0:5-0 1:5-4 2:5-5 3:6-4; final team points [42, 0]; success True.
