#!/bin/zsh
# Collect a fresh snapshot; the orchestrating session pushes the newest results/monitor/snap-*.json to the artifact db.
cd "$(dirname "$0")" && /Users/jason/.local/share/mise/installs/python/3.12/bin/python3 monitor_collect.py
