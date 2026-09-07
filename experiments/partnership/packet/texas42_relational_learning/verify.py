#!/usr/bin/env python3
"""Exact finite checks for the relational-learning proposal.

Standard library only. These are generic decision-process checks, not Texas 42
benchmarks and not tests of the repository's Rust implementation.
"""
from __future__ import annotations

import argparse
import json
import random
from dataclasses import dataclass
from fractions import Fraction as F
from pathlib import Path
from typing import Dict, Tuple

State = Tuple[int, ...]


def rational(x: F) -> str:
    return str(x)


@dataclass
class Game:
    horizon: int
    probability: Dict[Tuple[State, int], F]
    terminal: Dict[State, F]
    v: Dict[State, F]
    q: Dict[Tuple[State, int], F]
    lower: Dict[Tuple[State, int], F]
    upper: Dict[Tuple[State, int], F]

    @classmethod
    def random(cls, rng: random.Random, horizon: int = 3) -> 'Game':
        g = cls(horizon, {}, {}, {}, {}, {}, {})

        def build(s: State) -> F:
            if len(s) == horizon:
                value = F(rng.randrange(5), 4)
                g.terminal[s] = g.v[s] = value
                return value
            for a in (0, 1):
                p = F(rng.randrange(1, 4), 4)
                g.probability[s, a] = p
                vals = [build(s + (2 * a + o,)) for o in (0, 1)]
                q = p * vals[0] + (1 - p) * vals[1]
                g.q[s, a] = q
                g.lower[s, a] = max(F(0), q - F(rng.randrange(4), 10))
                g.upper[s, a] = min(F(1), q + F(rng.randrange(4), 10))
            g.v[s] = max(g.q[s, a] for a in (0, 1))
            return g.v[s]

        build(())
        return g

    def check_policy(self, choices: Dict[State, int]) -> tuple[F, F, F]:
        def evaluate(s: State) -> tuple[F, F, F]:
            if s in self.terminal:
                return self.terminal[s], F(0), F(0)
            a = choices[s]
            p = self.probability[s, a]
            children = [evaluate(s + (2 * a + o,)) for o in (0, 1)]
            continuation = tuple(p * children[0][j] + (1 - p) * children[1][j]
                                 for j in range(3))
            regret = self.v[s] - self.q[s, a]
            # Clipping to 1 is valid because terminal utility is in [0,1].
            cost = min(F(1), max(self.upper[s, b] for b in (0, 1))
                       - self.lower[s, a])
            assert F(0) <= regret <= cost <= F(1)
            return continuation[0], continuation[1] + regret, continuation[2] + cost

        value, true_cost, bounded_cost = evaluate(())
        assert self.v[()] - value == true_cost, 'performance-difference identity'
        assert self.v[()] - value <= bounded_cost, 'interval regret certificate'
        return value, true_cost, bounded_cost


def sample_fitting_counterexample() -> dict:
    # h is an observed starting-context identifier. c=h%2 is a reusable visible
    # feature; s is an irrelevant public observation. The hidden outcome y equals
    # c with probability 3/4, independently of s. Correctly guessing y pays 1.
    # One training draw per exact (h,s) lets a table fit noise perfectly.
    training = [(h, s, h % 2 if s % 4 != 3 else 1 - h % 2)
                for h in range(4) for s in range(4)]
    table = {(h, s): y for h, s, y in training}
    table_train = F(sum(table[h, s] == y for h, s, y in training), len(training))
    rule_train = F(sum(h % 2 == y for h, s, y in training), len(training))
    # Test has completely unseen h and s. Integrate its true hidden distribution.
    test = [(h, s) for h in range(4, 8) for s in range(4, 8)]
    table_test = F(0)
    rule_test = F(0)
    for h, s in test:
        c = h % 2
        for y, mass in ((c, F(3, 4)), (1-c, F(1, 4))):
            table_test += mass * (table.get((h, s), 0) == y) / len(test)
            rule_test += mass * (c == y) / len(test)
    assert (table_train, rule_train, table_test, rule_test) == (F(1), F(3,4), F(1,2), F(3,4))
    # On each empirical singleton posterior any centered feature is zero;
    # the empirical optimum and its perfect dual both equal one. This does not
    # transfer to the true posterior, whose lawful optimum is 3/4.
    for h, s, y in training:
        empirical_values = [F(a == y) for a in (0, 1)]
        empirical_penalties = [q-q for q in empirical_values]
        assert max(q-p for q,p in zip(empirical_values, empirical_penalties)) == 1
    return {
        'scope': 'synthetic one-choice example; not a 42 benchmark',
        'training_observations': len(training),
        'table_training': rational(table_train), 'shared_rule_training': rational(rule_train),
        'table_unseen_contexts': rational(table_test), 'shared_rule_unseen_contexts': rational(rule_test),
        'empirical_optimum_and_exact_empirical_dual': '1',
        'true_conditional_lawful_optimum': '3/4',
    }


