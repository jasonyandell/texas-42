// Memory/undefined-behavior smoke: a complete L2 game, both sampling modes.
#include "turbo.cpp"
#include <iostream>
int main() {
    for(bool addressed:{false,true})for(bool reuse:{false,true}) {
        // A fixed physical partition belongs to the referee, not the policy.
        World hands{};
        for(int t=0;t<28;++t)hands[t%4]|=Mask(1)<<t;
        int ns[]={2,2};Context field(3,ns,2,1.0,42,reuse,true,1000,100000,120);
        field.indexed=addressed;
        Pub pub;
        while(!pub.terminal()) {
            int actor=pub.turn();Mask hand=hands[actor]&~pub.played;
            Result result=field.action(2,pub,hand,true);
            if(!(field.rules.legal(hand,pub)&(Mask(1)<<result.tile)))return 1;
            pub=field.rules.play(pub,result.tile);
        }
        std::cout<<"sanitizer game complete, addressed="<<addressed<<", reuse="<<reuse<<", depth="<<pub.depth<<"\n";
    }
}
