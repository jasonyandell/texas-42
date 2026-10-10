#!/usr/bin/env python3
"""Targeted post-pilot diagnostics; does not select/train another model."""
import json
import numpy as np
from pilot import HERE,dataset,positions,predict,dump
rows=positions();low,_=dataset('labels0');control,W=dataset('control');lookup={int(i):j for j,i in enumerate(low['ids'])};li=np.array([lookup[int(i)] for i in control['ids']]);m=control['M'];n=m.sum(1);lq=low['Q'][li];cq=control['Q'];la=(lq-(lq*m).sum(1,keepdims=True)/n[:,None])*m;ca=(cq-(cq*m).sum(1,keepdims=True)/n[:,None])*m
lc=np.argmax(np.where(m,lq,-np.inf),1);best=np.max(np.where(m,cq,-np.inf),1);teacher_regret=best-cq[np.arange(len(cq)),lc];nc=np.argmax(np.where(m,predict(dict(np.load(HERE/'models/n0-32.npz')),control['X']),-np.inf),1);nr=best-cq[np.arange(len(cq)),nc]
out=dict(tier='exploratory',post_pilot=True,selection='first96trainingroots in batch0; not extra heldout proof and no new model tuning',roots=len(cq),independent_control_worlds=W,centered_advantage_label_RMSE=float(np.sqrt(np.sum((la-ca)**2)/m.sum())),low_teacher_reference_regret=float(np.mean(teacher_regret)),selected_n0_training_root_regret=float(np.mean(nr)),teacher_action_exact_ref_agreement=float(np.mean(teacher_regret==0)),model_action_exact_ref_agreement=float(np.mean(nr==0)))
out['heldout_by_band']={}
for name in ['test0','test1']:
 details=json.loads((HERE/'results'/name/'details.json').read_text());bands={}
 for b in range(3):
  rr=[r for r in details if rows[r['position_index']]['band']==b];bands[str(b)]={'roots':len(rr),'mean_regret':{policy:float(np.mean([r['regrets'][policy] for r in rr])) for policy in ['uniform_random','teacher128','n0-32','n1-32']}}
 out['heldout_by_band'][name]=bands
out['training_generalization']={}
for name in ['n0-32','n1-32']:
 meta=json.loads((HERE/'models'/f'{name}.json').read_text());h=meta['history'];out['training_generalization'][name]={'selected_epoch':meta['selected_epoch'],'selected_validation_scaled_MSE':h[meta['selected_epoch']-1]['validation_advantage_mse_scaled'],'epoch100_train_loss_including_L2':h[-1]['train_loss'],'epoch100_validation_scaled_MSE':h[-1]['validation_advantage_mse_scaled']}
out['interpretation']='128-world teacher selection regret is much smaller than net regret on both train-control and heldout roots; extra label samples alone do not explain the fit gap. Training loss falls while new-deal validation worsens: representation/coverage/generalization are candidates, not causally isolated. Rung difference is inconclusive; do not automatically scale or promise improvement.'
dump(HERE/'results/diagnostics.json',out);print(json.dumps(out))
