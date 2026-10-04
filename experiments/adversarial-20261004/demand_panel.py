#!/usr/bin/env python3
"""Bounded native modeled-rung work census. Invoke through run_capped.py."""
import argparse, hashlib, json, random, subprocess, time
from pathlib import Path
from parallel_roots import fixture

HERE=Path(__file__).resolve().parent
def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
    p.add_argument('--start',type=int,default=940000);p.add_argument('--count',type=int,default=12)
    p.add_argument('--min-ply',type=int,default=16);args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
    rows=[fixture(args.start+i,args.min_ply+i%4,8) for i in range(args.count)]
    binary=HERE/'native/target/release/adversarial-demand'
    plan=dict(rows=rows,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
              outer=8,inner=[4,2,2,2],levels=[0,1,2,3],solver_seconds=3,child_seconds=5,
              note='Core response to modeled Field::Level(k). Outer samples honor historical voids. Shared::new defaults to actor-specific Voidless inner beliefs: own hand and capacities, without historical void conditioning. Fixed sampling budgets, not exact population best response. pi_calls count cache misses, not all consultations or unique keys. No phone wrapper/review or strength claim.')
    (args.out/'plan.json').write_text(json.dumps(plan,indent=2)+'\n')
    result=[];start=time.monotonic();rng=random.Random(999903)
    for row in rows:
        if row['status']!='ready':continue
        for level in range(4):
            modes=[False,True];rng.shuffle(modes)
            for parallel in modes:
                if time.monotonic()-start>230:
                    result.append(dict(seed=row['seed'],level=level,parallel=parallel,status='panel_refused'));continue
                inp=dict(request=row['request'],outer=8,level=level,parallel=parallel,seconds=3)
                tick=time.monotonic()
                try:
                    proc=subprocess.run([str(binary)],input=json.dumps(inp)+'\n',capture_output=True,text=True,timeout=5)
                    rr=dict(seed=row['seed'],ply=row['ply'],level=level,parallel=parallel,host_s=time.monotonic()-tick)
                    if proc.returncode:rr.update(status='process_error',stderr=proc.stderr,returncode=proc.returncode)
                    else:
                        value=json.loads(proc.stdout);rr.update(result=value,status='complete' if value.get('counts') is not None else 'refused')
                except subprocess.TimeoutExpired:
                    rr=dict(seed=row['seed'],level=level,parallel=parallel,status='timeout',host_s=time.monotonic()-tick)
                result.append(rr)
                (args.out/'results.json').write_text(json.dumps(result,indent=2)+'\n')
                print(json.dumps(rr),flush=True)
    for row in rows:
        for level in range(4):
            pair=[r for r in result if r['seed']==row['seed'] and r['level']==level]
            if len(pair)==2 and all(r['status']=='complete' for r in pair):
                assert pair[0]['result']['tiles']==pair[1]['result']['tiles']
                assert pair[0]['result']['counts']==pair[1]['result']['counts']
    summary=dict(planned=len(rows),ready=sum(r['status']=='ready' for r in rows),runs=len(result),
                 statuses={s:sum(r['status']==s for r in result) for s in {r['status'] for r in result}},
                 all_complete_mode_pairs_equal=True,elapsed_s=time.monotonic()-start)
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
if __name__=='__main__':main()
