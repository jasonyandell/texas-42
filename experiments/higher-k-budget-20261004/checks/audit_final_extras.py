#!/usr/bin/env python3
"""Saved refusal-cluster interval arithmetic and portable byte archive audit."""
import gzip,hashlib,json,math,sys
sys.dont_write_bytecode=True
from audit_data import HERE,ROOT,read,close,sha
def main():
    artifacts=read(HERE/'ARTIFACTS.json');archives=[]
    for key in['measured_native','measured_wasm','final_native','final_wasm']:
        row=artifacts[key];path=ROOT/row['archive_path'];packed=path.read_bytes()
        assert len(packed)==row['archive_bytes']and sha(path)==row['archive_sha256']
        raw=gzip.decompress(packed);assert len(raw)==row['bytes']and hashlib.sha256(raw).hexdigest()==row['sha256']and raw==(ROOT/row['path']).read_bytes()
        assert (ROOT/row['path']).resolve().is_relative_to((HERE/'adapter/target').resolve())
        archives.append(dict(key=key,compressed_bytes=len(packed),raw_bytes=len(raw),actual_bytes_match=True))
    result=read(HERE/'results/budget-intervals.json');assert len(result['cells'])==20
    expected_seeds=set(read(HERE/'plan.json')['seeds']);radius=math.sqrt(math.log(40)/(2*54));assert result['radius']==radius
    for cell in result['cells']:
        panel='fixed-roots'if cell['plies']==16 else 'earlier-roots';assert cell['plies']in[12,16]
        files=[read(p)for p in(HERE/'results'/panel).glob('deal-*.json')];assert len(files)==54
        expected={}
        for f in files:
            q=[r for r in f['records']if(r['call']['budget_ms'],r['call']['k'])==(cell['budget_ms'],cell['k'])];assert len(q)==4
            flags=[int('refusal'in r['response'])for r in q];seed=q[0]['seed'];assert seed not in expected
            expected[seed]=dict(seed=seed,refused=flags,mean=sum(flags)/4)
        actual={c['seed']:c for c in cell['clusters']};assert len(actual)==len(cell['clusters'])==54 and set(actual)==set(expected)==expected_seeds
        for seed in expected:close(actual[seed],expected[seed])
        refused=sum(sum(q['refused'])for q in expected.values());mean=refused/216
        close(cell,dict(planned=216,refused=refused,source_deals=54,mean_planned_root_refusal_rate=mean,hoeffding95=[max(0,mean-radius),min(1,mean+radius)]))
    print(json.dumps(dict(binary_archives=archives,refusal_interval_cells=20,clusters_per_cell=54,rotations_correlated=True,radius=radius,scope='Artifact-byte and pointwise conditional arithmetic audit, not proof of PRNG/runtime independence, simultaneous family or post-selection coverage, or universal latency.'),indent=2))
if __name__=='__main__':main()
