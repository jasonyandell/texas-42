#!/usr/bin/env python3
"""Fixed-budget rung/world demand and refusal matrix; use run_capped.py."""
import argparse,json,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
from compare import call,fixture,HERE

def main():
 p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
 binary=HERE/'native/target/release/native-frontier';records=[];started=time.monotonic()
 for n in [2,8,40]:
  tick=time.perf_counter();all_rows=[fixture(962000+i,16+i%4,n) for i in range(32)];generation=time.perf_counter()-tick
  rows=[r for r in all_rows if r['status']=='ready' and r['request']['decl']<=6]
  for field in [0,1,2]:
   if time.monotonic()-started>220:raise TimeoutError('matrix 220s cap')
   inp=dict(rows=rows,mode='policy',field_level=field,budgets=[4,2,2,2],counted=False,warm=False,workers=1,caps=dict(rows=500000,work=20000000,queries=50000,cache_entries=50000,seconds=30))
   reference=call(binary,inp|{'reference':True,'reference_only':True})
   runs={}
   for name,native in [('full',False),('hybrid',True)]:
    runs[name]=call(binary,inp|{'reference':False,'native_choices':native})
    result=runs[name]['result']
    if 'answers' in result:assert result['answers']==reference['result']['answers']
    else:assert 'error' in result and 'answers' not in result and result.get('completed_cache_entries',0)>=0
   records.append(dict(worlds=n,field=field,roots=len(rows),selected=[r['seed'] for r in rows],generation_actual_n_s=generation,reference=reference,runs=runs))
   (args.out/'records.json').write_text(json.dumps(records,indent=2)+'\n')
   print(json.dumps(dict(worlds=n,field=field,status={name:r['result'].get('error','complete') for name,r in runs.items()})),flush=True)
 summary=[]
 for r in records:
  row={k:r[k] for k in ['worlds','field','roots','selected']};row['variants']={}
  for name,v in r['runs'].items():
   x=v['result'];stats=x.get('cold_stats',x.get('worker_stats',[{}])[0]);row['variants'][name]=dict(completed='answers' in x,error=x.get('error'),cold_or_refusal_us=x.get('frontier_cold_us',x.get('refusal_us')),stats=stats,completed_cache_entries=x.get('completed_cache_entries'))
  row['reference_us']=r['reference']['result']['reference_total_us'];summary.append(row)
 (args.out/'summary.json').write_text(json.dumps(dict(matrix=summary,elapsed_s=time.monotonic()-started,scope='One observed run per cell, six late roots; same budgets[4,2,2,2], one worker and50000-query allowance. Refusals publish no root vector. Declared hybrid budget and native-reported samples stay separate. Full eager schedule demand is not a minimum or linear-k theorem.'),indent=2)+'\n')
if __name__=='__main__':main()
