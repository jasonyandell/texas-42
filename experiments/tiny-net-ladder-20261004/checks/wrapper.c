#include "../kernel.c"
int audit_legal(uint32_t h,int led,int d){tables();return legal(h,led,d);}
int audit_strength(int t,int led,int d){tables();return strength(t,led,d);}
int audit_net(uint32_t own,int actor,int d,int bidder,int leader,int npl,const int *pts,const int *actors,const int *tiles,int plen,uint32_t L,int H,const float *weights){return netchoice(own,actor,d,bidder,leader,npl,pts,actors,tiles,plen,L,H,weights);}
