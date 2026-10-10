#!/usr/bin/env python3
"""Whole-deal matched decision comparisons on fresh heldout sources."""
import argparse,json,time
from pathlib import Path
import numpy as np
from coverage import HERE,BASE,p,positions
def ci(v,clusters):
 keys=np.unique(clusters);means=np.array([np.mean(v[clusters==k]) for k in keys]);rng=np.random.default_rng(61010699);boot=means[rng.integers(len(means),size=(5000,len(means)))].mean(1)
 return dict(mean=float(means.mean()),ci95=list(map(float,np.quantile(boot,[.025,.975]))),source_deals=len(keys),bootstrap='5000equal-weight original source-deal means')
def evaluate(a):
 data,W=p.dataset(a.reference);rows=positions();select=np.array([j for j,i in enumerate(data['ids']) if rows[int(i)]['split']==a.split and (a.band=='all' or rows[int(i)]['band']==int(a.band))]);assert len(select)
 ids=data['ids'][select];X=data['X'][select];m=data['M'][select];q=data['Q'][select];clusters=np.array([rows[int(i)]['source_id'] for i in ids]);legaln=m.sum(1);best=np.argmax(np.where(m,q,-np.inf),1);v=q[np.arange(len(q)),best];Y=np.unpackbits(data['B'][select],axis=2)[:,:,:W].astype(np.float32)
 low,LW=p.dataset('late1' if a.reference=='reference1' else 'teacher128');lookup={int(i):j for j,i in enumerate(low['ids'])};teach=[]
 for i in ids:
  if int(i) in lookup:j=lookup[int(i)];teach.append(int(np.argmax(np.where(low['M'][j],low['Q'][j],-np.inf))))
  else:teach.append(-1)
 choices={'ascending':np.argmax(m,1),'highest_tile':27-np.argmax(m[:,::-1],1)}
 if all(t>=0 for t in teach):choices['teacher128']=np.array(teach)
 for model in a.models:choices[model.stem]=np.argmax(np.where(m,p.predict(dict(np.load(model)),X),-np.inf),1)
 regret={'uniform_random':v-(q*m).sum(1)/legaln};regret.update({name:v-q[np.arange(len(q)),c] for name,c in choices.items()});second=np.argsort(np.where(m,q,-np.inf),1)[:,-2];gap=v-q[np.arange(len(q)),second];ds=Y[np.arange(len(q)),best]-Y[np.arange(len(q)),second];se=ds.std(1,ddof=1)/np.sqrt(W);resolved=gap>3*se;metrics={}
 for name,r in regret.items():
  entry=ci(r,clusters);entry.update(root_mean=float(r.mean()),p95=float(np.quantile(r,.95)),maximum=float(r.max()))
  if name in choices:entry.update(best_set_agreement=float(np.mean(r==0)),resolved_best_set_agreement=float(np.mean(r[resolved]==0)) if resolved.any() else None,vs_random=ci(r-regret['uniform_random'],clusters),vs_ascending=ci(r-regret['ascending'],clusters))
  metrics[name]=entry
 comps={f'{ma.stem}-minus-{mb.stem}':ci(regret[ma.stem]-regret[mb.stem],clusters) for j,ma in enumerate(a.models) for mb in a.models[j+1:]};gate=None
 if a.split=='validation' and a.reference=='reference0' and a.band=='2' and 'late32' in metrics and 'mixed32' in metrics:
  delta=ci(regret['late32']-regret['mixed32'],clusters);gate=dict(assessed_model='late32',mixed_relative_improvement=1-metrics['late32']['mean']/metrics['mixed32']['mean'],paired_vs_mixed=delta,pass_gate=metrics['late32']['mean']<=.9*metrics['mixed32']['mean'] and delta['ci95'][1]<0 and metrics['late32']['vs_random']['ci95'][1]<0 and metrics['late32']['vs_ascending']['ci95'][1]<0)
 details=[dict(position_index=int(i),source_id=clusters[j],band=rows[int(i)]['band'],legal=np.flatnonzero(m[j]).tolist(),Q=q[j].tolist(),best=int(best[j]),gap=float(gap[j]),paired_gap_se=float(se[j]),resolved=bool(resolved[j]),choices={n:int(c[j]) for n,c in choices.items()},regrets={n:float(r[j]) for n,r in regret.items()}) for j,i in enumerate(ids)]
 result=dict(tier='exploratory',split=a.split,band=a.band,roots=len(ids),source_deals=len(np.unique(clusters)),original_split_deals=256,missing_late_source_deals=256-len(np.unique(clusters)) if a.band=='2' else None,reference=a.reference,reference_worlds=W,metrics=metrics,paired_differences=comps,gate=gate,resolved_gap_roots=int(resolved.sum()),qualification='conditional live-late availability; empirical3SEgap heuristic without multiplicity correction; noisy maxQreference under fixed support-conditioned belief, not optimal-game regret',model_sha256={str(x):p.sha(x) for x in a.models})
 folder=HERE/'results'/a.output;folder.mkdir(parents=True,exist_ok=False);p.dump(folder/'summary.json',result);p.dump(folder/'details.json',details);print(json.dumps(result))
def benchmark(a):
 model=a.models[0];pn=dict(np.load(model));reqs=[r['request'] for r in positions() if r['split']=='validation' and r['band']==2][:128];X=np.array([p.encode(r) for r in reqs]);forward=[];total=[];p.predict(pn,X)
 for repeat in range(8):
  for req,x in zip(reqs,X):
   start=time.perf_counter_ns();v=p.predict(pn,x[None]);forward.append((time.perf_counter_ns()-start)/1000)
   start=time.perf_counter_ns();s=p.public(req)[0];z=p.predict(pn,p.encode(req)[None])[0];choice=max(s['legal'],key=lambda t:(z[t],-t));total.append((time.perf_counter_ns()-start)/1000)
 result=dict(model=model.name,sha256=p.sha(model),device='localCPU NumPy warm deployment; no loading/startup',samples=len(forward),network_us=dict(median=float(np.median(forward)),p95=float(np.quantile(forward,.95))),validated_decision_us=dict(median=float(np.median(total)),p95=float(np.quantile(total,.95))))
 p.dump(HERE/'results'/a.output,result);print(json.dumps(result))
parser=argparse.ArgumentParser();parser.add_argument('command',choices=['evaluate','benchmark']);parser.add_argument('--models',type=Path,nargs='+',required=True);parser.add_argument('--reference',default='reference0');parser.add_argument('--split',default='validation',choices=['validation','test']);parser.add_argument('--band',default='2',choices=['0','1','2','all']);parser.add_argument('--output',required=True);args=parser.parse_args();globals()[args.command](args)
