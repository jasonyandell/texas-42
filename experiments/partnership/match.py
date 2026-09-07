#!/usr/bin/env python3
"""Create and report two-player mirrored matches; execute them with pool.py."""

import argparse
import json
import math
import statistics
from collections import Counter
from fractions import Fraction
from pathlib import Path

import campaign as c
from matchup import Player


def show_players(path, include_all=False):
    """Describe actual preset coordinates without changing player identity."""
    presets = json.loads(Path(path).read_text())
    names = (
        sorted(presets)
        if include_all
        else ("l1-default", "l2-partner-default", "l2-partner-voids")
    )
    families = {
        "baseline": "L1",
        "partner": "L2 Partner",
        "all-l1": "L2 All",
        "phone": "Phone reference",
    }
    print(
        "| Preset | Family | Root search | Modeled L1 search | Inner belief | Worlds: root / L0 / L1 |"
    )
    print("|---|---|---|---|---|---|")
    for name in names:
        p = Player(**presets[name])
        modeled = p.modeled_selection if p.mode in ("partner", "all-l1") else "—"
        root = "archived racing" if p.mode == "phone" else p.selection
        n1 = str(p.n1) if p.mode in ("partner", "all-l1") else "—"
        print(
            f"| {name} | {families[p.mode]} | {root} | {modeled} | {p.inner_belief} | {p.n} / {p.n0} / {n1} |"
        )
    print(
        "\nDefault means fixed search. Phone is a separate archived reference. Use --all for historical and advanced presets; see PLAYERS.md for naming and fallback semantics."
    )


