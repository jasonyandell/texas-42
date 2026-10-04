# Adapted with attribution from preceding demand-bounds independent checker; fresh execution here.
#!/usr/bin/env python3
"""Independent arithmetic, fixed-deal sequence and cost audit of reuse profile."""
import json,random,sys,hashlib
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/demand-bounds-20261004';BASE=HERE/'results/reuse-final'
sys.path.insert(0,str(ROOT/'experiments/adversarial-20261004'))
from parallel_roots import fixture,replay_record
rows=json.loads((BASE/'inputs.json').read_text())['rows'];result=json.loads((BASE/'result.json').read_text());g=json.loads((BASE/'generation.json').read_text());data=result['result']
assert g['proposed']==64 and g['selected']==len(rows)==15 and g['actual_worlds']==8
proposed=[fixture(968000+i,ply,8) for i in range(16) for ply in [16,17,18,19]]
assert rows==json.loads(json.dumps([r for r in proposed if r['status']=='ready' and r['request']['decl']<=6]))
assert g['serialized_input_bytes']==(BASE/'inputs.json').stat().st_size
assert g['generation_actual_n_s']>0 and g['fixture_serialization_write_s']>=0
identities=set();histories={};world_checks=0
for row in rows:
 req=row['request'];seed=row['seed'];ply=row['ply'];assert len(row['worlds'])==8 and len(req['plays'])==2*ply
 deck=list(range(28));random.Random(seed).shuffle(deck);full=[sorted(deck[7*s:7*s+7]) for s in range(4)]
 assert req['hand']==full[req['seat']]
 points,leader,remaining,tail=replay_record(full,req['plays'],req['decl'],req['bidder']);assert list(points)==row['points'] and leader==row['leader'] and json.loads(json.dumps(tail))==row['trick']
 key=(req['decl'],req['bid'],sum(1<<t for t in req['plays'][1::2]),leader,tuple(tuple(p) for p in tail),tuple(points),req['seat'],tuple(req['hand']))
 assert key not in identities;identities.add(key)
 previous=histories.get(seed)
 if previous is not None:assert previous['ply']<ply;assert req['plays'][:len(previous['request']['plays'])]==previous['request']['plays'];assert req['decl']==previous['request']['decl'] and req['bidder']==previous['request']['bidder']
 histories[seed]=row
 past=[[t for s,t in zip(req['plays'][::2],req['plays'][1::2]) if s==seat] for seat in range(4)]
 for world in row['worlds']:
  deal=[past[s]+[t for t in range(28) if world[s]&(1<<t)] for s in range(4)]
  assert all(len(h)==7 for h in deal);pp,ll,rr,tt=replay_record(deal,req['plays'],req['decl'],req['bidder']);assert (pp,ll,tt)==(points,leader,tail);assert sorted(deal[req['seat']])==req['hand'];world_checks+=1
assert data['vector_checks']==90 and len(data['records'])==6
rerun_path=Path(__file__).resolve().parent/'reuse-rerun/stdout.log'
if rerun_path.exists():
 fresh=json.loads(rerun_path.read_text());assert fresh['vector_checks']==90
 for a,b in zip(data['records'],fresh['records']):
  for key in ['counted','field','roots','independent_cold_misses','successive_service_misses','actor_queries_avoided_by_cross_root_cache','independent_inner_worlds','successive_inner_worlds','inner_worlds_avoided','roots_detail']:assert a[key]==b[key],key
report=[]
for r in data['records']:
 details=r['roots_detail'];stats=r['final_stats'];assert r['roots']==len(details)==15
 assert [x['seed'] for x in details]==[x['seed'] for x in rows]
 for x,row in zip(details,rows):
  played=sum(1<<t for t in row['request']['plays'][1::2]);assert x['public_played']==played
  assert x['boundary_size']+bin(x['boundary']).count('1')//4==7;assert x['boundary']&~played==0
  assert x['cold_misses']>=x['reused_misses'];assert x['cold_inner_worlds']>=x['reused_inner_worlds'];assert x['new_completed_cache_entries']>=1
 assert r['independent_cold_misses']==sum(x['cold_misses'] for x in details)
 assert r['successive_service_misses']==sum(x['reused_misses'] for x in details)==stats['generated_actor_queries']==stats['completed_actor_queries']==sum(stats['unique_actor_misses_by_level'])
 assert r['actor_queries_avoided_by_cross_root_cache']==r['independent_cold_misses']-r['successive_service_misses']
 assert r['independent_inner_worlds']==sum(x['cold_inner_worlds'] for x in details)
 assert r['successive_inner_worlds']==sum(x['reused_inner_worlds'] for x in details)==sum(stats['inner_worlds_by_level'])
 assert stats['inner_worlds_by_level']==[m*n for m,n in zip(stats['unique_actor_misses_by_level'],[4,2,2,2])]
 assert r['inner_worlds_avoided']==r['independent_inner_worlds']-r['successive_inner_worlds']
 assert sum(x['new_completed_cache_entries'] for x in details)==stats['cache_entries']
 assert sum(x['reused_hits'] for x in details)==stats['cache_hits'];assert sum(x['lookups'] for x in details)==stats['query_lookups']
 assert r['cold_service_sum_us']>0 and r['successive_service_sum_us']>0
 report.append({k:r[k] for k in ['counted','field','independent_cold_misses','successive_service_misses','actor_queries_avoided_by_cross_root_cache','independent_inner_worlds','successive_inner_worlds','inner_worlds_avoided']})
for k in ['serialization_s','output_parse_s','harness_overhead_s','child_cpu_s']:assert result[k]>=0
assert result['host_s']>0 and result['process_peak_rss_bytes']>0
sources=['reuse.py','reuse/src/main.rs','results/reuse-final/inputs.json','results/reuse-final/result.json','results/reuse-final/generation.json']
print(json.dumps({'roots':15,'distinct_reduced_public_root_signatures':len(identities),'generating_deals':len(histories),'historically_legal_outer_world_checks':world_checks,'selected_fixture_generation_reproduced':True,'recorded_vector_checks':90,'fresh_rerun_counters_equal':rerun_path.exists(),'arithmetic':report,'validation_process_cpu_s':result['child_cpu_s'],'validation_process_rss_bytes':result['process_peak_rss_bytes'],'charged_generation_s':g['generation_actual_n_s'],'charged_fixture_serialization_write_s':g['fixture_serialization_write_s'],'recorded_profile_cost_s':sum(result[k] for k in ['serialization_s','host_s','output_parse_s','harness_overhead_s'])+g['generation_actual_n_s']+g['fixture_serialization_write_s'],'hashes':{s:hashlib.sha256((HERE/s).read_bytes()).hexdigest() for s in sources},'qualification':'One sequence of distinct filtered positions, with independently sampled support worlds per root. Arithmetic/source assertions of90vector checks audited; independently rerun profile supplies parity. Service sums interleave cold,reused,native work and are illustrative. Whole-process resources include all validation; no standalone latency or natural-match cache rate. Recorded profile sum excludes local wrapper construction/chmod and writing result/generation artifacts.'},indent=2))
