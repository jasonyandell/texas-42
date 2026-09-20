"""Local native player, immutable decision receipts, and a separate finished-hand examiner."""
import argparse
from dataclasses import replace
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import threading
import uuid
from urllib.parse import urlparse

import campaign as c
import gym
from matchup import Player
from player import normalize
from table_player import decide, auction, BINARY as TABLE_BINARY
from plunge_io import flag_root
from rules import information_state
from runtime import DecisionSession
from plunge_analysis import FUTURES, identity_for

PRESETS=('l1-default','l1-partner-rollout')
WATCHDOG=gym.HERE/'packet/texas42-partnership-launch-v0.1/tools/run_capped.py'
OPENING_WORLDS=160
OPENING_BUDGET_MS=20000


def fields(value,names):
    if not isinstance(value,dict) or set(value)!=set(names.split()):raise ValueError('unexpected or missing fields: '+names)


def identifier(value):
    if not isinstance(value,str) or not re.fullmatch('[a-zA-Z0-9_-]{1,80}',value):raise ValueError('invalid identifier')
    return value


class Store:
    def __init__(self,root,frontend='development'):
        self.root=Path(root);self.root.mkdir(parents=True,exist_ok=True)
        self.implementation=dict(table_binary=gym.file_hash(TABLE_BINARY), table_adapter=gym.file_hash(gym.HERE/'table_player.py'), player=c.identities(),bridge=gym.file_hash(__file__),
                                 importer=gym.file_hash(gym.HERE/'plunge_io.py'),frontend=frontend)
        self.session=DecisionSession();self.lock=threading.Lock();self.job_lock=threading.Lock();self.jobs={}
        self.estimate_lock=threading.Lock()
        self.players={name:Player(**c.read(gym.HERE/'players.json')[name]) for name in PRESETS}

    def decision(self,body):
        optional=isinstance(body,dict) and 'think_deeper' in body
        fields(body,'request player game_id hand_number'+(' think_deeper' if optional else ''))
        think_deeper=body.get('think_deeper',False)
        if type(think_deeper) is not bool:raise ValueError('think_deeper must be boolean')
        fields(body['request'],'decl bid bidder seat hand plays seed')
        req=normalize(body['request'])
        information_state(req)
        game_id=identifier(body['game_id']);hand_number=body['hand_number']
        if type(hand_number) is not int or not 1<=hand_number<=10000:raise ValueError('invalid hand number')
        if body['player'] not in self.players:raise ValueError('unknown live player')
        player=self.players[body['player']]
        # The bidder's opening and opt-in deeper play use the inspection
        # profile. The bounded partner review requires 40/8, so stays off.
        if think_deeper or (req['seat']==req['bidder'] and not req['plays']):
            player=replace(player,n=OPENING_WORLDS,budget_ms=OPENING_BUDGET_MS,review='off')
        identity=dict(request=req,player=c.asdict(player),implementation=self.implementation,
                      game_id=game_id,hand_number=hand_number)
        rid=gym.digest(identity);path=self.root/'decisions'/(rid+'.json')
        with self.lock:
            saved=c.read(path)
            if saved is not None:
                if saved['identity']!=identity:raise ValueError('receipt identity mismatch')
                return self.receipt(rid)
            response=decide(req,session=self.session,**player.kwargs())
            saved=dict(schema='plunge-decision-v1',id=rid,identity=identity,response=response,created=c.now())
            saved['sha256']=gym.digest(saved)
            gym.atomic(path,saved)
            return saved

    def receipt(self,rid):
        path=self.root/'decisions'/(identifier(rid)+'.json');value=c.read(path)
        if value is None:raise FileNotFoundError('decision receipt unavailable')
        if value['id']!=rid or gym.digest(value['identity'])!=rid:raise ValueError('receipt identity mismatch')
        if value.get('sha256')!=gym.digest({k:v for k,v in value.items() if k!='sha256'}):
            raise ValueError('receipt contents changed')
        return value

    def estimate(self,body):
        """Reinspect one own/public position without changing its original decision.

        Inspection has its own worker and the normal decision deadline. Reject
        concurrent inspections instead of queuing unbounded work behind a play.
        Only completed primary estimates are cached; a timeout can be retried.
        """
        fields(body,'request worlds')
        fields(body['request'],'decl bid bidder seat hand plays seed')
        req=normalize(body['request']);worlds=body['worlds']
        information_state(req)
        if type(worlds) is not int or worlds not in (40,160):raise ValueError('choose 40 or 160 sampled worlds')
        player=replace(self.players['l1-default'],n=worlds,
                       budget_ms=OPENING_BUDGET_MS if worlds==OPENING_WORLDS else 14000)
        identity=dict(request=req,player=c.asdict(player),implementation={
            k:v for k,v in self.implementation.items() if k!='frontend'})
        eid=gym.digest(identity);path=self.root/'estimates'/(eid+'.json')
        if not self.estimate_lock.acquire(blocking=False):raise ValueError('another move is being inspected; retry in a moment')
        try:
            saved=c.read(path)
            if saved is not None:
                if saved['identity']!=identity or saved['id']!=eid:raise ValueError('estimate identity mismatch')
                if saved.get('sha256')!=gym.digest({k:v for k,v in saved.items() if k!='sha256'}):
                    raise ValueError('estimate contents changed')
                return saved
            with DecisionSession() as session:
                response=decide(req,session=session,**player.kwargs())
            saved=dict(schema='plunge-estimate-v1',id=eid,identity=identity,response=response,created=c.now())
            saved['sha256']=gym.digest(saved)
            if response['route'] in ('baseline','forced'):gym.atomic(path,saved)
            return saved
        finally:self.estimate_lock.release()

    def flag(self,body):
        fields(body,'share_code ply seed note alternative receipt_id')
        note=body['note']
        if not isinstance(note,str) or len(note)>4000:raise ValueError('note must be at most 4000 characters')
        receipt=self.receipt(body['receipt_id']) if body['receipt_id'] is not None else None
        seed=receipt['identity']['request']['seed'] if receipt else body['seed']
        req,played,game=flag_root(body['share_code'],body['ply'],seed)
        state=information_state(req);alternative=body['alternative']
        if alternative is not None and (type(alternative) is not int or alternative not in state['legal']):
            raise ValueError('suggested alternative must be legal at that position')
        if receipt and (receipt['identity']['request']!=req or receipt['response']['choice']!=played):
            raise ValueError('original receipt does not describe this recorded play')
        fid=uuid.uuid4().hex
        item=dict(id=gym.digest(req)[:20],request=req,source=dict(kind='plunge-flag',flag=fid,played=played,alternative=alternative,note=note))
        value=dict(schema='plunge-flag-v1',id=fid,created=c.now(),request=req,played=played,alternative=alternative,
                   note=note,ply=body['ply'],share_code=body['share_code'],receipt_id=body['receipt_id'],
                   original_receipt=receipt,examiner_game=game,gym=item)
        gym.atomic(self.root/'flags'/(fid+'.json'),value)
        gym.atomic(self.root/'gym-inputs'/(fid+'.json'),item)
        return value

    def get_flag(self,fid):
        value=c.read(self.root/'flags'/(identifier(fid)+'.json'))
        if value is None:raise FileNotFoundError('flag unavailable')
        return value

    def analysis_dir(self,fid,future):
        flag=self.get_flag(fid)
        if future not in FUTURES:raise ValueError('unknown continuation model')
        key=gym.digest(identity_for(flag['request'],future))[:20]
        return self.root/'analyses'/fid/(future+'-'+key)

    def analysis(self,fid,future,start=False):
        if self.get_flag(fid)['request']['bid']!=30:
            return dict(status='outside-scope',message='This research comparison is scoped to bid 30. Original scores and move rechecks use the actual contract.')
        output=self.analysis_dir(fid,future);key=str(output)
        with self.job_lock:
            proc=self.jobs.get(key)
            if proc and proc.poll() is not None:self.jobs.pop(key,None);proc=None
            result=c.read(output/'result.json')
            if result and result['status'] in ('complete','outside-scope'):return result
            if start and proc is None:
                if any(p.poll() is None for p in self.jobs.values()):raise ValueError('another comparison is running; let it finish first')
                output.mkdir(parents=True,exist_ok=True)
                run=output/'runs'/uuid.uuid4().hex
                command=[sys.executable,str(WATCHDOG),'--seconds','295','--output-dir',str(run),'--',
                         sys.executable,str(gym.HERE/'plunge_analysis.py'),str(self.root/'flags'/(fid+'.json')),
                         str(output),'--future',future]
                proc=subprocess.Popen(command,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,start_new_session=True,
                                      env={**os.environ,'WALT_RAYON_THREADS':'2'})
                self.jobs[key]=proc
            progress=c.read(output/'trajectories/status.json',{})
            if proc:return dict(status='running',saved=progress.get('saved_total',0),message='Comparing complete continuations. Progress is saved.')
            if result:return result
            runs=list((output/'runs').glob('*/run.json'))
            if runs:
                last=c.read(max(runs,key=lambda p:p.stat().st_mtime))
                return dict(status='partial',saved=progress.get('saved_total',0),
                            message=f"Previous slice {last['status']}; saved work survives. Resume to retry.")
            return dict(status='ready',message='Ready for a full comparison, up to 400 compatible hands.')

    def pause(self,fid,future):
        output=self.analysis_dir(fid,future)
        with self.job_lock:
            proc=self.jobs.pop(str(output),None)
            if proc and proc.poll() is None:
                proc.terminate()  # The watchdog terminates its whole workload group.
                proc.wait(timeout=5)
            result=c.read(output/'result.json')
            if result and result['status'] in ('complete','outside-scope'):return result
            result=dict(status='partial',message='Paused. Completed trajectories and decisions are saved; resume whenever you like.')
            gym.atomic(output/'result.json',result)
            return result

    def close(self):
        with self.job_lock:
            for proc in self.jobs.values():
                if proc.poll() is None:
                    os.killpg(proc.pid,signal.SIGTERM)
            for proc in self.jobs.values():
                try:proc.wait(timeout=5)
                except subprocess.TimeoutExpired:os.killpg(proc.pid,signal.SIGKILL);proc.wait()
        with self.lock:self.session.close()
        # An inspection owns and closes its worker, including during shutdown.
        with self.estimate_lock:pass


