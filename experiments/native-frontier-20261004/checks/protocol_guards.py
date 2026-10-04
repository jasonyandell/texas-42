#!/usr/bin/env python3
import copy,json,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[2]
binary=HERE/'runner-target/release/native-frontier'
row=json.loads((HERE/'independent-panel.json').read_text())['rows'][0]
base={'rows':[row],'mode':'native','budgets':[4,2,2,2],'caps':{'rows':500000,'work':5000000,'queries':50000,'cache_entries':50000,'seconds':20}}
cases=[]
def changed(label,fn):
    obj=copy.deepcopy(base);fn(obj);cases.append((label,obj))
changed('unsupported doubles-suit',lambda x:x['rows'][0]['request'].__setitem__('decl',8))
changed('duplicate original tile',lambda x:x['rows'][0]['request']['hand'].__setitem__(0,x['rows'][0]['request']['hand'][1]))
changed('odd history length',lambda x:x['rows'][0]['request']['plays'].pop())
changed('unsupported actor',lambda x:x['rows'][0]['request'].__setitem__('seat',4))
changed('illegal exposed tile',lambda x:x['rows'][0]['request']['plays'].__setitem__(1,28))
changed('remaining overlap',lambda x:x['rows'][0]['worlds'][0].__setitem__(1,x['rows'][0]['worlds'][0][0]))
changed('tape dimensions',lambda x:(x.__setitem__('mode','tape'),x['rows'][0]['tape'][0].pop()))
changed('undeclared field rung',lambda x:(x.__setitem__('mode','policy'),x.__setitem__('field_level',2),x.__setitem__('budgets',[4])))
changed('initial row cap',lambda x:x['caps'].__setitem__('rows',1))
changed('invalid cache cap',lambda x:x['caps'].__setitem__('cache_entries',0))
changed('reference-only requires reference',lambda x:x.__setitem__('reference_only',True))
changed('reference-only tape unsupported',lambda x:(x.__setitem__('mode','tape'),x.__setitem__('reference',True),x.__setitem__('reference_only',True)))
# Explicitly ensure a refused request never kills or poisons the next one.
cases.append(('valid following refusals',base))
run=subprocess.run([str(binary)],input=''.join(json.dumps(x)+'\n' for _,x in cases),text=True,capture_output=True,timeout=30)
assert run.returncode==0,run.stderr
answers=[json.loads(line) for line in run.stdout.splitlines()]
assert len(answers)==len(cases)
report=[]
for (label,_),answer in zip(cases,answers):
    if label=='valid following refusals':assert len(answer['answers'])==1
    else:assert 'error' in answer,(label,answer)
    report.append({'case':label,'error':answer.get('error'),'valid':label=='valid following refusals','cache_after_refusal':answer.get('completed_cache_entries')})
print(json.dumps({'requests':len(cases),'all_guard_refusals_correct':True,'process_survived_and_valid_request_completed':True,'cases':report},indent=2))
