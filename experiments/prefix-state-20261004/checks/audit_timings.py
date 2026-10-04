#!/usr/bin/env python3
"""Independent arithmetic/semantic audit of retained final compare.py records."""
import json,statistics,math,sys,hashlib,collections,gzip
from pathlib import Path
BASE=Path(__file__).resolve().parent.parent
def read_json(path):
 if path.exists():return json.loads(path.read_text())
 with gzip.open(str(path)+'.gz','rt') as f:return json.load(f)
reports=[]
for dirname in sys.argv[1:]:
 folder=BASE/'results'/dirname;s=read_json(folder/'summary.json');records=read_json(folder/'records.json');args=s['arguments'];selected=s['selected'];checks=0
 for name in ['factored','factored-hash','recursive','predecessor']:
  path=Path(args['control'] if name=='predecessor' else args['candidate']);assert hashlib.sha256(path.read_bytes()).hexdigest()==s['binary_sha256'][name]
 for n in args['counts']:
  inp=read_json(folder/f'inputs-{n}.json');assert [r['seed'] for r in inp]==selected;assert all(len(r['worlds'])==n for r in inp)
 if args['repeats']==4:
  for n in args['counts']:
   orders=[r['order'] for r in records if r['worlds']==n]
   for position in range(4):assert len({order[position] for order in orders})==4
 for rec in records:
  assert rec['roots']==len(selected);assert set(rec['order'])=={'factored','factored-hash','predecessor','recursive'}
  native=rec['runs']['recursive']['result']['answers']
  for name in ['factored','factored-hash','predecessor']:
   assert rec['runs'][name]['result']['answers']==native;checks+=len(selected)
  for key in ['row_edges','public_coordinates','raw_policy_consultations_by_level','unique_actor_misses_by_level','inner_worlds_by_level']:
   assert len({json.dumps(rec['runs'][n]['result']['cold_stats'][key]) for n in ['factored','factored-hash','predecessor']})==1
  for run in rec['runs'].values():assert run['host_s']>0 and run['serialization_s']>=0 and run['child_cpu_s']>=0 and run['process_peak_rss_bytes']>0
 assert checks==s['vector_checks']
 for panel in s['panels']:
  rr=[r for r in records if r['worlds']==panel['worlds']];assert len(rr)==args['repeats'];assert panel['roots']==len(selected)
  def eq(a,b):assert math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-12),(dirname,a,b)
  recomputed={}
  for name,variant in panel['variants'].items():
   runs=[r['runs'][name] for r in rr]
   for key in ['serialization_s','host_s','output_parse_s','child_cpu_s','process_peak_rss_bytes']+(['harness_overhead_s'] if 'harness_overhead_s' in variant else []):eq(variant[key],statistics.median(x[key] for x in runs))
   cold=statistics.median(x['result']['reference_total_us'] if name=='recursive' else x['result']['frontier_cold_us'] for x in runs)/1e6
   standalone=statistics.median(sum(x[k] for k in ['serialization_s','host_s','output_parse_s'])+x.get('harness_overhead_s',0) for x in runs)
   eq(cold,variant['cold_service_s']);eq(standalone,variant['standalone_s']);eq(variant['total_with_generation_s'],panel['generation_actual_n_s']+standalone)
   recomputed[name]=(cold,standalone)
  f=recomputed['factored'];p=recomputed['predecessor'];r=recomputed['recursive']
  ratios={'predecessor_over_factored':p[0]/f[0],'recursive_over_factored':r[0]/f[0],'standalone_recursive_over_factored':r[1]/f[1]}
  for key,value in ratios.items():eq(value,panel[key])
  reports.append({'directory':dirname,'worlds':panel['worlds'],'roots':len(selected),'factored_cold_ms':1000*f[0],'recursive_cold_ms':1000*r[0],**ratios})
print(json.dumps({'all_arithmetic_and_vector_checks_pass':True,'panels':reports,'note':'Ratios are medians on retained local finite workloads, not inferential confidence intervals. Current standalone includes JSON, child lifetime, output parsing and measured harness overhead; historical panels lacking overhead retain their narrower metric.'},indent=2))
