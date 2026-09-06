#!/usr/bin/env python3
"""Conditional outcome report for fixed opening hands, varying hidden deals."""
import argparse
from collections import Counter
from fractions import Fraction
from pathlib import Path
import campaign as c
from rules import TILES


def tile(t): return '-'.join(str(x) for x in TILES[t])


def report(path):
    path=Path(path).resolve()
    spec=c.load(path)
    assert spec['panel']=='worlds'
    rows=c.complete_results(path,spec)
    groups={}
    for r in rows:
        group=r['arms']['phone']['fixture']['group']
        groups.setdefault(group,[]).append(r)
    data=[]
    for group,rs in sorted(groups.items()):
        base=rs[0]['arms']['phone']['fixture']
        bidder=base['bidder']
        own=base['hands'][bidder]
        worlds=set(); partners=set()
        for r in rs:
            f=r['arms']['phone']['fixture']
            assert (f['bidder'],f['decl'],f['bid'],f['hands'][bidder])==(bidder,base['decl'],30,own)
            worlds.add(tuple(tuple(f['hands'][s]) for s in range(4) if s!=bidder))
            partners.add(tuple(f['hands'][(bidder+2)%4]))
        item={'group':group,'worlds':len(rs),'bidder':bidder,'decl':base['decl'],'opening_hand':own,
              'distinct_hidden_deals':len(worlds),'distinct_partner_hands':len(partners),'arms':{},
              'paired':{k:sum(r['paired'][k] for r in rs) for k in ('wins','losses','ties')}}
        for arm in c.ARMS:
            results=[r['arms'][arm] for r in rs]
            pts=[r['points'][bidder%2] for r in results]
            openings=Counter(tile(r['opening_choice']) for r in results)
            completed={r['opening_choice'] for r in results if 'fallback' not in r['opening_route']}
            assert len(completed)<=1,'same-information completed opening differs'
            model_scores=Counter()
            for r in rs:
                d=c.read(path/'seeds'/str(r['seed'])/arm/'checkpoint.json')['decisions'][0]['response']
                if d['route']!='partner': continue
                evaluation=d.get('evaluation') or {}
                for t,num,den in evaluation.get('options',[]):
                    if t==d['choice']:
                        q=Fraction(int(num),int(den))
                        model_scores[f"{q.numerator}/{q.denominator} at {evaluation['outer_worlds']} outer worlds"]+=1
            item['arms'][arm]={'makes':sum(r['made'] for r in results),'points':pts,'min_points':min(pts),'max_points':max(pts),
                'opening_choices':dict(openings),'opening_fallbacks':sum('fallback' in r['opening_route'] for r in results),
                'opening_model_scores':dict(model_scores),
                'same_opening_mixed_outcomes':len(openings)==1 and 0<sum(r['made'] for r in results)<len(rs)}
        data.append(item)
    summary={'campaign':spec['id'],'completed_worlds':len(rows),'completed_games':len(rows)*3,
             'complete_hand_groups':sum(g['worlds']==spec['worlds_per_hand'] for g in data),'groups':data}
    summary['all_observed_opening_choices_fixed']=all(len(a['opening_choices'])==1 for g in data for a in g['arms'].values())
    summary['opening_fallbacks']=sum(a['opening_fallbacks'] for g in data for a in g['arms'].values())
    summary['mixed_outcome_groups']={a:sum(g['arms'][a]['same_opening_mixed_outcomes'] for g in data) for a in c.ARMS}
    c.atomic(path/'world-summary.json',summary)
    out=(f"# Same opening hand, different hidden deals\n\n"
         f"**{len(rows)}/{spec['count']} worlds, {len(rows)*3} completed games**, grouped under {len(data)} fixed opening hands. "
         f"{summary['complete_hand_groups']} groups have all ten worlds. Bid is always 30.\n\n"
         "Within each group the opening player, own seven tiles, trump, empty public history, bid, and policy seed are identical. "
         "Only the other three hands change; each player receives its own hand and public play. "
         "A completed opening calculation must choose the same tile across those worlds. "
         "Later moves may change as the public play changes.\n\n"
         "These are ten hand clusters with ten hidden completions each, not 100 independent opening positions. "
         "The independent-deal sequential stopping test is disabled. Make counts below are descriptive conditional frequencies, not calibrated probabilities.\n\n"
         "The first two outcome columns favor more makes. The defense column favors fewer makes by its opponents. "
         "Every number below 30 is an equally complete set; excess points do not rank results.\n\n"
         "| Hand | Trump | Worlds | Phone makes | Candidate declaring makes | Makes allowed by candidate defense | Paired W/L/T |\n"
         "|---|---|---:|---:|---:|---:|---|\n")
    for g in data:
        a=g['arms'];p=g['paired'];n=g['worlds']
        out+=f"| H{g['group']+1} (S{g['bidder']}) | {g['decl']} | {n} | {a['phone']['makes']}/{n} | {a['declaring']['makes']}/{n} | {a['defending']['makes']}/{n} | {p['wins']}/{p['losses']}/{p['ties']} |\n"
    out+='\n## Opening choices and outcome ranges\n\n'
    for g in data:
        out+=f"### H{g['group']+1}: "+', '.join(tile(t) for t in g['opening_hand'])+'\n\n'
        out+=f"{g['distinct_hidden_deals']} distinct hidden deals; {g['distinct_partner_hands']} distinct partner hands.\n\n"
        for arm,label in [('phone','Phone'),('declaring','Candidate declaring'),('defending','Phone facing candidate defense')]:
            a=g['arms'][arm]
            choices=', '.join(f'{t} ({n} worlds)' for t,n in a['opening_choices'].items())
            out+=f"- {label}: opening {choices}; {a['makes']}/{g['worlds']} makes; declaring points {a['min_points']}–{a['max_points']}; {a['opening_fallbacks']} opening fallbacks.\n"
            if a['opening_model_scores']:
                out+='  Native opening model scores: '+', '.join(f"{score} ({n} repetitions)" for score,n in a['opening_model_scores'].items())+'. These are sampled scores against the modeled field, not calibrated probabilities against the executed players.\n'
        out+='\n'
    out+='All completed primary opening choices were checked for agreement within each hand and lineup. Deadline fallbacks are listed separately because timing can change the executed policy. The ten-game shared pool and its execution records remain visible in sessions/ and seeds/.\n'
    c.atomic(path/'WORLD-RESULTS.md',out)
    print(out[:out.index('## Opening')])
    return summary


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('path',type=Path)
    report(p.parse_args().path)
