#!/usr/bin/env python3
"""Exact-stream experiment: compile repeated historical legality predicates.

The generator is the old fixture AST with one proposal acceptance block changed.
Initial full legal replay, own/public checks, proposal capacities, shuffle/RNG,
attempt ceiling and tape production remain the original code. This is only a
known-lawful generated Straight fixture frontend, not a request validator or
behavioral posterior. Never use these constraints as an actor's hidden input.
"""
import argparse,ast,gzip,hashlib,importlib.util,json,resource,statistics,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('arena_compare',HERE/'compare.py')
compare=importlib.util.module_from_spec(spec);spec.loader.exec_module(compare)
original=compare.fixture
source=HERE.parent/'adversarial-20261004/parallel_roots.py'
SOURCE_SHA256='b2da6c2cd211d1c4b340a10f9760a15b33f4765a33d99f438dec40f3d9f84aa6'
assert hashlib.sha256(source.read_bytes()).hexdigest()==SOURCE_SHA256
from rules import context,follows

def void_masks(request):
 """All historical off-suit obligations, with no terminal threshold cutoff.

 Premise: original replay already establishes lawful distinct exposed plays,
 known historical ownership/turns, capacities, and known future plays. Therefore
 proposal legality depends only on its final remaining tiles at each seat.
 """
 masks=[0]*4;trick=[];decl=request['decl']
 for seat,tile in zip(request['plays'][::2],request['plays'][1::2]):
  if trick:
   led=context(trick[0][1],decl)
   if not follows(tile,led,decl):
    masks[seat] |= sum(1<<t for t in range(28) if follows(t,led,decl))
  trick.append((seat,tile))
  if len(trick)==4:trick=[]
 return masks

tree=ast.parse(source.read_text())
function=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='fixture')
acceptance=[n for n in ast.walk(function) if isinstance(n,ast.Try)]
assert len(acceptance)==1
old=acceptance[0]
assert isinstance(old.body[0],ast.Assign) and isinstance(old.body[0].value,ast.Call)
assert old.body[0].value.func.id=='replay_record'
replacement=ast.parse('''rr = [set(full[s]).difference(past[s]) for s in range(4)]
if any(sum(1 << t for t in rr[s]) & forbidden[s] for s in range(4)):
    continue
pp, ll, tt = points, lead, tail
''').body
loop=next(n for n in function.body if isinstance(n,ast.While))
index=loop.body.index(old);loop.body[index:index+1]=replacement
index=function.body.index(loop)
function.body[index:index]=ast.parse('forbidden = void_masks(req)').body
function.name='compiled_fixture'
module=ast.fix_missing_locations(ast.Module(body=[function],type_ignores=[]))
scope=dict(original.__globals__,void_masks=void_masks)
exec(compile(module,str(source)+' [exact acceptance compilation]','exec'),scope)
compiled_fixture=scope['compiled_fixture']

def main():
 p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
 p.add_argument('--start',type=int,default=962000);p.add_argument('--count',type=int,default=32)
 p.add_argument('--worlds',type=int,nargs='+',default=[8,40]);p.add_argument('--repeats',type=int,default=8)
 a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False);panels=[];records=[]
 for n in a.worlds:
  # Validation is independently charged and excluded from generation timing.
  t=time.perf_counter();old=[original(a.start+i,16+i%4,n) for i in range(a.count)]
  new=[compiled_fixture(a.start+i,16+i%4,n) for i in range(a.count)]
  assert old==new;validation=time.perf_counter()-t
  with gzip.open(a.out/f'inputs-{n}.json.gz','wt') as f:json.dump(old,f)
  hashes={};timings={name:[] for name in ['original','compiled']}
  for repeat in range(a.repeats):
   order=['original','compiled'] if repeat%2==0 else ['compiled','original']
   for name in order:
    fn=original if name=='original' else compiled_fixture
    t=time.perf_counter();cpu=time.process_time()
    rows=[fn(a.start+i,16+i%4,n) for i in range(a.count)]
    cpu=time.process_time()-cpu;wall=time.perf_counter()-t
    assert rows==old
    record=dict(worlds=n,repeat=repeat,variant=name,order=order,wall_s=wall,cpu_s=cpu)
    timings[name].append(record);records.append(record)
  for name in timings:hashes[name]=hashlib.sha256(json.dumps(old,sort_keys=True).encode()).hexdigest()
  variants={name:{'generation_s':statistics.median(r['wall_s'] for r in rr),'generation_cpu_s':statistics.median(r['cpu_s'] for r in rr)} for name,rr in timings.items()}
  panels.append(dict(worlds=n,proposed=a.count,selected=[r['seed'] for r in old if r['status']=='ready' and r['request']['decl']<=6],
    sampling_attempts=sum(r.get('sampling_attempts',r.get('attempts',0)) for r in old),
    all_proposal_rows_exact=True,all_ordered_worlds_and_tapes_exact=True,validation_s=validation,
    variants=variants,exact_rows_sha256=hashes,
    original_over_compiled=variants['original']['generation_s']/variants['compiled']['generation_s']))
 result=dict(arguments=vars(a),panels=panels,records=records,
   process_lifetime_peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*(1 if sys.platform=='darwin' else 1024),
   source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
   scope='Known-lawful generated Straight fixtures only. Same proposal/shuffle/tape RNG stream and exact accepted multiplicity/order/attempt ceilings. Complete public history; no score cutoff. Single generator worker both variants. Setup+constraints+all proposed roots+sampling+tapes charged in generation. Python invocation/import and validation separately present in capped receipt but excluded from generation medians. Lifetime RSS includes both variants and validation; no variant RSS advantage claim. No policy, belief, sample budget, phone player or strength change.')
 (a.out/'summary.json').write_text(json.dumps(result,indent=2,default=str)+'\n')
 print(json.dumps({'panels':panels}))
if __name__=='__main__':main()
