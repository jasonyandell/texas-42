// walt.c — Walt (walt42.py to the line) vs the new player (exact-tape L1 + no-fusion exact tail). Match driver.
// build: gcc -O2 -o walt walt.c -lm
// match:  ./walt <deals> <seed> <A> <B> [walt_outer=30] [new_worlds=128]   A,B in {new, walt, inner, random, tape}
// decide: ./walt decide <seat> <bid> <bidder> <trump> <dealt-hand-mask> [actor tile]...   (bidding team = odd seats; relabel so bidder is seat 1)
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>
typedef uint32_t mask; typedef uint64_t u64;
static int HI[28], LO[28], PTS[28]; static int BID = 30;
static u64 rs; static u64 rnd64(void) { rs ^= rs << 13; rs ^= rs >> 7; rs ^= rs << 17; return rs; }
static double rnd(void) { return (rnd64() >> 11) * (1.0 / 9007199254740992.0); }
static int rndint(int n) { return (int)(rnd() * n); }
// ---- rules (walt42.py Rules)
static mask SUIT[8]; static int LEAD[28]; static mask TRUMPS;
static void rules_init(int trump) {
  TRUMPS = 0; for (int t = 0; t < 28; t++) if (HI[t] == trump || LO[t] == trump) TRUMPS |= 1u << t;
  for (int q = 0; q < 7; q++) { SUIT[q] = 0; for (int t = 0; t < 28; t++) if ((HI[t] == q || LO[t] == q) && !(TRUMPS >> t & 1)) SUIT[q] |= 1u << t; }
  SUIT[7] = TRUMPS; for (int t = 0; t < 28; t++) LEAD[t] = (TRUMPS >> t & 1) ? 7 : HI[t];
}
static mask legal(mask hand, int led) { if (led < 0) return hand; mask f = hand & SUIT[led]; return f ? f : hand; }
static int strength(int t, int led) { int tier = (TRUMPS >> t & 1) ? 2 : ((SUIT[led] >> t & 1) ? 1 : 0); return tier * 100 + (HI[t] == LO[t] ? 12 : HI[t] + LO[t]); }
static int popc(mask m) { return __builtin_popcount(m); }
static int kth(mask m, int k) { for (int t = 0; t < 28; t++) if (m >> t & 1) { if (k == 0) return t; k--; } return -1; }
// ---- public state
typedef struct { mask played; int leader; int trick[4]; int n; int t1, t0; } State;
static int turn(const State *s) { return (s->leader + s->n) % 4; }
static int outcome(const State *s) { return s->t1 >= BID ? 1 : (s->t0 > 42 - BID ? 0 : -1); }
static int ledsuit(const State *s) { return s->n ? LEAD[s->trick[0]] : -1; }
static State play(State s, int t) {
  s.played |= 1u << t; s.trick[s.n++] = t; if (s.n < 4) return s;
  int led = LEAD[s.trick[0]], best = 0; for (int i = 1; i < 4; i++) if (strength(s.trick[i], led) > strength(s.trick[best], led)) best = i;
  int w = (s.leader + best) % 4, won = 1; for (int i = 0; i < 4; i++) won += PTS[s.trick[i]];
  if (w % 2) s.t1 += won; else s.t0 += won; s.leader = w; s.n = 0; return s;
}
// ---- deals hold REMAINING hands at sampling time; later plays are removed via state.played
#define MAXD 4096
typedef struct { mask h[4]; double w; } Deal;
static void hand_sizes(const State *s, int *size) { int base = 7 - (popc(s->played) - s->n) / 4; for (int i = 0; i < 4; i++) size[i] = base; for (int i = 0; i < s->n; i++) size[(s->leader + i) % 4]--; }
static void sample(int seat, mask hand, const State *s, const mask *voids, int n, Deal *out) {
  int unseen[28], nu = 0; mask seen = s->played | hand; for (int t = 0; t < 28; t++) if (!(seen >> t & 1)) unseen[nu++] = t;
  int size[4]; hand_sizes(s, size);
  int got = 0;
  while (got < n) {
    for (int i = nu - 1; i > 0; i--) { int j = rndint(i + 1); int x = unseen[i]; unseen[i] = unseen[j]; unseen[j] = x; }
    mask d[4] = {0, 0, 0, 0}; int at = 0, ok = 1; d[seat] = hand;
    for (int q = 0; q < 4; q++) if (q != seat) { for (int i = 0; i < size[q]; i++) d[q] |= 1u << unseen[at++]; if (d[q] & voids[q]) ok = 0; }
    if (ok) { for (int q = 0; q < 4; q++) out[got].h[q] = d[q]; out[got].w = 1; got++; }
  }
}
// ---- policies: (seat, remaining hand, state, voids) -> tile
typedef int (*Policy)(int, mask, const State *, const mask *);
static int random_policy(int seat, mask hand, const State *s, const mask *voids) { mask L = legal(hand, ledsuit(s)); return kth(L, rndint(popc(L))); }
// walt42.value: deals that make the bid; `me` commits to one tile per node (no fusion); others play `assume`
static double value(int me, State s, Deal *deals, int nd, Policy assume, const mask *voids) {
  int done = outcome(&s); if (done >= 0) { double w = 0; for (int i = 0; i < nd; i++) w += deals[i].w; return done ? w : 0; }
  int seat = turn(&s);
  if (seat == me) {
    mask L = legal(deals[0].h[me] & ~s.played, ledsuit(&s)); double best = (me % 2) ? -1e18 : 1e18;
    for (int t = 0; t < 28; t++) if (L >> t & 1) { double v = value(me, play(s, t), deals, nd, assume, voids); if (me % 2 ? v > best : v < best) best = v; }
    return best;
  }
  int *tile = malloc(sizeof(int) * nd);
  for (int i = 0; i < nd; i++) { mask hand = deals[i].h[seat] & ~s.played; mask L = legal(hand, ledsuit(&s)); tile[i] = popc(L) == 1 ? kth(L, 0) : assume(seat, hand, &s, voids); }
  double total = 0; Deal *g = malloc(sizeof(Deal) * nd);
  for (int t = 0; t < 28; t++) { int ng = 0; for (int i = 0; i < nd; i++) if (tile[i] == t) g[ng++] = deals[i]; if (ng) total += value(me, play(s, t), g, ng, assume, voids); }
  free(g); free(tile); return total;
}
static Policy G_assume; static int G_n;
static int walt_policy(int seat, mask hand, const State *s, const mask *voids) {
  Deal *deals = malloc(sizeof(Deal) * G_n); sample(seat, hand, s, voids, G_n, deals);
  mask L = legal(hand, ledsuit(s)); int best = -1; double bv = 0;
  for (int t = 0; t < 28; t++) if (L >> t & 1) { double v = value(seat, play(*s, t), deals, G_n, G_assume, voids); if (best < 0 || (seat % 2 ? v > bv : v < bv)) { best = t; bv = v; } }
  free(deals); return best;
}
static int inner_L0(int seat, mask hand, const State *s, const mask *voids) {  // walt(random_policy, 8)
  Policy pa = G_assume; int pn = G_n; G_assume = random_policy; G_n = 8; int t = walt_policy(seat, hand, s, voids); G_assume = pa; G_n = pn; return t;
}
static int WALT_OUTER = 30;
static int walt(int seat, mask hand, const State *s, const mask *voids) { G_assume = inner_L0; G_n = WALT_OUTER; return walt_policy(seat, hand, s, voids); }  // walt(L0, 30)
// ---- new player: exact-tape L1 (others from a per-world tape, me best-responding per world) + no-fusion exact tail
static double TAPE[512][28]; static int NEW_W = 128;
static int br_tape(int me, State s, const mask *d, int w, int ply) {
  int done = outcome(&s); if (done >= 0) return done; int seat = turn(&s); mask hand = d[seat] & ~s.played; mask L = legal(hand, ledsuit(&s));
  if (seat == me) { int best = (me % 2) ? -1 : 2; for (int t = 0; t < 28; t++) if (L >> t & 1) { int v = br_tape(me, play(s, t), d, w, ply + 1); if (me % 2 ? v > best : v < best) best = v; } return best; }
  int n = popc(L); int k = (int)(TAPE[w][ply] * n); if (k >= n) k = n - 1; return br_tape(me, play(s, kth(L, k)), d, w, ply + 1);
}
static int newplayer_tape(int seat, mask hand, const State *s, const mask *voids) {
  Deal *deals = malloc(sizeof(Deal) * NEW_W); sample(seat, hand, s, voids, NEW_W, deals);
  for (int w = 0; w < NEW_W; w++) for (int p = 0; p < 28; p++) TAPE[w][p] = rnd();   // common random numbers across candidates
  mask L = legal(hand, ledsuit(s)); int best = -1; double bv = 0;
  for (int t = 0; t < 28; t++) if (L >> t & 1) { double v = 0; for (int w = 0; w < NEW_W; w++) v += br_tape(seat, play(*s, t), deals[w].h, w, 0); if (best < 0 || (seat % 2 ? v > bv : v < bv)) { best = t; bv = v; } }
  free(deals); return best;
}
static int all_deals(int seat, mask hand, const State *s, const mask *voids, Deal *out, int cap) {
  int unseen[28], nu = 0; mask seen = s->played | hand; for (int t = 0; t < 28; t++) if (!(seen >> t & 1)) unseen[nu++] = t;
  int size[4]; hand_sizes(s, size); int o[3], no = 0; for (int q = 0; q < 4; q++) if (q != seat) o[no++] = q;
  int cnt = 0; mask all = (nu == 32) ? 0xffffffffu : ((1u << nu) - 1);
  for (mask a = 0; a <= all; a++) { if (popc(a) != size[o[0]]) continue; mask rest = all & ~a;
    for (mask b = rest;; b = (b - 1) & rest) {
      if (popc(b) == size[o[1]]) { mask c = rest & ~b; mask h[4] = {0, 0, 0, 0}; h[seat] = hand;
        for (int i = 0; i < nu; i++) { if (a >> i & 1) h[o[0]] |= 1u << unseen[i]; else if (b >> i & 1) h[o[1]] |= 1u << unseen[i]; else if (c >> i & 1) h[o[2]] |= 1u << unseen[i]; }
        int ok = 1; for (int q = 0; q < 4; q++) if (q != seat && (h[q] & voids[q])) ok = 0;
        if (ok) { if (cnt >= cap) return -1; for (int q = 0; q < 4; q++) out[cnt].h[q] = h[q]; out[cnt].w = 1.0; cnt++; } }
      if (b == 0) break; }
    if (a == all) break; }
  return cnt;
}
static double exact_value(int me, State s, Deal *deals, int nd) {   // expectimax: others branch over every legal tile at weight 1/n
  int done = outcome(&s); if (done >= 0) { double w = 0; for (int i = 0; i < nd; i++) w += deals[i].w; return done ? w : 0; }
  int seat = turn(&s);
  if (seat == me) { mask L = legal(deals[0].h[me] & ~s.played, ledsuit(&s)); double best = (me % 2) ? -1e18 : 1e18;
    for (int t = 0; t < 28; t++) if (L >> t & 1) { double v = exact_value(me, play(s, t), deals, nd); if (me % 2 ? v > best : v < best) best = v; } return best; }
  double total = 0; Deal *g = malloc(sizeof(Deal) * nd);
  for (int t = 0; t < 28; t++) { int ng = 0;
    for (int i = 0; i < nd; i++) { mask L = legal(deals[i].h[seat] & ~s.played, ledsuit(&s)); if (L >> t & 1) { g[ng] = deals[i]; g[ng].w /= popc(L); ng++; } }
    if (ng) total += exact_value(me, play(s, t), g, ng); }
  free(g); return total;
}
static int TAIL_AT = 12;
static int newplayer(int seat, mask hand, const State *s, const mask *voids) {
  if (28 - popc(s->played) <= TAIL_AT) { Deal *deals = malloc(sizeof(Deal) * MAXD); int nd = all_deals(seat, hand, s, voids, deals, MAXD);
    if (nd > 0) { mask L = legal(hand, ledsuit(s)); int best = -1; double bv = 0;
      for (int t = 0; t < 28; t++) if (L >> t & 1) { double v = exact_value(seat, play(*s, t), deals, nd); if (best < 0 || (seat % 2 ? v > bv : v < bv)) { best = t; bv = v; } }
      free(deals); return best; } free(deals); }
  return newplayer_tape(seat, hand, s, voids);
}
// ---- contract (walt42.main): seat & trump with the most trumps, then trump double, then most doubles; rotate so bidder = seat 1
static void contract(const mask *hs, int *bb, int *bt) {
  long bs = -1; *bb = -1; *bt = 0;
  for (int s = 0; s < 4; s++) for (int p = 0; p < 7; p++) { int cnt = 0, dbl = 0, hasd = 0;
    for (int t = 0; t < 28; t++) if (hs[s] >> t & 1) { if (HI[t] == p || LO[t] == p) cnt++; if (HI[t] == LO[t]) { dbl++; if (HI[t] == p) hasd = 1; } }
    long sc = cnt * 100 + hasd * 10 + dbl; if (sc > bs || (sc == bs && (s > *bb || (s == *bb && p > *bt)))) { bs = sc; *bb = s; *bt = p; } }
}
static int play_hand(const mask *hands0, Policy *seatpol, int bb, int bt) {
  mask hands[4]; Policy pol[4]; for (int i = 0; i < 4; i++) { hands[i] = hands0[(bb + i + 3) % 4]; pol[i] = seatpol[(bb + i + 3) % 4]; }
  rules_init(bt); State s = {0, 1, {0, 0, 0, 0}, 0, 0, 0}; mask voids[4] = {0, 0, 0, 0};
  while (outcome(&s) < 0) { int seat = turn(&s); mask hand = hands[seat] & ~s.played; mask L = legal(hand, ledsuit(&s));
    int t = popc(L) == 1 ? kth(L, 0) : pol[seat](seat, hand, &s, voids);
    if (s.n && !(SUIT[LEAD[s.trick[0]]] >> t & 1)) voids[seat] |= SUIT[LEAD[s.trick[0]]];
    s = play(s, t); }
  return outcome(&s);
}
static Policy by_name(const char *n) { return !strcmp(n, "walt") ? walt : !strcmp(n, "inner") ? inner_L0 : !strcmp(n, "random") ? random_policy : !strcmp(n, "tape") ? newplayer_tape : newplayer; }
int main(int argc, char **argv) {
  for (int t = 0, h = 0; h < 7; h++) for (int l = 0; l <= h; l++, t++) { HI[t] = h; LO[t] = l; int sm = h + l; PTS[t] = (sm == 5 || sm == 10) ? sm : 0; }
  if (argc > 1 && !strcmp(argv[1], "decide")) {   // decide <seat> <bid> <bidder> <trump> <dealt-hand-mask> [actor tile]... -> tile
    int seat = atoi(argv[2]); BID = atoi(argv[3]); int bidder = atoi(argv[4]); rules_init(atoi(argv[5])); mask dealt = (mask)strtoul(argv[6], 0, 10);
    rs = 0x9E3779B97F4A7C15ULL ^ (u64)dealt ^ ((u64)argc << 32); rnd64();
    State s = {0, bidder, {0, 0, 0, 0}, 0, 0, 0}; mask voids[4] = {0, 0, 0, 0};
    for (int i = 7; i + 1 < argc; i += 2) { int a = atoi(argv[i]), t = atoi(argv[i + 1]); if (s.n && !(SUIT[LEAD[s.trick[0]]] >> t & 1)) voids[a] |= SUIT[LEAD[s.trick[0]]]; s = play(s, t); rs ^= (u64)t * 0x2545F4914F6CDD1DULL; rnd64(); }
    mask hand = dealt & ~s.played; mask L = legal(hand, ledsuit(&s));
    printf("%d\n", popc(L) == 1 ? kth(L, 0) : newplayer(seat, hand, &s, voids)); return 0;
  }
  int n = argc > 1 ? atoi(argv[1]) : 50; u64 seed = argc > 2 ? strtoull(argv[2], 0, 10) : 1; const char *A = argc > 3 ? argv[3] : "new"; const char *B = argc > 4 ? argv[4] : "walt";
  if (argc > 5) WALT_OUTER = atoi(argv[5]); if (argc > 6) NEW_W = atoi(argv[6]);
  Policy pa = by_name(A), pb = by_name(B); int winsA = 0, hands = 0;
  for (int h = 0; h < n; h++) {
    rs = seed * 1000003ULL + h * 7919ULL + 12345; rnd64(); int order[28]; for (int i = 0; i < 28; i++) order[i] = i;
    for (int i = 27; i > 0; i--) { int j = rndint(i + 1); int x = order[i]; order[i] = order[j]; order[j] = x; }
    mask hs[4] = {0, 0, 0, 0}; for (int i = 0; i < 4; i++) for (int k = 0; k < 7; k++) hs[i] |= 1u << order[7 * i + k];
    int bb, bt; contract(hs, &bb, &bt);
    for (int side = 0; side < 2; side++) {    // A holds the original seats with parity == side; the contract decides who bids
      Policy sp[4]; for (int q = 0; q < 4; q++) sp[q] = ((q % 2) == side) ? pa : pb;
      rs = seed * 7777ULL + h * 131ULL + side + 99; rnd64(); int made = play_hand(hs, sp, bb, bt);
      int A_bids = ((bb % 2) == side); hands++; winsA += A_bids ? made : !made;
      if (hands % 20 == 0) { double w = (double)winsA / hands; printf("%s vs %s: %d hands, %s wins %.1f%% ± %.1f\n", A, B, hands, A, 100 * w, 100 * sqrt(w * (1 - w) / hands)); fflush(stdout); }
    }
  }
  double w = (double)winsA / hands; printf("FINAL %s vs %s: %d hands, %s wins %.1f%% ± %.1f\n", A, B, hands, A, 100 * w, 100 * sqrt(w * (1 - w) / hands));
  return 0;
}
