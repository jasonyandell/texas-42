"""Recompute paired H2H intervals and retain every panel, including timeouts."""
import json
from pathlib import Path
import numpy as np


def run():
    panels=[]
    for path in sorted(Path('h2h').glob('*/summary.json')):
        if not path.parent.name.startswith(('screen-','holdout-','confirm-')):continue
        d=json.loads(path.read_text());n=d['completed_pairs']
        if path.parent.name.startswith('confirm-') and not d['finished']:continue
        result={k:v for k,v in d.items() if k not in ('pairs','rows')}
        result['panel']=path.parent.name
        result['phase']=path.parent.name.split('-')[0]
        result['complete_panel']=d['finished'] and d['unscored_pairs']==0 and d['errors']==0
        if n:
            w,l,t=(d[k] for k in ('pair_wins','pair_losses','pair_ties'))
            assert w+l+t==n
            draws=np.random.default_rng(83047).multinomial(n,[w/n,l/n,t/n],size=100000)
            result['candidate_game_win_fraction']=.5+(w-l)/(2*n)
            result['paired_deal_bootstrap_95_interval']=np.quantile(
                .5+(draws[:,0]-draws[:,1])/(2*n),[.025,.975]).tolist()
            if result['phase']!='confirm':
                result['bonferroni_seven_settings_p']=min(1.,7*d['exploratory_mcnemar_exact_two_sided_p'])
            else:
                result['confirmation_pass']=result['complete_panel'] and w>l and d['exploratory_mcnemar_exact_two_sided_p']<.05
            scored=[p for p in d['pairs'] if p['delta'] is not None]
            result['candidate_declaring_make_rate']=sum(p['candidate_make'] for p in scored)/n
            result['walt_declaring_make_rate']=sum(p['walt_make'] for p in scored)/n
        panels.append(result)
    out={'schema':'walt42x-paired-h2h-analysis-v1',
         'uncertainty_method':'100000 paired-deal multinomial bootstrap draws; seed 83047; percentile 95% interval for game win fraction. Exact two-sided binomial/McNemar p on discordant pairs. Seven-setting Bonferroni is a conservative diagnostic for exploratory screens/holdouts. The one fresh fixed-setting confirmation is separate, with a predeclared p<0.05 positive-net criterion and no earlier data pooled.',
         'timeout_rule':'No strength inference from censored panels. Incomplete pairs never count as losses.',
         'panels':panels}
    Path('h2h/analysis.json').write_text(json.dumps(out,indent=2)+'\n')
    for p in panels:
        print(p['panel'],p['pair_wins'],p['pair_losses'],p['pair_ties'],
              'median',p['median_seconds'],'max',p['max_completed_seconds'],
              'timeouts',p['timeouts'],'finished',p['finished'])


if __name__=='__main__':run()
