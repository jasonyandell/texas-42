#!/usr/bin/env python3
"""Independent frozen-panel reconstruction, complete suffix replay and arithmetic.
No solver or timing reruns. Run under run_capped.py.
"""
import argparse,collections,hashlib,json,math,random,statistics,sys
from pathlib import Path
sys.dont_write_bytecode=True
CHECK=Path(__file__).resolve().parent;HERE=CHECK.parent;ROOT=HERE.parents[1]
DOMINO=[(high,low)for high in range(7)for low in range(high+1)]
def trumps(tile,decl):return decl in DOMINO[tile]if decl<7 else decl==7 and DOMINO[tile][0]==DOMINO[tile][1]
def lead(tile,decl):return -1 if trumps(tile,decl)else DOMINO[tile][0]
def follows(tile,context,decl):return trumps(tile,decl)if context==-1 else not trumps(tile,decl)and context in DOMINO[tile]
def legal(hand,trick,decl):
    following=[tile for tile in hand if trick and follows(tile,lead(trick[0][1],decl),decl)]
    return sorted(following or hand)
def winner(trick,decl):
    def key(play):
        high,low=DOMINO[play[1]]
        return (2 if trumps(play[1],decl)else 1 if follows(play[1],lead(trick[0][1],decl),decl)else 0,high if decl==7 and high==low else 12 if high==low else high+low)
    return max(trick,key=key)[0]
