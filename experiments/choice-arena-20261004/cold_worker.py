#!/usr/bin/env python3
"""One fresh Python frontend plus cold native process, for cold_pipeline.py."""
import time
BOOT=time.perf_counter()
import argparse,hashlib,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
from sampling import original,compiled_fixture,compare
HERE=Path(__file__).resolve().parent
IMPORTS=time.perf_counter()-BOOT
p=argparse.ArgumentParser();p.add_argument('--variant',choices=['original','compiled'],required=True)
p.add_argument('--start',type=int,required=True);p.add_argument('--count',type=int,required=True)
p.add_argument('--worlds',type=int,required=True);p.add_argument('--workers',type=int,required=True)
a=p.parse_args();cpu=time.process_time();t=time.perf_counter()
fn=original if a.variant=='original' else compiled_fixture
all_rows=[fn(a.start+i,16+i%4,a.worlds) for i in range(a.count)]
generation=time.perf_counter()-t
rows=[r for r in all_rows if r['status']=='ready' and r['request']['decl']<=6]
payload=dict(rows=rows,mode='policy',field_level=2,budgets=[4,2,2,2],counted=False,
 workers=a.workers,reference=True,reference_only=True,batch_reference=a.workers==4,
 caps=dict(rows=500000,work=20000000,queries=50000,cache_entries=50000,seconds=30))
run=compare.call(HERE/'checks/baseline-target/release/native-frontier',payload)
assert 'answers' in run['result'],run['result']
out=dict(variant=a.variant,python_import_s=IMPORTS,generation_s=generation,run=run,
 selected=[r['seed'] for r in rows],all_rows_sha256=hashlib.sha256(json.dumps(all_rows,sort_keys=True).encode()).hexdigest(),
 python_cpu_after_import_s=time.process_time()-cpu,python_body_before_output_s=time.perf_counter()-BOOT)
print(json.dumps(out))
