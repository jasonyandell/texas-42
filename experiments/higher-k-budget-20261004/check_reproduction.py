import json
from pathlib import Path
h=Path(__file__).resolve().parent
original=json.loads((h/'results/fixed-roots/deal-3851398.json').read_text())['records']
new=json.loads((h/'results/reproduce-routing-data/fixed-roots/deal-3851398.json').read_text())['records']
reference={(r['rotation'],r['call']['k']):r for r in original if r['call']['budget_ms']==200}
assert len(new)==40
complete=refused=0
for r in new:
 q=reference[r['rotation'],r['call']['k']];assert r['call']['request']==q['call']['request']
 if r['response']['completed']:
  assert r['response']['evaluation']==q['response']['evaluation'];complete+=1
 else:
  assert r['response'].get('refusal')and'evaluation'not in r['response'];refused+=1
print(json.dumps(dict(routed_to_fresh_directory=True,calls=len(new),complete_vector_comparisons=complete,refusals_retained=refused,no_gold_overwrite=True)))
