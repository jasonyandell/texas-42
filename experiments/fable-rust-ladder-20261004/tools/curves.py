import json, sys
M = '/Users/jason/Documents/Codex/2026-10-04/task-15/fable-worktree/experiments/fable-rust-ladder-20261004/models/'
for n in sys.argv[1:]:
    m = json.load(open(M + n + '.json')); print('##', n, 'params', m['params'], 'rows', m['train_rows'], 'steps', m['global_step'] if 'global_step' in m else m.get('selected_update'))
    for h in m['history']:
        print('| %d | %.4f | %.4f | %.4f | %.4f | %.4f | %.4f |' % (h['update'], h['train_agree'], h['agree'], h['train_regret'], h['regret'], h['train_bce'], h['val_bce']))
