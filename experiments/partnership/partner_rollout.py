"""A bounded, fallible continuation check in either count-offer direction.

Native values describe complete paired worlds only. A sample-prefix decision
is a heuristic guess; it carries no confidence guarantee. Failure keeps L1.
"""
from pathlib import Path

from partner_review import offers

BINARY = Path(__file__).resolve().parents[2] / 'walt/target/release/partner_rollout'
ID = 'partner-rollout-v1'
FIELD = 'completed-deployed-l1-fixed-40-8-voidless-request-seed-v1'
MAX_SECONDS = .500
MAX_SUPPORT = 400
MAX_SAMPLES = 64
MIN_SAMPLES = 8


def validate_configuration(n, n0, inner_belief, selection):
    if (n,n0,inner_belief,selection) != (40,8,'voidless','fixed'):
        raise ValueError('partner-rollout currently models default fixed L1 40/8 with voidless inner belief')


def investigate(req, state, baseline, allowance, send):
    targets = offers(req, state)
    result = dict(schema=ID, status='inactive', baseline=baseline, choice=baseline, offers=targets)
    if not targets or len(targets) == len(state['legal']):
        return baseline, result
    allowance = min(MAX_SECONDS, allowance)
    if allowance < .050:
        return baseline, {**result, 'status': 'unresolved-no-time'}
    payload = '\n'.join(k+' '+' '.join(map(str,v if isinstance(v,list) else [v])) for k,v in req.items())+'\n'
    value, status = send([str(BINARY), '--baseline', str(baseline), '--milliseconds', str(int(allowance*1000)-35)],
                         payload, allowance)
    if value is None:
        return baseline, {**result, 'status': 'unresolved', 'reason': status}
    try:
        if (value.get('schema') != ID or value.get('field') != FIELD or value.get('baseline') != baseline
                or value.get('offers') != targets or value.get('legal') != state['legal']):
            raise ValueError('identity or detector mismatch')
        support, samples, requested = (value.get(k) for k in ('support','samples','requested'))
        if any(type(n) is not int for n in (support,samples,requested)) or not 0 <= samples <= requested <= support:
            raise ValueError('invalid coverage')
        if value.get('stop') == 'support-cap':
            if support <= MAX_SUPPORT or samples or value.get('status') != 'unresolved' or value.get('choice') != baseline:
                raise ValueError('invalid support refusal')
            return baseline, value
        if not 1 <= support <= MAX_SUPPORT or requested != min(MAX_SAMPLES,support):
            raise ValueError('invalid investigation scope')
        census = samples == support
        if value.get('coverage') != ('census' if census else 'budgeted-sample-prefix'):
            raise ValueError('incorrect coverage claim')
        pairs = value.get('values')
        if (not isinstance(pairs,list) or len(pairs) != len(state['legal'])
                or any(not isinstance(p,list) or len(p)!=2 or any(type(n) is not int for n in p) for p in pairs)):
            raise ValueError('malformed action values')
        values = dict(pairs)
        if [p[0] for p in pairs] != state['legal'] or any(not 0 <= n <= samples for n in values.values()):
            raise ValueError('incomplete action vector')
        paired = value.get('paired')
        if not isinstance(paired,list) or len(paired) != len(pairs):
            raise ValueError('missing paired outcomes')
        for (tile,mass), row in zip(pairs,paired):
            if (not isinstance(row,list) or len(row)!=4 or any(type(n) is not int or n<0 for n in row)
                    or sum(row)!=samples or row[0]+row[2]!=mass or row[1]+row[2]!=values[baseline]):
                raise ValueError('paired outcome disagreement')
        best = baseline
        resolved = samples >= min(MIN_SAMPLES,support)
        if resolved:
            for tile in sorted(values):
                if values[tile] > values[best]:best=tile
        expected = ('changed' if best != baseline else 'retained') if resolved else 'unresolved'
        if value.get('choice') != best or value.get('status') != expected:
            raise ValueError('choice disagrees with completed worlds')
        return best, value
    except (AttributeError,TypeError,ValueError) as error:
        return baseline, {**result,'status':'unresolved-rejected','reason':str(error)}
