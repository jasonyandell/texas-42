#!/usr/bin/env python3
"""Compare cold processes, matched finite inputs and native worker counts.
Invoke only through run_capped.py. No dependencies or network needed.
"""
import argparse, hashlib, itertools, json, os, signal, statistics, subprocess, sys, tempfile, time
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent/'adversarial-20261004'))
from parallel_roots import fixture

def call(binary, inp):
    tick=time.perf_counter();payload=json.dumps(inp)+'\n';serialization=time.perf_counter()-tick
    # wait4 attributes rusage to this PID, without cumulative CHILDREN maxima
    # or Darwin time -l's sandbox-denied sysctl. Files avoid pipe deadlocks.
    harness_start=time.perf_counter()
    with tempfile.TemporaryFile(mode='w+') as source, tempfile.TemporaryFile(mode='w+') as output, tempfile.TemporaryFile(mode='w+') as errors:
        source.write(payload);source.seek(0);tick=time.perf_counter()
        proc=subprocess.Popen([str(binary)],stdin=source,stdout=output,stderr=errors)
        def expired(signum,frame):
            proc.kill();os.wait4(proc.pid,0);raise TimeoutError('single process exceeded 60s')
        old=signal.signal(signal.SIGALRM,expired);signal.alarm(60)
        try:pid,status,usage=os.wait4(proc.pid,0)
        finally:signal.alarm(0);signal.signal(signal.SIGALRM,old)
        host=time.perf_counter()-tick;proc.returncode=os.waitstatus_to_exitcode(status)
        errors.seek(0)
        if proc.returncode:raise RuntimeError(errors.read())
        output.seek(0);tick=time.perf_counter();result=json.load(output);parse=time.perf_counter()-tick
    harness_overhead=time.perf_counter()-harness_start-host-parse
    return dict(result=result,serialization_s=serialization,host_s=host,output_parse_s=parse,harness_overhead_s=harness_overhead,serialized_bytes=len(payload.encode()),child_cpu_s=usage.ru_utime+usage.ru_stime,process_peak_rss_bytes=usage.ru_maxrss*(1 if sys.platform=='darwin' else 1024))

