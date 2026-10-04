# ---- Flat Walt: one row per world, one sheet per ordering of seat 2's tiles, dice on a tape ----
import itertools
tape_json = json.load(open('tape.json'))   # "tree|w|seat|trick" -> u
SEATS = [0, 1, 3]; TRICKS = [5, 6, 7]; NW = 8
WROW = lambda w: 6 + w            # world w (0-based) lives on row 6+w on Tape / Line / Flat sheets
OUTLINE_LVL = 1

# ================================================================ Tape
tp = wb.create_sheet('Tape')
put(tp, 'A1', 'The tape: one die per (seat, trick) per world', H1)
put(tp, 'A2', 'A die is a number u in [0,1). It is resolved only when it is needed: the seat\'s legal tiles at that moment are listed, and u picks the 1+INT(u×n)-th. '
             'So the same u means "the k-th legal tile", whatever legal turns out to be. Pinned values reproduce the trace\'s rolls; set B3 to "live" and press F9 to roll fresh dice.', GREY, al=WRAP)
tp.merge_cells('A2:T2'); tp.row_dimensions[2].height = 44
put(tp, 'A3', 'Dice mode', BOLD); put(tp, 'B3', 'pinned', BLUE, fill=INPUT_FILL); put(tp, 'C3', '← "pinned" or "live"', GREY)
# columns: A world | pinned 6-3 line (9) | pinned 6-0 line (9) | effective 6-3 (9) | effective 6-0 (9)
hdr = ['World']
groups = [('pinned, 6-3 line', 'P63'), ('pinned, 6-0 line', 'P60'), ('in use, 6-3 line', 'U63'), ('in use, 6-0 line', 'U60')]
TAPE_COL = {}  # (grp, seat, trick) -> column letter
col = 2
for gname, gkey in groups:
    tp.cell(row=4, column=col, value=gname).font = BOLD
    tp.merge_cells(start_row=4, start_column=col, end_row=4, end_column=col + 8)
    for seat in SEATS:
        for t in TRICKS:
            TAPE_COL[(gkey, seat, t)] = get_column_letter(col)
            hdr.append(f'S{seat} t{t}')
            col += 1
header(tp, 5, hdr)
for w in range(NW):
    r = WROW(w)
    put(tp, f'A{r}', f'W{w+1}', BOLD)
    for tree, pk, uk in (('6-3', 'P63', 'U63'), ('6-0', 'P60', 'U60')):
        for seat in SEATS:
            for t in TRICKS:
                pc = TAPE_COL[(pk, seat, t)]; uc = TAPE_COL[(uk, seat, t)]
                if seat == 1 and t == 5:
                    put(tp, f'{pc}{r}', '—', GREY); put(tp, f'{uc}{r}', '—', GREY)   # walt already led the 6-4
                    continue
                key = f'{tree}|{w}|{seat}|{t}'
                v = tape_json.get(key)
                put(tp, f'{pc}{r}', v if v is not None else 0.5, BLUE, nf='0.00')
                if v is None: tp[f'{pc}{r}'].font = font(color='0000FF', italic=True)
                put(tp, f'{uc}{r}', f'=IF($B$3="live",RAND(),{pc}{r})', nf='0.00')
put(tp, f'A{WROW(NW)+1}', 'Blue = pinned from the trace (italic blue = a roll the trace never made because that branch was decided early; 0.5 is a placeholder). '
                           'The trace rolled fresh dice for the 6-0 line, so there is one tape per line. Sharing one tape across both lines would be the better design (a paired comparison); it is what "live" mode does not yet do.', GREY, al=WRAP)
tp.merge_cells(f'A{WROW(NW)+1}:T{WROW(NW)+1}'); tp.row_dimensions[WROW(NW)+1].height = 44
widths(tp, [8] + [7] * 36)
tp.freeze_panes = 'B6'

