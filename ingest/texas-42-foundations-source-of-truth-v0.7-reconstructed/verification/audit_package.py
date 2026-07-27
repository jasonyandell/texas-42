#!/usr/bin/env python3
"""Deterministic structural audit for the Texas 42 Foundations package."""

from __future__ import annotations

import hashlib
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED = (
    "README.md",
    "docs/00_THESIS_AND_SCOPE.md",
    "docs/10_RULES.md",
    "docs/20_MATHEMATICAL_FOUNDATION.md",
    "docs/30_EXECUTABLE_SPECIFICATION.md",
    "docs/40_CLAIM_STATUS.md",
    "docs/50_CODEX_IMPLEMENTATION_PROMPT.md",
    "docs/60_PROOF_ASSISTANT_KERNEL.md",
    "verification/verify_foundation.py",
    "verification/verify_minimality_and_reachability.py",
    "verification/verify_reduced_kernel.py",
    "verification/audit_package.py",
)

FORBIDDEN_DOC_TERMS = (
    "Atlas",
    "Walt",
    "Hoyt",
    "Forge",
    "mk5-main",
    "CoordinateV1",
)

CLAIM_ROW = re.compile(r"^\| ([A-Z][A-Z0-9]*-[A-Z0-9-]+) \|")
LINK = re.compile(r"\[[^\]]+\]\(([^)]+)\)")
CONTROL = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f\x7f]")


def fail(message: str) -> None:
    raise AssertionError(message)


def text_files() -> list[Path]:
    return sorted(
        path
        for path in ROOT.rglob("*")
        if path.is_file()
        and path.name != "MANIFEST.sha256"
        and path.suffix in {".md", ".py", ".txt"}
    )


def check_required() -> None:
    for relative in REQUIRED:
        if not (ROOT / relative).is_file():
            fail(f"missing required file: {relative}")


def check_no_transients() -> None:
    transients = sorted(
        path.relative_to(ROOT).as_posix()
        for path in ROOT.rglob("*")
        if path.name == "__pycache__" or path.suffix == ".pyc"
    )
    if transients:
        fail(f"transient Python files present: {transients}")


def check_utf8_controls_and_fences() -> tuple[int, int]:
    files = text_files()
    total_lines = 0
    for path in files:
        raw = path.read_bytes()
        try:
            text = raw.decode("utf-8")
        except UnicodeDecodeError as exc:
            fail(f"non-UTF-8 file: {path.relative_to(ROOT)}: {exc}")
        if CONTROL.search(text):
            fail(f"control character in {path.relative_to(ROOT)}")
        if path.suffix == ".md" and "\t" in text:
            fail(f"tab character in Markdown file: {path.relative_to(ROOT)}")
        total_lines += len(text.splitlines())
        if path.suffix == ".md" and text.count("```") % 2:
            fail(f"unbalanced fenced code block: {path.relative_to(ROOT)}")
    return len(files), total_lines


def check_local_links() -> int:
    checked = 0
    for path in sorted(ROOT.rglob("*.md")):
        text = path.read_text(encoding="utf-8")
        for target in LINK.findall(text):
            if target.startswith(("http://", "https://", "mailto:")):
                continue
            target = target.split("#", 1)[0]
            if not target:
                continue
            resolved = (path.parent / target).resolve()
            try:
                resolved.relative_to(ROOT.resolve())
            except ValueError:
                fail(f"link escapes package: {path.relative_to(ROOT)} -> {target}")
            if not resolved.exists():
                fail(f"broken local link: {path.relative_to(ROOT)} -> {target}")
            checked += 1
    return checked


def check_claim_ledger() -> int:
    path = ROOT / "docs/40_CLAIM_STATUS.md"
    claims: list[str] = []
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        match = CLAIM_ROW.match(line)
        if not match:
            continue
        claim_id = match.group(1)
        if claim_id == "ID":
            continue
        columns = line[2:-2].split(" | ") if line.endswith(" |") else []
        if len(columns) not in (3, 6):
            fail(f"malformed claim row at line {line_number}: {line}")
        claims.append(claim_id)
    duplicates = sorted({claim for claim in claims if claims.count(claim) > 1})
    if duplicates:
        fail(f"duplicate claim IDs: {duplicates}")
    if len(claims) < 250:
        fail(f"unexpectedly small claim ledger: {len(claims)} rows")
    return len(claims)


