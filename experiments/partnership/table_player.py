"""Transport and independent rules check for the shared native/browser player.

The older configurable player.py remains available to reproduce historical
research profiles. Live Plunge policy, budgets and fallbacks live in Rust.
"""
import json
import os
from pathlib import Path
import subprocess

from player import normalize, checked_response, checked_status
from rules import information_state

BINARY = Path(__file__).resolve().parents[2] / 'walt/target/release/walt-table'


def decide(raw, *, mode='baseline', n=40, n0=8, n1=2, budget_ms=14000,
           inner_belief='voidless', selection='fixed', modeled_selection='fixed',
           review='off', session=None):
    if (mode, n0, n1, inner_belief, selection, modeled_selection) != (
            'baseline', 8, 2, 'voidless', 'fixed', 'fixed') or review not in ('off', 'partner-rollout'):
        raise ValueError('shared table player requires default L1 or L1 + partner rollout')
    req = normalize(raw)
    state = information_state(req)
    payload = dict(request=req, worlds=n, partner=review == 'partner-rollout', budget_ms=budget_ms)
    def check(value):
        if checked_status(value,state) != 'completed' or checked_response(value,state)[0] is None:
            raise ValueError('shared player disagrees with independent rules')
    return call_player(payload, check)


def auction(body):
    if not isinstance(body,dict) or set(body) != {'auction','budget_ms'}:
        raise ValueError('auction needs auction and budget_ms')
    req=body['auction']
    if not isinstance(req,dict) or set(req) != {'hand','seat','bid','seed'}:
        raise ValueError('auction accepts only own hand, seat, bid, and seed')
    if type(body['budget_ms']) is not int or not 100 <= body['budget_ms'] <= 14000:
        raise ValueError('invalid auction budget')
    normalized=normalize(dict(decl=0,bidder=req['seat'],plays=[],**req))
    information_state(normalized)
    def check(value):
        if value.get('schema') != 'walt-auction-v1' or any(value.get(k)!=v for k,v in req.items()):
            raise ValueError('auction request mismatch')
        if value.get('decl') not in (*range(8),9):raise ValueError('invalid declaration')
    return call_player(body,check)


def call_player(payload, check):
    budget_ms=payload['budget_ms']
    interrupted = None
    try:
        result = subprocess.run([str(BINARY)], input=json.dumps(payload)+'\n', text=True,
                                capture_output=True, timeout=budget_ms/1000+4,
                                env={**os.environ, 'RAYON_NUM_THREADS': os.environ.get('WALT_RAYON_THREADS','2')})
        output = result.stdout
        if result.returncode: interrupted = 'native worker stopped'
    except subprocess.TimeoutExpired as error:
        output = error.stdout or ''
        if isinstance(output,bytes): output = output.decode('utf-8', errors='replace')
        interrupted = 'host deadline; last completed decision retained'
    saved = None
    for line in output.splitlines():
        try: message = json.loads(line)
        except json.JSONDecodeError: continue
        value = message.get('result', message.get('checkpoint'))
        if not isinstance(value,dict): continue
        if 'error' in value: raise ValueError(value['error'])
        check(value)
        saved = value
    if saved is None: raise RuntimeError('shared player produced no complete decision')
    if interrupted: saved = {**saved, 'interruption':interrupted}
    return saved
