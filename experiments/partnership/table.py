#!/usr/bin/env python3
"""Play one straight-42 hand locally with the experimental partner.

You sit at seat 0 and declare; your partner sits at seat 2. This is a play
table with a chosen contract, not an auction or a match-level bidding system.
"""
import argparse
import random

from player import decide
from rules import TILES, legal_tiles, winner, trick_points


def tile_name(tile):
    hi, lo = TILES[tile]
    return str(hi)+"-"+str(lo)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--bid", type=int, default=30)
    p.add_argument("--decl", type=int, choices=[0,1,2,3,4,5,6,7,9], help="0..6 pip trump; 7 doubles; 9 no trump")
    p.add_argument("--deal-seed", type=int)
    p.add_argument("--n1", type=int, default=2)
    args = p.parse_args()
    if not 30 <= args.bid <= 42:
        p.error("bid must be 30..42")
    rng = random.Random(args.deal_seed) if args.deal_seed is not None else random.SystemRandom()
    tiles = list(range(28))
    rng.shuffle(tiles)
    hands = [sorted(tiles[7*s:7*s+7]) for s in range(4)]
    remaining = [set(h) for h in hands]
    print("\nTexas 42 — partnership experiment\nYou and Partner play against Left and Right.")
    print("Your hand: " + "  ".join(tile_name(t) for t in hands[0]))
    decl = args.decl
    while decl is None:
        try:
            selected = int(input("Choose trump (0–6, 7=doubles, 9=no trump): "))
            if selected in (0,1,2,3,4,5,6,7,9):
                decl = selected
        except ValueError:
            pass
    print("You declare " + str(args.bid) + ". Each computer move has a 14-second allowance.\n")
    names = ["You", "Left", "Partner", "Right"]
    points, record, leader = [0, 0], [], 0
    for ti in range(7):
        trick = []
        print("Trick " + str(ti+1) + " — " + names[leader] + " leads")
        for pos in range(4):
            seat = (leader+pos)%4
            legal = legal_tiles(remaining[seat], trick, decl)
            if seat == 0:
                print("Your hand: " + "  ".join(tile_name(t) for t in sorted(remaining[0])))
                print("Legal: " + "  ".join(str(i+1)+":"+tile_name(t) for i,t in enumerate(legal)))
                tile = None
                while tile is None:
                    raw = input("Your play (number or tile): ").strip()
                    try:
                        if "-" in raw:
                            tile = next((t for t in legal if tile_name(t)==raw), None)
                        else:
                            index = int(raw)-1
                            if 0 <= index < len(legal):
                                tile = legal[index]
                    except ValueError:
                        pass
            else:
                print(names[seat] + " is thinking…", flush=True)
                response = decide({"decl": decl, "bid": args.bid, "seat": seat,
                                   "bidder": 0, "hand": hands[seat], "plays": record[:],
                                   "seed": 420600},
                                  "partner" if seat == 2 else "phone", n1=args.n1)
                tile = response["choice"]
                if "fallback" in response["route"]:
                    print("  (used the move kept in reserve)")
            assert tile in legal
            print("  " + names[seat] + " plays " + tile_name(tile))
            remaining[seat].remove(tile)
            record.extend([seat, tile])
            trick.append((seat, tile))
        leader = winner(trick, decl)
        count = trick_points(trick)
        points[leader%2] += count
        print(names[leader] + " wins " + str(count) + ". You: " + str(points[0]) + " — Them: " + str(points[1]) + "\n")
    print("You made it!" if points[0] >= args.bid else "Set. You needed " + str(args.bid) + ".")


if __name__ == "__main__":
    try:
        main()
    except (EOFError, KeyboardInterrupt):
        print("\nTable closed.")