def check_no_stale_or_inherited_terms() -> int:
    checked = 0
    for path in sorted([ROOT / "README.md", *ROOT.joinpath("docs").glob("*.md")]):
        text = path.read_text(encoding="utf-8")
        if "v0.6" in text:
            fail(f"stale version reference in {path.relative_to(ROOT)}")
        for term in FORBIDDEN_DOC_TERMS:
            if term in text:
                fail(f"inherited project term {term!r} in {path.relative_to(ROOT)}")
        checked += 1
    return checked


def check_cross_document_markers() -> int:
    required_markers = {
        "docs/20_MATHEMATICAL_FOUNDATION.md": (
            "Exact support normal form as a transition state",
            "Holder-edge monotonicity and the finite deletion budget",
            "Folded physical play residue and the reduced viewer kernel",
            "Three unscored declaration-mechanics classes",
            "Oriented-frame dihedral gauge",
            "Mechanical future equivalence and global transition minimality",
        ),
        "docs/30_EXECUTABLE_SPECIFICATION.md": (
            "Minimal exact support transition state",
            "Folded trick and reduced viewer play/support kernel",
            "SymbolicSupportTraceWitness",
            "FutureOutputContract",
        ),
        "docs/40_CLAIM_STATUS.md": (
            "TRANS-08",
            "PLAY-17",
            "QUO-10",
            "FAC-02",
        ),
        "docs/60_PROOF_ASSISTANT_KERNEL.md": (
            "K10. Dynamic support",
            "K12. Folded play/support kernel",
            "K14. Future-equivalence minimum",
        ),
    }
    count = 0
    for relative, markers in required_markers.items():
        text = (ROOT / relative).read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                fail(f"missing cross-document marker {marker!r} in {relative}")
            count += 1
    return count


def check_manifest_if_present() -> int:
    manifest = ROOT / "MANIFEST.sha256"
    if not manifest.exists() or manifest.stat().st_size == 0:
        return 0
    listed: dict[str, str] = {}
    for line_number, line in enumerate(manifest.read_text(encoding="utf-8").splitlines(), 1):
        if not line:
            continue
        try:
            digest, relative = line.split("  ", 1)
        except ValueError:
            fail(f"malformed manifest line {line_number}")
        if relative in listed:
            fail(f"duplicate manifest path: {relative}")
        path = ROOT / relative
        if not path.is_file():
            fail(f"manifest path missing: {relative}")
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != digest:
            fail(f"manifest digest mismatch: {relative}")
        listed[relative] = digest

    actual_files = {
        path.relative_to(ROOT).as_posix()
        for path in ROOT.rglob("*")
        if path.is_file() and path != manifest
    }
    if set(listed) != actual_files:
        missing = sorted(actual_files - set(listed))
        extra = sorted(set(listed) - actual_files)
        fail(f"manifest coverage mismatch: missing={missing}, extra={extra}")
    return len(listed)


def main() -> None:
    check_required()
    check_no_transients()
    file_count, line_count = check_utf8_controls_and_fences()
    local_links = check_local_links()
    claims = check_claim_ledger()
    docs_checked = check_no_stale_or_inherited_terms()
    markers = check_cross_document_markers()
    check_manifest_if_present()

    print("Texas 42 Foundations structural audit: PASS")
    print(f"UTF-8 text files checked: {file_count}; lines: {line_count:,}")
    print(f"local Markdown links checked: {local_links}")
    print(f"unique substantive claim IDs: {claims}")
    print(f"normative/project-neutral documents checked: {docs_checked}")
    print(f"cross-document kernel markers checked: {markers}")


if __name__ == "__main__":
    main()