def count(trick):return 1+sum(sum(DOMINO[tile])if sum(DOMINO[tile])in[5,10]else 0 for _,tile in trick)
def read(path):return json.loads(path.read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def replay(hands,plays,decl,bidder):
    remaining=list(map(set,hands));leader=bidder;trick=[];points=[0,0]
    for actor,tile in zip(plays[::2],plays[1::2]):
        assert actor==(leader+len(trick))%4 and tile in legal(remaining[actor],trick,decl)
        remaining[actor].remove(tile);trick.append((actor,tile))
        if len(trick)==4:leader=winner(trick,decl);points[leader%2]+=count(trick);trick=[]
    return remaining,leader,trick,points
def rotated(fixture,rotation,plies):
    hands=[fixture['hands'][(seat-rotation)%4]for seat in range(4)]
    plays=[v if i%2 else (v+rotation)%4 for i,v in enumerate(fixture['prefixes'][str(plies)])]
    return hands,plays
def request(hands,plays,decl,bidder):
    _,leader,trick,_=replay(hands,plays,decl,bidder);actor=(leader+len(trick))%4
    return dict(decl=decl,bid=30,bidder=bidder,seat=actor,hand=hands[actor],plays=plays,seed=7042104)
def signature(v):return json.dumps(v,sort_keys=True,separators=(',',':'))
def identity(value):
    artifacts=read(HERE/'ARTIFACTS.json')
    for field,labels in [('native_sha256',['measured_native','final_native']),('wasm_sha256',['measured_wasm','final_wasm'])]:
        matches=[artifacts[label]for label in labels if artifacts[label]['sha256']==value[field]]
        assert len(matches)==1 and sha(ROOT/matches[0]['path'])==value[field]
    assert value['plan_sha256']==sha(HERE/'plan.json')and value['fixtures_sha256']==sha(HERE/'fixtures.json')
    for name,digest in value['source_sha256'].items():
        path=ROOT/name
        if name=='experiments/higher-k-budget-20261004/experiment.py'and sha(path)!=digest:
            snapshots=[HERE/'snapshots/experiment-measured.py',HERE/'snapshots/experiment-parallel-measured.py']
            matching=[p for p in snapshots if sha(p)==digest]
            assert len(matching)==1, 'Unrecognized measured source identity'
            path=matching[0]
        if name=='experiments/higher-k-budget-20261004/adapter/src/lib.rs'and sha(path)!=digest:
            path=HERE/'snapshots/adapter-measured.rs'
        assert sha(path)==digest,name
def response(req,v,allowance):
    # Own/public state and exact completed root tie arithmetic, without hidden
    # hands entering chooser requests. The independent full referee owns hands.
    assert set(req)=={'decl','bid','bidder','seat','hand','plays','seed'}
    if 'completed'in v:
        assert v['inner_budgets']==[4,2,2,2,2,2]and v['field_level']==v['k']-1 and v['outer_worlds']==40 and v['budget_ms']==allowance
        if v['completed']:
            e=v['evaluation'];assert e['tiles']==v['legal']and len(e['counts'])==len(e['tiles'])
            assert all(type(n)is int and 0<=n<=40 for n in e['counts'])
            best=(max if req['seat']%2==req['bidder']%2 else min)(e['counts'])
            assert v['choice']==e['choice']==e['tiles'][e['counts'].index(best)]
            assert v['outer_accepted']==40 and v['outer_attempts']>=40
        else:
            assert 'evaluation'not in v and v['choice']==v['legal'][0]
            assert v.get('ineligible')or v.get('refusal')in['outer-sampling-budget','solver-deadline']
        for field in ['pi_calls_by_level','inner_worlds_by_level']:
            if field in v:assert len(v[field])==6 and all(type(x)is int and x>=0 for x in v[field])
        # No modeled rung higher than the selected field can be computed.
        if 'pi_calls_by_level'in v:assert all(n==0 for n in v['pi_calls_by_level'][v['k']:])
def stats(values):
    x=sorted(values);return dict(n=len(x),sum=sum(x),median=statistics.median(x)if x else None,p95=x[math.ceil(.95*len(x))-1]if x else None,max=max(x)if x else None)
def close(actual,expected):
    if isinstance(expected,dict):
        for key,v in expected.items():close(actual[key],v)
    elif isinstance(expected,list):
        assert len(actual)==len(expected)
        for a,b in zip(actual,expected):close(a,b)
    elif isinstance(expected,float):assert math.isclose(actual,expected,rel_tol=1e-11,abs_tol=1e-9),(actual,expected)
    else:assert actual==expected,(actual,expected)
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--partial',action='store_true');a=parser.parse_args()
    plan=read(HERE/'plan.json');fixtures=read(HERE/'fixtures.json');seeds=random.Random(202610041202).sample(range(3000000,4000000),54)
    for artifact in read(HERE/'ARTIFACTS.json').values():
        path=ROOT/artifact['path'];assert sha(path)==artifact['sha256']and path.stat().st_size==artifact['bytes']
    old=(HERE/'snapshots/adapter-measured.rs').read_text()
    expected=old.replace('let k=scalar(input,"k")? as usize;','let raw_k=scalar(input,"k")?;\n    if !(1..=5).contains(&raw_k) {return Err("invalid k/budget".into());}\n    let k=raw_k as usize;').replace('if !(1..=5).contains(&k) || ms>30_000','if ms>30_000')
    assert expected==(HERE/'adapter/src/lib.rs').read_text(),'Final adapter changed beyond raw-k guard'
    assert plan['seeds']==seeds and len(fixtures)==54
    frozen=read(HERE/'results/initialize/stdout.log')
    assert frozen['plan_sha256']==sha(HERE/'plan.json')and frozen['fixtures_sha256']==sha(HERE/'fixtures.json')
    for index,(seed,f)in enumerate(zip(seeds,fixtures)):
        assert f['seed']==seed and f['decl']==(*range(8),9)[index%9]
        deck=list(range(28));random.Random(seed).shuffle(deck);hands=[sorted(deck[s*7:(s+1)*7])for s in range(4)];assert f['hands']==hands
        rng=random.Random(seed^0x507265666978);remaining=list(map(set,hands));leader=0;plays=[]
        for t in range(4):
            trick=[]
            for turn in range(4):
                actor=(leader+turn)%4;tile=rng.choice(legal(remaining[actor],trick,f['decl']));remaining[actor].remove(tile);plays.extend([actor,tile]);trick.append((actor,tile))
            leader=winner(trick,f['decl'])
            if t>=2:assert f['prefixes'][str(4*(t+1))]==plays
    fixture_index={f['seed']:f for f in fixtures}
    root_files=[read(p)for p in sorted((HERE/'results/fixed-roots').glob('deal-*.json'))]
    root_records=[]
    for f in root_files:
        identity(f['identity']);rows=f['records'];seed=rows[0]['seed'];fixture=fixture_index[seed]
        assert len(rows)==40
        keys=set()
        for q in rows:
            r=q['rotation'];b=q['call']['budget_ms'];k=q['call']['k'];assert q['seed']==seed and q['decl']==fixture['decl']and q['plies']==16
            assert q['call']==dict(action='ladder',request=request(*rotated(fixture,r,16),fixture['decl'],r),k=k,budget_ms=b,validate=False)
            assert (r,b,k)not in keys;keys.add((r,b,k));response(q['call']['request'],q['response'],b)
        assert keys=={(r,b,k)for r in range(4)for b in[20,200]for k in range(1,6)}
        root_records+=rows
    game_files=[read(p)for p in sorted((HERE/'results/games').glob('deal-*.json'))];games=[];moves=0
    for f in game_files:
        identity(f['identity']);rows=f['games'];seed=rows[0]['seed'];fixture=fixture_index[seed]
        assert len(rows)==56;keys=set()
        for g in rows:
            assert g['seed']==seed and g['decl']==fixture['decl']and g['complete']and len(g['moves'])==12
            r=g['rotation'];hands,plays=rotated(fixture,r,16);assert plays==g['initial_plays'];team=(r+(g['role']=='defending'))%2
            key=(r,g['role'],g['reference'],g['candidate'],g['budget_ms']);assert key not in keys;keys.add(key)
            for m in g['moves']:
                req=request(hands,plays,fixture['decl'],r);assert req==m['request']
                remain,leader,trick,points=replay(hands,plays,fixture['decl'],r);v=m['response'];allowed=legal(remain[req['seat']],trick,fixture['decl'])
                assert v['choice']in allowed and v['legal']==allowed and v['points']==points and v['leader']==leader
                cand=req['seat']%2==team;assert m['candidate']==cand and m['policy']==(g['candidate']if cand else g['reference'])
                response(req,v,g['budget_ms']);plays=plays+[req['seat'],v['choice']];moves+=1
            remain,_,trick,points=replay(hands,plays,fixture['decl'],r)
            assert points==g['points']and sum(points)==42 and not trick and all(not h for h in remain)and g['made']==(points[r%2]>=30)
        assert keys=={(r,role,1 if cell['reference']=='k1'else 'phone',cell['candidate'],cell['budget_ms'])for cell in plan['matches']for r in range(4)for role in['declaring','defending']}
        games+=rows
    earlier_files=[read(p)for p in sorted((HERE/'results/earlier-roots').glob('deal-*.json'))]
    earlier_rows=[]
    if earlier_files:
        earlier_plan=read(HERE/'earlier-plan.json')
        assert earlier_plan['seeds']==seeds and earlier_plan['prefix_plies']==12 and earlier_plan['new_independent_deals']==0
        assert earlier_plan['prior_plan_sha256']==sha(HERE/'plan.json')and earlier_plan['fixture_sha256']==sha(HERE/'fixtures.json')
        for f in earlier_files:
            identity(f['identity']);assert f['earlier_plan_sha256']==sha(HERE/'earlier-plan.json')and f['extension_sha256']in[sha(HERE/'extension.py'),sha(HERE/'snapshots/extension-measured.py')]
            rows=f['records'];seed=rows[0]['seed'];fixture=fixture_index[seed];assert len(rows)==40;keys=set()
            retained=[json.loads(line)for line in (HERE/'results/earlier-roots'/f'partial-{seed}.jsonl').read_text().splitlines()]
            assert retained==rows
            for q in rows:
                r=q['rotation'];b=q['call']['budget_ms'];k=q['call']['k'];assert q['seed']==seed and q['decl']==fixture['decl']and q['plies']==12
                assert q['call']==dict(action='ladder',request=request(*rotated(fixture,r,12),fixture['decl'],r),k=k,budget_ms=b,validate=False)
                assert (r,b,k)not in keys;keys.add((r,b,k));response(q['call']['request'],q['response'],b)
            assert keys=={(r,b,k)for r in range(4)for b in[20,200]for k in range(1,6)}
            earlier_rows+=rows
    if not a.partial:
        assert len(root_files)==len(game_files)==54
        summary=read(HERE/'results/summary.json');assert summary['source_deals']==54 and summary['suffix_games']==len(games)==3024 and summary['suffix_moves']==moves==36288
        for s in summary['matches']:
            reference=1 if s['reference']=='k1'else 'phone'
            rows=[g for g in games if(g['reference'],g['candidate'],g['budget_ms'])==(reference,s['candidate'],s['budget_ms'])]
            index={(g['seed'],g['rotation'],g['role']):g for g in rows};clusters=[];differences=[]
            for seed in seeds:
                dd=[int(index[seed,r,'declaring']['made'])-int(index[seed,r,'defending']['made'])for r in range(4)];differences+=dd;clusters.append(dict(seed=seed,differences=dd,mean=sum(dd)/4))
            mean=statistics.mean(x['mean']for x in clusters);rad=math.sqrt(2*math.log(40)/54)
            close(s,dict(games=432,clusters=clusters,pair_wins=differences.count(1),pair_losses=differences.count(-1),pair_ties=differences.count(0),mean=mean,hoeffding95=[max(-1,mean-rad),min(1,mean+rad)]))
            for is_candidate,label in[(True,'candidate_cost'),(False,'reference_cost')]:
                mm=[m for g in rows for m in g['moves']if m['candidate']==is_candidate];attempts=[m for m in mm if 'completed'in m['response']and not m['response'].get('ineligible')]
                close(s[label],dict(turns=len(mm),host_ms=stats([m['timing']['host_ms']for m in mm]),ladder_attempts=len(attempts),completed=sum(m['response']['completed']for m in attempts),refused=sum('refusal'in m['response']for m in attempts),nodes=sum(m['response'].get('nodes',0)for m in attempts),pi_computations_by_level=[sum(m['response'].get('pi_calls_by_level',[0]*6)[k]for m in attempts)for k in range(6)]))
        for s in summary['fixed_roots']:
            qs=[q for q in root_records if(q['call']['k'],q['call']['budget_ms'])==(s['k'],s['budget_ms'])];eligible=[q for q in qs if not q['response'].get('ineligible')]
            close(s,dict(planned=216,eligible=len(eligible),completed=sum(q['response']['completed']for q in eligible),refused=sum('refusal'in q['response']for q in eligible),host_ms=stats([q['timing']['host_ms']for q in qs]),nodes=stats([q['response'].get('nodes',0)for q in eligible]),pi_computations_by_level=[sum(q['response'].get('pi_calls_by_level',[0]*6)[k]for q in eligible)for k in range(6)],inner_worlds_by_level=[sum(q['response'].get('inner_worlds_by_level',[0]*6)[k]for q in eligible)for k in range(6)]))
        if earlier_files:
            assert len(earlier_files)==54 and len(earlier_rows)==2160
            earlier=read(HERE/'results/earlier-summary.json')
            assert earlier['new_independent_deals']==0 and earlier['earlier_plan_sha256']==sha(HERE/'earlier-plan.json')
            for s in earlier['cells']:
                qs=[q for q in earlier_rows if(q['call']['k'],q['call']['budget_ms'])==(s['k'],s['budget_ms'])];eligible=[q for q in qs if not q['response'].get('ineligible')]
                close(s,dict(planned=216,eligible=len(eligible),completed=sum(q['response']['completed']for q in eligible),refused=sum('refusal'in q['response']for q in eligible),host_ms=stats([q['timing']['host_ms']for q in qs]),eligible_host_ms=stats([q['timing']['host_ms']for q in eligible]),nodes=stats([q['response'].get('nodes',0)for q in eligible]),pi_computations_by_level=[sum(q['response'].get('pi_calls_by_level',[0]*6)[k]for q in eligible)for k in range(6)],inner_worlds_by_level=[sum(q['response'].get('inner_worlds_by_level',[0]*6)[k]for q in eligible)for k in range(6)],wrapper_overruns=stats([q['timing']['wrapper_overrun_ms']for q in qs])))
    receipts=[]
    for p in sorted((HERE/'results').rglob('run.json')):
        r=read(p);assert 0<r['allowance_seconds']<=295 and r['elapsed_seconds']<300 and not r['cleanup_errors']and r['child_returncode']is not None
        receipts.append(dict(path=str(p.relative_to(HERE)),status=r['status']))
    print(json.dumps(dict(frozen_source_deals=54,reconstructed_fixtures=54,complete_root_deal_files=len(root_files),complete_game_deal_files=len(game_files),root_calls=len(root_records),earlier_root_calls=len(earlier_rows),earlier_new_independent_deals=0,independently_replayed_suffix_games=len(games),independently_replayed_suffix_moves=moves,complete_panel=not a.partial,source_identity_matches=True,capped_receipts=receipts,scope='Finite public-state/referee/arithmetic audit; conditional PRNG deal independence, random-prefix suffix distribution, no fresh timing or player-strength theorem.'),indent=2))
if __name__=='__main__':main()
