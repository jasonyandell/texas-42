"""Local reproducible final static/hash/cap audit. Run under inherited watchdog."""
import ast,collections,gzip,hashlib,json,re,subprocess
from pathlib import Path
h=Path(__file__).resolve().parent;root=h.parents[1]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
for p in h.rglob('*.py'):
 if'target'not in p.parts:ast.parse(p.read_text(),filename=str(p))
links=0
for p in h.rglob('*.md'):
 if'target' in p.parts:continue
 for target in re.findall(r'\]\(([^\)]+)\)',p.read_text()):
  if '://'in target:continue
  file=target.split('#')[0];file=re.sub(r':\d+$','',file)
  if file:assert(p.parent/file).exists(),(str(p),target);links+=1
for key,row in json.loads((h/'ARTIFACTS.json').read_text()).items():
 assert sha(root/row['path'])==row['sha256']
 if 'archive_path'in row:
  packed=(root/row['archive_path']).read_bytes();assert hashlib.sha256(packed).hexdigest()==row['archive_sha256']
  assert hashlib.sha256(gzip.decompress(packed)).hexdigest()==row['sha256']
rr=[]
for base in ['results','checks']:
 for p in(h/base).rglob('run.json'):
  if'target'in p.parts:continue
  q=json.loads(p.read_text())
  assert 0<q['allowance_seconds']<=295 and q['elapsed_seconds']<300 and not q['cleanup_errors']
  assert q['child_returncode']is not None and q['status']in ['completed','failed']
  rr.append((str(p.relative_to(h)),q))
failed=[p for p,q in rr if q['status']=='failed'];assert sorted(failed)==sorted(['checks/build-runner/run.json','checks/scalar-boundaries/run.json','checks/scalar-boundaries-import-fixed/run.json'])
assert not(root/'.git/objects/info/alternates').exists()
tracked=subprocess.check_output(['git','diff','--name-only','56794dc321f3546223ff1359ed30b87911130fd9'],cwd=root,text=True).splitlines()
assert all(p.startswith('experiments/higher-k-budget-20261004/')for p in tracked)
report=dict(schema='higher-k-final-checkpoint-audit-v1',python_sources_parse=True,relative_document_links=links,all_artifact_hashes=True,independent_git_objects=True,preserved_inherited_tracked_sources=True,receipts=len(rr),failed_development_receipts=failed,max_allowance_seconds=max(q['allowance_seconds']for _,q in rr),max_elapsed_seconds=max(q['elapsed_seconds']for _,q in rr),no_cleanup_errors=True)
print(json.dumps(report,indent=2))
