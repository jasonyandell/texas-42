#!/usr/bin/env python3
"""Complete late player-call costs on saved public game turns, same L3 policy."""
import argparse,gzip,hashlib,importlib.util,json,statistics,sys
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('old_compare',HERE.parent/'prefix-state-20261004/compare.py')
old=importlib.util.module_from_spec(spec);spec.loader.exec_module(old)
p=argparse.ArgumentParser();p.add_argument('--game',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--repeats',type=int,default=8)
a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
game=json.loads(a.game.read_text());requests=[m['call']['request'] for m in game['moves'] if m['response']['route']=='late-l3-resumable-40-4-2-2']
names=['resumable','arena','lazy','native'];binary=HERE/'integration/target/release/late-choice-player';records=[]
for repeat in range(a.repeats):
 for idx,req in enumerate(requests):
  order=names[(repeat+idx)%4:]+names[:(repeat+idx)%4];runs={}
  for name in order:runs[name]=old.call(binary,dict(request=req,variant=name))
  expected=runs['native']['result']['evaluation']
  for name in names[:-1]:
   result=runs[name]['result']
   if 'error' not in result:assert result['evaluation']==expected
  records.append(dict(repeat=repeat,turn=idx,request=req,order=order,runs=runs))
 with gzip.open(a.out/'records.json.gz','wt') as f:json.dump(records,f)
variants={}
for name in names:
 rr=[r['runs'][name] for r in records];completed=all('error' not in r['result'] for r in rr)
 variants[name]=dict(completed=completed,errors=[r['result'].get('error') for r in rr],cold_complete_calls_s=sum(r['serialization_s']+r['host_s']+r['output_parse_s']+r['harness_overhead_s'] for r in rr)/a.repeats,cpu_s=sum(r['child_cpu_s'] for r in rr)/a.repeats,peak_rss_median_bytes=statistics.median(r['process_peak_rss_bytes'] for r in rr))
out=dict(turns=len(requests),repeats=a.repeats,variants=variants,complete_vector_comparisons=sum('evaluation' in r['runs'][n]['result'] for r in records for n in names[:-1]),binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),scope='Same complete experimental L3 policy40/[4,2,2,2], same actor own/public saved turns, production outer stream; one worker each; same2s solver allowance, frontier rows/work/query allowances500k/20m/50k. Native recursion has its own node/cache model and deadline only; logical caps are not equivalent memory budgets. Cold Rust startup/status/full-history validation/sampling/serialization/parse/teardown included; build receipts separate. Requests sampled from one pilot game, not population latency. All complete vectors compared; refused calls excluded from speed claims.')
(a.out/'summary.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
