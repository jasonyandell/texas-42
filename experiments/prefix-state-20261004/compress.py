#!/usr/bin/env python3
"""Losslessly compact new retained fixture/results JSON. No export or deletion."""
import gzip,hashlib,json
from pathlib import Path
HERE=Path(__file__).resolve().parent
items=[]
for p in sorted((HERE/'results').rglob('*.json')):
 if p.name not in ['records.json','plan.json'] and not p.name.startswith('inputs-'):continue
 raw=p.read_bytes();data=gzip.compress(raw,mtime=0);assert gzip.decompress(data)==raw
 out=Path(str(p)+'.gz');assert not out.exists();out.write_bytes(data)
 items.append(dict(path=str(out.relative_to(HERE)),original_bytes=len(raw),gzip_bytes=len(data),original_sha256=hashlib.sha256(raw).hexdigest(),gzip_sha256=hashlib.sha256(data).hexdigest()))
 p.unlink()
(HERE/'COMPRESSED.json').write_text(json.dumps(dict(files=items,original_bytes=sum(x['original_bytes'] for x in items),gzip_bytes=sum(x['gzip_bytes'] for x in items)),indent=2)+'\n')
print(json.dumps({k:v for k,v in json.loads((HERE/'COMPRESSED.json').read_text()).items() if k!='files'}))
