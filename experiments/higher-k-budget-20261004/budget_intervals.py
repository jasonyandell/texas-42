"""Pointwise bounded CIs for planned-root refusal; whole source deals are units."""
import json,math
from pathlib import Path
h=Path(__file__).resolve().parent
out=[]
for panel,plies in [('fixed-roots',16),('earlier-roots',12)]:
 source=[json.loads(p.read_text())for p in(h/'results'/panel).glob('deal-*.json')];assert len(source)==54
 for budget in [20,200]:
  for k in range(1,6):
   clusters=[]
   for f in source:
    q=[r for r in f['records']if(r['call']['budget_ms'],r['call']['k'])==(budget,k)];assert len(q)==4
    flags=[int('refusal'in r['response'])for r in q]
    clusters.append(dict(seed=q[0]['seed'],refused=flags,mean=sum(flags)/4))
   mean=sum(q['mean']for q in clusters)/54;radius=math.sqrt(math.log(40)/(2*54))
   out.append(dict(plies=plies,k=k,budget_ms=budget,planned=216,refused=sum(sum(q['refused'])for q in clusters),source_deals=54,mean_planned_root_refusal_rate=mean,hoeffding95=[max(0,mean-radius),min(1,mean+radius)],clusters=clusters))
result=dict(cells=out,unit='Source-deal average of four rotation refusal indicators, each in[0,1]. All54planned source deals retained; ineligible/settled/forced requests do not attempt kernel and have refusalindicator0. This is unconditional planned-root risk, distinct from refusal conditional on eligibility.',assumption='Independent PRNG source-deal draws, fixed nine-declaration stratification. Pointwise intervals, not simultaneous family coverage; horizons share deals. No PRNG independence proof or universal deadline guarantee.',radius=math.sqrt(math.log(40)/(2*54)))
p=h/'results/budget-intervals.json';assert not p.exists();p.write_text(json.dumps(result,separators=(',',':'))+'\n')
print(json.dumps({k:result[k]for k in ['unit','assumption','radius']}))
