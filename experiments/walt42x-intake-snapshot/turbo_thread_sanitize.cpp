// Separate concurrent fields share only thread-local mathematical tables.
#include "turbo.cpp"
#include <thread>
#include <atomic>
#include <iostream>
int main(){
    std::atomic<bool> ok{true};std::vector<std::thread> workers;
    for(int worker=0;worker<4;++worker)workers.emplace_back([worker,&ok]{
        for(int game=0;game<3;++game){
            World hands{};for(int t=0;t<28;++t)hands[t%4]|=Mask(1)<<t;
            int ns[]={2,2};int trump=(worker+game)%7;
            Context full(trump,ns,2,1.,42,false,false,1000,100000,120);
            Context fast(trump,ns,2,1.,42,false,true,1000,100000,120);
            full.indexed=true;full.pruning=false;full.tables=false;full.node_cache=false;
            fast.indexed=true;Pub p;
            while(!p.terminal()){
                Mask h=hands[p.turn()]&~p.played;
                auto choice=fast.action(2,p,h,false);
                auto a=full.action(2,p,h,true),b=fast.action(2,p,h,true);
                if(a.tile!=b.tile || choice.tile!=b.tile)ok=false;
                for(int t=0;t<28;++t)if(!(std::isnan(a.values[t]) && std::isnan(b.values[t])) && a.values[t]!=b.values[t])ok=false;
                p=fast.rules.play(p,b.tile);
            }
        }
    });
    for(auto& worker:workers)worker.join();
    if(!ok)return 1;
    std::cout<<"concurrent native fields and partial-cache upgrades: pass (12 games)\n";
}
