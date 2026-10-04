#!/usr/bin/env python3
"""Four-position matched full cold pipelines. Invoke through run_capped."""
import argparse,gzip,hashlib,importlib.util,json,statistics,sys
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('old_cold',HERE.parent/'choice-arena-20261004/cold_pipeline.py')
old=importlib.util.module_from_spec(spec);spec.loader.exec_module(old)
p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);p.add_argument('--start',type=int,default=962000);p.add_argument('--count',type=int,default=32);p.add_argument('--worlds',type=int,default=40);p.add_argument('--workers',type=int,choices=[1,4],default=4);p.add_argument('--repeats',type=int,default=8)
a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
names=['resumable','arena','bounded','recursive'];records=[];expected=None;rowhash=None
for repeat in range(a.repeats):
 order=names[repeat%4:]+names[:repeat%4]
 runs={}
 for name in order:
  argv=[sys.executable,str(HERE/'cold_worker.py'),'--variant',name,'--start',str(a.start),'--count',str(a.count),'--worlds',str(a.worlds),'--workers',str(a.workers)]
  runs[name]=old.call(argv)
  r=runs[name]['result']
  if rowhash is None:rowhash=r['all_rows_sha256']
  assert rowhash==r['all_rows_sha256']
 native=runs['recursive']['result']['run']['result']['answers']
 if expected is None:expected=native
 assert native==expected
 for name in names[:-1]:
  v=runs[name]['result']['run']['result']
  if 'answers' in v:assert v['answers']==expected
  else:assert 'error' in v
 records.append(dict(repeat=repeat,order=order,runs=runs))
 with gzip.open(a.out/'records.json.gz','wt') as f:json.dump(records,f)
variants={}
for name in names:
 rr=[r['runs'][name] for r in records]
 v={k:statistics.median(r[k] for r in rr) for k in ['complete_cold_pipeline_s','python_process_host_s','wrapper_parse_s','wrapper_overhead_s','frontend_wait4_cpu_s','frontend_peak_rss_bytes']}
 v.update({k:statistics.median(r['result'][k] for r in rr) for k in ['python_import_s','generation_s']})
 v.update({k:statistics.median(r['result']['run'][k] for r in rr) for k in ['child_cpu_s','process_peak_rss_bytes','serialization_s','host_s','output_parse_s','harness_overhead_s']})
 v['completed']=all('answers' in r['result']['run']['result'] for r in rr);v['errors']=[r['result']['run']['result'].get('error') for r in rr];variants[name]=v
summary=dict(arguments=vars(a),variants=variants,selected=records[0]['runs']['recursive']['result']['selected'],exact_rows_sha256=rowhash,binary_sha256={n:records[0]['runs'][n]['result']['binary_sha256'] for n in names},vector_comparisons=sum(len(expected)*sum('answers' in r['runs'][n]['result']['run']['result'] for n in names[:-1]) for r in records),scope='Matched common compiled support frontend; every request starts fresh Python and native, includes imports/masks/generation/JSON/native validation/scheduling/tempfiles/parse/teardown. Build receipts separately retained. Refusals excluded from speed claims. Frontend CPU may include waited descendants and is not added to nested native CPU. No phone-device inference.')
(a.out/'summary.json').write_text(json.dumps(summary,indent=2,default=str)+'\n');print(json.dumps(summary,default=str))