# ================================================================ helpers for formula text
def hi(x): return f'VALUE(LEFT({x},1))'
def lo(x): return f'VALUE(RIGHT({x},1))'
def istrump(x): return f'OR(LEFT({x},1)="5",RIGHT({x},1)="5")'
def ledsuit(L): return f'IF({L}="","",IF({istrump(L)},"t",LEFT({L},1)))'
def follows(x, q):   # q is a cell holding "", "t" or a pip digit as text
    return f'IF({q}="",1,IF({q}="t",IF({istrump(x)},1,0),IF({istrump(x)},0,IF(OR(LEFT({x},1)={q},RIGHT({x},1)={q}),1,0))))'
def strength(x, q):  # q as above; "" never happens for a played trick
    return (f'IF({istrump(x)},100+IF(LEFT({x},1)=RIGHT({x},1),7,{hi(x)}+{lo(x)}-5),'
            f'IF({q}="t",0,IF(OR(LEFT({x},1)={q},RIGHT({x},1)={q}),IF(LEFT({x},1)=RIGHT({x},1),7,{hi(x)}+{lo(x)}-VALUE({q})),0)))')
def countpts(x): return f'IF({hi(x)}+{lo(x)}=5,5,IF({hi(x)}+{lo(x)}=10,10,0))'

# ================================================================ Line sheets
ORDERINGS = list(itertools.permutations(['6-3', '6-0', '5-5']))
LINE_SHEETS = []
SEAT_TILES = {0: 3, 1: 2, 3: 3}
WORLD_COL = {0: ['B', 'C', 'D'], 1: ['E', 'F'], 3: ['G', 'H', 'I']}   # on the Worlds sheet

