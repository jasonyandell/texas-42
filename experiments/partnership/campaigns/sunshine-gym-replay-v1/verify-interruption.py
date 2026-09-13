"""Stop the actual replay after a durable trajectory; retain its byte hashes."""
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

panel, output = map(Path, sys.argv[1:])
output.mkdir(parents=True, exist_ok=True)
assert not (output / 'manifest.json').exists(), 'use a fresh output directory'
runner = Path(__file__).resolve().parents[2] / 'gym_replay.py'
with (output / 'interrupt-child.log').open('w') as log:
    process = subprocess.Popen([sys.executable, str(runner), 'run', str(panel), str(output)],
                               stdout=log, stderr=log)
    deadline = time.monotonic() + 45
    while not list((output / 'items').glob('*.json')):
        assert process.poll() is None, 'runner ended before interruption'
        assert time.monotonic() < deadline, 'no durable trajectory in allowance'
        time.sleep(.005)
    os.kill(process.pid, signal.SIGINT)
    code = process.wait(timeout=180)
assert code == 75, ('expected resumable incomplete status', code)
files = {str(p): hashlib.sha256(p.read_bytes()).hexdigest()
         for folder in ('items', 'decisions') for p in (output / folder).glob('*.json')}
receipt = dict(schema='sunshine-gym-replay-interruption-v1', returncode=code,
               completed_trajectories=len(list((output / 'items').glob('*.json'))),
               durable_files=files, runner_sha256=hashlib.sha256(runner.read_bytes()).hexdigest())
(output / 'interruption.json').write_text(json.dumps(receipt, sort_keys=True, indent=2) + '\n')
print(json.dumps({k: v for k, v in receipt.items() if k != 'durable_files'}))
