#!/usr/bin/env python3
"""Fresh-deal frontier scaling; same worlds/tapes prefixes across sample counts."""
import argparse,hashlib,json,time
from pathlib import Path
from parallel_roots import fixture,kernel_of
from frontier import run
HERE=Path(__file__).resolve().parent
def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
    tick=time.perf_counter();rows=[fixture(950000+i,12+i%4,128) for i in range(128)];generation=time.perf_counter()-tick
    plan=dict(rows=rows,generation_128_s=generation,world_counts=[8,40,128],source_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
    (args.out/'plan.json').write_text(json.dumps(plan,indent=2)+'\n')
    selected=[r for r in rows if r['status']=='ready' and r['request']['decl']<=6]
    summaries=[]
    for n in [8,40,128]:
        cases=[dict(r,worlds=r['worlds'][:n],tape=r['tape'][:n]) for r in selected]
        timings=[]
        for repeat in range(4):
            def ref():
                t=time.perf_counter();answer={str(r['seed']):{str(a):v for a,v in kernel_of(r).reference(r['tape'],30)[0].items()} for r in cases}
                return answer,time.perf_counter()-t
            if repeat%2:result=run(cases);values,ref_s=ref()
            else:values,ref_s=ref();result=run(cases)
            assert result['answers']==values
            result.pop('answers');result.update(reference_s=ref_s);timings.append(result)
        # Weighted-scenario identity: triple each same world AND its tape.
        dupe=[dict(r,worlds=[w for w in r['worlds'] for _ in range(3)],tape=[t for t in r['tape'] for _ in range(3)]) for r in cases[:4]]
        duplicated=run(dupe)['answers']
        assert all(duplicated[str(r['seed'])]=={a:3*v for a,v in values[str(r['seed'])].items()} for r in dupe)
        summaries.append(dict(worlds=n,roots=len(cases),all_vectors_equal=True,duplicate_weight_cases=len(dupe),runs=timings))
        (args.out/'summary.json').write_text(json.dumps(dict(planned=len(rows),selected_pip=len(selected),ready_all=sum(r['status']=='ready' for r in rows),
            generation_128_s=generation,panels=summaries,
            caveat='Freshseed950000..950127,preexclude forced/settled and non-pip. All sample-sizepanels share nestedrandomworld/tapeprefixes. ConditionalfiniteDice only, batchthroughput notsingleturnlatency. Importexcluded;arrayprep included. Sampling128worlds chargedseparately; no runtime/memory/k linearitytheorem.'),indent=2)+'\n')
    print((args.out/'summary.json').read_text())
if __name__=='__main__':main()
