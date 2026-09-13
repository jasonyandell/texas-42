#!/usr/bin/env python3
"""Run the Mac sunshine table and its native player; Ctrl-C stops both safely."""
import argparse
import hashlib
import os
from pathlib import Path
import signal
import socket
import subprocess
import sys
import time
from urllib.request import urlopen

HERE=Path(__file__).resolve().parent


def frontend_identity(root):
    files=sorted(p for folder in ('src','public') for p in (root/folder).rglob('*') if p.is_file())
    files += [root/p for p in ('package.json','package-lock.json','vite.config.ts','index.html')]
    digest=hashlib.sha256()
    for path in files:
        digest.update(str(path.relative_to(root)).encode()+b'\0'+path.read_bytes()+b'\0')
    return digest.hexdigest()


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plunge',type=Path,default=Path('/Users/jason/code/plunge-sunshine'))
    parser.add_argument('--data',type=Path,default=Path('/Users/jason/data/texas-42/plunge-sunshine'))
    parser.add_argument('--port',type=int,default=4244)
    parser.add_argument('--bridge-port',type=int,default=4245)
    args=parser.parse_args();root=args.plunge.resolve();data=args.data.resolve()
    if not (root/'node_modules/.bin/vite').exists():parser.error('Install Plunge dependencies with npm ci first.')
    for binary in ('partnership','partnership_gym','partner_rollout'):
        if not (HERE.parents[1]/'walt/target/release'/binary).exists():parser.error('Build native binary '+binary+' first (see PLUNGE.md).')
    if args.port==args.bridge_port:parser.error('The table and player need different ports.')
    for port in (args.port,args.bridge_port):
        if not 1024<=port<=65535:parser.error('Use ports between 1024 and 65535.')
        with socket.socket() as sock:
            sock.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1)
            try:sock.bind(('127.0.0.1',port))
            except OSError:parser.error(f'Port {port} is already in use; stop the earlier launcher or choose other ports.')
    logdir=data/'server';logdir.mkdir(parents=True,exist_ok=True)
    children=[];logs=[]
    env={**os.environ,'PLUNGE_SOURCE_ID':frontend_identity(root),'WALT_RAYON_THREADS':'2',
         'PLUNGE_BRIDGE_PORT':str(args.bridge_port),'VITE_NATIVE_TABLE':'1'}
    def stop(*_):raise KeyboardInterrupt()
    signal.signal(signal.SIGTERM,stop)
    try:
        commands=[('bridge',[sys.executable,str(HERE/'plunge_bridge.py'),'--data',str(data),'--port',str(args.bridge_port)],HERE),
                  ('table',[str(root/'node_modules/.bin/vite'),'--host','127.0.0.1','--port',str(args.port),'--strictPort'],root)]
        for name,command,cwd in commands:
            log=(logdir/(name+'.log')).open('ab');logs.append(log)
            children.append(subprocess.Popen(command,cwd=cwd,env=env,stdout=log,stderr=log,start_new_session=True))
        for port,path in ((args.bridge_port,'/api/health'),(args.port,'/')):
            deadline=time.monotonic()+15
            while True:
                if any(p.poll() is not None for p in children):raise RuntimeError('A service stopped; see '+str(logdir))
                try:
                    with urlopen(f'http://127.0.0.1:{port}{path}',timeout=1) as response:
                        if response.status==200:break
                except OSError:pass
                if time.monotonic()>deadline:raise RuntimeError('Startup timed out; see '+str(logdir))
                time.sleep(.1)
        print(f'Ready: http://127.0.0.1:{args.port}\nRecords: {data}\nCtrl-C to stop; launcher PID {os.getpid()}.',flush=True)
        while all(p.poll() is None for p in children):time.sleep(.5)
        raise RuntimeError('A service stopped; see '+str(logdir))
    except KeyboardInterrupt:
        print('Stopping the table and saving completed work.',flush=True)
    finally:
        for proc in reversed(children):
            if proc.poll() is None:os.killpg(proc.pid,signal.SIGTERM)
        for proc in children:
            try:proc.wait(timeout=20)
            except subprocess.TimeoutExpired:os.killpg(proc.pid,signal.SIGKILL);proc.wait()
        for log in logs:log.close()


if __name__=='__main__':main()
