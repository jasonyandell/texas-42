#!/usr/bin/env python3
"""Research-only one-statement repair of the pinned archived history sampler.

Same call signature and proposal stream; historical legality replay cannot
stop at a bidder-dependent score threshold. This expects a known valid public
history; it is not a hardened request validator, behavioral posterior, or
production integration. The original engine/source is never modified.
"""
import sys
sys.dont_write_bytecode=True
import ast,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
SOURCE=ROOT/'experiments/astra-sol-20261004/phase2/incoming/drive/engine42.py'
SOURCE_SHA256='0f6a36b483ab7659ad7b23eed2c562cb06e90d8c2ecd69e17702d7f5daf52852'
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest()==SOURCE_SHA256
sys.path.insert(0,str(SOURCE.parent))
import engine42 as engine

tree=ast.parse(SOURCE.read_text())
function=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='consistent_worlds')
loops=[n for n in ast.walk(function) if isinstance(n,ast.For) and isinstance(n.iter,ast.Name) and n.iter.id=='history']
assert len(loops)==1
loop=loops[0]
assert isinstance(loop.body[-1],ast.Expr) and isinstance(loop.body[-1].value,ast.Call)
assert isinstance(loop.body[-1].value.func,ast.Attribute) and loop.body[-1].value.func.attr=='play'
loop.body.append(ast.parse('g.done[:] = False').body[0])
module=ast.fix_missing_locations(ast.Module(body=[function],type_ignores=[]))
scope=dict(vars(engine))
exec(compile(module,str(SOURCE)+' [historical-replay repair]','exec'),scope)
consistent_worlds=scope['consistent_worlds']

def accepts_fixture_world(rules,full,history):
    """The same repaired replay predicate, exposed only for exhaustive diagnostics."""
    np=engine.np;g=engine.Game(rules,full.copy(),30,0)
    g.leader[:]=history[0][0] if history else 0
    ok=np.ones(len(full),bool)
    for seat,tile in history:
        legal=g.legal(np.full(len(full),seat));ok &= ((legal>>tile)&1)==1
        g.play(np.full(len(full),tile));g.done[:]=False
    return ok
