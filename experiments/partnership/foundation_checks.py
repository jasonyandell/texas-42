#!/usr/bin/env python3
"""Foundation regression, archive parity, and persistent-worker checks."""

import hashlib
import json
import os
import subprocess

from player import BINARY, HERE, ROOT, child, native_text
from runtime import DecisionSession


def main():
    deps = ROOT / "walt/target/release/deps"
    for suite in ["solver_selection", "solver_partnership"]:
        cmd = [
            "rustc",
            "--edition=2021",
            "--test",
            str(ROOT / f"walt/walt/tests/{suite}.rs"),
            "-L",
            "dependency=" + str(deps),
            "-C",
            "opt-level=3",
            "-C",
            "overflow-checks=yes",
        ]
        for name in ["walt", "num_rational"]:
            libs = list(deps.glob("lib" + name + "-*.rlib"))
            assert len(libs) == 1
            cmd += ["--extern", name + "=" + str(libs[0])]
        target = ROOT / "walt/target/release" / ("foundation-" + suite)
        cmd += ["-o", str(target)]
        subprocess.run(
            cmd,
            check=True,
            env={**os.environ, "CARGO_MANIFEST_DIR": str(ROOT / "walt/walt")},
        )
        subprocess.run(
            [str(target), "--test-threads=1"],
            check=True,
            env={**os.environ, "RAYON_NUM_THREADS": "6"},
        )
    golden = json.loads((HERE / "runs/voids-before/golden.json").read_text())
    with DecisionSession() as session:
        for row in golden["rows"]:
            request = native_text(
                row["request"],
                row["mode"],
                golden["n"],
                golden["n0"],
                golden["n1"],
                13000,
            )
            got, status = session.call([str(BINARY)], request, 14)
            old = row["response"][0]
            assert status == "completed", (status, got)
            assert (
                got["options"] == old["options"] and got["choice"] == old["choice"]
            ), (old, got)
    print(
        "PASS nine frozen fixed-policy comparisons, exact values and moves", flush=True
    )
    records = []
    campaign = HERE / "campaigns/native-l1-vs-phone-620600-649"
    # Fixed pre-existing panel; no selection based on newly observed parity.
    for seed in [620600, 620601]:
        row = json.loads((campaign / "results" / f"{seed}.json").read_text())
        fixture = row["arms"]["phone"]["fixture"]
        history = []
        saved = json.loads(
            (campaign / "seeds" / str(seed) / "phone/checkpoint.json").read_text()
        )["decisions"]
        with DecisionSession() as session:
            for i, d in enumerate(saved):
                seat = d["seat"]
                req = {
                    "decl": fixture["decl"],
                    "bid": 30,
                    "bidder": fixture["bidder"],
                    "seat": seat,
                    "hand": fixture["hands"][seat],
                    "plays": history[:],
                    "seed": 420600,
                }
                if len(d["response"]["legal"]) > 1:
                    for race, rule in [(True, "race-refine"), (False, "refine")]:
                        phone_req = json.dumps({**req, "n": 40, "n0": 8, "race": race})
                        old, ps = session.call(
                            ["node", str(HERE / "phone.mjs")], phone_req, 14
                        )
                        native, ns = session.call(
                            [str(BINARY)],
                            native_text(
                                req, "baseline", 40, 8, 2, 13500, "voidless", rule
                            ),
                            14,
                        )
                        assert ps == ns == "completed", (seed, i, rule, ps, ns)
                        assert old["choice"] == native["choice"], (
                            seed,
                            i,
                            rule,
                            old,
                            native,
                        )
                        if not race:
                            assert old["opts"] == [
                                [t, int(a) * 10000 // int(b)]
                                for t, a, b in native["options"]
                            ]
                        records.append(
                            {
                                "seed": seed,
                                "ply": i,
                                "rule": rule,
                                "choice": old["choice"],
                            }
                        )
                        if i == 0 and race:
                            cold, status = child(
                                ["node", str(HERE / "phone.mjs")], phone_req, 14
                            )
                            assert status == "completed" and cold == old
                            again, status = session.call(
                                ["node", str(HERE / "phone.mjs")], phone_req, 14
                            )
                            assert status == "completed" and again == old
                history.extend((seat, d["response"]["choice"]))
    (HERE / "runs/foundation-parity.json").write_text(
        json.dumps(
            {
                "native_sha256": hashlib.sha256(BINARY.read_bytes()).hexdigest(),
                "phone_sha256": hashlib.sha256(
                    (HERE / "reference/phone/walt.wasm").read_bytes()
                ).hexdigest(),
                "comparisons": len(records),
                "rows": records,
            },
            indent=2,
        )
        + "\n"
    )
    print(
        f"PASS {len(records)} native/archived-WASM comparisons at identical information states, racing and refinement",
        flush=True,
    )


if __name__ == "__main__":
    main()
