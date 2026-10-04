#!/usr/bin/env python3
"""Complete cold local fixture + pinned native recursion pipeline.

Two generation variants; same cold executable, workers, samples and budgets.
No player integration or hand-to-hand playing-strength claim.
"""
import time
BOOT=time.perf_counter()
import argparse,gzip,hashlib,json,resource,statistics,sys
from pathlib import Path
sys.dont_write_bytecode=True
from sampling import original,compiled_fixture,compare
HERE=Path(__file__).resolve().parent
SETUP=time.perf_counter()-BOOT

def main():
 p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
 p.add_argument('--start',type=int,default=962000);p.add_argument('--count',type=int,default=32)
 p.add_argument('--worlds',type=int,default=40);p.add_argument('--workers',type=int,choices=[1,4],default=4)
 p.add_argument('--repeats',type=int,default=8);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
 binary=HERE/'checks/baseline-target/release/native-frontier';records=[];expected=None;expected_rows=None
 for repeat in range(a.repeats):
  order=['original','compiled'] if repeat%2==0 else ['compiled','original']
  for name in order:
   fn=original if name=='original' else compiled_fixture
   started=time.perf_counter();cpu=time.process_time();g=time.perf_counter()
   all_rows=[fn(a.start+i,16+i%4,a.worlds) for i in range(a.count)]
   generation=time.perf_counter()-g
   rows=[r for r in all_rows if r['status']=='ready' and r['request']['decl']<=6]
   payload=dict(rows=rows,mode='policy',field_level=2,budgets=[4,2,2,2],counted=False,
      workers=a.workers,reference=True,reference_only=True,batch_reference=a.workers==4,
      caps=dict(rows=500000,work=20000000,queries=50000,cache_entries=50000,seconds=30))
   run=compare.call(binary,payload)
   elapsed=time.perf_counter()-started;parent_cpu=time.process_time()-cpu
   assert 'answers' in run['result'],run['result']
   if expected is None:expected=run['result']['answers'];expected_rows=all_rows
   assert all_rows==expected_rows and run['result']['answers']==expected
   records.append(dict(repeat=repeat,variant=name,order=order,generation_s=generation,
     complete_pipeline_s=elapsed,parent_cpu_s=parent_cpu,run=run))
 variants={}
 for name in ['original','compiled']:
  rr=[r for r in records if r['variant']==name]
  variants[name]={k:statistics.median(r[k] for r in rr) for k in ['generation_s','complete_pipeline_s','parent_cpu_s']}
  variants[name].update({k:statistics.median(r['run'][k] for r in rr) for k in ['serialization_s','host_s','output_parse_s','harness_overhead_s','child_cpu_s','process_peak_rss_bytes']})
  variants[name]['native_wall_s']=statistics.median(r['run']['result']['reference_total_us'] for r in rr)/1e6
  variants[name]['replay_validation_s']=statistics.median(r['run']['result']['outer_validation_us'] for r in rr)/1e6
 with gzip.open(a.out/'records.json.gz','wt') as f:json.dump(records,f)
 with gzip.open(a.out/'all-inputs.json.gz','wt') as f:json.dump(expected_rows,f)
 summary=dict(arguments=vars(a),selected=[r['seed'] for r in expected_rows if r['status']=='ready' and r['request']['decl']<=6],
    variants=variants,original_over_compiled=variants['original']['complete_pipeline_s']/variants['compiled']['complete_pipeline_s'],
    exact_all_rows_and_vectors=True,vector_comparisons=len(records)*len(expected),shared_python_import_setup_s=SETUP,
    parent_lifetime_peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*(1 if sys.platform=='darwin' else 1024),
    binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
    scope='Finite late Straight component. Every run freshly regenerates every proposed deal and launches cold pinned native recursion; same worlds/RNG/order/tapes/workers/budgets. Direct complete time includes generation, input construction/filtering, JSON, process/replay/native scheduling, tempfile and parse. Shared script imports measured separately; capped receipt includes them plus verification/record writing. Parent RSS is lifetime max; child RSS PID-specific. No production phone latency/strength or generic higher-k inference.')
 (a.out/'summary.json').write_text(json.dumps(summary,indent=2,default=str)+'\n')
 print(json.dumps({k:summary[k] for k in ['selected','variants','original_over_compiled','vector_comparisons']}))
if __name__=='__main__':main()
