#!/usr/bin/env python3
"""Reviewer-local fresh own/public roots and complete native/WASM policy audit.
All execution must be inside run_capped.py. No network, bytecode or secret access.
"""
import argparse,copy,hashlib,json,random,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];CHECK=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT/'experiments/partnership'))
from rules import information_state,legal_tiles,winner,replay_record

def generate():
 rows=[];counts={d:0 for d in (*range(8),9)};kinds={};proposals=0
 for seed in range(1071300,1071800):
  decl=(*range(8),9)[seed%9];bidder=seed%4;rng=random.Random(seed);deck=list(range(28));rng.shuffle(deck);hands=[sorted(deck[7*s:7*s+7]) for s in range(4)];remaining=list(map(set,hands));plays=[];trick=[];leader=bidder;added=False
  for ply in range(28):
   seat=(leader+len(trick))%4;req=dict(decl=decl,bid=30,bidder=bidder,seat=seat,hand=hands[seat][:],plays=plays[:],seed=7042104)
   st=information_state(req);eligible=16<=ply<=24 and len(st['legal'])>1 and st['points'][bidder%2]<30 and st['points'][1-bidder%2]<=12
   if eligible and not added and counts[decl]<4:rows.append(req);counts[decl]+=1;added=True
   if not eligible:kinds.setdefault('early'if ply<16 else'forced'if len(st['legal'])==1 else'settled',req)
   tile=rng.choice(legal_tiles(remaining[seat],trick,decl));remaining[seat].remove(tile);plays.extend([seat,tile]);trick.append((seat,tile))
   if len(trick)==4:leader=winner(trick,decl);trick=[]
  proposals+=1
  if all(c==4 for c in counts.values())and len(kinds)==3:break
 assert len(rows)==36
 (CHECK/'fresh-roots.json').write_text(json.dumps(rows,indent=2)+'\n');(CHECK/'ineligible-roots.json').write_text(json.dumps(kinds,indent=2)+'\n')
 return rows,kinds,proposals

def call(binary,payload):
 command=['node',str(CHECK/'wasm_worker.mjs'),str(binary)]if binary.suffix=='.wasm'else[str(binary)]
 p=subprocess.run(command,input=json.dumps(payload)+'\n',capture_output=True,text=True,timeout=10);assert p.returncode==0,(p.returncode,p.stderr)
 v=json.loads(p.stdout);return v.get('result',v),v

def audit(binary):
 expected=json.loads((CHECK/'independent-fresh-fixed/stdout.log').read_text());rows=expected['cases'];binary=Path(binary);done=0;worlds=0;ties=0
 for e in rows:
  r=e['request'];v,full=call(binary,dict(action='late',request=r,validate=True));assert 'error'not in v,(r,v)
  assert v['evaluation']==e['evaluation'];assert v['worlds']==e['worlds'];assert int(v['rng_final'])==e['final_rng'];assert v['outer_attempts']==e['attempts'];st=information_state(r)
  assert v['legal']==st['legal']and v['points']==st['points']and v['leader']==st['leader'];rot=1 if r['bidder']%2==0 else 0
  for w in v['worlds']:
   hh=[sorted([t for t in range(28)if w[(s+rot)%4]&(1<<t)]+[t for s0,t in zip(r['plays'][::2],r['plays'][1::2])if s0==s])for s in range(4)]
   p,l,_,_=replay_record(hh,r['plays'],r['decl'],r['bidder']);assert hh[r['seat']]==sorted(r['hand'])and p==st['points']and l==st['leader'];worlds+=1
  vals=e['evaluation']['counts'];ties+=len(vals)!=len(set(vals));done+=1
 bad=[];base=rows[0]['request']
 for k,v in [('decl',8),('decl',2**32+5),('bid',286),('seat',4),('seed',True),('seed',2**64),('hand',[0]*7),('plays',[0]),('worlds',[]),('opponent_hands',[[1,2]])]:
  r=copy.deepcopy(base);r[k]=v;bad.append(r)
 for k in base:r=copy.deepcopy(base);del r[k];bad.append(r)
 for r in bad:
  v,_=call(binary,dict(action='late',request=r));assert 'error'in v,(r,v)
 for kind,r in json.loads((CHECK/'ineligible-roots.json').read_text()).items():
  v,_=call(binary,dict(action='late',request=r));assert v.get('ineligible'),(kind,v)
 print(json.dumps(dict(fresh_roots=done,full_vectors=done,independent_replayed_worlds=worlds,tied_vectors=ties,malformed_refusals=len(bad),ineligible_roots=3,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),scope='Finite complete late policy parity; native status replay shared, Python full deal replay independent. No strength inference.'),indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--generate',action='store_true');p.add_argument('--binary');a=p.parse_args()
 if a.generate:
  rows,kinds,proposals=generate();print(json.dumps(dict(proposals=proposals,fresh_roots=len(rows),declarations=sorted({r['decl']for r in rows}),actors=sorted({r['seat']for r in rows}),ineligible=list(kinds))))
 else:audit(a.binary)
