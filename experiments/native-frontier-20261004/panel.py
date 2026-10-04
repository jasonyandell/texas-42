#!/usr/bin/env python3
"""Fresh native finite-bundle scaling. Always invoke through run_capped.py."""
import argparse, hashlib, json, resource, statistics, subprocess, sys, time
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
sys.path.insert(0,str(HERE.parent/'adversarial-20261004'))
from parallel_roots import fixture,kernel_of

def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
    p.add_argument('--start',type=int,default=960000);p.add_argument('--count',type=int,default=128)
    p.add_argument('--mode',choices=['tape','native','policy'],default='native')
    p.add_argument('--counts',type=int,nargs='+',default=[8,40,128,512]);p.add_argument('--field',type=int,default=0)
    p.add_argument('--min-ply',type=int,default=12);p.add_argument('--limit',type=int,default=1000)
    p.add_argument('--repeats',type=int,default=4);p.add_argument('--counted',action='store_true');p.add_argument('--parallel-reference',action='store_true')
    p.add_argument('--native-choices',action='store_true')
    args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
    binary=HERE/'native/target/release/native-frontier'
    tick=time.perf_counter();rows=[fixture(args.start+i,args.min_ply+i%4,max(args.counts)) for i in range(args.count)];generation=time.perf_counter()-tick
    chosen=[r for r in rows if r['status']=='ready' and r['request']['decl']<=6][:args.limit]
    plan=dict(arguments=vars(args)|{'out':str(args.out)},rows=rows,generation_max_worlds_s=generation,selected=[r['seed'] for r in chosen],binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest())
    (args.out/'plan.json').write_text(json.dumps(plan,indent=2,default=str)+'\n')
    records=[];started=time.monotonic()
    for n in args.counts:
        cases=[dict(r,worlds=r['worlds'][:n],tape=r['tape'][:n]) for r in chosen]
        expected=None
        if args.mode=='tape':
            t=time.perf_counter();expected=[{str(a):v for a,v in kernel_of(r).reference(r['tape'],r['request']['bid'])[0].items()} for r in cases];python_reference_s=time.perf_counter()-t
        for repeat in range(args.repeats):
            if time.monotonic()-started>220:raise TimeoutError('panel elapsed cap')
            inp=dict(rows=cases,mode=args.mode,field_level=args.field,budgets=[4,2,2,2],counted=args.counted,reference=args.mode!='tape',parallel_reference=args.parallel_reference,warm=True,reference_after=bool(repeat%2),native_choices=args.native_choices,caps=dict(rows=500000,work=20000000,queries=50000,cache_entries=50000,seconds=30))
            t=time.perf_counter();payload=json.dumps(inp)+'\n';serialization_s=time.perf_counter()-t
            t=time.perf_counter();proc=subprocess.run([str(binary)],input=payload,text=True,capture_output=True,timeout=60);host_s=time.perf_counter()-t
            assert proc.returncode==0,proc.stderr
            t=time.perf_counter();result=json.loads(proc.stdout);parse_s=time.perf_counter()-t
            rr=dict(worlds=n,repeat=repeat,roots=len(cases),mode=args.mode,field=args.field,serialization_s=serialization_s,host_s=host_s,output_parse_s=parse_s,serialized_bytes=len(payload.encode()),child_process_peak_rss_bytes=resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,result=result)
            if expected is not None and 'answers' in result:
                for row,got,exp in zip(cases,result['answers'],expected):
                    vals={str(t):c if (row['request']['seat']%2==row['request']['bidder']%2) else n-c for t,c in zip(got['tiles'],got['counts'])};assert vals==exp,(row['seed'],vals,exp)
                rr['python_full_vectors_equal']=True;rr['python_reference_s']=python_reference_s
            records.append(rr);(args.out/'records.json').write_text(json.dumps(records,indent=2)+'\n')
            print(json.dumps({k:rr[k] for k in ['worlds','repeat','roots','host_s']}|{'frontier_us':result.get('frontier_cold_us'),'reference_us':result.get('reference_solve_us'),'error':result.get('error')}),flush=True)
    panels=[]
    for n in args.counts:
        runs=[r for r in records if r['worlds']==n];complete=[r for r in runs if 'answers' in r['result']]
        panel=dict(worlds=n,roots=len(chosen),completed=len(complete),refused=len(runs)-len(complete))
        if complete:
            front=statistics.median(r['result']['frontier_cold_us'] for r in complete)/1e6
            ref=statistics.median(r['result']['reference_solve_us'] for r in complete)/1e6 if args.mode!='tape' else complete[0]['python_reference_s']
            # Common generation charged at max sample count. Solver entry includes
            # support validation; host adds process+serialization/output+reference.
            panel.update(frontier_median_s=front,reference_median_s=ref,reference_over_frontier=ref/front,common_max_generation_s=generation,end_to_end_common_ratio=(generation+ref)/(generation+front),outer_validation_median_s=statistics.median(r['result']['outer_validation_us'] for r in complete)/1e6,warm_median_s=statistics.median(r['result']['frontier_warm_us'] for r in complete)/1e6,serialization_median_s=statistics.median(r['serialization_s'] for r in complete),parse_median_s=statistics.median(r['output_parse_s'] for r in complete),host_median_s=statistics.median(r['host_s'] for r in complete),peak_process_rss_bytes=max(r['child_process_peak_rss_bytes'] for r in complete),stats=complete[-1]['result']['cold_stats'])
        panels.append(panel)
    summary=dict(planned=len(rows),selected=len(chosen),partial_roots=sum(r['ply']%4!=0 for r in chosen),panels=panels,all_complete_vectors_checked=True,elapsed_s=time.monotonic()-started,scope='Local native component throughput, pip trumps, finite frozen scenario bundles. Native production-core serial or existing four-thread root parallel reference; not full phone wrapper, no strength/k complexity claim. RSS is child high-water maximum so far; retained-array metric excludes maps, prepared queries, caches and allocator overhead. Source generation excluded solve and explicitly charged separately at maximum sample count.')
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
