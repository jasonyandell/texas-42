#!/usr/bin/env python3
"""Time one process/thread layout on a fixed retained workload, bounded to 60s.

This is a diagnostic replay, not production. Run without a competing producer.
Compare elapsed times only when every layout completes the same workload.
Persist each completed case so timeout evidence is explicit, never extrapolated.
"""
import argparse
import asyncio
import json
import os
import time
from pathlib import Path
from kiln import atomic_json, canonical, connect, digest


async def run(args):
    db = connect(args.directory)
    rows = db.execute('''SELECT payload FROM results WHERE worlds=?
        ORDER BY (job_id*7919)%104729 LIMIT ?''', (args.worlds, args.cases)).fetchall()
    db.close()
    if len(rows) != args.cases:
        raise ValueError('Not enough retained cases at requested depth')
    queue = asyncio.Queue()
    for index, row in enumerate(rows):
        queue.put_nowait((index, json.loads(row[0])))
    binary_sha = digest(args.binary)
    completed, errors = [], []
    started = time.monotonic()

    def report():
        elapsed = time.monotonic() - started
        return {
            'schema': 'kiln-pool-timing-v1', 'binary': binary_sha,
            'workers': args.workers, 'threads': args.threads, 'worlds': args.worlds,
            'requested': args.cases, 'completed': len(completed),
            'all_completed': len(completed) == args.cases and not errors,
            'elapsed_seconds': elapsed, 'prices_per_second': len(completed) / elapsed,
            'cases': sorted(completed, key=lambda x: x['index']), 'errors': errors,
        }

    async def worker():
        process = await asyncio.create_subprocess_exec(str(args.binary),
            stdin=asyncio.subprocess.PIPE, stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.DEVNULL,
            env={**os.environ, 'RAYON_NUM_THREADS': str(args.threads)})
        try:
            while not queue.empty():
                index, old = queue.get_nowait()
                req = {'auction': old['auction'], 'decl': old['price'][0],
                       'worlds': old['worlds'], 'budget_ms': 60000}
                process.stdin.write((canonical(req)+'\n').encode())
                await process.stdin.drain()
                line = await process.stdout.readline()
                value = json.loads(line)
                if value.get('price') != old['price']:
                    raise ValueError(f'Case {index} price mismatch or failed request: {value}')
                # Concurrent cache misses can recompute work; those counts are
                # not invariant under parallel scheduling. Values must be.
                if args.threads == 1:
                    for key in ('nodes', 'pi_calls', 'inner_worlds'):
                        if value['work'][key] != old['work'][key]:
                            raise ValueError(f'Case {index} {key} mismatch')
                completed.append({'index': index, 'request': req, 'price': value['price'],
                                  'work': value['work']})
                atomic_json(args.output, report())
        except asyncio.CancelledError:
            raise
        except Exception as error:
            errors.append(str(error))
            raise
        finally:
            if process.returncode is None:
                process.kill()
            await process.wait()

    tasks = [asyncio.create_task(worker()) for _ in range(args.workers)]
    try:
        await asyncio.wait_for(asyncio.gather(*tasks), args.seconds)
    except asyncio.TimeoutError:
        pass
    finally:
        for task in tasks:
            task.cancel()
        await asyncio.gather(*tasks, return_exceptions=True)
        result = report()
        atomic_json(args.output, result)
        print(canonical({k: v for k, v in result.items() if k != 'cases'}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory')
    parser.add_argument('binary', type=Path)
    parser.add_argument('output')
    parser.add_argument('--workers', type=int, default=18)
    parser.add_argument('--threads', type=int, default=1)
    parser.add_argument('--worlds', type=int, choices=[8, 40, 160], default=160)
    parser.add_argument('--cases', type=int, default=128)
    parser.add_argument('--seconds', type=float, default=60)
    args = parser.parse_args()
    if min(args.workers, args.threads, args.cases, args.seconds) <= 0 or args.seconds > 60:
        parser.error('Positive bounds required; maximum duration is 60 seconds')
    args.binary = args.binary.resolve()
    asyncio.run(run(args))