def report(path):
    path = Path(path).resolve()
    spec = c.load(path, verify=False)
    if "players" not in spec:
        raise ValueError("not a two-player match")
    rows = c.complete_results(path, spec)
    width = spec["worlds_per_hand"] if spec["panel"] == "worlds" else 1
    complete = len(rows) // width * width
    scored = rows[:complete]
    groups = [scored[i : i + width] for i in range(0, complete, width)]
    units = [
        Fraction(sum(r["paired"]["seed_delta"] for r in group), width)
        for group in groups
    ]
    average = sum(units, Fraction()) / len(units) if units else None
    win_rate = (1 + average) / 2 if average is not None else None

    def ratio(value):
        return (
            {"numerator": value.numerator, "denominator": value.denominator}
            if value is not None
            else None
        )

    se = (
        statistics.stdev(float(x) / 2 for x in units) / math.sqrt(len(units))
        if len(units) >= 10
        else None
    )
    if se == 0:
        se = None  # Identical observed scores do not establish zero uncertainty.
    counts = Counter(r["paired"]["seed_delta"] for r in scored)
    runtime = c.summarize(path, spec)
    deals = [
        {
            "seed": row["seed"],
            "a_declaring_made": row["arms"]["declaring"]["made"],
            "b_declaring_made": row["arms"]["defending"]["made"],
            "pair_score": row["paired"]["seed_delta"],
            "in_complete_analysis_group": i < complete,
        }
        for i, row in enumerate(rows)
    ]
    result = {
        "campaign": spec["id"],
        "panel_id": spec["panel_id"],
        "players": spec["players"],
        "completed_pairs": len(rows),
        "scored_pairs": complete,
        "independent_units": len(units),
        "unit": "focal hand" if width > 1 else "deal",
        "a_pair_wins": counts[1],
        "a_pair_losses": counts[-1],
        "pair_ties": counts[0],
        "a_contract_wins": sum(r["paired"]["a_contract_wins"] for r in scored),
        "b_contract_wins": sum(r["paired"]["b_contract_wins"] for r in scored),
        "mean_pair_score": ratio(average),
        "a_contract_win_fraction": ratio(win_rate),
        "rough_two_se_win_fraction": [
            float(win_rate) - 2 * se,
            float(win_rate) + 2 * se,
        ]
        if se is not None
        else None,
        "runtime": runtime,
        "deals": deals,
        "groups": [
            {
                "first_seed": group[0]["seed"],
                "worlds": len(group),
                "pair_score": ratio(score),
                "a_pair_wins": sum(r["paired"]["seed_delta"] == 1 for r in group),
                "a_pair_losses": sum(r["paired"]["seed_delta"] == -1 for r in group),
                "pair_ties": sum(r["paired"]["seed_delta"] == 0 for r in group),
            }
            for group, score in zip(groups, units)
        ],
    }
    c.atomic(path / "MATCH.json", result)
    a, b = (spec["players"][side]["name"] for side in ("a", "b"))
    body = f"# {a} versus {b}\n\nEXPLORATORY. Bid 30, identical deals and contracts, player teams swapped.\n\n"
    body += f"Completed {len(rows)}/{spec['count']} mirrored deals; {complete} in complete analysis groups ({len(units)} independent {result['unit']} units).\n\n"
    body += f"A pair wins/losses/ties: **{counts[1]}/{counts[-1]}/{counts[0]}**. Contract wins: **{result['a_contract_wins']} A / {result['b_contract_wins']} B**.\n\n"
    if win_rate is not None:
        body += f"A contract win fraction: **{float(win_rate):.1%}**. Equal strength is 50%. Points never break ties.\n\n"
    if se is not None:
        lo, hi = result["rough_two_se_win_fraction"]
        body += f"Descriptive mean ±2 SE: {lo:.1%} to {hi:.1%}, clustered by {result['unit']}. This is a rough interval, not a formal equivalence or optional-stopping claim.\n\n"
    else:
        body += "No uncertainty interval is shown: the report requires at least ten independent units and nonzero observed variance for its rough approximation. Raw scores remain descriptive evidence.\n\n"
    body += (
        "| Player | Mean seconds / move | Fallbacks / nonforced |\n|---|---:|---:|\n"
    )
    for name, v in runtime["player_counts"].items():
        body += f"| {name} | {v['elapsed_us'] / 1e6 / v['moves']:.3f} | {v['fallbacks']} / {v['nonforced']} |\n"
    if runtime["stop"]:
        body += "\nStopped: " + runtime["stop"]["reason"] + "\n"
    if width > 1:
        body += "\n| Focal hand: first seed | Completions | A wins / losses / ties | Mean pair score |\n|---|---:|---:|---:|\n"
        for group in result["groups"]:
            score = group["pair_score"]
            body += f"| {group['first_seed']} | {group['worlds']} | {group['a_pair_wins']} / {group['a_pair_losses']} / {group['pair_ties']} | {score['numerator'] / score['denominator']:+.2f} |\n"
    body += "\n`MATCH.json` retains each seed's two make indicators and paired score, plus complete hand-group scores. A seed score is comparative against this opponent, not an absolute hand-difficulty or probability estimate.\n"
    body += "\nTiming includes the bounded wrapper and its fallback work. Completed deadline fallbacks remain in the score. Incomplete pairs/groups do not enter the primary score.\n"
    c.atomic(path / "MATCH.md", body)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    init = sub.add_parser("init")
    init.add_argument("path", type=Path)
    init.add_argument("--a", required=True, help="player name in players.json")
    init.add_argument("--b", required=True, help="player name in players.json")
    init.add_argument("--players", type=Path, default=c.HERE / "players.json")
    init.add_argument("--start", type=int, required=True)
    init.add_argument("--count", type=int, default=50)
    init.add_argument("--panel", choices=["random", "worlds"], default="random")
    init.add_argument("--worlds-per-hand", type=int, default=10)
    init.add_argument("--threads", type=int, default=6)
    init.add_argument("--cold", action="store_true")
    rep = sub.add_parser("report")
    rep.add_argument("path", type=Path)
    catalog = sub.add_parser(
        "players", help="show the main player families and exact settings"
    )
    catalog.add_argument(
        "--all", action="store_true", help="include historical and advanced presets"
    )
    catalog.add_argument("--players", type=Path, default=c.HERE / "players.json")
    args = parser.parse_args()
    if args.command == "players":
        show_players(args.players, args.all)
    elif args.command == "init":
        players = json.loads(args.players.read_text())
        result = c.initialize_match(
            args.path,
            players[args.a],
            players[args.b],
            args.start,
            args.count,
            args.panel,
            args.worlds_per_hand,
            args.threads,
            not args.cold,
        )
        print(json.dumps({"campaign": result["id"], "path": str(args.path)}))
    else:
        result = report(args.path)
        print(
            json.dumps(
                {
                    k: result[k]
                    for k in (
                        "completed_pairs",
                        "scored_pairs",
                        "a_pair_wins",
                        "a_pair_losses",
                        "pair_ties",
                        "a_contract_win_fraction",
                    )
                },
                indent=2,
            )
        )


if __name__ == "__main__":
    main()
