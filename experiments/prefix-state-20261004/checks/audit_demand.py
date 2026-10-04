#!/usr/bin/env python3
import json,gzip
from pathlib import Path
BASE=Path(__file__).resolve().parent.parent/'results/demand-matrix'
def read_json(path):
 if path.exists():return json.loads(path.read_text())
 with gzip.open(str(path)+'.gz','rt') as f:return json.load(f)
r=read_json(BASE/'records.json');s=read_json(BASE/'summary.json');assert len(r)==len(s['matrix'])==9
checks=refusals=0;report=[]
for row,summary in zip(r,s['matrix']):
 for key in ['worlds','field','roots','selected']:assert row[key]==summary[key]
 assert row['roots']==6;assert len(row['selected'])==6;assert row['generation_actual_n_s']>=0
 expected=row['reference']['result']['answers'];assert len(expected)==6;assert summary['reference_us']==row['reference']['result']['reference_total_us']
 for name,v in row['runs'].items():
  x=v['result'];ss=summary['variants'][name];completed='answers' in x;assert ss['completed']==completed;assert ss['error']==x.get('error');assert ss['cold_or_refusal_us']==x.get('frontier_cold_us',x.get('refusal_us'));assert ss['completed_cache_entries']==x.get('completed_cache_entries')
  stats=x['cold_stats'] if completed else x['worker_stats'][0];assert ss['stats']==stats;assert stats['completed_actor_queries']<=stats['generated_actor_queries']<=50000
  if completed:assert x['answers']==expected;checks+=6
  else:
   assert 'answers' not in x;assert x['error']=='actor query cap';assert stats['refused_batches']>=1;refusals+=1
  assert v['host_s']>0;assert v['child_cpu_s']>=0;assert v['harness_overhead_s']>=0
 if row['worlds']==40 and row['field']==2:
  full=row['runs']['full']['result'];assert 'answers' not in full;assert row['runs']['hybrid']['result']['answers']==expected
  stats=full['worker_stats'][0];assert stats['generated_actor_queries']==49534;assert stats['completed_actor_queries']==49202
 report.append({'worlds':row['worlds'],'field':row['field'],'full_complete':summary['variants']['full']['completed'],'full_actor_demands':summary['variants']['full']['stats']['actor_demands_by_level'],'full_unique_misses':summary['variants']['full']['stats']['unique_actor_misses_by_level'],'full_inner_worlds':summary['variants']['full']['stats']['inner_worlds_by_level'],'hybrid_complete':summary['variants']['hybrid']['completed']})
assert refusals==1
print(json.dumps({'completed_vectors':checks,'refusals':refusals,'matrix':report,'note':'One observation per cell; partial lower caches may survive refusal; absence of root answers is checked, not a proof of transactional rollback.'},indent=2))