def build_line(idx, ordering):
    name = f'Line {idx+1}'
    ws = wb.create_sheet(name); LINE_SHEETS.append(name)
    cols = {}; C = [2]            # column allocator
    def new(key, width=9, group=False):
        L = get_column_letter(C[0]); cols[key] = L; C[0] += 1
        ws.column_dimensions[L].width = width
        if group: ws.column_dimensions[L].outlineLevel = OUTLINE_LVL
        return L
    def ref(key, r, abs_=False): return f'${cols[key]}${r}' if abs_ else f'{cols[key]}{r}'
    # ---- header: the ordering and the line's root tile
    put(ws, 'A1', f'{name}: seat 2 plays its tiles in the order', H2)
    for i, t in enumerate(ordering): put(ws, f'{get_column_letter(6+i)}1', t, BLUE, fill=INPUT_FILL)
    ORD = [f'$F$1', f'$G$1', f'$H$1']
    put(ws, 'A2', 'At each of its turns seat 2 plays the first tile in this order that is still in hand and legal. Rows are worlds; the dice come from the Tape sheet.', GREY)
    put(ws, 'A3', 'Root tile this line plays at trick 5 (must follow the 6-4):', BOLD)
    # first legal in ordering vs led 6-4 (sixes): follows = contains a 6 and not trump; any follower exists (6-3,6-0 always do)
    q0 = '"6"'
    legal0 = ','.join(f'IF({follows(o, q0)}=1,1,0)' for o in ORD)
    put(ws, 'H3', f'=INDEX(F1:H1,MATCH(1,CHOOSE({{1,2,3}},{legal0}),0))', BOLD)
    ws.merge_cells('A3:G3')
    put(ws, 'I3', '→ dice line', GREY)
    ROOT = '$H$3'
    # ---- column plan
    # readable part
    ws.column_dimensions['A'].width = 8
    play = {}
    play[(5, 1)] = new('t5p1'); play[(5, 2)] = new('t5p2'); play[(5, 3)] = new('t5p3'); play[(5, 4)] = new('t5p4')
    new('t5win', 7); new('t5pts', 6); new('t5bid', 6); new('t5def', 6)
    for t in (6, 7):
        new(f't{t}lead', 7)
        for p in (1, 2, 3, 4): new(f't{t}s{p}', 5)
        for p in (1, 2, 3, 4): play[(t, p)] = new(f't{t}p{p}')
        new(f't{t}win', 7); new(f't{t}pts', 6); new(f't{t}bid', 6); new(f't{t}def', 6)
    new('decided', 9); new('made', 8); new('hist2', 30)
    # workings (grouped)
    for s in SEATS:
        for i in range(SEAT_TILES[s]): new(f'h{s}_{i}', 7, True)
    for t in TRICKS: new(f'q{t}', 5, True)
    # own picks
    for t in TRICKS:
        for mode in (['fol'] if t == 5 else ['lead', 'fol']):
            for i in range(3): new(f'own{t}{mode}_rem{i}', 5, True)
            if mode == 'fol':
                for i in range(3): new(f'own{t}{mode}_fol{i}', 5, True)
                new(f'own{t}{mode}_nf', 5, True)
            for i in range(3): new(f'own{t}{mode}_leg{i}', 5, True)
            new(f'own{t}{mode}', 8, True)
        if t > 5: new(f'own{t}', 8, True)
    # rolls
    for s in SEATS:
        for t in TRICKS:
            if s == 1 and t == 5: continue
            for mode in (['fol'] if t == 5 else ['lead', 'fol']):
                n = SEAT_TILES[s]
                for i in range(n): new(f'r{s}{t}{mode}_rem{i}', 5, True)
                if mode == 'fol':
                    for i in range(n): new(f'r{s}{t}{mode}_fol{i}', 5, True)
                    new(f'r{s}{t}{mode}_nf', 5, True)
                for i in range(n): new(f'r{s}{t}{mode}_leg{i}', 5, True)
                for i in range(n): new(f'r{s}{t}{mode}_cum{i}', 5, True)
                new(f'r{s}{t}{mode}_n', 5, True); new(f'r{s}{t}{mode}_u', 6, True); new(f'r{s}{t}{mode}_k', 5, True)
                new(f'r{s}{t}{mode}', 8, True)
            if t > 5: new(f'r{s}{t}', 8, True)
    # strengths
    for t in TRICKS:
        for p in (1, 2, 3, 4): new(f'st{t}p{p}', 6, True)
    # ---- headers
    labels = {'t5p1': 'S1 led', 't5p2': 'S2', 't5p3': 'S3', 't5p4': 'S0', 't5win': 'winner', 't5pts': 'pts', 't5bid': 'bid', 't5def': 'def',
              'decided': 'decided at trick', 'made': 'bid made?', 'hist2': 'history seen before seat 2\'s 2nd play'}
    for t in (6, 7):
        labels[f't{t}lead'] = 'leads'
        for p in (1, 2, 3, 4): labels[f't{t}s{p}'] = f'seat@{p}'; labels[f't{t}p{p}'] = f'tile@{p}'
        labels[f't{t}win'] = 'winner'; labels[f't{t}pts'] = 'pts'; labels[f't{t}bid'] = 'bid'; labels[f't{t}def'] = 'def'
    for k, L in cols.items():
        c = ws[f'{L}5']; c.value = labels.get(k, k); c.font = BOLD; c.fill = HEAD_FILL; c.border = BOX
        c.alignment = Alignment(wrap_text=True, vertical='center', text_rotation=0)
    ws.row_dimensions[5].height = 30
    c = ws['A5']; c.value = 'World'; c.font = BOLD; c.fill = HEAD_FILL; c.border = BOX
    # group banners on row 4
    for t, k0, k1 in ((5, 't5p1', 't5def'), (6, 't6lead', 't6def'), (7, 't7lead', 't7def')):
        a, b = cols[k0], cols[k1]; ws[f'{a}4'] = f'Trick {t}'; ws[f'{a}4'].font = BOLD; ws.merge_cells(f'{a}4:{b}4')
    ws[f'{cols["h0_0"]}4'] = 'Workings (collapse with the − above)'; ws[f'{cols["h0_0"]}4'].font = GREY

    # ---- per-world rows
    for w in range(NW):
        r = WROW(w)
        def R(key, abs_=False): return ref(key, r, abs_)
        put(ws, f'A{r}', f'W{w+1}', BOLD)
        # hands (links to Worlds)
        for s in SEATS:
            for i in range(SEAT_TILES[s]):
                put(ws, R(f'h{s}_{i}'), f"=Worlds!{WORLD_COL[s][i]}{5+w}", GREEN)
        H = {s: [R(f'h{s}_{i}') for i in range(SEAT_TILES[s])] for s in SEATS}
        # trick 5 fixed order: S1 (6-4), S2, S3, S0
        put(ws, R('t5p1'), '6-4', BLUE)
        put(ws, R('q5'), f'={ledsuit(R("t5p1"))}')
        # ---- own pick builder
        def own_pick(t, mode):
            pre = f'own{t}{mode}'
            earlier = [R('own5')] if t == 6 else ([R('own5'), R('own6')] if t == 7 else [])
            for i in range(3):
                o = ORD[i]
                remf = '1' if not earlier else 'IF(AND(' + ','.join(f'{o}<>{e}' for e in earlier) + '),1,0)'
                put(ws, R(f'{pre}_rem{i}'), f'={remf}')
            if mode == 'fol':
                for i in range(3): put(ws, R(f'{pre}_fol{i}'), f'={follows(ORD[i], R(f"q{t}"))}')
                put(ws, R(f'{pre}_nf'), '=' + '+'.join(f'{R(f"{pre}_rem{i}")}*{R(f"{pre}_fol{i}")}' for i in range(3)))
                for i in range(3): put(ws, R(f'{pre}_leg{i}'), f'={R(f"{pre}_rem{i}")}*IF({R(f"{pre}_nf")}>0,{R(f"{pre}_fol{i}")},1)')
            else:
                for i in range(3): put(ws, R(f'{pre}_leg{i}'), f'={R(f"{pre}_rem{i}")}')
            put(ws, R(pre), f'=INDEX($F$1:$H$1,MATCH(1,{R(f"{pre}_leg0")}:{R(f"{pre}_leg2")},0))')
        # ---- roll builder
        def roll(s, t, mode):
            pre = f'r{s}{t}{mode}'; n = SEAT_TILES[s]
            if s == 1: earlier = [] if t <= 6 else [R('r16')]      # walt's trick-5 tile (6-4) was never in the dealt hand
            else: earlier = [R(f'r{s}5')] if t == 6 else ([R(f'r{s}5'), R(f'r{s}6')] if t == 7 else [])
            for i in range(n):
                x = H[s][i]
                remf = '1' if not earlier else 'IF(AND(' + ','.join(f'{x}<>{e}' for e in earlier) + '),1,0)'
                put(ws, R(f'{pre}_rem{i}'), f'={remf}')
            if mode == 'fol':
                for i in range(n): put(ws, R(f'{pre}_fol{i}'), f'={follows(H[s][i], R(f"q{t}"))}')
                put(ws, R(f'{pre}_nf'), '=' + '+'.join(f'{R(f"{pre}_rem{i}")}*{R(f"{pre}_fol{i}")}' for i in range(n)))
                for i in range(n): put(ws, R(f'{pre}_leg{i}'), f'={R(f"{pre}_rem{i}")}*IF({R(f"{pre}_nf")}>0,{R(f"{pre}_fol{i}")},1)')
            else:
                for i in range(n): put(ws, R(f'{pre}_leg{i}'), f'={R(f"{pre}_rem{i}")}')
            for i in range(n):
                put(ws, R(f'{pre}_cum{i}'), '=' + '+'.join(R(f'{pre}_leg{j}') for j in range(i + 1)))
            put(ws, R(f'{pre}_n'), f'={R(f"{pre}_cum{n-1}")}')
            u63 = f"Tape!{TAPE_COL[('U63', s, t)]}{r}"; u60 = f"Tape!{TAPE_COL[('U60', s, t)]}{r}"
            put(ws, R(f'{pre}_u'), f'=IF({ROOT}="6-3",{u63},{u60})', GREEN, nf='0.00')
            put(ws, R(f'{pre}_k'), f'=1+INT({R(f"{pre}_u")}*{R(f"{pre}_n")})')
            put(ws, R(pre), f'=INDEX({H[s][0]}:{H[s][n-1]},MATCH({R(f"{pre}_k")},{R(f"{pre}_cum0")}:{R(f"{pre}_cum{n-1}")},0))')
        # trick 5
        own_pick(5, 'fol'); put(ws, R('t5p2'), f'={R("own5fol")}'); cols.setdefault('own5', cols['own5fol'])
        roll(3, 5, 'fol'); put(ws, R('t5p3'), f'={R("r35fol")}'); cols.setdefault('r35', cols['r35fol'])
        roll(0, 5, 'fol'); put(ws, R('t5p4'), f'={R("r05fol")}'); cols.setdefault('r05', cols['r05fol'])
        def resolve(t, leader_expr, seats_of_pos):
            # strengths, winner, points, running score
            for p in (1, 2, 3, 4):
                put(ws, R(f'st{t}p{p}'), f'={strength(R(f"t{t}p{p}"), R(f"q{t}"))}')
            S = ','.join(R(f'st{t}p{p}') for p in (1, 2, 3, 4))
            put(ws, R(f't{t}win'), f'=INDEX(CHOOSE({{1,2,3,4}},{seats_of_pos}),MATCH(MAX({S}),CHOOSE({{1,2,3,4}},{S}),0))')
            put(ws, R(f't{t}pts'), '=' + '+'.join(countpts(R(f't{t}p{p}')) for p in (1, 2, 3, 4)) + '+1')
            prev_b = 'Table!$B$5' if t == 5 else R(f't{t-1}bid'); prev_d = 'Table!$B$6' if t == 5 else R(f't{t-1}def')
            put(ws, R(f't{t}bid'), f'={prev_b}+IF(MOD({R(f"t{t}win")},2)=1,{R(f"t{t}pts")},0)')
            put(ws, R(f't{t}def'), f'={prev_d}+IF(MOD({R(f"t{t}win")},2)=0,{R(f"t{t}pts")},0)')
        resolve(5, '1', '1,2,3,0')
        for t in (6, 7):
            put(ws, R(f't{t}lead'), f'={R(f"t{t-1}win")}')
            for p in (1, 2, 3, 4): put(ws, R(f't{t}s{p}'), f'=MOD({R(f"t{t}lead")}+{p-1},4)')
            # lead versions (no led suit) and follower versions
            own_pick(t, 'lead'); roll(0, t, 'lead'); roll(1, t, 'lead'); roll(3, t, 'lead')
            # led tile = whoever leads, playing as leader
            put(ws, R(f't{t}p1'), f'=CHOOSE({R(f"t{t}lead")}+1,{R(f"r0{t}lead")},{R(f"r1{t}lead")},{R(f"own{t}lead")},{R(f"r3{t}lead")})')
            put(ws, R(f'q{t}'), f'={ledsuit(R(f"t{t}p1"))}')
            own_pick(t, 'fol'); roll(0, t, 'fol'); roll(1, t, 'fol'); roll(3, t, 'fol')
            put(ws, R(f'own{t}'), f'=IF({R(f"t{t}lead")}=2,{R(f"own{t}lead")},{R(f"own{t}fol")})')
            for s in SEATS: put(ws, R(f'r{s}{t}'), f'=IF({R(f"t{t}lead")}={s},{R(f"r{s}{t}lead")},{R(f"r{s}{t}fol")})')
            for p in (2, 3, 4):
                put(ws, R(f't{t}p{p}'), f'=CHOOSE({R(f"t{t}s{p}")}+1,{R(f"r0{t}")},{R(f"r1{t}")},{R(f"own{t}")},{R(f"r3{t}")})')
            resolve(t, R(f't{t}lead'), ','.join(R(f't{t}s{p}') for p in (1, 2, 3, 4)))
        put(ws, R('decided'), f'=IF(OR({R("t5bid")}>=Table!$B$4,{R("t5def")}>=Table!$B$7),5,IF(OR({R("t6bid")}>=Table!$B$4,{R("t6def")}>=Table!$B$7),6,7))')
        put(ws, R('made'), f'=IF({R("t7bid")}>=Table!$B$4,1,0)', BOLD)
        # history seen before seat 2's second play: trick 5 complete + trick-6 tiles played before seat 2's turn
        pos2 = f'MOD(2-{R("t6lead")}+4,4)'   # 0..3 position of seat 2 in trick 6
        put(ws, R('hist2'), f'={R("t5p1")}&" "&{R("t5p2")}&" "&{R("t5p3")}&" "&{R("t5p4")}&" | "&IF({pos2}>=1,{R("t6p1")}&" ","")&IF({pos2}>=2,{R("t6p2")}&" ","")&IF({pos2}>=3,{R("t6p3")},"")')
    last = WROW(NW - 1)
    rng = f'A6:{cols["hist2"]}{last}'
    ws.conditional_formatting.add(rng, FormulaRule(formula=[f'${cols["made"]}6=1'], fill=MADE_FILL))
    ws.conditional_formatting.add(rng, FormulaRule(formula=[f'${cols["made"]}6=0'], fill=SET_FILL))
    ws.freeze_panes = f'{cols["t5p1"]}6'
    ws.sheet_properties.outlinePr = Outline(summaryBelow=False, summaryRight=False)
    return ws, cols

