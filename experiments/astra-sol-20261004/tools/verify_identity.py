#!/usr/bin/env python3
"""Recompute the release source-v2 identity and pin every research input."""
import hashlib
import json
from pathlib import Path
import platform
import subprocess

ROOT=Path(__file__).resolve().parents[1]
REPO=ROOT.parents[1]
manifest=json.loads((ROOT/'reference/production-phone/manifest.json').read_text())
paths=('walt/walt/src','walt/gym/queries','walt/gym/offer-count.scheme',
       'walt/walt-player/src','walt/Cargo.toml','walt/Cargo.lock',
       'walt/walt/Cargo.toml','walt/walt-player/Cargo.toml','walt/rust-toolchain.toml')
files=[]
for name in paths:
    p=REPO/name
    files.extend(x for x in p.rglob('*') if x.is_file()) if p.is_dir() else files.append(p)
h=hashlib.sha256()
for path in sorted(files):h.update(str(path.relative_to(REPO)).encode()+b'\0'+path.read_bytes()+b'\0')
assert h.hexdigest()==manifest['source_sha256'],'Release source hash mismatch'
phone=ROOT/'reference/production-phone/walt-player.wasm'
assert hashlib.sha256(phone.read_bytes()).hexdigest()==manifest['wasm_sha256']
subprocess.run(['git','diff','--quiet',manifest['source_commit'],'--',*paths],cwd=REPO,check=True,timeout=15)
tree=json.loads((ROOT/'reference/production-tree.json').read_text())
blobs={r['path']:r['sha'] for r in tree if r['type']=='blob'}
for remote,name in [('src/ai/phone/walt-player.wasm','walt-player.wasm'),
                    ('src/ai/phone/manifest.json','manifest.json'),
                    ('src/ai/native.ts','native.ts'),('src/ui/store.ts','store.ts')]:
    data=(ROOT/'reference/production-phone'/name).read_bytes()
    blob=hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()
    assert blob==blobs[remote],remote
print(json.dumps(dict(production_commit='a0d9fa806166b0e63fe016bb49d93f91f47b1af8',
    source_commit=manifest['source_commit'],source_files=len(files),
    source_sha256=h.hexdigest(),wasm_sha256=manifest['wasm_sha256'],
    source_inputs_match=True,production_git_blobs_match=True,
    platform=platform.platform(),machine=platform.machine()),indent=2))
