/* Exploratory frozen-policy rollouts. No maximization inside a hidden world.
 * Straight 42 pip trump, fixed bid 30. Actor policy input is own current hand,
 * public ordered plays, trump, bidder, leader, score only. */
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

static int hi[28],lo[28],count[28],init=0;
static void tables(void){if(init)return; int t=0;for(int h=0;h<7;h++)for(int l=0;l<=h;l++,t++){hi[t]=h;lo[t]=l;count[t]=(h+l==5||h+l==10)?h+l:0;}init=1;}
static int trump(int t,int d){return hi[t]==d||lo[t]==d;}
static int follows(int t,int led,int d){return led==7?trump(t,d):!trump(t,d)&&(hi[t]==led||lo[t]==led);}
static int strength(int t,int led,int d){if(trump(t,d))return 100+(hi[t]==lo[t]?7:hi[t]+lo[t]-d);return follows(t,led,d)?1+(hi[t]==lo[t]?7:hi[t]+lo[t]-led):0;}
static uint32_t legal(uint32_t h,int led,int d){if(led<0)return h;uint32_t f=0;for(int t=0;t<28;t++)if((h>>t&1)&&follows(t,led,d))f|=1u<<t;return f?f:h;}
static int kth(uint32_t m,int k){for(int t=0;t<28;t++)if(m>>t&1){if(k--==0)return t;}return -1;}
static uint64_t next(uint64_t *s){uint64_t z=(*s+=0x9e3779b97f4a7c15ULL);z=(z^(z>>30))*0xbf58476d1ce4e5b9ULL;z=(z^(z>>27))*0x94d049bb133111ebULL;return z^(z>>31);}
static uint64_t below(uint64_t *s,uint64_t n){uint64_t x,threshold=(-n)%n;do{x=next(s);}while(x<threshold);return x%n;}

/* Exact capacity-DP counts; uniform capacity assignments, ignoring action
 * likelihood/bidding. Returns CURRENT masks, not future training examples. */
int sample(uint32_t own,const int *unseen,int n,const int *allowed,const int *others,const int *need,int w,uint64_t seed,uint32_t *out){
 uint64_t dp[22][8][8][8];memset(dp,0,sizeof(dp));dp[n][0][0][0]=1;
 for(int i=n-1;i>=0;i--)for(int a=0;a<=need[0];a++)for(int b=0;b<=need[1];b++)for(int c=0;c<=need[2];c++){
  uint64_t v=0;if(a&&(allowed[i]&1))v+=dp[i+1][a-1][b][c];if(b&&(allowed[i]&2))v+=dp[i+1][a][b-1][c];if(c&&(allowed[i]&4))v+=dp[i+1][a][b][c-1];dp[i][a][b][c]=v;
 }
 if(!dp[0][need[0]][need[1]][need[2]])return 0;
 for(int j=0;j<w;j++){int cap[3]={need[0],need[1],need[2]};for(int s=0;s<4;s++)out[4*j+s]=s==others[3]?own:0;
  for(int i=0;i<n;i++){uint64_t r=below(&seed,dp[i][cap[0]][cap[1]][cap[2]]);for(int q=0;q<3;q++)if(cap[q]&&(allowed[i]&(1<<q))){cap[q]--;uint64_t v=dp[i+1][cap[0]][cap[1]][cap[2]];if(r<v){out[4*j+others[q]]|=1u<<unseen[i];break;}r-=v;cap[q]++;}}
 }
 return w;
}

/* Raw information encoding identical to Python: 28 categorical tile states
 * (current own / unseen / played-relative-seat-and-trick), 22 public scalars.
 * Dense model 862 -> H -> 28. Only legal argmax, ascending exact ties. */
static int netchoice(uint32_t own,int actor,int d,int bidder,int leader,int npl,const int *pts,const int *actors,const int *tiles,int plen,uint32_t L,int H,const float *weights){
 float acc[128];const float *w1=weights,*b1=w1+862*H,*w2=b1+H,*b2=w2+28*H;
 int st[28];for(int t=0;t<28;t++)st[t]=(own>>t&1)?0:1;
 for(int p=0;p<plen;p++)st[tiles[p]]=2+((actors[p]-actor+4)%4)*7+p/4;
 float ctx[22]={0};ctx[d]=1;ctx[7+(bidder-actor+4)%4]=1;ctx[11+(leader-actor+4)%4]=1;ctx[15+npl]=1;ctx[19]=pts[actor%2]/42.f;ctx[20]=pts[1-actor%2]/42.f;ctx[21]=30.f/42.f;
 for(int h=0;h<H;h++){float v=b1[h];for(int t=0;t<28;t++)v+=w1[(t*30+st[t])*H+h];for(int k=0;k<22;k++)v+=ctx[k]*w1[(840+k)*H+h];acc[h]=v>0?v:0;}
 int choice=-1;float best=-INFINITY;for(int t=0;t<28;t++)if(L>>t&1){float v=b2[t];for(int h=0;h<H;h++)v+=acc[h]*w2[t*H+h];if(v>best){best=v;choice=t;}}return choice;
}

/* outcomes is W x A Bernoulli payoff for root actor's team; same tape at the
 * same absolute future ply for every candidate; fresh tape per sampled world.
 * net used for ALL future actors when H>0, otherwise uniform legal ALL actors. */
void rollout(const uint32_t *worlds,int W,const int *actions,int A,int d,int bidder,int root,const int *public_actors,const int *public_tiles,int plen,int initial_leader,const int *initial_pts,const double *tapes,int H,const float *weights,uint8_t *outcomes){
 tables();for(int w=0;w<W;w++)for(int a=0;a<A;a++){
  uint32_t hands[4];memcpy(hands,worlds+4*w,sizeof(hands));int actors[28],tiles[28];memcpy(actors,public_actors,plen*sizeof(int));memcpy(tiles,public_tiles,plen*sizeof(int));
  int leader=initial_leader,pts[2]={initial_pts[0],initial_pts[1]},table[4],npl=plen%4;
  for(int k=0;k<npl;k++)table[k]=public_tiles[plen-npl+k];
  for(int p=plen;p<28;p++){
   int actor=(leader+npl)%4,led=npl?(trump(table[0],d)?7:hi[table[0]]):-1;uint32_t L=legal(hands[actor],led,d);
   int t=p==plen?actions[a]:H?netchoice(hands[actor],actor,d,bidder,leader,npl,pts,actors,tiles,p,L,H,weights):kth(L,(int)(tapes[w*28+p]*__builtin_popcount(L)));
   hands[actor]&=~(1u<<t);actors[p]=actor;tiles[p]=t;table[npl++]=t;
   if(npl==4){int q=trump(table[0],d)?7:hi[table[0]],b=0,v=-1,c=1;for(int k=0;k<4;k++){int s=strength(table[k],q,d);if(s>v){v=s;b=k;}c+=count[table[k]];}leader=(leader+b)%4;pts[leader%2]+=c;npl=0;}
   if(pts[bidder%2]>=30||pts[1-bidder%2]>=13)break;
  }
  int made=pts[bidder%2]>=30;outcomes[w*A+a]=(root%2==bidder%2)?made:!made;
 }
}
