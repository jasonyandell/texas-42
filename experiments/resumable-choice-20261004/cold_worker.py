#!/usr/bin/env python3
"""Fresh Python fixture frontend and native kernel, common compiled sampling."""
import time
BOOT=time.perf_counter()
import argparse,hashlib,importlib.util,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('prior_sampling',HERE.parent/'choice-arena-20261004/sampling.py')
sampling=importlib.util.module_from_spec(spec);spec.loader.exec_module(sampling)
IMPORT=time.perf_counter()-BOOT
p=argparse.ArgumentParser();p.add_argument('--variant',choices=['resumable','arena','bounded','recursive'],required=True);p.add_argument('--start',type=int,required=True);p.add_argument('--count',type=int,required=True);p.add_argument('--worlds',type=int,required=True);p.add_argument('--workers',type=int,required=True)
a=p.parse_args();t=time.perf_counter();rows0=[sampling.compiled_fixture(a.start+i,16+i%4,a.worlds) for i in range(a.count)];gen=time.perf_counter()-t
rows=[r for r in rows0 if r['status']=='ready' and r['request']['decl']<=6]
payload=dict(rows=rows,mode='policy',field_level=2,budgets=[4,2,2,2],counted=False,workers=a.workers,warm=False,caps=dict(rows=500000,work=20000000,queries=50000,cache_entries=50000,seconds=30))
binary=HERE/'native/target/release/native-frontier'
if a.variant=='arena':binary=HERE/'checks/old-arena-target/release/native-frontier';payload['arena_choices']=True
if a.variant=='bounded':binary=HERE/'checks/old-demand-target/release/native-frontier';payload['bounded_choices']=True
if a.variant=='resumable':payload['resumable_choices']=True
if a.variant=='recursive':binary=HERE/'checks/old-demand-target/release/native-frontier';payload.update(reference=True,reference_only=True,batch_reference=a.workers==4)
run=sampling.compare.call(binary,payload)
print(json.dumps(dict(variant=a.variant,python_import_s=IMPORT,generation_s=gen,run=run,selected=[r['seed'] for r in rows],all_rows_sha256=hashlib.sha256(json.dumps(rows0,sort_keys=True).encode()).hexdigest(),binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest())))
