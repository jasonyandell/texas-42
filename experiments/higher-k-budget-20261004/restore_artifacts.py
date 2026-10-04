#!/usr/bin/env python3
"""Restore pinned local byte artifacts; verify first and preserve different files."""
import gzip,hashlib,json
from pathlib import Path
here=Path(__file__).resolve().parent;root=here.parents[1]
manifest=json.loads((here/'ARTIFACTS.json').read_text())
for key in ['measured_native','measured_wasm','final_native','final_wasm']:
 row=manifest[key];packed=(root/row['archive_path']).read_bytes()
 assert hashlib.sha256(packed).hexdigest()==row['archive_sha256']
 raw=gzip.decompress(packed);assert len(raw)==row['bytes']and hashlib.sha256(raw).hexdigest()==row['sha256']
 out=root/row['path'];assert out.is_relative_to(here/'adapter/target')
 if out.exists():assert out.read_bytes()==raw,f'preserve different artifact {out}'
 else:
  out.parent.mkdir(parents=True,exist_ok=True);out.write_bytes(raw)
  if key.endswith('native'):out.chmod(0o755)
 print(json.dumps(dict(artifact=key,sha256=row['sha256'],verified=True)))
