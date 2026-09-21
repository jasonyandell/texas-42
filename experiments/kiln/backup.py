#!/usr/bin/env python3
"""Consistent Kiln snapshots outside git, optionally mirrored to a private HF dataset.

SQLite's online backup API pins the database snapshot while production continues.
Every archive member is read back and hashed before the snapshot is published.
The original live campaign is never modified or moved.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import shutil
import sqlite3
import subprocess
import sys
import tarfile
import tempfile

from kiln import ROOT, atomic_json, canonical

DATA = Path.home()/'data/texas-42'
REPO = 'jasonyandell/texas-42-walt-archive'
CAMPAIGNS = {'survey':'kiln-v1','played':'kiln-played-v1'}


def sha256(path):
    with Path(path).open('rb') as handle:
        return hash_stream(handle)


def hash_stream(handle):
    value = hashlib.sha256()
    for block in iter(lambda:handle.read(1024*1024),b''):
        value.update(block)
    return value.hexdigest()


def sync_file(path):
    with Path(path).open('rb') as handle:
        os.fsync(handle.fileno())


def database_snapshot(source,target):
    src = sqlite3.connect(source.resolve().as_uri()+'?mode=ro',uri=True,timeout=30)
    dst = sqlite3.connect(target)
    try:
        src.execute('BEGIN')
        tables = {r[0] for r in src.execute("SELECT name FROM sqlite_master WHERE type='table'")}
        counts = {name:src.execute('SELECT COUNT(*) FROM '+name).fetchone()[0]
                  for name in ('games','results','jobs','hands','deals','cells') if name in tables}
        src.backup(dst,pages=1024,sleep=.01)
        dst.execute('PRAGMA journal_mode=DELETE')
        if dst.execute('PRAGMA integrity_check').fetchall() != [('ok',)]:
            raise ValueError('SQLite integrity check failed: '+str(source))
        for name,n in counts.items():
            if dst.execute('SELECT COUNT(*) FROM '+name).fetchone()[0] != n:
                raise ValueError('Snapshot count mismatch: '+str(source))
        return {'counts':counts,'integrity_check':'ok'}
    finally:
        dst.close();src.close()


def archive_verified(folder,archive):
    paths = sorted(p for p in folder.rglob('*') if p.is_file())
    expected = {str(p.relative_to(folder.parent)):sha256(p) for p in paths}
    with tarfile.open(archive,'w:gz',compresslevel=1) as out:
        for path in paths:
            out.add(path,arcname=str(path.relative_to(folder.parent)),recursive=False)
    seen = set()
    with tarfile.open(archive,'r|gz') as source:
        for member in source:
            if not member.isfile() or member.name not in expected or member.name in seen:
                raise ValueError('Unexpected archive member')
            if hash_stream(source.extractfile(member)) != expected[member.name]:
                raise ValueError('Archive verification failed: '+member.name)
            seen.add(member.name)
    if seen != set(expected):raise ValueError('Archive is missing files')
    sync_file(archive)
    return {'file':archive.name,'bytes':archive.stat().st_size,'sha256':sha256(archive),
            'members':len(seen),'verified_members':True}


def snapshot(data,backup_root,campaigns):
    backup_root.mkdir(parents=True,exist_ok=True)
    stamp = datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S.%fZ')
    pending = Path(tempfile.mkdtemp(prefix=stamp+'.partial-',dir=backup_root))
    report = {'schema':'kiln-backup-v1','created_utc':stamp,
              'source_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
              'campaigns':{},'archives':[]}
    for label in campaigns:
        name = CAMPAIGNS[label]
        source = data/name
        if not source.is_dir():raise ValueError('Missing campaign: '+str(source))
        target = pending/name
        target.mkdir()
        databases = {}
        # Pin all database snapshots before copying their immutable producers.
        for path in sorted(source.rglob('*.sqlite')):
            if path.is_symlink():raise ValueError('Refusing a symlinked database')
            relative = path.relative_to(source)
            destination = target/relative
            destination.parent.mkdir(parents=True,exist_ok=True)
            databases[str(relative)] = database_snapshot(path,destination)
        omitted = []
        for path in sorted(source.rglob('*')):
            if path.is_symlink():raise ValueError('Refusing a symlink in campaign: '+str(path))
            if not path.is_file():continue
            relative = path.relative_to(source)
            if path.suffix == '.sqlite':continue
            if path.name.endswith(('-wal','-shm','.lock')) or any('.pending-' in part for part in relative.parts):
                omitted.append(str(relative));continue
            destination = target/relative
            destination.parent.mkdir(parents=True,exist_ok=True)
            shutil.copy2(path,destination)
        details = {'source_directory':str(source.resolve()),'databases':databases,
                   'omitted_ephemeral_files':omitted,
                   'note':'Database and immutable producers are authoritative. Copied logs/status/previews are ancillary views and may have a different timestamp.'}
        atomic_json(target/'SNAPSHOT.json',details)
        report['campaigns'][name] = details
        for path in target.rglob('*'):
            if path.is_file():sync_file(path)
        archived = archive_verified(target,pending/(name+'.tar.gz'))
        report['archives'].append(archived)
        print(canonical({'campaign':name,'databases':databases,'archive':archived}),flush=True)
    # Runner tools and guides also survive deletion of the worktree. Exact player
    # binaries/core source are already pinned in each campaign's producer bundles.
    tools = pending/'runner-tools'
    for path in sorted((ROOT/'experiments/kiln').iterdir()):
        if path.is_file() and path.suffix in ('.py','.md'):
            dest = tools/'experiments/kiln'/path.name
            dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(path,dest)
    dest = tools/'experiments/partnership/rules.py'
    dest.parent.mkdir(parents=True,exist_ok=True)
    shutil.copy2(ROOT/'experiments/partnership/rules.py',dest)
    report['archives'].append(archive_verified(tools,pending/'runner-tools.tar.gz'))
    restore = pending/'README.md'
    restore.write_text('''# Kiln data snapshot

Private research backup. Actual-play outcomes and the older model survey are
separate campaigns. The manifest records exact snapshot counts and archive hashes.

Download this snapshot directory from the private Hugging Face dataset, then
check each archive against MANIFEST.json before extracting it into a fresh local
directory. The archives contain ordinary SQLite databases, original receipts,
campaign manifests, immutable native binaries and their source bundles. All WAL
transactions committed at snapshot time are incorporated into the database;
the old WAL, shared-memory and lock files are deliberately omitted.

For an actual-play restore, extract runner-tools.tar.gz alongside the campaign.
Run `python3 runner-tools/experiments/kiln/played.py audit kiln-played-v1` first.
To resume from a normal Texas42 repository checkout on a compatible Mac, use
the recorded source commit and pass `--binary` pointing to a preserved
`kiln-played-v1/producers/<producer>/kiln-play-worker`, with `--workers 18
--games 160 --seconds 0`. Pending leases are recovered automatically. This creates
a new run; do not restore over a live campaign. Old PID/status/log files are
historical metadata, not evidence that a process is running on this computer.
The older kiln-v1 model survey is frozen and should not be restarted automatically.

The empirical results measure final points from the deployed player targeting30,
not a perfect-information oracle or a policy optimized for each other bid level.
''')
    sync_file(restore)
    atomic_json(pending/'MANIFEST.json',report)
    final = backup_root/stamp
    os.rename(pending,final)
    fd = os.open(backup_root,os.O_RDONLY)
    try:os.fsync(fd)
    finally:os.close(fd)
    return final


def upload(folder,repo):
    os.environ.setdefault('HF_HUB_DISABLE_PROGRESS_BARS','1')
    try:
        from huggingface_hub import HfApi, hf_hub_download
    except ModuleNotFoundError as error:
        if error.name != 'huggingface_hub':raise
        # This checkout may select a different Python from the authenticated HF
        # CLI. Reuse that installed CLI's interpreter rather than modifying either
        # environment or asking the user to log in again.
        cli = shutil.which('hf')
        interpreter = Path(cli).read_text().splitlines()[0][2:].strip() if cli else ''
        if not interpreter or not Path(interpreter).is_absolute() or not Path(interpreter).is_file() or Path(interpreter).resolve() == Path(sys.executable).resolve():
            raise RuntimeError('Local backup is complete; upload needs a Python with huggingface_hub installed') from None
        subprocess.run([interpreter,str(Path(__file__).resolve()),'--existing',str(folder),'--repo',repo,'--upload'],check=True)
        return
    api = HfApi()
    info = api.repo_info(repo,repo_type='dataset')
    if not info.private:raise ValueError('Backup destination must remain private')
    prefix = 'kiln/snapshots/'+folder.name
    report = json.loads((folder/'MANIFEST.json').read_text())
    result = api.upload_folder(repo_id=repo,repo_type='dataset',folder_path=folder,
        path_in_repo=prefix,allow_patterns=[a['file'] for a in report['archives']]+['MANIFEST.json','README.md'],
        commit_message='Back up Kiln survey and actual-play evidence '+folder.name)
    # Verify against the exact immutable remote commit, not a moving branch.
    verified = []
    for archive in report['archives']:
        downloaded = hf_hub_download(repo,filename=prefix+'/'+archive['file'],repo_type='dataset',revision=result.oid)
        if sha256(downloaded) != archive['sha256']:
            raise ValueError('Remote readback checksum mismatch: '+archive['file'])
        verified.append(archive['file'])
    for name in ('MANIFEST.json','README.md'):
        downloaded = hf_hub_download(repo,filename=prefix+'/'+name,repo_type='dataset',revision=result.oid)
        if Path(downloaded).read_bytes() != (folder/name).read_bytes():
            raise ValueError('Remote metadata mismatch: '+name)
        verified.append(name)
    remote = {'schema':'kiln-remote-backup-v1','repository':repo,'private':True,'commit':result.oid,
              'path':prefix,'verified_downloads':verified,
              'url':f'https://huggingface.co/datasets/{repo}/tree/{result.oid}/{prefix}'}
    atomic_json(folder/'REMOTE.json',remote)
    print(canonical(remote),flush=True)


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--data',type=Path,default=DATA)
    parser.add_argument('--backup-root',type=Path,default=DATA/'backups')
    parser.add_argument('--campaign',choices=list(CAMPAIGNS),action='append')
    parser.add_argument('--upload',action='store_true')
    parser.add_argument('--repo',default=REPO)
    parser.add_argument('--existing',type=Path,help='Retry upload of an already verified snapshot')
    args=parser.parse_args()
    folder=args.existing or snapshot(args.data,args.backup_root,args.campaign or list(CAMPAIGNS))
    print(canonical({'local_snapshot':str(folder)}),flush=True)
    if args.upload:upload(folder,args.repo)
