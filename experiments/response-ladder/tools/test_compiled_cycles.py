#!/usr/bin/env python3
"""Focused regression checks for compiled_cycles.py normalization."""

import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
RUNNER = HERE / "compiled_cycles.py"


class CompiledCyclePreparationTest(unittest.TestCase):
    def test_cpu_flat_pairs_follow_changed_trick_leader(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "source"
            output = root / "campaign"
            source.mkdir()
            hands = [list(range(start, start + 7)) for start in (0, 7, 14, 21)]
            remaining = [set(hand) for hand in hands]
            leader = 0
            moves = []
            history = []
            for trick in range(7):
                trick_tiles = []
                for offset in range(4):
                    actor = (leader + offset) % 4
                    tile = min(remaining[actor])
                    if trick == 0 and actor == 2:
                        tile = 20
                    if trick == 0 and actor == 3:
                        tile = 21
                    remaining[actor].remove(tile)
                    legal = sorted(remaining[actor] | {tile})
                    moves.append({
                        "actor": actor,
                        "legal": legal,
                        "player": "cpu",
                        "tile": tile,
                        "trick": trick + 1,
                        "request": {
                            "decl": 0,
                            "bid": 30,
                            "bidder": 0,
                            "hand": hands[actor],
                            "plays": [value for pair in history for value in pair],
                            "seat": actor,
                            "seed": 123,
                        },
                        "response": {"reason": "bounded", "route": "partner"},
                    })
                    history.append([actor, tile])
                    trick_tiles.append((actor, tile))
                leader = max(trick_tiles, key=lambda pair: pair[1])[0]
            game = {
                "schema": "synthetic",
                "fixture": {"bidder": 0, "decl": 0, "hands": hands, "index": 0, "policy_seed": 123, "repeat": 0},
                "moves": moves,
            }
            (source / "0-declaring.json").write_text(json.dumps(game))
            (source / "0-defending.json").write_text(json.dumps({**game, "arm": "defending"}))
            subprocess.run([sys.executable, str(RUNNER), "prepare", "--source", str(source), "--output", str(output)], check=True)
            rows = [json.loads(line) for line in (output / "requests.jsonl").read_text().splitlines()]
            changed_leader = [row for row in rows if row["request"]["history"][:4] == [[0, 0], [1, 7], [2, 20], [3, 21]]]
            self.assertTrue(changed_leader)
            self.assertEqual(changed_leader[0]["request"]["history"][4][0], 3)
            self.assertEqual(changed_leader[0]["origins"][0]["source"], "0-declaring.json")

            # A second prepare preserves the already-created campaign byte for byte.
            manifest_before = (output / "manifest.json").read_bytes()
            requests_before = (output / "requests.jsonl").read_bytes()
            subprocess.run([sys.executable, str(RUNNER), "prepare", "--source", str(source), "--output", str(output)], check=True)
            self.assertEqual(manifest_before, (output / "manifest.json").read_bytes())
            self.assertEqual(requests_before, (output / "requests.jsonl").read_bytes())


if __name__ == "__main__":
    unittest.main()
