#!/usr/bin/env python3
"""Read-only exact task8 binary conformance for the inherited named k3 policy."""
import hashlib,json,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
CHECK=Path(__file__).resolve().parent;HERE=CHECK.parent
def run(binary,value):
    p=subprocess.run([str(binary)],input=json.dumps(value)+'\n',capture_output=True,text=True,timeout=10)
    assert p.returncode==0,p.stderr
    return json.loads(p.stdout)
def main():
    old=Path('/Users/jason/Documents/Codex/2026-10-04/task-8/research-worktree/experiments/native-policy-check-20261004/adapter/target/release/native-late-player')
    digest=hashlib.sha256(old.read_bytes()).hexdigest()
    assert digest=='51259e4088de71f813ab1c8ad0b81345129ea01c26dbefec60cba462c1878de7'
    new=HERE/'adapter/target/release/higher-k-player';records=[]
    for r in json.loads((CHECK/'fresh-roots.json').read_text()):
        a=run(old,dict(action='late',request=r,budget_ms=4000,validate=True))
        b=run(new,dict(action='ladder',request=r,k=3,budget_ms=4000,validate=True))
        assert a['evaluation']==b['evaluation']and a['worlds']==b['worlds']and a['rng_final']==b['rng_final']and a['outer_attempts']==b['outer_attempts']
        assert a['pi_calls_by_level']==b['pi_calls_by_level'][:4]and a['inner_worlds_by_level']==b['inner_worlds_by_level'][:4]
        assert b['pi_calls_by_level'][4:]==[0,0]and b['inner_worlds_by_level'][4:]==[0,0]
        records.append(dict(request=r,inherited=a,current=b))
    path=CHECK/'inherited-k3-records.json';assert not path.exists();path.write_text(json.dumps(records,separators=(',',':'))+'\n')
    print(json.dumps(dict(fresh_roots=9,all9declarations=True,complete_vector_ordered_world_final_rng_comparisons=9,prefix_counter_comparisons=True,inherited_exact_native_sha256=digest,current_inner_vector=[4,2,2,2,2,2],inherited_inner_vector=[4,2,2,2],scope='Finite same namedk3 policy, unusedextra budgets do not affect these executions. Exacttask8 executable read-only, no originalfilemodified.'),indent=2))
if __name__=='__main__':main()
