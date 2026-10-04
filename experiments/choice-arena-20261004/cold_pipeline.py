#!/usr/bin/env python3
"""Every timing request starts a fresh Python interpreter AND native process.

Invoke via run_capped.py. POSIX wait4 measures each frontend child separately.
"""
import argparse,gzip,hashlib,json,os,signal,statistics,subprocess,sys,tempfile,time
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
def call(argv):
 tick=time.perf_counter()
 with tempfile.TemporaryFile(mode='w+') as output,tempfile.TemporaryFile(mode='w+') as errors:
  p=subprocess.Popen(argv,stdin=subprocess.DEVNULL,stdout=output,stderr=errors)
  def expired(signum,frame):p.kill();os.wait4(p.pid,0);raise TimeoutError('fresh frontend exceeded60s')
  old=signal.signal(signal.SIGALRM,expired);signal.alarm(60)
  try:pid,status,usage=os.wait4(p.pid,0)
  finally:signal.alarm(0);signal.signal(signal.SIGALRM,old)
  host=time.perf_counter()-tick;p.returncode=os.waitstatus_to_exitcode(status);errors.seek(0)
  if p.returncode:raise RuntimeError(errors.read())
  output.seek(0);parse_started=time.perf_counter();result=json.load(output);parse=time.perf_counter()-parse_started
 total=time.perf_counter()-tick
 return dict(result=result,complete_cold_pipeline_s=total,python_process_host_s=host,
  wrapper_parse_s=parse,wrapper_overhead_s=total-host-parse,
  frontend_wait4_cpu_s=usage.ru_utime+usage.ru_stime,
  frontend_peak_rss_bytes=usage.ru_maxrss*(1 if sys.platform=='darwin' else 1024))
def main():
 p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
 p.add_argument('--start',type=int,default=962000);p.add_argument('--count',type=int,default=32)
 p.add_argument('--worlds',type=int,default=40);p.add_argument('--workers',type=int,choices=[1,4],default=4)
 p.add_argument('--repeats',type=int,default=8);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
 records=[];expected=None;rows_hash=None
 for repeat in range(a.repeats):
  order=['original','compiled'] if repeat%2==0 else ['compiled','original']
  for name in order:
   argv=[sys.executable,str(HERE/'cold_worker.py'),'--variant',name,'--start',str(a.start),'--count',str(a.count),'--worlds',str(a.worlds),'--workers',str(a.workers)]
   run=call(argv);result=run['result'];answer=result['run']['result']['answers']
   if expected is None:expected=answer;rows_hash=result['all_rows_sha256']
   assert answer==expected and result['all_rows_sha256']==rows_hash
   records.append(dict(repeat=repeat,variant=name,order=order,run=run))
 variants={}
 for name in ['original','compiled']:
  rr=[r['run'] for r in records if r['variant']==name]
  variants[name]={k:statistics.median(r[k] for r in rr) for k in ['complete_cold_pipeline_s','python_process_host_s','wrapper_parse_s','wrapper_overhead_s','frontend_wait4_cpu_s','frontend_peak_rss_bytes']}
  variants[name].update({k:statistics.median(r['result'][k] for r in rr) for k in ['python_import_s','generation_s','python_cpu_after_import_s']})
  variants[name].update({k:statistics.median(r['result']['run'][k] for r in rr) for k in ['child_cpu_s','process_peak_rss_bytes','serialization_s','host_s','output_parse_s','harness_overhead_s']})
 with gzip.open(a.out/'records.json.gz','wt') as f:json.dump(records,f)
 summary=dict(arguments=vars(a),selected=records[0]['run']['result']['selected'],variants=variants,
  original_over_compiled=variants['original']['complete_cold_pipeline_s']/variants['compiled']['complete_cold_pipeline_s'],
  vector_comparisons=len(records)*len(expected),exact_all_rows_sha256=rows_hash,
  binary_sha256=hashlib.sha256((HERE/'checks/baseline-target/release/native-frontier').read_bytes()).hexdigest(),
  source_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [HERE/'cold_worker.py',HERE/'sampling.py',Path(__file__)]},
  scope='Finite late Straight component. Fresh Python interpreter/import, all proposal generation and compiled constraint setup, filtering/input JSON, cold matched-worker native subprocess/replay/schedule, temporary files, result hashing/printing, frontend process teardown and wrapper parse/files included in direct cold wall. Driver shared startup and final aggregate record writing are outside per-request wall, retained in capped receipt. wait4 frontend RSS/CPU and nested native PID-specific RSS/CPU are separate; CPU attribution can include waited descendants so never add these two CPU figures. No production phone or strength claim.')
 (a.out/'summary.json').write_text(json.dumps(summary,indent=2,default=str)+'\n');print(json.dumps(summary,default=str))
if __name__=='__main__':main()
