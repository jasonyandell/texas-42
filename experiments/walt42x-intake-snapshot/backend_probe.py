"""Reproduce the installed MPS integer-key issue independently of game logic."""
import json
import torch

rows=[]
for dtype in (torch.int32,torch.int64):
    cpu=torch.arange(20_000_000,20_000_100,dtype=dtype)
    a=cpu.to('mps')
    unique,inverse=torch.unique(a,return_inverse=True)
    sorted_values,_=torch.sort(a.flip(0))
    rows.append({'dtype':str(dtype),'input_count':len(cpu),
                 'upload_exact':torch.equal(a.cpu(),cpu),
                 'unique_count':len(unique),
                 'unique_reconstructs':torch.equal(unique[inverse].cpu(),cpu),
                 'sort_exact':torch.equal(sorted_values.cpu(),cpu),
                 'multiply_exact':torch.equal((a*28).cpu(),cpu*28)})
mask=torch.zeros(20_000_100,dtype=torch.int32,device='mps')
mask[20_000_001]=1
coordinate=torch.nonzero(mask).cpu().item()
report={'torch':torch.__version__,'device':'mps','integer_cases':rows,
        'nonzero_expected':20_000_001,'nonzero_actual':coordinate}
with open('results/backend-precision.json','w') as f: json.dump(report,f,indent=2)
print(json.dumps(report))
