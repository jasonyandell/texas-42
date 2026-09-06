#!/usr/bin/env python3
"""Independently replay every committed arm and check stored aggregates."""
import argparse
from collections import Counter
from pathlib import Path
import campaign as c
from rules import replay_record


def verify(path):
    path=Path(path).resolve()
    spec=c.load(path)
    assert c.digest({k:v for k,v in spec.items() if k!="id"})==spec["id"], "manifest changed"
    rows=c.complete_results(path,spec)
    assert [r['seed'] for r in rows]==list(range(spec['start'],spec['start']+len(rows))), 'committed seed prefix has a gap'
    for r in rows:
        seed=r["seed"]
        assert spec["start"]<=seed<spec["start"]+spec["count"]
        assert r["fresh"]==(seed not in spec["known_development_seeds"])
        f=c.fixture(spec,seed)
        for a in c.ARMS:
            result=r["arms"][a]
            assert result==c.read(path/"seeds"/str(seed)/a/"result.json")
            snap=c.read(path/"seeds"/str(seed)/a/"checkpoint.json")
            assert (snap["campaign"],snap["seed"],snap["arm"])==(spec["id"],seed,a)
            ds=snap["decisions"]
            assert len(ds)==28
            modes=c.modes_for(a,f["bidder"])
            for i,d in enumerate(ds):
                points,lead,_,trick=replay_record(f["hands"],c.record_of(ds[:i]),f["decl"],f["bidder"])
                assert d["seat"]==(lead+len(trick))%4
                response=d["response"]
                assert response["leader"]==lead and response["points"]==points
                assert response["mode"]==modes[d["seat"]]
                assert not response["over_budget"]
            points,_,remaining,trick=replay_record(f["hands"],c.record_of(ds),f["decl"],f["bidder"])
            assert not any(remaining) and not trick and sum(points)==42
            assert result["fixture"]==f and result["modes"]==modes
            assert result["points"]==points
            assert result["made"]==(points[f["bidder"]%2]>=30)
            assert result["overbid_points_diagnostic"]==max(0,points[f["bidder"]%2]-30)
            assert result["routes"]==dict(Counter(d["response"]["route"] for d in ds))
            assert result["elapsed_us"]==sum(d["response"]["elapsed_us"] for d in ds)
            assert result["max_decision_us"]==max(d["response"]["elapsed_us"] for d in ds)
            assert result["trick_us"]==[sum(d["response"]["elapsed_us"] for d in ds[t:t+4]) for t in range(0,28,4)]
            assert max(result["trick_us"])<60_000_000
        assert r["paired"]==c.paired(*(r["arms"][a] for a in c.ARMS))
    receipt={"campaign":spec["id"],"verified_utc":c.now(),"seeds":len(rows),"games":3*len(rows),"decisions":84*len(rows),"status":"passed"}
    c.atomic(path/"verification.json",receipt)
    print(receipt)


if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("path",type=Path)
    verify(p.parse_args().path)
