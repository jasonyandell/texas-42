#!/usr/bin/env python3
"""Build human-readable tables from committed seeds only."""
import argparse
import csv
import io
from pathlib import Path
import campaign as c


def report(path):
    path=Path(path).resolve()
    spec=c.load(path,verify=False)
    rows=c.complete_results(path,spec)
    s=c.summarize(path,spec)
    fresh=[r for r in rows if r["fresh"]]
    sessions=[c.read(p) for p in (path/"sessions").glob("*.json")]
    seconds=sum(r.get("elapsed_seconds",0) for r in sessions)
    pair=s["fresh_paired"]
    out=(f"# Bid-30 {spec['panel']} campaign results\n\n"
         f"Exploratory executed-policy comparison; generated contracts, no auction evaluation.\n\n"
         f"Completed **{len(rows)}/{spec['count']} seeds**, **{3*len(rows)} games**. "
         f"Fresh comparisons: **{pair['wins']} wins, {pair['losses']} losses, {pair['ties']} ties** "
         f"across {len(fresh)} seeds. Make/set is the only ranking criterion.\n\n")
    if s["stop"]:
        out+=f"Paused: `{s['stop']['reason']}`.\n\n"
    out+="| Fresh role | Wins | Losses | Ties |\n|---|---:|---:|---:|\n"
    for name,key in (("Candidate declaring","declaring_delta"),("Candidate defending","defending_delta")):
        vals=[r["paired"][key] for r in fresh]
        out+=f"| {name} | {sum(v>0 for v in vals)} | {sum(v<0 for v in vals)} | {sum(v==0 for v in vals)} |\n"
    out+=(f"\nPhone references made {s['fresh_reference_makes']}/{len(fresh)} fresh contracts. "
          f"Recorded fallbacks: {s['fallbacks']}/{s['nonforced_decisions']} nonforced decisions. "
          f"Longest decision {s['max_decision_us']/1e6:.3f}s; longest four-play trick {s['max_trick_us']/1e6:.3f}s.\n\n"
          f"Completed runner slices used {seconds/60:.2f} minutes. "
          "An active slice's time is added when it closes.\n\n")
    pooled=[r for r in sessions if 'workers' in r]
    if pooled:
        out+=(f"Execution includes the user-authorized shared pool, up to {max(r['workers'] for r in pooled)} simultaneous games across seeds. "
              "Earlier slices used three games within one seed. Per-game attempt receipts identify resumed moves and concurrency; deadline-dependent outcomes must be interpreted with that execution change visible.\n\n")
    else:
        out+="Three games run concurrently within one seed.\n\n"
    e=s["downside_e_value"]
    out+=f"Downside monitor: {e['numerator']}/{e['denominator']} (pause threshold 20, minimum ten fresh seeds; see CAMPAIGN.md for assumptions).\n\n"
    out+="Points below are diagnostic. Every number below 30 is an equally complete set; every number at least 30 is a make.\n\n"
    out+="| Seed | Fresh | Bidder / trump | Phone declaring points | Candidate declaring points | Phone points vs candidate defense | Candidate W/L/T | Fallbacks |\n|---|---|---|---:|---:|---:|---|---:|\n"
    buf=io.StringIO()
    writer=csv.writer(buf)
    writer.writerow(["seed","fresh","bidder","decl","bid","phone_points","candidate_declaring_points","against_candidate_defending_points","wins","losses","ties","fallbacks"])
    for r in rows:
        f=r["arms"]["phone"]["fixture"]
        points=[r["arms"][a]["points"][f["bidder"]%2] for a in c.ARMS]
        p=r["paired"]
        fallback=sum(r["arms"][a]["fallbacks"] for a in c.ARMS)
        out+=f"| {r['seed']} | {'yes' if r['fresh'] else 'development'} | S{f['bidder']} / {f['decl']} | {points[0]} | {points[1]} | {points[2]} | {p['wins']}/{p['losses']}/{p['ties']} | {fallback} |\n"
        writer.writerow([r["seed"],r["fresh"],f["bidder"],f["decl"],30,*points,p["wins"],p["losses"],p["ties"],fallback])
    out+="\nCheckpoints preserve all 28 decisions per game. Source/binary identities and the complete protocol are pinned in manifest.json. Seeds 420601–420603 are excluded from fresh random-panel evidence because they were previously examined; 420601 now uses the campaign's declaration heuristic rather than its earlier fixed sixes contract.\n"
    c.atomic(path/"RESULTS.md",out)
    c.atomic(path/"results.csv",buf.getvalue())
    print(out[:out.index("| Seed")])


if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("path",type=Path)
    report(p.parse_args().path)
