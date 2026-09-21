#!/usr/bin/env python3
"""Verify and unpack the immutable v34 build input in a fresh checkout."""
import hashlib
import json
from pathlib import Path
import subprocess
import tarfile

root=Path(__file__).resolve().parents[1]
ref=root/'reference'
identity=json.loads((ref/'IDENTITY.json').read_text())
archive=ref/identity['archive']
actual=hashlib.sha256(archive.read_bytes()).hexdigest()
if actual!=identity['sha256']:
    raise SystemExit('Frozen source archive checksum mismatch')
if not (ref/'native').exists():
    subprocess.run(['tar','-xzf',str(archive),'-C',str(ref)],check=True)
checked=0
with tarfile.open(archive,'r:gz') as source:
    for member in source.getmembers():
        if not member.isfile():
            continue
        path=ref/member.name
        expected=hashlib.sha256(source.extractfile(member).read()).digest()
        if not path.is_file() or hashlib.sha256(path.read_bytes()).digest()!=expected:
            raise SystemExit('Frozen reference differs: '+str(path))
        checked+=1
print('Verified immutable v34 reference: '+str(checked)+' files')
