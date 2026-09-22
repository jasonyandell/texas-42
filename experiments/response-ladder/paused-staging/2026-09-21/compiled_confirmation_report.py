#!/usr/bin/env python3
"""Strict full-panel confirmation report. No arm selection or partial inference.

Use only after both frozen panels finish. Reads receipts and replays games;
never calls player binaries or the harness's mutating analyze function.
"""
import argparse
from collections import Counter, defaultdict
from fractions import Fraction
import hashlib
import importlib.util
import json
import math
from pathlib import Path
import re
import shlex
import subprocess
import sys
import time

DECLS = (0,1,2,3,4,5,6,7,9)
ARMS = ('declaring','defending')
Z99 = 2.3263478740408408
Z95 = 1.959963984540054


def require(ok, why):
    if not ok:
        raise ValueError(why)


def canonical(value):
    return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=True)


def read(path):
    return json.loads(Path(path).read_text())


def digest(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as stream:
        for chunk in iter(lambda:stream.read(1024*1024),b''): h.update(chunk)
    return h.hexdigest()


def load_module(name,path):
    spec = importlib.util.spec_from_file_location(name,path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def frac(value):
    require(type(value) in (int,float) and math.isfinite(value) and value>=0,'invalid elapsed milliseconds')
    return Fraction(str(value))


def rational(value):
    value = Fraction(value)
    return {'numerator':value.numerator,'denominator':value.denominator,'decimal':float(value)}


def nearest_rank(values,p):
    require(bool(values) and 0<p<=1,'invalid quantile population/probability')
    ordered = sorted(values)
    p = Fraction(p)
    index = (p.numerator*len(ordered)+p.denominator-1)//p.denominator-1
    return ordered[index]


def latency_stats(values):
    require(bool(values),'empty latency population')
    values = [v if isinstance(v,Fraction) else frac(v) for v in values]
    mean = sum(values,Fraction())/len(values)
    return {'n':len(values),'mean_ms':float(mean),'mean_ms_exact':rational(mean),
        'median_ms':float(nearest_rank(values,Fraction(1,2))),
        'p90_ms':float(nearest_rank(values,Fraction(9,10))),
        'p95_ms':float(nearest_rank(values,Fraction(19,20))),
        'p95_ms_exact':rational(nearest_rank(values,Fraction(19,20))),
        'p99_ms':float(nearest_rank(values,Fraction(99,100))),
        'max_ms':float(max(values)),'total_ms':float(sum(values,Fraction()))}


def sample_stats(deltas):
    require(len(deltas)>1 and all(type(x) is int and x in (-1,0,1) for x in deltas),'invalid paired outcomes')
    n = len(deltas)
    mean = Fraction(sum(deltas),n)
    variance = (sum(Fraction(x*x) for x in deltas)-n*mean*mean)/(n-1)
    return mean,variance


def sign_probability(wins,losses):
    require(type(wins) is int and type(losses) is int and min(wins,losses)>=0,'invalid sign counts')
    n = wins+losses
    return Fraction(sum(math.comb(n,k) for k in range(wins,n+1)),1<<n) if n else Fraction(1)


def inference(strata,panels):
    require(len(panels)==2 and len(set(panels))==2,'need exactly two panels')
    expected = {(p,d,b) for p in panels for d in DECLS for b in range(4)}
    require(set(strata)==expected,'missing/extra panel-by-cell strata')
    cells,means,variances,all_deltas = [],[],[],[]
    panel_deltas = defaultdict(list)
    for key in sorted(strata):
        pairs = strata[key]
        require(len(pairs)==16,'stratum must have16 physical pairs')
        ds = [p['delta'] for p in pairs]
        mean,var = sample_stats(ds)
        means.append(mean);variances.append(var);all_deltas.extend(ds)
        panel_deltas[key[0]].extend(ds)
        cells.append({'panel':key[0],'declaration':key[1],'bidder':key[2],'pairs':16,
            'wins':ds.count(1),'losses':ds.count(-1),'ties':ds.count(0),
            'mean':rational(mean),'unbiased_sample_variance':rational(var),
            'candidate_declaring_make_rate':rational(Fraction(sum(p['candidate_declaring_make'] for p in pairs),16)),
            'candidate_defending_set_rate':rational(Fraction(sum(p['candidate_defending_set'] for p in pairs),16))})
    delta = sum(means,Fraction())/72
    require(delta==Fraction(sum(all_deltas),1152),'balanced estimand/pooled mean mismatch')
    variance = sum((v/16 for v in variances),Fraction())/(72*72)
    wins,losses,ties = (all_deltas.count(k) for k in (1,-1,0))
    exact_sign = sign_probability(wins,losses)
    se = math.sqrt(float(variance))
    lower = float(delta)-Z99*se if variance>0 else None
    panel_results = {p:{'pairs':len(ds),'wins':ds.count(1),'losses':ds.count(-1),'ties':ds.count(0),
        'mean':rational(Fraction(sum(ds),len(ds)))} for p,ds in panel_deltas.items()}
    for p,entry in panel_results.items():
        own=[cell for cell in cells if cell['panel']==p]
        pv=sum((Fraction(c['unbiased_sample_variance']['numerator'],c['unbiased_sample_variance']['denominator'])/16 for c in own),Fraction())/(36*36)
        entry['variance_estimate']=rational(pv)
        entry['standard_error']=math.sqrt(float(pv))
        all_pairs=[pair for key,pairs in strata.items() if key[0]==p for pair in pairs]
        entry['candidate_declaring_make_rate']=rational(Fraction(sum(x['candidate_declaring_make'] for x in all_pairs),576))
        entry['candidate_defending_set_rate']=rational(Fraction(sum(x['candidate_defending_set'] for x in all_pairs),576))
    half = math.sqrt(2*math.log(2/0.05)/len(all_deltas))
    return {'estimand':'equal mean of 72 panel-by-cell means with 16 pairs each',
        'pairs':1152,'games':2304,'wins':wins,'losses':losses,'ties':ties,'delta':rational(delta),
        'variance_estimate':rational(variance),'standard_error':se,
        'mean_inference_status':'resolved' if variance>0 else 'unresolved_zero_variance',
        'one_sided_99_normal_lower':lower,
        'one_sided_normal_p':0.5*math.erfc(float(delta)/(se*math.sqrt(2))) if variance>0 else None,
        'two_sided_95_normal_interval':[float(delta)-Z95*se,float(delta)+Z95*se] if variance>0 else None,
        'exact_conditional_sign_probability':rational(exact_sign),
        'hoeffding_95_interval':[max(-1.,float(delta)-half),min(1.,float(delta)+half)],
        'hoeffding_half_width':half,'panels':panel_results,'cells':cells,
        'gates':{'positive_each_panel':all(sum(ds)>0 for ds in panel_deltas.values()),
                 'positive_variance':variance>0,'normal99_lower_positive':lower is not None and lower>0,
                 'exact_sign_at_most_001':exact_sign<=Fraction(1,100)},
        'interpretation':'Normal inference approximates a heterogeneous mean-effect test. Exact sign conditions on discordance under sharp within-cell exchangeability; it is not an exact heterogeneous mean test. Probabilities are not multiplied.'}


def validate_spec(spec):
    require(spec['schema']=='compiled-confirmation-protocol-v1','unknown confirmation schema')
    require(len(spec['panels'])==2 and len(set(spec['panels']))==2,'two panel names required')
    expected = {'deals_per_cell':16,'planned_pairs_per_panel':576,'planned_pairs_total':1152,'planned_games_total':2304,
                'candidate_ms':20,'cpu_ms':14000,'threads':6}
    require(all(spec.get(k)==v for k,v in expected.items()),'frozen design constants differ')
    require(spec['candidate_config']=={'outer':40,'plans':1,'horizon':7,'work':2000000,'compiled_tail':True},'candidate config differs')
    a = spec['analysis']
    for k,v in {'strata':72,'pairs_per_stratum':16,'mean_test_z':Z99,'mean_test_alpha':0.01,
                'sign_test_alpha':0.01,'latency_max_ratio_each_panel':1.1,'positive_each_panel':True,
                'mean_test_requires_positive_variance':True,'technical_failures_allowed':0,'missing_pairs_allowed':0,
                'automatic_promotion':False,'quantile':'nearest-rank ceil(p*n), one-based'}.items():
        require(a.get(k)==v,'frozen analysis differs: '+k)
    require(spec['no_outcome_stopping'] is True and spec['further_candidates_allowed'] is False,'stopping rule differs')
    doc = spec['authority_document']
    require(digest(doc['path'])==doc['sha256'],'confirmation authority document drift')


def check_current_identities(identities,harness,harness_path):
    for key in ('c0','c1','cpu_binary'):
        require(harness.input_identity(Path(identities[key]['path']))==identities[key],'current file identity drift: '+key)
    command = identities['candidate_command']
    require(harness.executable_identity(command['argv'])==command,'current candidate binary/command drift')
    for key,entry in identities['candidate_sources'].items():
        require(harness.input_identity(Path(entry['path']))==entry,'current source identity drift: '+key)
    cpu_root = Path(identities['cpu_root'])
    for name,sha in identities['cpu_sources'].items():
        require(digest(cpu_root/name)==sha,'current CPU source drift: '+name)
    head = subprocess.check_output(['git','rev-parse','HEAD'],cwd=cpu_root,text=True).strip()
    require(head==identities['cpu_head'],'current CPU head drift')
    require(digest(harness_path)==identities['harness_sha256'],'current harness drift')


def check_fixtures(fixtures,panel,spec,harness):
    require(fixtures==harness.fixture_panel(panel,16),'fixture list differs from frozen generator')
    require(hashlib.sha256(canonical(fixtures).encode()).hexdigest()==spec['fixture_hashes'][panel],'fixture hash drift')
    require(len(fixtures)==576 and {f['index'] for f in fixtures}==set(range(576)),'global fixture indices invalid')
    cells = Counter((f['decl'],f['bidder']) for f in fixtures)
    require(cells==Counter({(d,b):16 for d in DECLS for b in range(4)}),'unbalanced fixture cells')
    for f in fixtures:
        require(f['decl'] in DECLS and f['bidder'] in range(4),'unknown contract cell')
        require(f['balanced_order']==(list(ARMS) if f['repeat']%2==0 else list(reversed(ARMS))),'arm order changed')
        require(sorted(x for hand in f['hands'] for x in hand)==list(range(28)) and all(len(h)==7 for h in f['hands']),'invalid deal partition')


def check_cycles(records,identities,planned,threads):
    starts,ends = {},{}
    previous = 0
    for record in records:
        at = record['started_at']
        if record.get('schema')=='compiled-h2h-cycle-v1':
            require(at not in ends and at in starts,'duplicate/unpaired cycle end')
            require(record['start_identities']==identities and record['end_identities']==identities
                    and record['identity_drift'] is False,'cycle identity drift')
            require(record['failed_arms']==0 and record['planned_arms']==planned,'cycle failed/planned counts differ')
            require(record['stop_reason'] in ('budget','exhausted'),'technical cycle failure: '+record['stop_reason'])
            require(record['threads']==threads and record['seconds']<=55,'cycle runtime configuration differs')
            require(previous<=record['completed_arms']<=planned,'cycle completion count regressed')
            require(record['finished_at']>=at,'negative cycle duration')
            previous = record['completed_arms'];ends[at]=record
        else:
            require(at not in starts and 'warmup' in record,'unknown/duplicate cycle start')
            warm = record['warmup']
            require(warm['started_at']==at and warm['start_identities']==identities,'startup identity mismatch')
            require(warm['threads']==threads and 'error' not in warm,'startup failure/configuration drift')
            require(warm['ready'].get('backend')=='compiled-cpu','startup backend is not nominated CPU')
            require('error' not in warm['candidate_warmup'] and 'tile' in warm['candidate_warmup'],'candidate warmup failed')
            require('choice' in warm['cpu_warmup'],'CPU warmup failed')
            starts[at]=record
    require(starts and set(starts)==set(ends),'missing cycle receipt; possibly interrupted watchdog')
    last = list(ends.values())[-1]
    require(last['completed_arms']==planned and last['summary_complete'] is True,'last cycle is incomplete')
    return {'cycles':len(ends),'last_receipt':last,
        'startup_ms':latency_stats([x['warmup']['startup_ms'] for x in starts.values()]),
        'candidate_warmup_ms':latency_stats([x['warmup']['candidate_warmup_ms'] for x in starts.values()]),
        'cpu_warmup_ms':latency_stats([x['warmup']['cpu_warmup_ms'] for x in starts.values()])}



def check_outer_cycles(spec_path,panel,output,spec,harness_path,inner_count):
    paths=sorted(spec_path.parent.glob(panel+'-cycle-*/run.json'))
    # R2 manifest-driven runner provides an independent outer-watchdog receipt
    # for every cycle, including preflight refusals before an inner receipt.
    require(paths or 'output_directories' not in spec,'missing outer-watchdog receipts')
    if not paths: return {}
    require(len(paths)==inner_count,'outer/inner cycle counts differ')
    hashes={}
    for index,path in enumerate(paths,1):
        receipt=read(path)
        require(path.parent.name==f'{panel}-cycle-{index:03d}','missing/duplicate outer cycle sequence')
        require(receipt['schema']=='texas42-partnership-run-v1' and receipt['status']=='completed',
                'outer watchdog did not complete: '+str(path))
        require(receipt['child_returncode']==0 and receipt['runner_returncode']==0
                and receipt['interruption_signal'] is None and receipt['error'] is None
                and not receipt['cleanup_errors'],'outer cycle failed: '+str(path))
        require(receipt['allowance_seconds']==60 and receipt['elapsed_seconds']<=60,'outer watchdog allowance/time differs')
        command=receipt['command']
        require(Path(command[1]).resolve()==harness_path.resolve(),'outer harness command differs')
        arguments=command[2:]
        require(len(arguments)%2==0,'invalid outer command arguments')
        options=dict(zip(arguments[::2],arguments[1::2]))
        require(len(options)*2==len(arguments),'duplicate outer command option')
        expected={'--output':str(output.resolve()),'--cpu-root':spec['identities']['cpu_root'],
            '--panel':panel,'--deals-per-cell':'16','--seconds':'55','--candidate-ms':'20','--threads':'6',
            '--confirmation-protocol':str(spec_path.resolve())}
        require(all(options.get(k)==v for k,v in expected.items()),'outer frozen command differs')
        require(shlex.split(options['--candidate-command'])==spec['identities']['candidate_command']['argv'],
                'outer candidate command differs')
        require(read(options['--config-json'])==spec['candidate_config'],'outer config differs')
        hashes[str(path.resolve())]=digest(path)
    return hashes


def game_inventory(directory,fixtures):
    expected = {f"{f['index']}-{arm}.json" for f in fixtures for arm in ARMS}
    actual = {p.name for p in directory.glob('*.json') if re.match(r'^\d+-(declaring|defending)\.json$',p.name)}
    require(actual==expected,f'game inventory incomplete/extra: missing={sorted(expected-actual)[:8]}, extra={sorted(actual-expected)[:8]}')
    require(not list(directory.glob('*.partial.json')),'leftover partial game checkpoint')
    # Reject renamed/duplicated games as well as extra numeric filenames.
    for p in directory.glob('*.json'):
        if p.name not in expected and read(p).get('schema')=='compiled-h2h-game-v1':
            raise ValueError('unexpected duplicate/renamed game receipt: '+p.name)
    return expected


def validate_game(game,fixture,arm,protocol,harness,rules):
    require(game.get('schema')=='compiled-h2h-game-v1' and game.get('fixture')==fixture
            and game.get('arm')==arm,'game/fixture identity mismatch')
    require(game.get('pairgroup')==fixture['pairgroup'] and game.get('balanced_order')==fixture['balanced_order'],'game group/order mismatch')
    harness.verify_game(game,rules)
    require(game.get('independent_replay_passed') is True,'stored replay flag false')
    moves,record = game['moves'],game['record']
    require(len(moves)==28 and len(record)==56,'not all28 physical plays retained')
    parity = fixture['bidder']%2 if arm=='declaring' else 1-fixture['bidder']%2
    player_counts = Counter()
    for index,move in enumerate(moves):
        prefix = record[:2*index]
        _,leader,remaining,trick = rules.replay_record(fixture['hands'],prefix,fixture['decl'],fixture['bidder'])
        actor = (leader+len(trick))%4
        tile = record[2*index+1]
        name = 'candidate' if actor%2==parity else 'cpu'
        require(move['actor']==actor==record[2*index] and move['tile']==tile and move['player']==name,'move actor/tile/player mismatch')
        require(move['trick']==index//4+1,'incorrect trick label')
        legal = rules.legal_tiles(remaining[actor],trick,fixture['decl'])
        require(move['legal']==legal and tile in legal,'move legal set mismatch')
        request,response = move['request'],move['response']
        for key,value in {'decl':fixture['decl'],'bid':30,'bidder':fixture['bidder'],'seat':actor,'seed':fixture['policy_seed']}.items():
            require(request.get(key)==value,'request public identity differs: '+key)
        require('error' not in response,'error response in complete game')
        if name=='candidate':
            require(request['hand']==sorted(remaining[actor]) and request['original_hand']==fixture['hands'][actor]
                    and request['history']==[prefix[i:i+2] for i in range(0,len(prefix),2)],'candidate observation differs')
            require(request['config']==protocol['candidate_config'] and request['budget_ms']==protocol['candidate_ms'],'candidate request config differs')
            require(response.get('tile')==tile,'candidate response tile mismatch')
            compute = response.get('compute',{})
            if 'backend' in compute: require(compute['backend']=='compiled-cpu','non-CPU candidate backend')
        else:
            require(request['hand']==fixture['hands'][actor] and request['plays']==prefix,'CPU observation differs')
            require(response.get('choice')==tile,'CPU response tile mismatch')
        frac(move['elapsed_ms'])
        player_counts[name]+=1
    require(player_counts=={'candidate':14,'cpu':14},'partnership does not have14 measured decisions')
    require(type(game['declaring_made']) is bool,'outcome must be boolean')


def latency(games):
    result = {}
    for player in ('candidate','cpu'):
        own = [m for g in games for m in g['moves'] if m['player']==player]
        times = [sum((frac(m['elapsed_ms']) for m in g['moves'] if m['player']==player),Fraction()) for g in games]
        by_role = {}
        for role in ARMS:
            selected = [g for g in games if (g['arm'] if player=='candidate' else ('defending' if g['arm']=='declaring' else 'declaring'))==role]
            by_role[role] = latency_stats([sum((frac(m['elapsed_ms']) for m in g['moves'] if m['player']==player),Fraction()) for g in selected])
        result[player] = {'decision_time':latency_stats([m['elapsed_ms'] for m in own]),'partnership_time':latency_stats(times),
            'partnership_by_role':by_role,'moves':len(own),
            'fallbacks':sum(bool(m['response'].get('fallback',False)) if player=='candidate' else str(m['response'].get('route','')).endswith('fallback') for m in own),
            'by_trick':{str(t):latency_stats([m['elapsed_ms'] for m in own if m['trick']==t]) for t in range(1,8)}}
    gates,ratios = {},{}
    for name,key in [('mean','mean_ms_exact'),('p95','p95_ms_exact')]:
        def value(p):
            f=result[p]['partnership_time'][key]
            return Fraction(f['numerator'],f['denominator'])
        candidate,cpu = value('candidate'),value('cpu')
        require(cpu>0,'zero control latency denominator')
        ratios[name]=rational(candidate/cpu)
        gates[name+'_at_most_110pct']=candidate<=Fraction(11,10)*cpu
    return {'players':result,'ratios':ratios,'gates':gates}


def prepare_panel(directory,spec,spec_path,harness,harness_path):
    directory = Path(directory)
    p = read(directory/'protocol.json')
    panel = p['panel']
    require(panel in spec['panels'],'unplanned panel')
    if 'output_directories' in spec:
        require(directory.resolve()==Path(spec['output_directories'][panel]).resolve(),'wrong nominated output directory')
    require(p['schema']=='compiled-h2h-protocol-v1' and p['data_role']=='CONFIRMATION'
            and p['final_holdout']==spec['id'],'not the frozen confirmation run')
    meta = {'path':str(spec_path.resolve()),'sha256':digest(spec_path),'schema':spec['schema'],'id':spec['id']}
    require(p['confirmation_protocol']==meta,'root protocol binding differs')
    for key in ('deals_per_cell','candidate_ms','cpu_ms','threads','candidate_config'):
        require(p[key]==spec[key],'panel config drift: '+key)
    for key,value in spec['identities'].items():
        require(p['identities'].get(key)==value,'panel identity differs: '+key)
    require(p['identities']['harness_sha256']==digest(harness_path),'panel harness differs')
    config_source=p['config_source']
    require(config_source.get('path') and digest(config_source['path'])==config_source['sha256']
            and read(config_source['path'])==spec['candidate_config'],'candidate config file drift')
    check_fixtures(p['fixtures'],panel,spec,harness)
    failures = directory/'failures.jsonl'
    require(not failures.exists() or not failures.read_text().strip(),'recorded technical game failure; no outcomes omitted')
    require(not (directory/'startup-error.json').exists(),'recorded startup failure')
    records = [json.loads(line) for line in (directory/'cycles.jsonl').read_text().splitlines() if line.strip()]
    for record in records:
        if 'warmup' in record:
            require(record['candidate_config']==spec['candidate_config'] and record['seconds']<=55,'cycle startup config drift')
    cycles = check_cycles(records,p['identities'],1152,spec['threads'])
    require(read(directory/'cycle-receipt.json')==cycles['last_receipt'],'last cycle sidecar differs')
    progress = read(directory/'progress.json')
    require(progress['completed_arms']==1152 and progress['planned_arms']==1152 and progress['failed_arms']==0,'incomplete progress')
    expected = game_inventory(directory,p['fixtures'])
    cycles.pop('last_receipt')
    outer_hashes=check_outer_cycles(spec_path,panel,directory,spec,harness_path,cycles['cycles'])
    metadata_hashes={name:digest(directory/name) for name in
        ('protocol.json','cycles.jsonl','cycle-receipt.json','progress.json')}
    return {'panel':panel,'directory':str(directory),'protocol':p,'expected':expected,
            'startup_separate':cycles,'fixture_hash':spec['fixture_hashes'][panel],
            'metadata_hashes':metadata_hashes,'outer_cycle_hashes':outer_hashes}


def compact_game(game):
    """Keep only checked sufficient statistics, not bulky search responses."""
    return {'arm':game['arm'],'declaring_made':game['declaring_made'],
        'fixture_index':game['fixture']['index'],
        'moves':[{'elapsed_ms':m['elapsed_ms'],'player':m['player'],'trick':m['trick'],
                  'response':{'fallback':bool(m['response'].get('fallback',False)),
                              'route':m['response'].get('route','')}} for m in game['moves']]}



def validate_compact(game,fixture,arm):
    require(set(game)=={'arm','declaring_made','fixture_index','moves'},'cached compact shape differs')
    require(game['arm']==arm and type(game['fixture_index']) is int
            and game['fixture_index']==fixture['index'],'cached compact fixture/arm differs')
    require(type(game['declaring_made']) is bool,'cached compact outcome is not boolean')
    moves=game['moves']
    require(isinstance(moves,list) and len(moves)==28,'cached compact must have28 moves')
    counts=Counter();tricks=Counter()
    for index,move in enumerate(moves):
        require(set(move)=={'elapsed_ms','player','trick','response'},'cached compact move shape differs')
        require(move['player'] in ('candidate','cpu') and type(move['trick']) is int
                and move['trick']==index//4+1,'cached compact player/trick differs')
        require(isinstance(move['response'],dict) and set(move['response'])=={'fallback','route'}
                and type(move['response']['fallback']) is bool,'cached response shape differs')
        frac(move['elapsed_ms']);counts[move['player']]+=1;tricks[(move['player'],move['trick'])]+=1
    require(counts=={'candidate':14,'cpu':14},'cached compact needs14 moves per partnership')
    require(all(tricks[(player,trick)]==2 for player in ('candidate','cpu') for trick in range(1,8)),
            'cached compact needs2 moves per partnership per trick')


def atomic(path,value):
    path=Path(path);path.parent.mkdir(parents=True,exist_ok=True)
    temporary=path.with_suffix(path.suffix+'.tmp')
    temporary.write_text(json.dumps(value,sort_keys=True,indent=2)+'\n')
    temporary.replace(path)


def validate_cached_game(source,cache,fixture,arm,protocol,harness,rules):
    before=source.stat()
    sha=digest(source)
    if cache.exists():
        saved=read(cache)
        require(saved['source_sha256']==sha and saved['source_path']==str(source.resolve()),
                'previously validated game changed: '+str(source))
        require(saved['fixture']==fixture and saved['arm']==arm,'cached fixture differs')
        require(saved['validation']=='complete','incomplete cached game transaction')
        game=saved['compact']
        require(hashlib.sha256(canonical(game).encode()).hexdigest()==saved['compact_sha256'],
                'cached compact checksum differs')
        validate_compact(game,fixture,arm)
    else:
        raw=source.read_bytes()
        require(hashlib.sha256(raw).hexdigest()==sha,'game changed during validation')
        full=json.loads(raw)
        validate_game(full,fixture,arm,protocol,harness,rules)
        game=compact_game(full)
        validate_compact(game,fixture,arm)
        atomic(cache,{'validation':'complete','source_path':str(source.resolve()),'source_sha256':sha,
                      'fixture':fixture,'arm':arm,'compact':game,
                      'compact_sha256':hashlib.sha256(canonical(game).encode()).hexdigest()})
    after=source.stat()
    require((before.st_size,before.st_mtime_ns)==(after.st_size,after.st_mtime_ns),'game mutated during read')
    return game,sha,(after.st_size,after.st_mtime_ns)


def write_report(output,result,hashes):
    output.mkdir(parents=True,exist_ok=False)
    atomic(output/'input-hashes.json',hashes)
    atomic(output/'summary.json',result)
    text=['# Frozen v5 confirmation','',f"Technical status: {result['technical_status']}. Confirmation pass: {result['confirmation_pass']}.",'']
    if 'analysis' in result:
        a=result['analysis'];text += [f"Both fixed panels: {a['wins']} wins, {a['losses']} losses, {a['ties']} ties across 1152 pairs.",
            f"Mean paired delta: {a['delta']['decimal']:.6f}. Approximate one-sided 99% lower bound: {a['one_sided_99_normal_lower']}. Exact conditional sign p: {a['exact_conditional_sign_probability']['decimal']:.8g}.",'',
            'Normal and sign checks have distinct nulls; neither implies superiority in every subgroup. Every cell, interval, per-panel latency gate, role latency and warmup statistic is retained in summary.json. No automatic deployment.']
    else:text += ['No inferential result is published for technically invalid execution.',json.dumps(result['errors'],indent=2)]
    (output/'REPORT.md').write_text('\n'.join(text)+'\n')


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--spec',type=Path,required=True)
    parser.add_argument('--panels',type=Path,nargs=2,required=True)
    parser.add_argument('--harness',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--work',type=Path,help='separate resumable validation directory')
    parser.add_argument('--seconds',type=float,default=55)
    args=parser.parse_args()
    require(0<args.seconds<=55,'cycle budget must be in(0,55] seconds')
    require(not args.output.exists(),'final report output already exists')
    work=args.work or args.output.with_name(args.output.name+'.work')
    started=time.monotonic();deadline=started+args.seconds
    result={'schema':'compiled-confirmation-report-v1','technical_status':'invalid','confirmation_pass':False,
            'errors':[],'scope':'Both complete frozen panels only; no candidate selection or automatic deployment.'}
    hashes={'script':digest(__file__),'spec':digest(args.spec),'harness':digest(args.harness),'panels':{}}
    panels=[];verified_this_cycle=0;file_states={}
    try:
        spec=read(args.spec);validate_spec(spec)
        result['protocol_id']=spec['id']
        result['excluded_predecessor_setup']=spec.get('replaces_setup_failed_protocol')
        harness=load_module('confirmation_harness',args.harness)
        identities={**spec['identities'],'harness_sha256':digest(args.harness)}
        check_current_identities(identities,harness,args.harness)
        rules=load_module('confirmation_rules',Path(spec['identities']['cpu_root'])/'experiments/partnership/rules.py')
        for path in args.panels:
            panels.append(prepare_panel(path,spec,args.spec,harness,args.harness))
        require({p['panel'] for p in panels}==set(spec['panels']),'duplicate/missing panel output')
        panels.sort(key=lambda p:spec['panels'].index(p['panel']))
        binding={'schema':'confirmation-validation-work-v1','script_sha256':hashes['script'],
            'spec_sha256':hashes['spec'],'harness_sha256':hashes['harness'],
            'panels':{p['panel']:{'directory':str(Path(p['directory']).resolve()),
                'metadata_hashes':p['metadata_hashes'],'outer_cycle_hashes':p['outer_cycle_hashes'],'fixture_hash':p['fixture_hash']} for p in panels}}
        manifest=work/'manifest.json'
        if manifest.exists(): require(read(manifest)==binding,'validation work/source identity changed')
        else:
            require(not work.exists() or not any(work.iterdir()),'unbound work directory is not empty')
            atomic(manifest,binding)
        if (work/'technical-failure.json').exists():
            raise ValueError('previous technical validation failure remains recorded; never silently retry')
        strata=defaultdict(list);panel_results=[]
        for p in panels:
            directory=Path(p['directory']);games=[];receipt_hashes={**p['metadata_hashes'],**p['outer_cycle_hashes']}
            for fixture in p['protocol']['fixtures']:
                pair={}
                for arm in ARMS:
                    # Transactions stop only before reading the next whole game.
                    if time.monotonic()>=deadline-5:
                        atomic(work/'progress.json',{'status':'budget','validated_cache_files':len(list(work.glob('games/*/*.json'))),
                            'rehash_verified_this_cycle':verified_this_cycle,'planned_games':2304,
                            'elapsed_seconds':time.monotonic()-started})
                        print(json.dumps({'status':'budget','work':str(work),'verified_this_cycle':verified_this_cycle}))
                        return 0
                    name=f"{fixture['index']}-{arm}.json";source=directory/name
                    compact,sha,stat=validate_cached_game(source,work/'games'/p['panel']/name,
                        fixture,arm,p['protocol'],harness,rules)
                    games.append(compact);pair[arm]=compact;receipt_hashes[name]=sha
                    file_states[source]=stat;verified_this_cycle+=1
                cm=pair['declaring']['declaring_made'];ws=pair['defending']['declaring_made']
                strata[(p['panel'],fixture['decl'],fixture['bidder'])].append({
                    'index':fixture['index'],'candidate_declaring_make':cm,
                    'candidate_defending_set':not ws,'delta':int(cm)-int(ws)})
            hashes['panels'][p['panel']]=receipt_hashes
            panel_results.append({'panel':p['panel'],'directory':p['directory'],
                'latency':latency(games),'startup_separate':p['startup_separate'],'fixture_hash':p['fixture_hash']})
        require(verified_this_cycle==2304,'not all game hashes verified in final cycle')
        # Normal file mutation is rejected even if an earlier receipt was read
        # before a later one. Campaigns must remain frozen during reporting.
        for source,stat in file_states.items():
            now=source.stat();require(stat==(now.st_size,now.st_mtime_ns),'game mutated during report')
        for p in panels:
            require(all(digest(Path(p['directory'])/name)==sha for name,sha in p['metadata_hashes'].items()),'panel metadata drift during report')
            require(all(digest(name)==sha for name,sha in p['outer_cycle_hashes'].items()),'outer receipts changed during report')
            game_inventory(Path(p['directory']),p['protocol']['fixtures'])
        check_current_identities(identities,harness,args.harness)
        require(read(args.spec)==spec,'root spec changed during report')
        validate_spec(spec)
        analysis=inference(strata,spec['panels'])
        latency_ok=all(all(p['latency']['gates'].values()) for p in panel_results)
        result.update(technical_status='valid',analysis=analysis,panels=panel_results,
            confirmation_pass=all(analysis['gates'].values()) and latency_ok,latency_pass=latency_ok,
            technical_checks='all 2304 game hashes verified; each game independently replayed; all 64512 moves/requests/roles checked; zero failures, missingness, or identity drift')
        atomic(work/'progress.json',{'status':'complete','validated_games':2304,'elapsed_seconds':time.monotonic()-started})
    except (ValueError,KeyError,TypeError,OSError,AssertionError,subprocess.SubprocessError) as exc:
        result['errors'].append({'error':str(exc)})
        atomic(work/'technical-failure.json',result)
    write_report(args.output,result,hashes)
    print(json.dumps({'output':str(args.output),'technical_status':result['technical_status'],
                     'confirmation_pass':result['confirmation_pass'],'errors':result['errors']},indent=2))
    return 0 if result['technical_status']=='valid' else 2


if __name__=='__main__':
    raise SystemExit(main())
