"""Persistent, externally timed workers. One session belongs to one game.

Only process/thread-pool startup is retained. Every native request constructs
fresh evaluation state; the phone receives an explicit seed on every request.
Workers inherit the campaign watchdog group and cannot outlive this session.
"""

import json
import os
import select
import subprocess
import tempfile
import time


class Worker:
    def __init__(self, command):
        self.errors = tempfile.TemporaryFile()
        try:
            self.process = subprocess.Popen(
                [*command, "--stream"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=self.errors,
                bufsize=0,
                env={
                    **os.environ,
                    "RAYON_NUM_THREADS": os.environ.get("WALT_RAYON_THREADS", "6"),
                },
            )
        except BaseException:
            self.errors.close()
            raise

    def close(self):
        if self.process.poll() is None:
            self.process.kill()
        self.process.wait()
        self.process.stdin.close()
        self.process.stdout.close()
        self.errors.close()

    def call(self, payload, deadline):
        self.process.stdin.write(payload.encode())
        self.process.stdin.flush()
        data = bytearray()
        while True:
            left = deadline - time.monotonic()
            if left <= 0 or not select.select([self.process.stdout], [], [], left)[0]:
                raise TimeoutError()
            chunk = os.read(self.process.stdout.fileno(), 65536)
            if not chunk:
                self.errors.seek(0, os.SEEK_END)
                self.errors.seek(max(0, self.errors.tell() - 1000))
                raise RuntimeError(
                    "worker exited: " + self.errors.read().decode(errors="replace")
                )
            data.extend(chunk)
            if len(data) > 2_000_000:
                raise RuntimeError("oversized worker response")
            if b"\n" in data:
                if not data.endswith(b"\n") or data.count(b"\n") != 1:
                    raise RuntimeError("worker response framing violation")
                value = json.loads(data)
                if isinstance(value, dict) and value.get("status") == "error":
                    raise RuntimeError(str(value.get("error", "worker error")))
                return value


class DecisionSession:
    def __init__(self):
        self.workers = {}

    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.close()

    def close(self):
        for key in list(self.workers):
            self.workers.pop(key).close()

    def call(self, command, text, allowance):
        if allowance <= 0:
            return None, "no-time"
        key = tuple(command)
        deadline = time.monotonic() + allowance
        try:
            if key not in self.workers:
                self.workers[key] = Worker(command)
            # Native requests are blank-line framed; phone requests are JSONL.
            payload = text.rstrip("\n") + ("\n" if command[0] == "node" else "\n\n")
            return self.workers[key].call(payload, deadline), "completed"
        except (OSError, RuntimeError, ValueError, TimeoutError) as error:
            worker = self.workers.pop(key, None)
            if worker is not None:
                worker.close()
            return None, "timeout" if isinstance(
                error, TimeoutError
            ) else "worker-error: " + str(error)[-500:]
