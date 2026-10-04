import gzip,json,subprocess,statistics,sys
from pathlib import Path
root=Path(__file__).resolve().parents[3]
p=root/'experiments/choice-arena-20261004/results/final-one/inputs-8.json.gz'
a=json.load(gzip.open(p,'rt')); print(type(a).__name__, flush=True)
if isinstance(a,dict): a=[a]
records=[]
for mode in ['bounded','arena','resumable','resumable_zero','native']:
 for i in range(5):
  inp=dict(rows=[r for r in a if r.get('status')=='ready'],mode='policy',field_level=2,budgets=[4,2,2,2]); inp.update(reference=False,warm=False,workers=1,native_choices=mode=='native',bounded_choices=mode=='bounded',with_arena_choices=mode=='arena',resumable_choices=mode.startswith('resumable'),eager_zero=mode=='resumable_zero')
  r=subprocess.run([str(Path(__file__).parent/'target/release/native-frontier')],input=json.dumps(inp)+'\n',capture_output=True,text=True,check=True)
  out=json.loads(r.stdout); assert 'error' not in out, out; records.append(dict(mode=mode,output=out))
for mode in ['bounded','arena','resumable','resumable_zero','native']:
 rs=[r['output'] for r in records if r['mode']==mode]
 print(mode, statistics.median(r.get('frontier_cold_us',0) for r in rs), rs[0].get('error'),rs[0].get('cold_stats'),flush=True)
assert all(r['output']['answers']==records[0]['output']['answers'] for r in records)
Path(sys.argv[1]).write_text(json.dumps(records,indent=2)+'\n')