def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
    p.add_argument('--start',type=int,default=960000);p.add_argument('--count',type=int,default=128)
    p.add_argument('--counts',type=int,nargs='+',default=[8,40,128,512]);p.add_argument('--workers',type=int,choices=[1,4],default=1)
    p.add_argument('--repeats',type=int,default=4);p.add_argument('--mode',choices=['native','policy'],default='native')
    p.add_argument('--field',type=int,default=0);p.add_argument('--min-ply',type=int,default=12)
    p.add_argument('--native-choices',action='store_true');p.add_argument('--counted',action='store_true')
    p.add_argument('--limit',type=int,default=1000);p.add_argument('--candidate',type=Path,default=HERE/'native/target/release/native-frontier')
    p.add_argument('--control',type=Path,default=HERE/'control-target/release/native-frontier')
    args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
    binaries={'factored':args.candidate.resolve(),'factored-hash':args.candidate.resolve(),'predecessor':args.control.resolve(),'recursive':args.candidate.resolve()}
    records=[];generation={};selected=None;tick0=time.monotonic()
    for n in args.counts:
        tick=time.perf_counter();all_rows=[fixture(args.start+i,args.min_ply+i%4,n) for i in range(args.count)];generation[n]=time.perf_counter()-tick
        rows=[r for r in all_rows if r['status']=='ready' and r['request']['decl']<=6][:args.limit]
        if selected is None:selected=[r['seed'] for r in rows]
        assert selected==[r['seed'] for r in rows]
        (args.out/f'inputs-{n}.json').write_text(json.dumps(rows)+'\n')
        inp=dict(rows=rows,mode=args.mode,field_level=args.field,budgets=[4,2,2,2],counted=args.counted,warm=False,native_choices=args.native_choices,workers=args.workers,caps=dict(rows=500000,work=20000000,queries=50000,cache_entries=50000,seconds=30))
        names=list(binaries);order=[names[i:]+names[:i] for i in range(len(names))]
        for repeat in range(args.repeats):
            if time.monotonic()-tick0>220:raise TimeoutError('panel own 220s cap')
            runs={}
            for name in order[repeat%len(order)]:
                payload=inp|{'hashed_prefixes':name=='factored-hash'}|({'reference':True,'reference_only':True,'batch_reference':args.workers==4} if name=='recursive' else {'reference':False})
                runs[name]=call(binaries[name],payload)
            assert 'answers' in runs['recursive']['result'],runs['recursive']['result']
            for name in ['factored','factored-hash','predecessor']:
                assert runs[name]['result'].get('answers')==runs['recursive']['result']['answers'],(name,n,repeat,runs[name]['result'].get('error'))
            fs=runs['factored']['result']['cold_stats'];hs=runs['factored-hash']['result']['cold_stats'];ps=runs['predecessor']['result']['cold_stats']
            for key in ['row_edges','public_coordinates','raw_policy_consultations_by_level','unique_actor_misses_by_level','inner_worlds_by_level']:
                assert fs[key]==hs[key]==ps[key],(key,fs[key],hs[key],ps[key])
            records.append(dict(worlds=n,repeat=repeat,roots=len(rows),order=order[repeat%len(order)],runs=runs))
            (args.out/'records.json').write_text(json.dumps(records,indent=2)+'\n')
            print(json.dumps(dict(worlds=n,repeat=repeat,roots=len(rows),cold_us={name:r['result'].get('frontier_cold_us',r['result'].get('reference_total_us')) for name,r in runs.items()})),flush=True)
    panels=[]
    for n in args.counts:
        rr=[r for r in records if r['worlds']==n];panel=dict(worlds=n,roots=len(selected),generation_actual_n_s=generation[n],variants={})
        for name in binaries:
            vv=[r['runs'][name] for r in rr]
            keys=['serialization_s','host_s','output_parse_s','harness_overhead_s','child_cpu_s','process_peak_rss_bytes']
            variant={key:statistics.median(v[key] for v in vv) for key in keys}
            variant['cold_service_s']=statistics.median(v['result'].get('frontier_cold_us',v['result'].get('reference_total_us')) for v in vv)/1e6
            variant['standalone_s']=statistics.median(v['serialization_s']+v['host_s']+v['output_parse_s']+v['harness_overhead_s'] for v in vv)
            variant['total_with_generation_s']=generation[n]+variant['standalone_s']
            if name!='recursive':variant['stats']=vv[-1]['result']['cold_stats']
            panel['variants'][name]=variant
        f=panel['variants']['factored'];c=panel['variants']['predecessor'];r=panel['variants']['recursive']
        panel['predecessor_over_factored']=c['cold_service_s']/f['cold_service_s'];panel['recursive_over_factored']=r['cold_service_s']/f['cold_service_s']
        panel['standalone_recursive_over_factored']=r['standalone_s']/f['standalone_s']
        panels.append(panel)
    summary=dict(arguments=vars(args),selected=selected,panels=panels,vector_checks=len(records)*len(selected)*3,elapsed_s=time.monotonic()-tick0,binary_sha256={n:hashlib.sha256(b.read_bytes()).hexdigest() for n,b in binaries.items()},scope='Finite late pip-trump component queries. Separate cold processes, all worker counts matched. Recursive wall includes preparation; front wall includes service setup, partitions, prepare/group/expand/fold and cache insertion. JSON/process/replay separately charged; standalone includes temporary-file creation/write/seek/close via harness_overhead_s. No warm check in timed process. wait4 RSS attributed to each individual binary; summed array maxima remain a subset/upper bound. Generation charged at actual n over all proposed deals; no strength or k-linearity inference.')
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2,default=str)+'\n')
    print(json.dumps({k:summary[k] for k in ['vector_checks','elapsed_s']}|{'panels':[{k:p[k] for k in ['worlds','roots','predecessor_over_factored','recursive_over_factored','standalone_recursive_over_factored']} for p in panels]},indent=2))
if __name__=='__main__':main()
