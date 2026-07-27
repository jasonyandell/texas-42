#!/bin/zsh
# Poll all pending submissions every 10 min. Exits (re-invoking the operator)
# when any dispatch completes, times out (>3 h), or nothing is left pending.
ROOT=/Users/jason/code/texas-42
while true; do
  pending=0
  for j in "$ROOT"/exchange/outbox/*.submitted.json(N); do
    tag=$(basename "$j" .submitted.json)
    [ -f "$ROOT/exchange/inbox/$tag.md" ] && continue
    [ -f "$ROOT/exchange/outbox/$tag.timedout" ] && continue

    age_h=$(python3 -c "
import json, datetime
m = json.load(open('$j'))
t = datetime.datetime.fromisoformat(m['submittedAt'].replace('Z','+00:00'))
print((datetime.datetime.now(datetime.timezone.utc)-t).total_seconds()/3600)")
    if (( $(echo "$age_h > 3" | bc -l) )); then
      echo "TIMEOUT: $tag (${age_h}h)"
      exit 0
    fi

    node "$ROOT/automation/poll.mjs" "$j" >> "$ROOT/automation/logs/poll.log" 2>&1
    rc=$?
    if [ $rc -eq 0 ]; then
      echo "COMPLETE: $tag"
      exit 0
    elif [ $rc -ne 2 ]; then
      echo "POLL-ERROR($rc): $tag" >> "$ROOT/automation/logs/poll.log"
    fi
    pending=1
  done
  if [ $pending -eq 0 ]; then
    echo "ALL-DONE"
    exit 0
  fi
  sleep 600
done