LINE = []
for i, o in enumerate(ORDERINGS):
    LINE.append(build_line(i, o))

# ================================================================ Flat Walt (the fold)
fw = wb.create_sheet('Flat Walt')
widths(fw, [10] + [11] * 6 + [4, 13, 13, 4, 40])
put(fw, 'A1', 'Flat Walt: the same decision, no recursion', H1)
put(fw, 'A2', 'Each Line sheet plays one ordering of seat 2\'s tiles through to the end in every world. Here the lines fold back up: for each world and each root tile, the plan node takes the MIN over the lines that start with that tile; then the root sums the worlds.', GREY, al=WRAP)
fw.merge_cells('A2:L2'); fw.row_dimensions[2].height = 44
header(fw, 4, ['World'] + [f'Line {i+1}' for i in range(6)] + ['', 'min if 6-3', 'min if 6-0', '', 'Note'])
put(fw, 'A5', 'root tile →', GREY)
for i, (ws, cols) in enumerate(LINE):
    c = get_column_letter(2 + i)
    put(fw, f'{c}5', f"='Line {i+1}'!$H$3", GREEN)
for w in range(NW):
    r = WROW(w)
    put(fw, f'A{r}', f'W{w+1}', BOLD)
    for i, (ws, cols) in enumerate(LINE):
        c = get_column_letter(2 + i)
        put(fw, f'{c}{r}', f"='Line {i+1}'!{cols['made']}{r}", GREEN)
    for col, tile in (('I', '6-3'), ('J', '6-0')):
        terms = ','.join(f'IF({get_column_letter(2+i)}$5="{tile}",{get_column_letter(2+i)}{r},9)' for i in range(6))
        put(fw, f'{col}{r}', f'=MIN({terms})')
