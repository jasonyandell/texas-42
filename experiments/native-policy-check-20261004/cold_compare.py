#!/usr/bin/env python3
"""Same actual late public requests/policy, cold native and Node/WASM costs.
Every run only via process-group run_capped <=295s. No source mutations.
"""
import argparse,gzip,json,os,signal,statistics,subprocess,sys,tempfile,time
from pathlib import Path
sys.dont_write_bytecode=True
from play import HERE,NATIVE,WASM,phone
CONTROL=HERE/'control-target/release/late-choice-player'
def call(command,payload):
 start=time.perf_counter();raw=json.dumps(payload)+'\n';serialize=time.perf_counter()-start
 tick=time.perf_counter()
 with tempfile.TemporaryFile(mode='w+')as source,tempfile.TemporaryFile(mode='w+')as output,tempfile.TemporaryFile(mode='w+')as errors:
  source.write(raw);source.seek(0);host_start=time.perf_counter();p=subprocess.Popen(command,stdin=source,stdout=output,stderr=errors)
  def expired(signum,frame):p.kill();os.wait4(p.pid,0);raise TimeoutError('individual request10s cap')
  prior=signal.signal(signal.SIGALRM,expired);signal.alarm(10)
  try:pid,status,usage=os.wait4(p.pid,0)
  finally:signal.alarm(0);signal.signal(signal.SIGALRM,prior)
  host=time.perf_counter()-host_start;p.returncode=os.waitstatus_to_exitcode(status)
  errors.seek(0);assert p.returncode==0,errors.read()
  output.seek(0);t=time.perf_counter();wrapper=json.load(output);parse=time.perf_counter()-t
 overhead=time.perf_counter()-tick-host-parse
 return dict(result=wrapper.get('result',wrapper),serialization_s=serialize,host_s=host,parse_s=parse,harness_s=overhead,total_s=serialize+host+parse+overhead,child_cpu_s=usage.ru_utime+usage.ru_stime,rss_bytes=usage.ru_maxrss*(1 if sys.platform=='darwin'else 1024),wasm_memory_bytes=wrapper.get('wasm_memory_bytes'))
def main():
 p=argparse.ArgumentParser();p.add_argument('--out',type=Path,default=HERE/'results/cold-same-policy');args=p.parse_args()
 out=args.out;out.mkdir(parents=True,exist_ok=False)
 tick=time.perf_counter();selected=[]
 # One first eligible public turn per distinct actual-game source deal, first12
 # eligible sources in the predeclared plan. No timing/outcome-based selection.
 plan=json.loads((HERE/'plan.json').read_text());seeds=list(dict.fromkeys(r['seed']for r in plan['games']))
 for seed in seeds:
  for p in sorted((HERE/'results/native-games').glob(f'game-{seed}-*.json')):
   game=json.loads(p.read_text());late=next((m for m in game['moves']if m['candidate']and m['response']['route']=='late-l3-native-40-4-2-2'),None)
   if late:selected.append(dict(source_deal_seed=seed,request=late['call']['request'],expected=late['response']['evaluation']));break
  if len(selected)==12:break
 selection_s=time.perf_counter()-tick;assert len(selected)==12
 (out/'public-inputs.json').write_text(json.dumps(selected,indent=2)+'\n')
 names=['resumable','arena','lazy','control-native','native','wasm'];records=[];start=time.monotonic()
 for repeat in range(6):
  for i,row in enumerate(selected):
   if time.monotonic()-start>240:raise TimeoutError('whole panel240s cap')
   order=names[(repeat+i)%6:]+names[:(repeat+i)%6];runs={}
   for name in order:
    payload=dict(request=row['request'])
    if name in ['native','wasm']:
     payload['action']='late';command=[str(NATIVE)]if name=='native'else['node',str(HERE/'wasm_worker.mjs'),str(WASM)]
    else:payload['variant']='native'if name=='control-native'else name;command=[str(CONTROL)]
    result=call(command,payload)
    if 'evaluation'in result['result']:assert result['result']['evaluation']==row['expected'],(repeat,i,name,result)
    else:assert 'error'in result['result'],result
    runs[name]=result
   records.append(dict(repeat=repeat,input_index=i,source_deal_seed=row['source_deal_seed'],order=order,runs=runs))
  with gzip.open(out/'records.json.gz','wt')as f:json.dump(records,f)
  print(json.dumps(dict(repeat=repeat,total_ms={n:sum(r['runs'][n]['total_s']for r in records if r['repeat']==repeat)*1000 for n in names})),flush=True)
 all_complete=all('evaluation'in r['runs'][n]['result']for r in records for n in names)
 variants={}
 for name in names:
  rr=[r['runs'][name]for r in records];complete=[r for r in rr if 'evaluation'in r['result']]
  totals=[sum(r['runs'][name]['total_s']for r in records if r['repeat']==rep)*1000 for rep in range(6)]
  variants[name]=dict(calls=len(rr),complete=len(complete),errors=[r['result']['error']for r in rr if 'error'in r['result']],panel_total_median_ms=statistics.median(totals)if len(complete)==len(rr)else None,cold_request_median_ms=statistics.median(r['total_s']for r in complete)*1000,child_cpu_median_ms=statistics.median(r['child_cpu_s']for r in complete)*1000,peak_rss_median_bytes=statistics.median(r['rss_bytes']for r in complete),sample_median_us=statistics.median(r['result']['outer_sample_us']for r in complete),wasm_linear_memory_peak=max((r['wasm_memory_bytes']or 0)for r in complete))
 summary=dict(public_turns=len(selected),source_deals=len({r['source_deal_seed']for r in selected}),requests_per_variant=72,repeats=6,complete_vector_comparisons=sum('evaluation'in r['runs'][n]['result']for r in records for n in names),all_complete=all_complete,variants=variants,selection_of_saved_inputs_s=selection_s,elapsed_s=time.monotonic()-start,binary_sha256=dict(control=phone.sha(CONTROL),native=phone.sha(NATIVE),wasm=phone.sha(WASM)),workers_per_request=1,outer_worlds=40,budgets=[4,2,2,2],field='Level(2)',inner_belief='Voidless',ties='ascending',scope='Same12actual public late turns, one/source deal. Cold process/request. Includes request JSON, process startup/teardown, strict status/full-history replay, setup, every sampling attempt, solve, Node module compilation/instantiation and parse/tempfiles. Saved-referee setup is reported separately and shared; no unknown hands supplied. Frontier per-worker500k rows/20m work/50kqueries/50kcache/2s; recursive2s, own cache/work instrument. Logical caps not equivalent RSS budgets. Six rotated positions; refused observations excluded from speed claims. No whole-Walt phone performance, strength, arbitrary k, or parallel scaling inference.')
 (out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