def coarse_center_counterexample() -> dict:
    # X is PUBLIC. The player may choose A=X, giving true value one.
    # Every action's unconditional mean reward is 1/2. Subtracting reward-1/2
    # gives an invalid 'dual' of 1/2 because those centers ignored public X.
    apparent_dual = F(0)
    proper_dual = F(0)
    lawful_selected_penalty = F(0)
    for x in (0, 1):
        rewards = [F(a == x) for a in (0, 1)]
        coarse = [r - F(1, 2) for r in rewards]
        proper = [r-r for r in rewards]
        apparent_dual += max(r-p for r,p in zip(rewards, coarse)) / 2
        proper_dual += max(r-p for r,p in zip(rewards, proper)) / 2
        lawful_selected_penalty += coarse[x] / 2
    assert apparent_dual == F(1,2) < proper_dual == F(1)
    assert lawful_selected_penalty == F(1,2)
    return {
        'true_lawful_value': '1', 'incorrect_pooled_center_upper': rational(apparent_dual),
        'correct_information_conditional_upper': rational(proper_dual),
        'lawful_policy_mean_of_incorrect_penalty': rational(lawful_selected_penalty),
    }


def tied_label_counterexample() -> dict:
    q = [(F(90,100), F(89,100)), (F(89,100), F(90,100))]
    teacher_labels = [max(range(2), key=lambda a: row[a]) for row in q]
    regrets = [max(row)-row[0] for row in q]
    assert teacher_labels == [0,1]
    assert max(regrets) == F(1,100)
    return {'different_exact_teacher_labels': teacher_labels,
            'constant_action_maximum_regret': '1/100',
            'point': 'small regret does not require copying every teacher label'}


def run(seed: int, games: int, policies: int) -> dict:
    rng = random.Random(seed)
    checked = 0
    for _ in range(games):
        game = Game.random(rng)
        decision_states = [s for s in game.v if s not in game.terminal]
        for _ in range(policies):
            choices = {s: rng.randrange(2) for s in decision_states}
            game.check_policy(choices)
            checked += 1
    return {
        'status': 'PASS', 'seed': seed,
        'scope': 'independent exact-rational finite mathematics; no native Rust or Texas 42 training run',
        'random_games': games, 'policies_per_game': policies, 'policy_checks': checked,
        'horizon': 3, 'terminal_utility_range': '[0,1]',
        'checks': ['exact performance-difference identity', 'action-interval regret upper',
                   'sample fitting can coexist with zero empirical dual gap',
                   'pooling public information can invalidate an information price',
                   'label disagreement need not force policy complexity'],
        'sample_fitting': sample_fitting_counterexample(),
        'coarse_centering': coarse_center_counterexample(),
        'label_disagreement': tied_label_counterexample(),
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--seed', type=int, default=420907)
    parser.add_argument('--games', type=int, default=128)
    parser.add_argument('--policies', type=int, default=64)
    parser.add_argument('--output', type=Path, default=Path(__file__).with_name('results.json'))
    args = parser.parse_args()
    if args.games < 1 or args.policies < 1:
        parser.error('games and policies must be positive')
    result = run(args.seed, args.games, args.policies)
    text = json.dumps(result, indent=2)
    args.output.write_text(text + '\n', encoding='utf-8')
    print(text)


if __name__ == '__main__':
    main()