lastw = WROW(NW - 1); tot = lastw + 1
put(fw, f'A{tot}', 'Worlds where the bid is made', BOLD)
put(fw, f'I{tot}', f'=SUM(I6:I{lastw})', BOLD); put(fw, f'J{tot}', f'=SUM(J6:J{lastw})', BOLD)
put(fw, f'A{tot+1}', 'The tree sheets say', BOLD); put(fw, f'I{tot+1}', '=Decision!B5', GREEN); put(fw, f'J{tot+1}', '=Decision!C5', GREEN)
put(fw, f'A{tot+2}', 'Agree?', BOLD); put(fw, f'I{tot+2}', f'=IF(I{tot}=I{tot+1},"✓","✗")', BOLD); put(fw, f'J{tot+2}', f'=IF(J{tot}=J{tot+1},"✓","✗")', BOLD)
put(fw, f'L{tot}', 'With the tape pinned to the trace\'s rolls these must match the 151-row trees. Switch Tape!B3 to "live" and they become a fresh estimate.', GREY, al=WRAP)
fw.merge_cells(f'L{tot}:L{tot+2}')
put(fw, f'A{tot+4}', 'The no-fusion check', H2)
put(fw, f'A{tot+5}', 'Taking a MIN per world is only honest if no two worlds look identical to seat 2 at its second decision. Count the distinct histories per line (8 = all different):', GREY, al=WRAP)
fw.merge_cells(f'A{tot+5}:L{tot+5}'); fw.row_dimensions[tot+5].height = 30
for i, (ws, cols) in enumerate(LINE):
    c = get_column_letter(2 + i); h = cols['hist2']
    put(fw, f'{c}{tot+6}', f"=SUMPRODUCT(1/COUNTIF('Line {i+1}'!{h}6:{h}{lastw},'Line {i+1}'!{h}6:{h}{lastw}))")
