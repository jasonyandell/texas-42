# Adapted with attribution from preceding demand-bounds independent checker; fresh execution here.
#!/usr/bin/env python3
import hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
counts={}
for name in ['adversarial-20261004','native-frontier-20261004','prefix-state-20261004','demand-bounds-20261004','choice-arena-20261004']:
 folder=ROOT/'experiments'/name;manifest=folder/'SHA256SUMS';count=0
 for line in manifest.read_text().splitlines():
  expected,relative=line.split(maxsplit=1);relative=relative.strip().lstrip('*');p=ROOT/relative if relative.startswith('experiments/') else folder/relative
  assert p.exists(),str(p);assert hashlib.sha256(p.read_bytes()).hexdigest()==expected,str(p);count+=1
 counts[name]=count
print(json.dumps({'historical_manifests_verified':counts,'all_file_bytes_preserved':True},indent=2))
