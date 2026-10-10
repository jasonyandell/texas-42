"""Replay retained H2H receipts; check baseline fidelity and score censoring."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
from h2h import audit, summarize, ROOT, BASELINE


def verify(paths):
    binary=hashlib.sha256(BASELINE.read_bytes()).hexdigest()
    candidate=hashlib.sha256((ROOT/'turbo.dylib').read_bytes()).hexdigest()
    counts=Counter();routes=Counter();reviews=Counter();panels={}
    for path in paths:
        records=[json.loads(line) for line in path.read_text().splitlines()]
        if records[-1]['event']!='result':raise AssertionError('unfinished receipt '+str(path))
        job=records[0]['job']
        assert job['baseline_sha256']==binary and job['candidate_sha256']==candidate
        row=audit(path);panels.setdefault(str(path.parent),[]).append(row)
        counts[row['status']]+=1;counts['moves']+=row['moves']
        for move in records[1:-1]:
            if move['engine']!='walt':continue
            call=move['call'];response=move['response'];routes[response['route']]+=1
            if response['route']!='forced':
                assert response['evaluation']['outer_worlds']==call['worlds'],path
                assert response['route'] in ('baseline','baseline-reviewed'),path
                assert 'interruption' not in response,path
                counts['full_requested_walt_evaluations']+=1
            review=response.get('review_result')
            if review:reviews[review['status']]+=1
    return {'status':'pass','receipt_files':len(paths),'counts':dict(counts),
            'baseline_routes':dict(routes),'partner_review_statuses':dict(reviews),
            'panels':{name:{k:v for k,v in summarize(rows).items() if k!='pairs'} for name,rows in sorted(panels.items())}}


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--root',default='h2h')
    parser.add_argument('--output',default='h2h/verification.json');args=parser.parse_args()
    result=verify(sorted(Path(args.root).glob('*/*.jsonl')))
    Path(args.output).write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({k:v for k,v in result.items() if k!='panels'},indent=2))