put(fw, f'A{tot+6}', 'distinct histories', BOLD)
put(fw, f'I{tot+6}', f'=IF(MIN(B{tot+6}:G{tot+6})=8,"all distinct ✓","some worlds share a history: group them with SUMIFS before taking the MIN")', BOLD)
put(fw, f'A{tot+8}', 'Reading the lines', H2)
notes = [
 'Line sheets: one row per world, left to right is tricks 5, 6, 7 with the running score, then "decided at trick" and "bid made?". The workings (legal tiles, cumulative counts, the die, the pick) are in the grouped columns to the right.',
 'A roll is: list the seat\'s remaining tiles, flag the legal ones, count them (n), take the die u, pick the 1+INT(u×n)-th legal tile. Every dice node in the trees is this, done in place.',
 'A plan node is: the first tile in the line\'s order that is in hand and legal. Six lines cover every path through seat 2\'s choices; the MIN above is the plan node.',
 'Seat 2 is defending, so MIN. Bidding would be MAX, nothing else changes.',
]
for i, n in enumerate(notes):
    put(fw, f'A{tot+9+i}', n, al=WRAP); fw.merge_cells(f'A{tot+9+i}:L{tot+9+i}'); fw.row_dimensions[tot+9+i].height = 30
fw.conditional_formatting.add(f'B6:G{lastw}', CellIsRule(operator='equal', formula=['1'], fill=MADE_FILL))
fw.conditional_formatting.add(f'B6:G{lastw}', CellIsRule(operator='equal', formula=['0'], fill=SET_FILL))

