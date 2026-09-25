"""Static research figure from exact saved summaries; no solver work."""
import json
from fractions import Fraction
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter

HERE = Path(__file__).resolve().parent
rows = json.loads((HERE / 'ladder-summary.json').read_text())
colors = {'21':'#7B8794', '31':'#3269A8', '44':'#D27A29',
          '51':'#00877F', '66':'#9957A1'}
labels = {tile:tile[0]+'–'+tile[1] for tile in colors}
plt.rcParams.update({'font.family':'DejaVu Sans', 'font.size':11,
                     'axes.titleweight':'bold', 'axes.labelcolor':'#253243',
                     'text.color':'#172536', 'axes.spines.top':False,
                     'axes.spines.right':False})
fig, axes = plt.subplots(1, 2, figsize=(13, 6.5), sharey=True)
for ax, ply, title in zip(axes, (6, 9),
                         ('Before Ruby led 6–6', 'Before Ruby led 4–4')):
    panel = [r for r in rows if r['ply'] == ply]
    ns = [r['worlds'] for r in panel]
    for tile in colors:
        if tile not in panel[0]['mean_set']:
            continue
        ys = [100*float(Fraction(r['mean_set'][tile])) for r in panel]
        lo = [100*float(Fraction(r['min_set'][tile])) for r in panel]
        hi = [100*float(Fraction(r['max_set'][tile])) for r in panel]
        ax.fill_between(ns, lo, hi, color=colors[tile], alpha=.085, linewidth=0)
        ax.plot(ns, ys, marker='o', markersize=4.5, color=colors[tile],
                linewidth=2.7 if tile=='51' else 1.9, label=labels[tile])
    ax.set_xscale('log')
    ax.set_xticks(ns)
    ax.xaxis.set_major_formatter(FuncFormatter(lambda x, _: f'{int(x):,}'))
    ax.minorticks_off()
    ax.set_xlabel('Sampled hidden deals per evaluation', labelpad=12)
    ax.set_title(title, loc='left', pad=15)
    ax.grid(axis='y', alpha=.17)
    ax.tick_params(axis='both', length=0, pad=7)
    ax.spines['left'].set_color('#CBD3DC')
    ax.spines['bottom'].set_color('#CBD3DC')
axes[0].set_ylabel('Estimated chance to set Nel-O (%)', labelpad=12)
fig.suptitle('How sample size changes Walt’s lead estimates',
             fontsize=20, fontweight='bold', x=.075, y=.96, ha='left')
handles, names = axes[0].get_legend_handles_labels()
fig.legend(handles, names, loc='lower center', bbox_to_anchor=(.5,.115),
           ncol=5, frameon=False, title='Candidate lead')
fig.text(.075, .055,
         'Lines: means across eight seeds. Shading: observed seed range, not a confidence interval.',
         fontsize=10, color='#516170')
fig.text(.075, .026,
         'Optimized sampled estimates; modeled players retain eight inner worlds. This is one shared hand.',
         fontsize=10, color='#516170')
fig.subplots_adjust(left=.075, right=.98, bottom=.29, top=.81, wspace=.12)
for suffix in ('png','svg'):
    fig.savefig(HERE / f'sampling-ladder.{suffix}', dpi=180, facecolor='white')
print(HERE / 'sampling-ladder.png')