def handler(store):
    class Handler(BaseHTTPRequestHandler):
        def log_message(self,*_):pass
        def reply(self,status,value):
            data=json.dumps(value,separators=(',',':')).encode()
            self.send_response(status);self.send_header('Content-Type','application/json');self.send_header('Content-Length',str(len(data)))
            self.send_header('Cache-Control','no-store');self.end_headers()
            try:self.wfile.write(data)
            except (BrokenPipeError,ConnectionResetError):pass

        def dispatch(self,post):
            try:
                if urlparse('http://'+self.headers.get('Host','')).hostname not in ('127.0.0.1','localhost'):
                    raise ValueError('localhost requests only')
                origin=self.headers.get('Origin')
                if origin and urlparse(origin).hostname not in ('127.0.0.1','localhost'):raise ValueError('localhost origin required')
                parts=urlparse(self.path).path.strip('/').split('/')
                if parts[0]=='api':parts=parts[1:]
                body=None
                if post:
                    size=int(self.headers.get('Content-Length','0'))
                    if not 0<size<=100000:raise ValueError('invalid request size')
                    self.connection.settimeout(20)
                    body=json.loads(self.rfile.read(size))
                if parts==['health'] and not post:value=dict(status='ready',players=list(PRESETS),implementation=store.implementation)
                elif parts==['decide'] and post:value=store.decision(body)
                elif parts==['auction'] and post:value=auction(body)
                elif parts==['estimates'] and post:value=store.estimate(body)
                elif len(parts)==2 and parts[0]=='receipts' and not post:value=store.receipt(parts[1])
                elif parts==['flags'] and post:value=store.flag(body)
                elif len(parts)==2 and parts[0]=='flags' and not post:value=store.get_flag(parts[1])
                elif len(parts)==4 and parts[0]=='flags' and parts[2]=='compare':
                    if post:fields(body,'')
                    value=store.analysis(parts[1],parts[3],post)
                elif len(parts)==5 and parts[0]=='flags' and parts[2]=='compare' and parts[4]=='pause' and post:
                    fields(body,'');value=store.pause(parts[1],parts[3])
                else:raise FileNotFoundError('endpoint unavailable')
                self.reply(200,value)
            except FileNotFoundError as error:self.reply(404,dict(error=str(error)))
            except (ValueError,TypeError,KeyError,AssertionError) as error:self.reply(400,dict(error=str(error) or 'invalid record'))
            except Exception as error:self.reply(500,dict(error=str(error)))
        def do_GET(self):self.dispatch(False)
        def do_POST(self):self.dispatch(True)
    return Handler


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--data',type=Path,required=True);p.add_argument('--port',type=int,default=4245)
    a=p.parse_args();os.environ['WALT_RAYON_THREADS']='2'
    store=Store(a.data,os.environ.get('PLUNGE_SOURCE_ID','development'))
    server=ThreadingHTTPServer(('127.0.0.1',a.port),handler(store))
    print(f'Native player ready at http://127.0.0.1:{a.port}; records: {a.data}',flush=True)
    def stop(*_):raise KeyboardInterrupt()
    signal.signal(signal.SIGTERM,stop)
    try:server.serve_forever()
    except KeyboardInterrupt:pass
    finally:server.server_close();store.close()


if __name__=='__main__':main()