# ================================================================ Swap votes (pairwise "further back" matrix)
sv = wb.create_sheet('Swap votes')
widths(sv, [22] + [11] * 8 + [40])
T = ['6-3', '6-0', '5-5']
PAIRS = [(a, b) for a in T for b in T if a != b]
pcol = {p: get_column_letter(2 + i) for i, p in enumerate(PAIRS)}
put(sv, 'A1', 'Swap votes: should one tile travel further back than another?', H1)
put(sv, 'A2', 'A transposition: take a line, swap two of seat 2\'s tiles, keep everything else where it was. If the original beats the swap in a world, that world votes for the original order of those two tiles. '
             'The matrix sums the votes over lines and worlds. It reads straight off Flat Walt, so it goes live with the dice.', GREY, al=WRAP)
sv.merge_cells('A2:J2'); sv.row_dimensions[2].height = 44
# A: which lines keep X ahead of Y
put(sv, 'A4', 'Lines that keep X ahead of Y', H2)
header(sv, 5, ['Line'] + [f'{a} ▸ {b}' for a, b in PAIRS] + ['root tile'])
FLAG = {}
for k in range(6):
    r = 6 + k; put(sv, f'A{r}', f'Line {k+1}', BOLD)
    for a, b in PAIRS:
        c = pcol[(a, b)]
        put(sv, f'{c}{r}', f'=IF(MATCH("{a}",\'Line {k+1}\'!$F$1:$H$1,0)<MATCH("{b}",\'Line {k+1}\'!$F$1:$H$1,0),1,0)')
        FLAG[(k, (a, b))] = f'${c}${r}'
    put(sv, f'H{r}', f"='Line {k+1}'!$H$3", GREEN)
# B: partner line under each swap
put(sv, 'A13', 'Transpose two tiles in a line and you get another line (its partner under that swap)', H2)
UP = [('6-3', '6-0'), ('6-3', '5-5'), ('6-0', '5-5')]
header(sv, 14, ['Line'] + [f'swap {a}/{b} →' for a, b in UP])
PARTNER = {}
for k, o in enumerate(ORDERINGS):
    r = 15 + k; put(sv, f'A{r}', f'Line {k+1}', BOLD)
    for i, (a, b) in enumerate(UP):
        sw = tuple(b if t == a else a if t == b else t for t in o)
        kp = ORDERINGS.index(sw); PARTNER[(k, (a, b))] = kp; PARTNER[(k, (b, a))] = kp
        put(sv, f'{get_column_letter(2+i)}{r}', f'Line {kp+1}', GREY)
