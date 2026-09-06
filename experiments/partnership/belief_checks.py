#!/usr/bin/env python3
"""Bounded compatibility and option-wiring checks; no strength campaign."""
import json
from pathlib import Path
import sys
import tempfile
from unittest.mock import patch

import campaign
from player import BINARY, HERE, child, decide, native_text


def main():
    before = json.loads((HERE / "runs/voids-before/golden.json").read_text())
    rows = []
    for frozen in before["rows"]:
        expected, status = frozen["response"]
        assert status == "completed"
        for belief in ("voidless", "voids-counted"):
            actual, status = child([str(BINARY)], native_text(
                frozen["request"], frozen["mode"], before["n"], before["n0"], before["n1"],
                10000, belief), 11)
            assert status == "completed", (belief, status)
            assert actual["inner_belief"] == belief
            if belief == "voidless":
                for field in ("choice", "legal", "leader", "points", "trick", "options", "outer_worlds", "outer_draw_attempts"):
                    assert actual[field] == expected[field], (frozen["mode"], field, actual, expected)
            rows.append({"request": frozen["request"], "mode": frozen["mode"],
                         "inner_belief": belief, "response": actual})
    print("PASS nine pre-change decisions: legacy choices and every rational action value unchanged")
    print("PASS counted strategy completes the same nine opening/partial-trick decisions")

    request = before["rows"][0]["request"]
    # Exercise both the completed primary and the fallback through the real
    # JSON wrapper; neither may silently use the other belief strategy.
    response = decide(request, n=4, n0=2, n1=2, inner_belief="voids-counted")
    assert response["route"] == "partner", response
    assert response["evaluation"]["inner_belief"] == "voids-counted"
    assert response["fallback_evaluation"]["inner_belief"] == "voids-counted"
    for mode, belief in [("phone", "voids-counted"), ("partner", "unknown")]:
        try:
            decide(request, mode=mode, inner_belief=belief)
        except ValueError:
            pass
        else:
            raise AssertionError("unsupported strategy accepted")
    print("PASS explicit strategy reaches primary and fallback; archived phone rejects unsupported changes")

    with tempfile.TemporaryDirectory() as directory:
        p = Path(directory)
        for belief in ("voidless", "voids-counted"):
            with patch("builtins.print"):
                campaign.initialize(p / belief, count=1, inner_belief=belief)
            spec = campaign.load(p / belief)
            assert spec["inner_belief"] == belief
        a = campaign.read(p / "voidless/manifest.json")
        b = campaign.read(p / "voids-counted/manifest.json")
        assert a["id"] != b["id"]
        changed = {k: v for k, v in a.items() if k != "id"}
        changed["inner_belief"] = "voids-counted"
        assert campaign.digest(changed) != a["id"], "the setting itself changes the frozen identity"
    print("PASS campaign identity freezes the selected strategy (no games launched)")
    if len(sys.argv) > 1:
        Path(sys.argv[1]).write_text(json.dumps(rows, indent=2) + "\n")


if __name__ == "__main__":
    main()
