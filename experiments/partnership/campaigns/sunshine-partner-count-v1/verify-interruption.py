"""Interrupt the conditional runner after its first durable paired result."""
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

source,output=map(Path,sys.argv[1:])
output.mkdir(parents=True,exist_ok=True)
assert not (output/'manifest.json').exists(), 'use a fresh verification directory'
runner=Path(__file__).resolve().parents[2]/'sunshine_worlds.py'
with (output/'interrupt-child.log').open('w') as log:
    process=subprocess.Popen([sys.executable,str(runner),str(source),str(output)],stdout=log,stderr=log)
    deadline=time.monotonic()+30
    while not list((output/'items').glob('*.json')):
        assert process.poll() is None, 'runner ended before interruption'
        assert time.monotonic()<deadline, 'no durable result in allowance'
        time.sleep(.002)
    os.kill(process.pid,signal.SIGINT)
    code=process.wait(timeout=30)
assert code==75, ('expected resumable incomplete status',code)
files={str(p):hashlib.sha256(p.read_bytes()).hexdigest()
       for folder in ('items','decisions') for p in (output/folder).glob('*.json')}
receipt=dict(schema='sunshine-worlds-interruption-v1',returncode=code,
             completed_pairs=len(list((output/'items').glob('*.json'))),
             durable_files=files,runner_sha256=hashlib.sha256(runner.read_bytes()).hexdigest())
(output/'interruption.json').write_text(json.dumps(receipt,sort_keys=True,indent=2)+'\n')
print(json.dumps({k:v for k,v in receipt.items() if k!='durable_files'}))