put(sv, 'F15', 'Structure of the six orderings, not data: each swap pairs the lines up three ways.', GREY)
# C: votes per world: X ahead of Y, holding the rest of the order fixed
put(sv, 'A23', 'Vote: with everything else in the order fixed, X ahead of Y is strictly better than the swap', H2)
header(sv, 24, ['World'] + [f'{a} ▸ {b}' for a, b in PAIRS])
def made_ref(k, w): return f"'Flat Walt'!{get_column_letter(2+k)}{WROW(w)}"
for w in range(NW):
    r = 25 + w; put(sv, f'A{r}', f'W{w+1}', BOLD)
    for a, b in PAIRS:
        c = pcol[(a, b)]
        terms = []
        for k in range(6):
            kp = PARTNER[(k, (a, b))]
            terms.append(f'{FLAG[(k,(a,b))]}*IF(Table!$B$8="defending",IF({made_ref(k,w)}<{made_ref(kp,w)},1,0),IF({made_ref(k,w)}>{made_ref(kp,w)},1,0))')
        put(sv, f'{c}{r}', '=' + '+'.join(terms))
put(sv, 'I25', 'Up to 3 per cell: one for each line that keeps X ahead of Y and beats its partner.', GREY)
# D: matrix
put(sv, 'A35', 'The matrix: votes that the row tile belongs ahead of the column tile', H2)
header(sv, 36, ['ahead \\ behind'] + T + ['', 'net (row − col)'] + T)
for i, a in enumerate(T):
    r = 37 + i; put(sv, f'A{r}', a, BOLD); put(sv, f'F{r}', a, BOLD)
    for j, b in enumerate(T):
        c = get_column_letter(2 + j); cn = get_column_letter(7 + j)
        if a == b:
            put(sv, f'{c}{r}', '—', GREY); put(sv, f'{cn}{r}', '—', GREY); continue
        put(sv, f'{c}{r}', f'=SUM({pcol[(a, b)]}25:{pcol[(a, b)]}32)', BOLD)
        put(sv, f'{cn}{r}', f'={c}{r}-{get_column_letter(2 + i)}{37 + j}')
sv.conditional_formatting.add('B37:D39', CellIsRule(operator='greaterThan', formula=['0'], fill=PLAN_FILL))
sv.conditional_formatting.add('G37:I39', CellIsRule(operator='greaterThan', formula=['0'], fill=PLAN_FILL))
sv.conditional_formatting.add('G37:I39', CellIsRule(operator='lessThan', formula=['0'], fill=SET_FILL))
put(sv, 'A41', 'Reading', H2)
rr = 42
for i in range(3):
    for j in range(i + 1, 3):
        a, b = T[i], T[j]
        ci = get_column_letter(2 + j); cj = get_column_letter(2 + i)
        fwd = f'{ci}{37+i}'; rev = f'{cj}{37+j}'
        put(sv, f'A{rr}', f'{a} vs {b}', BOLD)
        put(sv, f'B{rr}', f'=IF({fwd}>{rev},"{a} ahead of {b}: "&{fwd}&" for, "&{rev}&" against",IF({rev}>{fwd},"{b} ahead of {a}: "&{rev}&" for, "&{fwd}&" against","no evidence either way ("&{fwd}&" / "&{rev}&")"))')
        rr += 1
put(sv, f'A{rr+1}', 'Caveats: a vote counts a line whether or not seat 2 would ever choose it, pairwise majorities can cycle, and the best slot for one tile can depend on where another went. '
                    'This is a lens on the search, not a decision rule; the decision is still the MIN on Flat Walt.', GREY, al=WRAP)
sv.merge_cells(f'A{rr+1}:J{rr+1}'); sv.row_dimensions[rr+1].height = 30
