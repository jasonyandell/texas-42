#!/usr/bin/python3
"""Render one exact freeze-56 outer-gate failure without a Rust binary.

This is the bootstrap path for failures that can occur before the checked
runner has been built.  It can only issue the fixed zero-accepted failure type;
it has no success encoding and never overwrites an existing artifact.
"""

from __future__ import annotations

import hashlib
import os
import stat
import struct
import sys


MANIFEST = "walt/math/gpu_native_trick1_m0_m2_sources_v1.sha256"
FREEZE56 = bytes.fromhex(
    "7bdc5e05513fd1d7e7b6c26870cf9bd4"
    "a16966c5daf48963729d999c4b6b28cf"
)
PARENT_COMMIT = bytes.fromhex("3b4c6d60fef371e3050de151ccf9eaefbc2d2da7")
ZERO_DIGEST = bytes(32)
UNAVAILABLE_U32 = (1 << 32) - 1
UNAVAILABLE_I32 = -(1 << 31)


def regular_bytes(path: str) -> bytes:
    before = os.lstat(path)
    if not stat.S_ISREG(before.st_mode) or stat.S_ISLNK(before.st_mode):
        raise ValueError("not a regular non-symlink file")
    flags = os.O_RDONLY
    if hasattr(os, "O_CLOEXEC"):
        flags |= os.O_CLOEXEC
    if hasattr(os, "O_NOFOLLOW"):
        flags |= os.O_NOFOLLOW
    descriptor = os.open(path, flags)
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (
            opened.st_dev,
            opened.st_ino,
        ) != (before.st_dev, before.st_ino):
            raise ValueError("regular file identity changed")
        chunks: list[bytes] = []
        while True:
            chunk = os.read(descriptor, 1 << 20)
            if not chunk:
                break
            chunks.append(chunk)
        after = os.fstat(descriptor)
        if (after.st_dev, after.st_ino, after.st_size) != (
            opened.st_dev,
            opened.st_ino,
            opened.st_size,
        ):
            raise ValueError("regular file changed while reading")
        data = b"".join(chunks)
        if len(data) != opened.st_size:
            raise ValueError("regular file extent changed")
        return data
    finally:
        os.close(descriptor)


def manifest_identity(repository_root: str) -> bytes:
    try:
        manifest = regular_bytes(os.path.join(repository_root, MANIFEST))
    except (OSError, ValueError):
        return ZERO_DIGEST
    return hashlib.sha256(manifest).digest()


def render_failure(build_identity: bytes, phase: int, code: int) -> bytes:
    if len(build_identity) != 32 or not 1 <= phase <= 16 or not 1 <= code <= 24:
        raise ValueError("closed failure field")
    out = bytearray(b"W42M2F01")
    out.extend(struct.pack("<HHIQ", 1, 256, 2, 256))
    out.extend(
        struct.pack(
            "<IIIIiI",
            phase,
            code,
            UNAVAILABLE_U32,
            UNAVAILABLE_U32,
            UNAVAILABLE_I32,
            UNAVAILABLE_U32,
        )
    )
    out.extend(struct.pack("<IIIIQII", 0, 0, 0, 0, 0, 1, 0))
    out.extend(build_identity)
    out.extend(FREEZE56)
    out.extend(ZERO_DIGEST)
    out.extend(PARENT_COMMIT)
    out.extend(bytes(60))
    if len(out) != 256:
        raise AssertionError("failure receipt width")
    return bytes(out)


def publish_new(path: str, payload: bytes) -> None:
    parent = os.path.dirname(path) or "."
    parent_stat = os.lstat(parent)
    if not stat.S_ISDIR(parent_stat.st_mode) or stat.S_ISLNK(parent_stat.st_mode):
        raise ValueError("output parent is not a real directory")
    try:
        os.lstat(path)
    except FileNotFoundError:
        pass
    else:
        raise FileExistsError("output already exists")

    basename = os.path.basename(path)
    if not basename or basename in (".", ".."):
        raise ValueError("noncanonical output basename")
    temporary = ""
    descriptor = -1
    for ordinal in range(100):
        candidate = os.path.join(
            parent, f".{basename}.m2failure.{os.getpid()}.{ordinal}"
        )
        try:
            descriptor = os.open(
                candidate,
                os.O_WRONLY | os.O_CREAT | os.O_EXCL,
                0o600,
            )
            temporary = candidate
            break
        except FileExistsError:
            continue
    if descriptor < 0:
        raise FileExistsError("temporary output namespace exhausted")
    try:
        view = memoryview(payload)
        while view:
            written = os.write(descriptor, view)
            if written <= 0:
                raise OSError("zero-byte failure write")
            view = view[written:]
        os.fsync(descriptor)
    except BaseException:
        os.close(descriptor)
        os.unlink(temporary)
        raise
    os.close(descriptor)
    try:
        os.link(temporary, path)
    except BaseException:
        os.unlink(temporary)
        raise
    os.unlink(temporary)
    directory = os.open(parent, os.O_RDONLY | getattr(os, "O_DIRECTORY", 0))
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


def main(arguments: list[str]) -> int:
    if len(arguments) != 5:
        raise ValueError("usage: render_m2_failure.py ROOT OUTPUT PHASE CODE")
    repository_root, output, phase_text, code_text = arguments[1:]
    phase = int(phase_text, 10)
    code = int(code_text, 10)
    payload = render_failure(manifest_identity(repository_root), phase, code)
    publish_new(output, payload)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv))
    except (OSError, ValueError) as error:
        print(f"render_m2_failure.py: {error}", file=sys.stderr)
        raise SystemExit(1) from error
