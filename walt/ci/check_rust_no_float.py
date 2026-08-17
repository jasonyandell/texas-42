#!/usr/bin/env python3
"""Reject inferred Rust float literals after stripping comments and literals."""

from __future__ import annotations

import pathlib
import re
import sys


FLOAT_TOKEN = re.compile(
    r"(?<!\w)(?:"
    r"[0-9](?:_?[0-9])*\.[0-9](?:_?[0-9])*(?:[eE][+-]?[0-9](?:_?[0-9])*)?"
    r"|[0-9](?:_?[0-9])*\.(?![.\w])"
    r"|[0-9](?:_?[0-9])*[eE][+-]?[0-9](?:_?[0-9])*"
    r")"
)


class LexError(Exception):
    pass


def float_tokens(source: str):
    for match in FLOAT_TOKEN.finditer(source):
        start = match.start()
        # `tuple.0` is an integer tuple-field projection.  A number after a
        # second range dot (`..1.5` or `foo..1.5`) remains a numeric literal.
        if (
            start > 0
            and source[start - 1] == "."
            and not (start > 1 and source[start - 2] == ".")
        ):
            continue
        yield match


def raw_string_start(source: str, at: int) -> tuple[int, str] | None:
    for prefix in ("br", "rb", "r"):
        if not source.startswith(prefix, at):
            continue
        cursor = at + len(prefix)
        hashes = 0
        while cursor < len(source) and source[cursor] == "#":
            hashes += 1
            cursor += 1
        if cursor < len(source) and source[cursor] == '"':
            return cursor + 1, '"' + "#" * hashes
    return None


def char_literal_end(source: str, at: int) -> int | None:
    """Return the end of one exact Rust character literal, never a lifetime.

    Being conservative is intentional: unrecognized syntax remains visible to
    the float scanner and will subsequently be rejected by rustc.  In
    particular, this must not scan from a lifetime such as `'a` to an unrelated
    apostrophe later on the line.
    """

    cursor = at + 1
    if cursor >= len(source) or source[cursor] in ("\n", "\r", "'"):
        return None
    if source[cursor] != "\\":
        cursor += 1
        return cursor + 1 if cursor < len(source) and source[cursor] == "'" else None

    cursor += 1
    if cursor >= len(source):
        return None
    escape = source[cursor]
    if escape in "\\'\"nrt0":
        cursor += 1
    elif escape == "x":
        digits = source[cursor + 1 : cursor + 3]
        if len(digits) != 2 or any(char not in "0123456789abcdefABCDEF" for char in digits):
            return None
        cursor += 3
    elif escape == "u" and source.startswith("u{", cursor):
        close = source.find("}", cursor + 2)
        if close < 0:
            return None
        digits = source[cursor + 2 : close]
        compact = digits.replace("_", "")
        if (
            not compact
            or len(compact) > 6
            or any(char not in "0123456789abcdefABCDEF" for char in compact)
        ):
            return None
        cursor = close + 1
    else:
        return None
    return cursor + 1 if cursor < len(source) and source[cursor] == "'" else None


def stripped_code(source: str, path: pathlib.Path) -> str:
    output = list(source)
    at = 0
    block_depth = 0
    while at < len(source):
        if block_depth:
            if source.startswith("/*", at):
                output[at : at + 2] = "  "
                block_depth += 1
                at += 2
            elif source.startswith("*/", at):
                output[at : at + 2] = "  "
                block_depth -= 1
                at += 2
            else:
                if source[at] != "\n":
                    output[at] = " "
                at += 1
            continue

        if source.startswith("//", at):
            end = source.find("\n", at)
            if end < 0:
                end = len(source)
            output[at:end] = " " * (end - at)
            at = end
            continue
        if source.startswith("/*", at):
            output[at : at + 2] = "  "
            block_depth = 1
            at += 2
            continue

        raw = raw_string_start(source, at)
        if raw is not None:
            content_at, terminator = raw
            end = source.find(terminator, content_at)
            if end < 0:
                raise LexError(f"{path}: unterminated raw string")
            finish = end + len(terminator)
            for index in range(at, finish):
                if source[index] != "\n":
                    output[index] = " "
            at = finish
            continue

        if source[at] == '"':
            cursor = at + 1
            escaped = False
            while cursor < len(source):
                char = source[cursor]
                if escaped:
                    escaped = False
                elif char == "\\":
                    escaped = True
                elif char == '"':
                    cursor += 1
                    break
                cursor += 1
            else:
                raise LexError(f"{path}: unterminated string")
            for index in range(at, cursor):
                if source[index] != "\n":
                    output[index] = " "
            at = cursor
            continue

        if source[at] == "'":
            end = char_literal_end(source, at)
            if end is not None:
                for index in range(at, end):
                    output[index] = " "
                at = end
                continue

        at += 1

    if block_depth:
        raise LexError(f"{path}: unterminated block comment")
    return "".join(output)


def rust_files(arguments: list[str]) -> list[pathlib.Path]:
    files: set[pathlib.Path] = set()
    for argument in arguments:
        path = pathlib.Path(argument)
        if path.is_dir():
            files.update(
                candidate
                for candidate in path.rglob("*.rs")
                if candidate.relative_to(path).parts[:1] != ("target",)
            )
        elif path.suffix == ".rs":
            files.add(path)
        else:
            raise LexError(f"not a Rust file or directory: {path}")
    return sorted(files, key=lambda path: path.as_posix().encode("utf-8"))


def main() -> int:
    try:
        lifetime_regression = "fn f<'a>() { let x = 1.5; let _: &'a str = \"\"; }\n"
        stripped_regression = stripped_code(
            lifetime_regression, pathlib.Path("<lifetime-regression>")
        )
        if next(float_tokens(stripped_regression), None) is None:
            raise LexError("lifetime tokens hid an inferred float")
        range_regression = "fn f() { let x = std::hint::black_box(..1.5); drop(x); }\n"
        if next(
            float_tokens(
                stripped_code(range_regression, pathlib.Path("<range-regression>"))
            ),
            None,
        ) is None:
            raise LexError("range punctuation hid an inferred float")
        files = rust_files(sys.argv[1:])
        if not files:
            raise LexError("no Rust files selected")
        failed = False
        for path in files:
            source = path.read_text(encoding="utf-8")
            code = stripped_code(source, path)
            for match in float_tokens(code):
                line = code.count("\n", 0, match.start()) + 1
                print(f"{path}:{line}: inferred float token {match.group(0)!r}", file=sys.stderr)
                failed = True
        return 1 if failed else 0
    except (OSError, UnicodeError, LexError) as error:
        print(f"check_rust_no_float.py: ERROR: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
