#!/usr/bin/env python3
"""Audit cold same-policy retained arithmetic/input and process identity."""
import gzip,hashlib,json,math,statistics,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/native-policy-check-20261004';base=HERE/'results/cold-same-policy'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def close(a,b):assert math.isclose(a,b,rel_tol=1e-11,abs_tol=1e-10),(a,b)
summary=json.loads((base/'summary.json').read_text());inputs=json.loads((base/'public-inputs.json').read_text());records=json.loads(gzip.decompress((base/'records.json.gz').read_bytes()));assert len(inputs)==12 and len(records)==72
plan=json.loads((HERE/'plan.json').read_text());seeds=list(dict.fromkeys(r['seed']for r in plan['games']));selected=[]
for seed in seeds:
 for p in sorted((HERE/'results/native-games').glob(f'game-{seed}-*.json')):
  game=json.loads(p.read_text());late=next((m for m in game['moves']if m['candidate']and m['response']['route']=='late-l3-native-40-4-2-2'),None)
  if late:selected.append(dict(source_deal_seed=seed,request=late['call']['request'],expected=late['response']['evaluation']));break
 if len(selected)==12:break
assert selected==inputs and len({r['source_deal_seed']for r in inputs})==12
names=['resumable','arena','lazy','control-native','native','wasm'];positions={n:[0]*6 for n in names};identities=set();complete=0
for r in records:
 rep=r['repeat'];i=r['input_index'];assert (rep,i)not in identities;identities.add((rep,i));assert r['source_deal_seed']==inputs[i]['source_deal_seed'];order=names[(rep+i)%6:]+names[:(rep+i)%6];assert r['order']==order and set(r['runs'])==set(names)
 assert len(inputs[i]['request'])==7 and set(inputs[i]['request'])=={'decl','bid','bidder','seat','hand','plays','seed'}
 for n in names:
  positions[n][order.index(n)]+=1;x=r['runs'][n];assert x['result']['evaluation']==inputs[i]['expected']and 'error'not in x['result'];complete+=1
  for k in ['serialization_s','host_s','parse_s','harness_s','child_cpu_s','rss_bytes']:assert x[k]>=0,(n,k,x[k])
  close(x['total_s'],sum(x[k]for k in ['serialization_s','host_s','parse_s','harness_s']))
assert identities=={(r,i)for r in range(6)for i in range(12)}and all(pos==[12]*6 for pos in positions.values())
for n in names:
 rr=[r['runs'][n]for r in records];s=summary['variants'][n];assert s['calls']==s['complete']==72 and not s['errors'];totals=[sum(r['runs'][n]['total_s']for r in records if r['repeat']==rep)*1000 for rep in range(6)]
 close(s['panel_total_median_ms'],statistics.median(totals));close(s['cold_request_median_ms'],statistics.median(x['total_s']for x in rr)*1000);close(s['child_cpu_median_ms'],statistics.median(x['child_cpu_s']for x in rr)*1000);assert s['peak_rss_median_bytes']==statistics.median(x['rss_bytes']for x in rr);assert s['sample_median_us']==statistics.median(x['result']['outer_sample_us']for x in rr);assert s['wasm_linear_memory_peak']==max(x['wasm_memory_bytes']or 0 for x in rr)
for key,path in [('control',HERE/'control-target/release/late-choice-player'),('native',HERE/'adapter/target/release/native-late-player'),('wasm',HERE/'adapter/target/wasm32-unknown-unknown/release/native_late_player.wasm')]:assert summary['binary_sha256'][key]==sha(path)
assert summary['complete_vector_comparisons']==complete==432 and summary['all_complete']and summary['requests_per_variant']==72 and summary['workers_per_request']==1 and summary['outer_worlds']==40 and summary['budgets']==[4,2,2,2]and summary['field']=='Level(2)'and summary['inner_belief']=='Voidless'and summary['ties']=='ascending'
print(json.dumps(dict(actual_source_deals=12,selected_public_turns=12,rotated_repetitions=6,full_vectors=432,refusals=0,balanced_process_positions=positions,checked_all_complete_cost_sums_and_medians=True,actual_binary_hashes_checked=True,cold_panel_median_ms={n:summary['variants'][n]['panel_total_median_ms']for n in names},scope='Retained same-policy actual-turn cold process/request arithmetic. Common persistent Python driver startup/imports and saved-referee selection are outside request intervals; selection reported separately. Node compilation/instantiation included. No timing rerun, physical-device or whole-game strength inference.'),indent=2))
