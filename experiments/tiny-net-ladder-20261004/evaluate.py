#!/usr/bin/env python3
"""Independent-reference decision metrics; validation gates precede test read."""
import argparse,json,time
from pathlib import Path
import numpy as np
from pilot import HERE,dataset,positions,predict,dump,sha,encode
def cluster_ci(values,clusters):
 keys=np.unique(clusters);means=np.array([np.mean(values[clusters==k]) for k in keys]);rng=np.random.default_rng(9876)
 draws=means[rng.integers(0,len(means),(4000,len(means)))].mean(1)
 return dict(mean=float(means.mean()),ci95=list(map(float,np.quantile(draws,[.025,.975]))),source_deals=len(keys),method='percentile bootstrap4000; equal-weight source-deal means')
def evaluate(a):
 low,Wlo=dataset(a.labels);ref,W=dataset(a.reference);rows=positions();lookup={int(i):j for j,i in enumerate(low['ids'])}
 ri=np.array([j for j,i in enumerate(ref['ids']) if rows[int(i)]['split']==a.split]);assert len(ri);ids=ref['ids'][ri];li=np.array([lookup[int(i)] for i in ids]);q=ref['Q'][ri];m=ref['M'][ri];X=ref['X'][ri];n=m.sum(1);clusters=np.array([rows[int(i)]['source_id'] for i in ids]);best=np.argmax(np.where(m,q,-np.inf),1);vbest=q[np.arange(len(q)),best]
 choices={'ascending':np.argmax(m,1),'highest_tile':27-np.argmax(m[:,::-1],1),'teacher128':np.argmax(np.where(m,low['Q'][li],-np.inf),1)}
 for p in a.models:choices[p.stem]=np.argmax(np.where(m,predict(dict(np.load(p)),X),-np.inf),1)
 Y=np.unpackbits(ref['B'][ri],axis=2)[:,:,:W].astype(np.float32)
 order=np.argsort(np.where(m,q,-np.inf),1);second=order[:,-2];gaps=vbest-q[np.arange(len(q)),second];D=Y[np.arange(len(q)),best]-Y[np.arange(len(q)),second];gap_se=D.std(1,ddof=1)/np.sqrt(W);resolved=gaps>3*gap_se
 regret={'uniform_random':vbest-(q*m).sum(1)/n};metrics={};details=[]
 for name,c in choices.items():regret[name]=vbest-q[np.arange(len(q)),c]
 for name,r in regret.items():
  entry=cluster_ci(r,clusters);entry.update(root_weighted_mean=float(r.mean()),regret_q95=float(np.quantile(r,.95)),maximum=float(r.max()))
  if name in choices:
   c=choices[name];entry.update(exact_best_set_agreement=float(np.mean(r==0)),ascending_argmax_agreement=float(np.mean(c==best)),resolved_roots=int(resolved.sum()),resolved_best_set_agreement=float(np.mean(r[resolved]==0)) if resolved.any() else None,vs_uniform_random=cluster_ci(r-regret['uniform_random'],clusters),vs_ascending=cluster_ci(r-regret['ascending'],clusters))
   diff=Y[np.arange(len(q)),best]-Y[np.arange(len(q)),c];entry['mean_paired_within_root_SE']=float(np.mean(diff.std(1,ddof=1)/np.sqrt(W)))
  metrics[name]=entry
 for j,i in enumerate(ids):details.append(dict(position_index=int(i),source_id=clusters[j],best=int(best[j]),gap=float(gaps[j]),gap_paired_se=float(gap_se[j]),resolved=bool(resolved[j]),Qref=q[j].tolist(),legal=np.flatnonzero(m[j]).tolist(),choices={name:int(c[j]) for name,c in choices.items()},regrets={name:float(r[j]) for name,r in regret.items()}))
 main=a.models[0].stem;net=metrics[main];teacher=metrics['teacher128'];rnd=metrics['uniform_random'];asc=metrics['ascending']
 gate=dict(teacher_useful=teacher['vs_uniform_random']['ci95'][1]<0 and teacher['vs_ascending']['ci95'][1]<0,net_survival=net['mean']<=.8*rnd['mean'] and net['vs_uniform_random']['ci95'][1]<0 and net['mean']<=asc['mean'])
 gate['pass']=gate['teacher_useful'] and gate['net_survival']
 comparisons={f'{pa.stem}-minus-{pb.stem}':cluster_ci(regret[pa.stem]-regret[pb.stem],clusters) for j,pa in enumerate(a.models) for pb in a.models[j+1:]}
 summary=dict(tier='exploratory',split=a.split,roots=len(ids),source_deals=len(np.unique(clusters)),reference_worlds=W,low_worlds=Wlo,reference='independent same surrogate belief and frozen continuation; max sampled Qref is noisy, not optimal-game truth',metrics=metrics,paired_model_regret_differences=comparisons,gap=dict(mean=float(gaps.mean()),median=float(np.median(gaps)),resolved_roots=int(resolved.sum()),zero_sample_gap=int(np.sum(gaps==0)),qualification='empirical top-two gap>3pairedSE is a heuristic resolved subset; no multiplicity-adjusted simultaneous confidence'),gate=gate,
  source_hashes={str(p.resolve().relative_to(HERE)):sha(p) for p in a.models},outcomes='paired Bernoulli outcomes retained in data/reference*.npz; SE uses within-world differences; one tape/world',band_roots={str(b):sum(rows[int(i)]['band']==b for i in ids) for b in range(3)})
 out=HERE/'results'/a.output;out.mkdir(parents=True,exist_ok=False);dump(out/'summary.json',summary);dump(out/'details.json',details);print(json.dumps(summary))
def benchmark(a):
 p=dict(np.load(a.models[0]));rows=positions();reqs=[r['request'] for r in rows if r['split']=='validation'][:128];X=np.array([encode(r) for r in reqs]);predict(p,X);forward=[];endtoend=[]
 for rep in range(8):
  for x,req in zip(X,reqs):
   t=time.perf_counter_ns();v=predict(p,x[None]);forward.append((time.perf_counter_ns()-t)/1000)
   t=time.perf_counter_ns();xx=encode(req);v=predict(p,xx[None]);s=__import__('pilot').public(req)[0];c=max(s['legal'],key=lambda z:(v[0,z],-z));endtoend.append((time.perf_counter_ns()-t)/1000)
 out=dict(tier='exploratory',model=a.models[0].name,weights_sha256=sha(a.models[0]),samples=len(forward),device='local NumPy CPU; loaded warm model; Python deployment path',network_only_us={'median':float(np.median(forward)),'p95':float(np.quantile(forward,.95))},information_validation_encoding_argmax_us={'median':float(np.median(endtoend)),'p95':float(np.quantile(endtoend,.95))},model_file_bytes=a.models[0].stat().st_size,note='forward timings exclude model load/process startup; end-to-end includes two information-state validations, public legality, encoding, masked decision')
 dump(HERE/'results'/a.output,out);print(json.dumps(out))
p=argparse.ArgumentParser();p.add_argument('command',choices=['evaluate','benchmark']);p.add_argument('--models',nargs='+',type=Path,required=True);p.add_argument('--labels',default='labels0');p.add_argument('--reference',default='reference0');p.add_argument('--split',choices=['validation','test'],default='validation');p.add_argument('--output',required=True);a=p.parse_args();globals()[a.command](a)
