/* Independent adjudication check for dispatch 012.
 * Direct orbit enumeration of j-subsets of the 28 edges of K7-with-loops
 * under S7, pure and count-label-preserving-on-the-carrier.
 * No shared code with the response's program.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

static int perm[5040][7];
static int nperm = 0;
static int edge_of[7][7];      /* edge index of pip pair */
static int elo[28], ehi[28];
static int emap[5040][28];
static int label[28];
static uint32_t badmask[5040];
static long long binom[32][32];

static void gen_perms(void) {
    int a[7];
    for (int i = 0; i < 7; i++) a[i] = i;
    /* simple recursive permutation generation */
    int idx[7], used[7];
    memset(used, 0, sizeof(used));
    int depth = 0;
    idx[0] = 0;
    while (depth >= 0) {
        if (depth == 7) {
            for (int i = 0; i < 7; i++) perm[nperm][i] = a[i];
            nperm++;
            depth--;
            if (depth >= 0) used[a[depth]] = 0;
            if (depth >= 0) idx[depth] = a[depth] + 1;
            continue;
        }
        int v = idx[depth];
        while (v < 7 && used[v]) v++;
        if (v == 7) {
            depth--;
            if (depth >= 0) { used[a[depth]] = 0; idx[depth] = a[depth] + 1; }
            continue;
        }
        a[depth] = v; used[v] = 1;
        depth++;
        if (depth < 7) idx[depth] = 0;
    }
}

int main(int argc, char **argv) {
    gen_perms();
    if (nperm != 5040) { printf("BAD nperm %d\n", nperm); return 1; }

    int n = 0;
    for (int i = 0; i < 7; i++) { edge_of[i][i] = n; elo[n] = i; ehi[n] = i; n++; }
    for (int i = 0; i < 7; i++)
        for (int j = i + 1; j < 7; j++) { edge_of[i][j] = edge_of[j][i] = n; elo[n] = i; ehi[n] = j; n++; }
    if (n != 28) return 1;

    for (int e = 0; e < 28; e++) label[e] = 0;
    label[edge_of[5][5]] = 10;
    label[edge_of[4][6]] = 10;
    label[edge_of[0][5]] = 5;
    label[edge_of[1][4]] = 5;
    label[edge_of[2][3]] = 5;

    for (int p = 0; p < 5040; p++) {
        uint32_t bad = 0;
        for (int e = 0; e < 28; e++) {
            int t = edge_of[perm[p][elo[e]]][perm[p][ehi[e]]];
            emap[p][e] = t;
            if (label[e] != label[t]) bad |= (1u << e);
        }
        badmask[p] = bad;
    }

    for (int i = 0; i < 32; i++) {
        binom[i][0] = 1;
        for (int k = 1; k <= i; k++)
            binom[i][k] = binom[i-1][k-1] + (k <= i-1 ? binom[i-1][k] : 0);
        for (int k = i + 1; k < 32; k++) binom[i][k] = 0;
    }

    int jlo = atoi(argv[1]), jhi = atoi(argv[2]);
    for (int j = jlo; j <= jhi; j++) {
        long long total = binom[28][j];
        unsigned char *seen = calloc(total, 1);
        long long classes_pure = 0, classes_lab = 0, visited_pure = 0, visited_lab = 0;

        /* pass 0 = pure, pass 1 = count-label preserving */
        for (int pass = 0; pass < 2; pass++) {
            memset(seen, 0, total);
            long long classes = 0, visited = 0;
            uint32_t mask = (j == 0) ? 0 : ((1u << j) - 1);
            uint32_t limit = 1u << 28;
            while (1) {
                /* rank of mask (colex) */
                long long r = 0;
                { uint32_t m = mask; int i = 1;
                  while (m) { int b = __builtin_ctz(m); r += binom[b][i]; i++; m &= m - 1; } }
                if (!seen[r]) {
                    classes++;
                    for (int p = 0; p < 5040; p++) {
                        if (pass == 1 && (mask & badmask[p])) continue;
                        uint32_t img = 0, m = mask;
                        while (m) { int b = __builtin_ctz(m); img |= 1u << emap[p][b]; m &= m - 1; }
                        long long r2 = 0; int i2 = 1; uint32_t mm = img;
                        while (mm) { int b = __builtin_ctz(mm); r2 += binom[b][i2]; i2++; mm &= mm - 1; }
                        if (!seen[r2]) { seen[r2] = 1; visited++; }
                    }
                }
                if (j == 0) break;
                /* Gosper next */
                uint32_t c = mask & (uint32_t)(-(int32_t)mask);
                uint32_t rr = mask + c;
                uint32_t nxt = (((rr ^ mask) >> 2) / c) | rr;
                if (nxt >= limit) break;
                mask = nxt;
            }
            if (j == 0) { classes = 1; visited = 1; }
            if (pass == 0) { classes_pure = classes; visited_pure = visited; }
            else { classes_lab = classes; visited_lab = visited; }
        }
        printf("j=%2d  a=%lld  b=%lld  exhaust_pure=%s exhaust_lab=%s (C(28,j)=%lld)\n",
               j, classes_pure, classes_lab,
               visited_pure == total ? "OK" : "NO",
               visited_lab == total ? "OK" : "NO", total);
        fflush(stdout);
        free(seen);
    }
    return 0;
}
