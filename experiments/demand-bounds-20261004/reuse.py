#!/usr/bin/env python3
"""Prospective successive-position cache study; invoke with run_capped.py."""
import json,shlex,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE))
from compare import fixture,call
def main():
 out=Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=False);started=time.perf_counter()
 proposed=[fixture(968000+i,ply,8) for i in range(16) for ply in [16,17,18,19]]
 rows=[r for r in proposed if r['status']=='ready' and r['request']['decl']<=6];generation=time.perf_counter()-started
 t=time.perf_counter();inputs=out/'inputs.json';inputs.write_text(json.dumps(dict(rows=rows))+'\n');serialization=time.perf_counter()-t
 # This runner takes a file rather than JSON stdin; adapt the same PID-specific
 # wait4 harness through a temporary local executable wrapper.
 wrapper=out/'run-local.sh';wrapper.write_text('#!/bin/sh\nexec '+shlex.quote(str(HERE/'reuse/target/release/demand-bounds-reuse'))+' '+shlex.quote(str(inputs.resolve()))+'\n');wrapper.chmod(0o700)
 result=call(wrapper.resolve(),dict())
 (out/'result.json').write_text(json.dumps(result,indent=2)+'\n')
 (out/'generation.json').write_text(json.dumps(dict(proposed=len(proposed),selected=len(rows),actual_worlds=8,generation_actual_n_s=generation,fixture_serialization_write_s=serialization,serialized_input_bytes=inputs.stat().st_size,scope='Local actual-n lawful support fixtures, every proposed seed/ply charged; raw service study sequential cold/reused/native alternation, component only. Total validation process includes cold/reused/reference work and outer replay, not a standalone player latency.'),indent=2)+'\n')
 print(json.dumps(dict(roots=len(rows),vectors=result['result']['vector_checks'],reuse=[{k:r[k] for k in ['counted','field','independent_cold_misses','successive_service_misses','actor_queries_avoided_by_cross_root_cache','inner_worlds_avoided']} for r in result['result']['records']]),indent=2))
if __name__=='__main__':main()
