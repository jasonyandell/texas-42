// Exhaustive canonical small-support table versus integer capacity DP.
#include "turbo.cpp"
#include <iostream>
int main(){
    uint64_t cases=0,ranks=0;
    for(int len=0;len<4;++len)for(int leader=0;leader<3;++leader){
        if(len==0 && leader)continue;
        int count=6-len;uint32_t patterns=uint32_t(1)<<(3*count);
        for(uint32_t mask=0;mask<patterns;++mask){
            Pub p;p.depth=20+len;p.len=len;p.leader=leader;
            p.played=Mask(0x0fffffff)^((Mask(1)<<(count+2))-1);
            int seats[3],at=0;for(int s=0;s<4;++s)if(s!=p.turn())seats[at++]=s;
            for(int i=0;i<count;++i)for(int j=0;j<3;++j)
                if(!((mask>>(3*i))&(1<<j)))p.voids[seats[j]]|=Mask(1)<<(i+2);
            bool fast_ok=false,slow_ok=false;
            try{Support a(p,3,true);fast_ok=true;}catch(const std::runtime_error&){}
            try{Support b(p,3,false);slow_ok=true;}catch(const std::runtime_error&){}
            if(fast_ok!=slow_ok)return 1;
            if(fast_ok){
                Support a(p,3,true),b(p,3,false);
                if(a.total()!=b.total())return 2;
                for(uint64_t rank=0;rank<a.total();++rank){
                    if(a.unrank(3,rank)!=b.unrank(3,rank))return 3;
                    ranks++;
                }
            }
            cases++;
        }
    }
    std::cout<<"{\"status\":\"pass\",\"patterns\":"<<cases<<",\"ranks\":"<<ranks<<"}\n";
}
