"""Audit a live-table receipt and exercise real native comparison pause/resume.

Run inside run_capped.py. Requires two saved flags: a completed L1 comparison
and a larger position with no 'reviewed' comparison yet. Produces a compact
receipt and replays all completed trajectories with the independent referee.
"""
import argparse
import json
from pathlib import Path
import time
from urllib.error import HTTPError
from urllib.request import Request, urlopen

import gym
import campaign as c
from gym_deployed import verify
from plunge_io import flag_root


def main(a):
    def api(path,body=None):
        request=Request(a.url+'/api/'+path,data=None if body is None else json.dumps(body).encode(),
                        headers={'Content-Type':'application/json'})
        with urlopen(request,timeout=20) as response:return json.load(response)
    flag=api('flags/'+a.flag);receipt=flag['original_receipt'];identity=receipt['identity']
    body=dict(request=identity['request'],player=identity['player']['name'],
              game_id=identity['game_id'],hand_number=identity['hand_number'])
    assert api('receipts/'+receipt['id'])==receipt
    current=api('decide',body)
    assert api('decide',body)==current  # Retry under the current implementation.
    for hidden in ('hands','worlds','teacher'):
        try:api('decide',{**body,'request':{**body['request'],hidden:[]}})
        except HTTPError as error:assert error.code==400
        else:raise AssertionError('hidden field accepted')
    req,played,_=flag_root(flag['share_code'],flag['ply'],flag['request']['seed'])
    assert req==identity['request'] and played==receipt['response']['choice']
    assert api('flags/'+a.flag+'/compare/l1')['status']=='complete'

    path='flags/'+a.pause_flag+'/compare/reviewed'
    assert api(path)['status']=='ready','choose a fresh pause-test flag/future'
    before=set((a.data/'analyses'/a.pause_flag).glob('reviewed-*'))
    started=time.monotonic();assert api(path,{})['status']=='running'
    dirs=set((a.data/'analyses'/a.pause_flag).glob('reviewed-*'))-before;assert len(dirs)==1
    output=dirs.pop()
    while not list((output/'trajectories/items').glob('*.json')):
        assert time.monotonic()-started<15,'no completed trajectory to pause after'
        time.sleep(.01)
    paused=api(path+'/pause',{});assert paused['status']=='partial','comparison finished before pause'
    saved={str(p.relative_to(output)):gym.file_hash(p) for folder in ('trajectories/items','decisions')
           for p in (output/folder).rglob('*.json')}
    assert saved
    assert api(path,{})['status']=='running'
    while True:
        result=api(path)
        if result['status']!='running':break
        assert time.monotonic()-started<240
        time.sleep(.1)
    assert result['status']=='complete',result
    assert all(gym.file_hash(output/p)==digest for p,digest in saved.items())
    audits={}
    for fid in (a.flag,a.pause_flag):
        for p in (a.data/'analyses'/fid).glob('*/items/*.json'):
            audits[str(p.relative_to(a.data))]=verify(c.read(p),query=False)
    value=dict(schema='plunge-live-audit-v1',created=c.now(),flag=a.flag,pause_flag=a.pause_flag,
               receipt=receipt['id'],current_receipt=current['id'],retry_identical=True,hidden_input_rejected=True,
               pause_status=paused['status'],preserved_files=len(saved),resumed_status=result['status'],
               elapsed_seconds=round(time.monotonic()-started,3),audits=audits)
    gym.atomic(a.output,value);print(gym.canonical(value))


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--url',default='http://127.0.0.1:4244');p.add_argument('--data',type=Path,required=True)
    p.add_argument('--flag',required=True);p.add_argument('--pause-flag',required=True);p.add_argument('--output',type=Path,required=True)
    main(p.parse_args())
