// Exploratory pure information-state policy ladder. No fastmath.
#include <array>
#include <vector>
#include <unordered_map>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <limits>
#include <stdexcept>
#include <algorithm>
#include <string>
#include <functional>

// Canonical allocation patterns contain no tile identities or private facts.
// Thread-local reuse is safe across games, actors, rungs and declarations.
struct PatternCache {
    std::unordered_map<uint32_t,std::vector<uint16_t>> data;
    uint64_t hits=0,misses=0;
};
static thread_local PatternCache pattern_cache;

using Mask=uint32_t;
using World=std::array<Mask,4>;
static uint64_t mix(uint64_t z) {
    z=(z^(z>>30))*0xbf58476d1ce4e5b9ULL;
    z=(z^(z>>27))*0x94d049bb133111ebULL;
    return z^(z>>31);
}
struct Random {
    uint64_t state;
    uint64_t next() { state+=0x9e3779b97f4a7c15ULL; return mix(state); }
    uint64_t bounded(uint64_t n) {
        uint64_t threshold=uint64_t(-n)%n;
        for (;;) { uint64_t x=next(); if (x>=threshold) return x%n; }
    }
    double uniform() { return (next()>>11)*0x1.0p-53; }
};
struct Pub {
    Mask played=0;
    int leader=1,len=0,t1=0,t0=0,depth=0;
    std::array<int,4> trick{};
    std::array<Mask,4> voids{};
    std::array<uint64_t,4> history{};
    int turn() const { return (leader+len)%4; }
    bool terminal() const { return t1>=30 || t0>12; }
};
struct Rules {
    std::array<Mask,8> suit{};
    std::array<int,28> lead{},points{};
    std::array<std::array<int,28>,8> strength{};
    explicit Rules(int trump) {
        int h[28],l[28],at=0;Mask trumps=0;
        for(int a=0;a<7;++a) for(int b=0;b<=a;++b) {
            h[at]=a;l[at]=b;
            if(a==trump || b==trump) trumps|=Mask(1)<<at;
            ++at;
        }
        suit[7]=trumps;
        for(int q=0;q<7;++q) for(int t=0;t<28;++t)
            if((h[t]==q || l[t]==q) && !(trumps&(Mask(1)<<t))) suit[q]|=Mask(1)<<t;
        for(int t=0;t<28;++t) {
            lead[t]=(trumps&(Mask(1)<<t))?7:h[t];
            points[t]=(h[t]+l[t]==5 || h[t]+l[t]==10)?h[t]+l[t]:0;
            for(int q=0;q<8;++q)
                strength[q][t]=20*((trumps&(Mask(1)<<t))?2:((suit[q]&(Mask(1)<<t))?1:0))+(h[t]==l[t]?12:h[t]+l[t]);
        }
    }
    Mask legal(Mask hand,const Pub& p) const {
        if(p.len) { Mask follow=hand&suit[lead[p.trick[0]]];if(follow) return follow; }
        return hand;
    }
    Pub play(const Pub& p,int tile) const {
        Pub q=p;int actor=p.turn();
        if(p.depth>=28) throw std::runtime_error("play after tile 28");
        if(p.len && !(suit[lead[p.trick[0]]]&(Mask(1)<<tile))) q.voids[actor]|=suit[lead[p.trick[0]]];
        q.history[p.depth/9]|=uint64_t(actor*32+tile)<<(7*(p.depth%9));
        q.depth++;q.played|=Mask(1)<<tile;q.trick[q.len++]=tile;
        if(q.len==4) {
            int best=-1,winner=0,won=1,s=lead[q.trick[0]];
            for(int i=0;i<4;++i) {
                int t=q.trick[i];won+=points[t];
                if(strength[s][t]>best) { best=strength[s][t];winner=(q.leader+i)%4; }
            }
            if(winner%2) q.t1+=won;else q.t0+=won;
            q.leader=winner;q.len=0;q.trick.fill(0);
        }
        return q;
    }
};
using Key=std::array<uint64_t,13>;
struct KeyHash {
    size_t operator()(const Key& key) const {
        uint64_t h=0x243f6a8885a308d3ULL;
        for(auto word:key) h=mix(h^word);
        return h;
    }
};
static Key info_key(int level,const Pub& p,Mask hand) {
    Key k{};k[0]=hand;k[1]=p.played;
    k[2]=p.leader|(uint64_t(p.len)<<3)|(uint64_t(p.depth)<<6)|(uint64_t(p.t1)<<12)|(uint64_t(p.t0)<<18);
    for(int i=0;i<4;++i) { k[3]|=uint64_t(p.trick[i])<<(5*i);k[4+i]=p.voids[i];k[8+i]=p.history[i]; }
    k[12]=level;return k;
}
static Key support_key(const Pub& p,Mask hand) {
    Key k{};k[0]=hand;k[1]=p.played;
    k[2]=p.leader|(uint64_t(p.len)<<3)|(uint64_t(p.depth)<<6);
    for(int i=0;i<4;++i) k[4+i]=p.voids[i];
    return k;
}
struct Support {
    int count=0,actor=0;
    std::array<int,21> tiles{},allowed{};
    std::array<int,3> seats{},caps{};
    uint64_t dp[22][8][8];
    const std::vector<uint16_t>* patterns=nullptr;
    std::vector<uint16_t> uncached_patterns;
    Support(const Pub& p,Mask hand,bool tables=true) {
        actor=p.turn();int base=7-(p.depth-p.len)/4;
        if((hand&p.played) || (hand&p.voids[actor]) || __builtin_popcount(hand)!=base)
            throw std::runtime_error("own hand contradicts public state");
        int at=0;
        for(int s=0;s<4;++s) if(s!=actor) {
            seats[at]=s;caps[at]=base;
            for(int i=0;i<p.len;++i) if((p.leader+i)%4==s) caps[at]--;
            ++at;
        }
        for(int t=0;t<28;++t) if(!((p.played|hand)&(Mask(1)<<t))) {
            if(count>=21) throw std::runtime_error("invalid unseen count");
            tiles[count]=t;
            for(int j=0;j<3;++j) if(!(p.voids[seats[j]]&(Mask(1)<<t))) allowed[count]|=1<<j;
            ++count;
        }
        if(count!=caps[0]+caps[1]+caps[2]) throw std::runtime_error("invalid public capacity");
        if(tables && count<=6 && *std::max_element(caps.begin(),caps.end())<=2) {
            uint32_t key=(caps[0]|(caps[1]<<2)|(caps[2]<<4))<<18;
            for(int i=0;i<count;++i)key|=uint32_t(allowed[i])<<(3*i);
            auto found=pattern_cache.data.find(key);
            if(found!=pattern_cache.data.end()) {patterns=&found->second;pattern_cache.hits++;}
            else {
                pattern_cache.misses++;
                std::vector<uint16_t> valid;
                std::function<void(int,int,int,int,uint16_t)> walk=[&](int i,int a,int b,int c,uint16_t code) {
                    if(i==count){valid.push_back(code);return;}
                    if(a && (allowed[i]&1))walk(i+1,a-1,b,c,code);
                    if(b && (allowed[i]&2))walk(i+1,a,b-1,c,code|uint16_t(1<<(2*i)));
                    if(c && (allowed[i]&4))walk(i+1,a,b,c-1,code|uint16_t(2<<(2*i)));
                };
                walk(0,caps[0],caps[1],caps[2],0);
                if(pattern_cache.data.size()<8192)patterns=&pattern_cache.data.emplace(key,std::move(valid)).first->second;
                else {uncached_patterns=std::move(valid);patterns=&uncached_patterns;}
            }
            if(patterns->empty())throw std::runtime_error("no consistent hidden deals");
            return;
        }
        std::memset(dp,0,sizeof(dp));
        dp[count][0][0]=1;
        for(int i=count-1;i>=0;--i) for(int a=0;a<=caps[0];++a) for(int b=0;b<=caps[1];++b) {
            int c=count-i-a-b;if(c<0 || c>caps[2]) continue;
            if(a && (allowed[i]&1)) dp[i][a][b]+=dp[i+1][a-1][b];
            if(b && (allowed[i]&2)) dp[i][a][b]+=dp[i+1][a][b-1];
            if(c && (allowed[i]&4)) dp[i][a][b]+=dp[i+1][a][b];
        }
        if(!total()) throw std::runtime_error("no consistent hidden deals");
    }
    uint64_t total() const { return patterns?patterns->size():dp[0][caps[0]][caps[1]]; }
    World unrank(Mask hand,uint64_t rank) const {
        if(rank>=total()) throw std::runtime_error("rank out of range");
        World w{};w[actor]=hand;int a=caps[0],b=caps[1];
        if(patterns) {
            uint16_t code=(*patterns)[rank];
            for(int i=0;i<count;++i)w[seats[(code>>(2*i))&3]]|=Mask(1)<<tiles[i];
            return w;
        }
        for(int i=0;i<count;++i) {
            uint64_t wa=(a && (allowed[i]&1))?dp[i+1][a-1][b]:0;
            uint64_t wb=(b && (allowed[i]&2))?dp[i+1][a][b-1]:0;
            int s;
            if(rank<wa) { s=seats[0];--a; }
            else if(rank<wa+wb) { rank-=wa;s=seats[1];--b; }
            else { rank-=wa+wb;s=seats[2]; }
            w[s]|=Mask(1)<<tiles[i];
        }
        return w;
    }
};
struct Fiber { int deal;double weight; };
struct Frame { std::array<std::vector<Fiber>,28> children;std::vector<std::pair<Mask,int>> choices;Mask used=0; };
struct Workspace { std::array<Frame,29> frames; };
struct Result { int tile=-1;bool complete=false;std::array<double,28> values;Result(){values.fill(std::numeric_limits<double>::quiet_NaN());} };
struct Stats { uint64_t queries=0,hits=0,forced=0,nodes=0,fibers=0,peak=0,samples=0,sample_hits=0; };
// Once a public branch carries exactly one sampled world, that branch has no
// remaining sampled uncertainty. Collapse its storage, not its information
// partition. This shortcut is ONLY for the memoryless L0 model under L1.
struct Lite { Mask played;uint8_t leader,len,t1,t0,suit,rank,winner,points; };
struct Context {
    Rules rules;std::vector<int> samples;double delta;uint64_t seed;
    bool reuse,cache;size_t cache_limit,fiber_cap;int trump;
    bool indexed=false,pruning=true,tables=true,node_cache=true;
    uint64_t base_seed=0,pruned=0,action_cuts=0,pattern_hits_start=0,pattern_misses_start=0;
    uint64_t deduplicated=0;
    std::vector<uint64_t> deal_salts,roll_salts;
    std::vector<Workspace> spaces;
    std::vector<Stats> stats;
    std::unordered_map<Key,Result,KeyHash> policies;
    std::unordered_map<Key,std::vector<World>,KeyHash> sampled;
    std::chrono::steady_clock::time_point deadline;
    uint64_t ticks=0;std::string error;
    Context(int tr,const int* ns,int levels,double d,uint64_t sd,bool reuse_,bool cache_,size_t cap,size_t fiber,double seconds):
        rules(tr),samples(ns,ns+levels),delta(d),seed(sd),reuse(reuse_),cache(cache_),cache_limit(cap),fiber_cap(fiber),trump(tr),spaces(levels),stats(levels),
        deadline(std::chrono::steady_clock::now()+std::chrono::milliseconds(int64_t(seconds*1000))) {
        uint64_t bits;std::memcpy(&bits,&delta,8);
        base_seed=mix(seed)^mix(uint64_t(trump))^mix(bits);
        uint64_t ds=0x73616d706c652d31ULL,rs=0x726f6c6c6f757431ULL;
        for(int n:samples){ds=mix(ds^uint64_t(n));rs=mix(rs^uint64_t(n));deal_salts.push_back(ds);roll_salts.push_back(rs);}
        pattern_hits_start=pattern_cache.hits;pattern_misses_start=pattern_cache.misses;
    }
    void check() { if(std::chrono::steady_clock::now()>deadline) throw std::runtime_error("whole-query/game deadline"); }
    uint64_t salted(const Key& key,uint64_t domain) const {
        return mix(KeyHash{}(key)^base_seed^domain);
    }
    std::vector<World> deals(int level,const Pub& p,Mask hand,const Key& ik) {
        int n=samples[level-1];Key sk=reuse?support_key(p,hand):ik;
        uint64_t salt=reuse?0x73616d706c652d31ULL:deal_salts[level-1];
        if(reuse && cache) {
            auto it=sampled.find(sk);
            if(it!=sampled.end() && int(it->second.size())>=n) {
                stats[level-1].sample_hits++;return std::vector<World>(it->second.begin(),it->second.begin()+n);
            }
        }
        Support support(p,hand,tables);Random rng{salted(sk,salt)};
        std::vector<World> worlds;worlds.reserve(n);
        for(int i=0;i<n;++i) worlds.push_back(support.unrank(hand,rng.bounded(support.total())));
        stats[level-1].samples+=n;
        if(reuse && cache) {
            auto it=sampled.find(sk);
            if(it!=sampled.end()) it->second=worlds;
            else if(sampled.size()<cache_limit) sampled.emplace(sk,worlds);
        }
        return worlds;
    }
    Result action(int level,const Pub& p,Mask hand,bool values=false) {
        check();if(level<1 || level>int(samples.size()) || p.terminal()) throw std::runtime_error("invalid policy query");
        if(p.depth<0 || p.depth>27 || p.len!=p.depth%4 || p.leader<0 || p.leader>3 ||
           __builtin_popcount(p.played)!=p.depth || (hand&~Mask(0x0fffffff)) ||
           (hand&p.played) || (hand&p.voids[p.turn()]) || __builtin_popcount(hand)!=7-p.depth/4)
            throw std::runtime_error("own hand contradicts public state");
        auto& st=stats[level-1];st.queries++;
        Mask legal=rules.legal(hand,p);
        if(!legal) throw std::runtime_error("empty policy hand");
        if(!values && !(legal&(legal-1))) { st.forced++;Result r;r.tile=__builtin_ctz(legal);return r; }
        Key key=info_key(level,p,hand);
        if(cache) { auto it=policies.find(key);if(it!=policies.end() && (!values || it->second.complete)) {st.hits++;return it->second;} }
        auto worlds=deals(level,p,hand,key);
        Random rng{salted(key,roll_salts[level-1])};
        std::vector<Fiber> fibers;fibers.reserve(worlds.size());
        for(int i=0;i<int(worlds.size());++i) fibers.push_back({i,1.0});
        Result r;search(level,p,p.turn(),worlds,fibers,rng,&r,values,rng.state);
        double best=p.turn()%2?-std::numeric_limits<double>::infinity():std::numeric_limits<double>::infinity();
        while(legal) {int t=__builtin_ctz(legal);legal&=legal-1;
            if(std::isnan(r.values[t]))continue;
            if(r.tile<0 || (p.turn()%2?r.values[t]>best:r.values[t]<best)) {r.tile=t;best=r.values[t];}}
        if(cache) {auto it=policies.find(key);if(it!=policies.end())it->second=r;else if(policies.size()<cache_limit)policies.emplace(key,r);}
        check();return r;
    }
    template<bool Indexed>
    double single(Lite p,int me,const World& world,Random& rng,uint64_t path,int deal) {
        if((++ticks&1023)==0) check();
        auto& st=stats[0];st.nodes++;st.fibers++;
        if(p.t1>=30 || p.t0>12) return p.t1>=30?1.0:0.0;
        int actor=(p.leader+p.len)%4;
        Mask moves=world[actor]&~p.played;
        if(p.len) {Mask follow=moves&rules.suit[p.suit];if(follow) moves=follow;}
        if(actor!=me) {
            int count=__builtin_popcount(moves);
            if(count>1) {
                uint64_t choice;
                if constexpr(Indexed){Random local{mix(path^mix(uint64_t(deal)+0xd1b54a32d192ed03ULL))};choice=local.bounded(count);}
                else choice=rng.bounded(count);
                int skip=int(choice);while(skip--)moves&=moves-1;moves&=-moves;
            }
        }
        double value=me%2?-1.0:2.0;
        while(moves) {
            int t=__builtin_ctz(moves);moves&=moves-1;Lite q=p;
            q.played|=Mask(1)<<t;
            if(!q.len) {q.suit=rules.lead[t];q.rank=rules.strength[q.suit][t];q.winner=actor;q.points=1+rules.points[t];}
            else {q.points+=rules.points[t];if(rules.strength[q.suit][t]>q.rank){q.rank=rules.strength[q.suit][t];q.winner=actor;}}
            q.len++;
            if(q.len==4) {
                if(q.winner%2)q.t1+=q.points;else q.t0+=q.points;
                q.leader=q.winner;q.len=0;
            }
            uint64_t child=Indexed?mix(path^(uint64_t(t+1)*0x9e3779b97f4a7c15ULL)):0;
            double v=single<Indexed>(q,me,world,rng,child,deal);
            value=me%2?std::max(value,v):std::min(value,v);
            if constexpr(Indexed) {
                if(pruning && actor==me && moves && value==(me%2?1.0:0.0)){pruned++;break;}
            }
        }
        return value;
    }
    double search(int level,const Pub& p,int me,const std::vector<World>& worlds,const std::vector<Fiber>& fibers,Random& rng,Result* root=nullptr,bool all_values=false,uint64_t path=0,double bound=std::numeric_limits<double>::quiet_NaN(),bool* exact=nullptr) {
        if(exact)*exact=true;
        const bool bounds=indexed && pruning && delta==1.0;
        const bool constrained=bounds && std::isfinite(bound);
        // A bounded call asks whether this shared information state can beat
        // an incumbent. A failed comparison returns a bound, not an exact value.
        if(constrained && (me%2?double(fibers.size())<=bound:0.0>=bound)) {
            pruned++;if(exact)*exact=false;return bound;
        }
        if(level==1 && delta==1.0 && fibers.size()==1 && !root) {
            Lite lite{p.played,uint8_t(p.leader),uint8_t(p.len),uint8_t(p.t1),uint8_t(p.t0),0,0,0,1};
            if(p.len)lite.suit=rules.lead[p.trick[0]];
            for(int i=0;i<p.len;++i){int t=p.trick[i];lite.points+=rules.points[t];if(i==0 || rules.strength[lite.suit][t]>lite.rank){lite.rank=rules.strength[lite.suit][t];lite.winner=(p.leader+i)%4;}}
            return fibers[0].weight*(indexed?single<true>(lite,me,worlds[fibers[0].deal],rng,path,fibers[0].deal):single<false>(lite,me,worlds[fibers[0].deal],rng,0,fibers[0].deal));
        }
        if((++ticks&1023)==0) check();
        auto& st=stats[level-1];st.nodes++;st.fibers+=fibers.size();st.peak=std::max(st.peak,uint64_t(fibers.size()));
        if(fibers.size()>fiber_cap) throw std::runtime_error("node fiber cap");
        if(p.terminal()) {double value=0;if(p.t1>=30) for(auto f:fibers) value+=f.weight;return value;}
        Frame& frame=spaces[level-1].frames[p.depth];
        frame.choices.clear();
        Mask old=frame.used;while(old){int t=__builtin_ctz(old);old&=old-1;frame.children[t].clear();}frame.used=0;
        int actor=p.turn();bool mine=actor==me;
        for(auto f:fibers) {
            Mask hand=worlds[f.deal][actor]&~p.played;
            Mask legal=rules.legal(hand,p);if(!legal) throw std::runtime_error("empty live fiber");
            int count=__builtin_popcount(legal);double weight=f.weight;
            if(!mine) {
                if(level>1) {
                    if(count==1){stats[level-2].queries++;stats[level-2].forced++;}
                    else {
                        int chosen=-1;
                        if(node_cache && fibers.size()>1)for(auto entry:frame.choices)if(entry.first==hand){chosen=entry.second;deduplicated++;break;}
                        if(chosen<0){chosen=action(level-1,p,hand).tile;if(node_cache && fibers.size()>1)frame.choices.emplace_back(hand,chosen);}
                        legal=Mask(1)<<chosen;
                    }
                }
                else if(weight/count<delta) {
                    uint64_t choice;
                    if(indexed){Random local{mix(path^mix(uint64_t(f.deal)+0xd1b54a32d192ed03ULL))};choice=local.bounded(count);}
                    else choice=rng.bounded(count);
                    int skip=int(choice);while(skip--) legal&=legal-1;
                    legal&=-legal;
                } else weight/=count;
            }
            frame.used|=legal;
            while(legal) {int t=__builtin_ctz(legal);legal&=legal-1;frame.children[t].push_back({f.deal,weight});}
        }
        double value=mine?(me%2?-std::numeric_limits<double>::infinity():std::numeric_limits<double>::infinity()):0;
        Mask remaining=frame.used;
        double remaining_mass=double(fibers.size());bool root_complete=true;
        while(remaining) {
            int t=__builtin_ctz(remaining);remaining&=remaining-1;
            uint64_t child=indexed?mix(path^(uint64_t(t+1)*0x9e3779b97f4a7c15ULL)):0;
            remaining_mass-=double(frame.children[t].size());
            double child_bound=std::numeric_limits<double>::quiet_NaN();
            if(bounds && !(root && all_values)) {
                if(mine)child_bound=constrained?(me%2?std::max(bound,value):std::min(bound,value)):value;
                else if(constrained)child_bound=bound-value-(me%2?remaining_mass:0.0);
            }
            bool child_exact=true;
            double v=search(level,rules.play(p,t),me,worlds,frame.children[t],rng,nullptr,false,child,child_bound,&child_exact);
            if(root && child_exact)root->values[t]=v;
            if(!child_exact) {
                root_complete=false;
                if(!mine){if(exact)*exact=false;pruned++;return bound;}
            } else if(mine) value=me%2?std::max(value,v):std::min(value,v);
            else value+=v;
            // At delta=1 every fiber has unit weight. These are exact 0/N
            // bounds on the SHARED information-state choice, never per world.
            if(indexed && pruning && delta==1.0 && mine && remaining && !(root && all_values) &&
               value==(me%2?double(fibers.size()):0.0)) {
                pruned++;if(root)action_cuts++;break;
            }
        }
        if(root)root->complete=remaining==0 && root_complete;
        if(constrained && mine && (me%2?value<=bound:value>=bound)){
            if(exact)*exact=false;return bound;
        }
        return value;
    }
};
static Pub unpack(const uint64_t* a) {
    Pub p;p.played=a[0];p.leader=a[1];p.len=a[2];p.t1=a[3];p.t0=a[4];p.depth=a[5];
    for(int i=0;i<4;++i){p.trick[i]=a[6+i];p.voids[i]=a[10+i];p.history[i]=a[14+i];}return p;
}
extern "C" {
void* walt_create(int trump,const int* ns,int levels,double delta,uint64_t seed,int reuse,int cache,uint64_t cache_cap,uint64_t fiber_cap,double seconds) {
    if(trump<0 || trump>6 || levels<1 || levels>8 || delta<0 || delta>1 || !std::isfinite(delta) || seconds<0) return nullptr;
    for(int i=0;i<levels;++i) if(ns[i]<1 || ns[i]>1000000) return nullptr;
    try {return new Context(trump,ns,levels,delta,seed,reuse,cache,cache_cap,fiber_cap,seconds);}catch(...){return nullptr;}
}
void walt_destroy(void* c){delete static_cast<Context*>(c);}
int walt_configure(void* handle,int indexed,int pruning,int tables,int node_cache){
    auto& c=*static_cast<Context*>(handle);
    for(auto s:c.stats)if(s.queries)return -1;
    c.indexed=indexed;c.pruning=pruning;c.tables=tables;c.node_cache=node_cache;return 0;
}
void walt_metrics(void* handle,uint64_t* out){
    auto& c=*static_cast<Context*>(handle);
    out[0]=c.pruned;out[1]=c.action_cuts;
    out[2]=pattern_cache.hits-c.pattern_hits_start;out[3]=pattern_cache.misses-c.pattern_misses_start;
    out[4]=pattern_cache.data.size();
    out[5]=c.deduplicated;
}
int walt_action(void* handle,int level,const uint64_t* a,uint32_t hand,int values,double* scores) {
    auto& c=*static_cast<Context*>(handle);
    try {auto r=c.action(level,unpack(a),hand,values);std::copy(r.values.begin(),r.values.end(),scores);return r.tile;}
    catch(const std::exception& e){c.error=e.what();return -1;}
}
const char* walt_error(void* c){return static_cast<Context*>(c)->error.c_str();}
void walt_stats(void* handle,uint64_t* out) {
    auto& c=*static_cast<Context*>(handle);int at=0;
    for(auto s:c.stats) for(auto n:{s.queries,s.hits,s.forced,s.nodes,s.fibers,s.peak,s.samples,s.sample_hits}) out[at++]=n;
}
int walt_sample(void* handle,int level,const uint64_t* a,uint32_t hand,uint32_t* out) {
    auto& c=*static_cast<Context*>(handle);
    try {auto p=unpack(a);auto worlds=c.deals(level,p,hand,info_key(level,p,hand));int at=0;for(auto w:worlds)for(auto h:w)out[at++]=h;return int(worlds.size());}
    catch(const std::exception& e){c.error=e.what();return -1;}
}
int walt_rank(const uint64_t* a,uint32_t hand,uint64_t rank,uint32_t* out,uint64_t* total) {
    try {Support support(unpack(a),hand);*total=support.total();auto w=support.unrank(hand,rank);std::copy(w.begin(),w.end(),out);return 0;}catch(...){return -1;}
}
// Test-only fixed-fiber evaluation: checks the focal hand is identical in all
// worlds. Production walt_action never receives an enclosing or physical deal.
int walt_evaluate_l1(void* handle,const uint64_t* a,const uint32_t* data,int n,uint64_t random_seed,double* out) {
    auto& c=*static_cast<Context*>(handle);
    try {
        Pub p=unpack(a);std::vector<World> worlds(n);std::vector<Fiber> fibers;
        for(int i=0;i<n;++i){std::copy(data+4*i,data+4*i+4,worlds[i].begin());if(i && worlds[i][p.turn()]!=worlds[0][p.turn()])throw std::runtime_error("mixed focal hands");fibers.push_back({i,1.0});}
        Result result;Random rng{random_seed};c.search(1,p,p.turn(),worlds,fibers,rng,&result,true,random_seed);
        std::copy(result.values.begin(),result.values.end(),out);return 0;
    }catch(const std::exception& e){c.error=e.what();return -1;}
}
int walt_play(int trump,const uint64_t* a,int tile,uint64_t* out) {
    try {Pub p=Rules(trump).play(unpack(a),tile);out[0]=p.played;out[1]=p.leader;out[2]=p.len;out[3]=p.t1;out[4]=p.t0;out[5]=p.depth;for(int i=0;i<4;++i){out[6+i]=p.trick[i];out[10+i]=p.voids[i];out[14+i]=p.history[i];}return 0;}catch(...){return -1;}
}
}
