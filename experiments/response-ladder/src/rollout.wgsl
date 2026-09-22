// Fixed priority policy x original scenario. One physical continuation per lane.
// No DFS stack, policy search, weighting, grouping, or maximum in this kernel.
override TRACE: bool = false;
struct State { played:u32, plays:u32, len:u32, leader:u32, t1:u32, t0:u32 }
@group(0) @binding(0) var<storage,read> packet:array<u32>;
@group(0) @binding(1) var<storage,read> rules:array<u32>;
@group(0) @binding(2) var<storage,read_write> results:array<u32>;
// Retained, directed-vector-checked two-limb arithmetic from gpu-tail/tail.wgsl.
// In particular, rem64 uses four base-65536 Horner digits after an earlier
// modulo formulation failed a directed rejection vector on this Metal host.
fn add64(a:vec2<u32>,b:vec2<u32>)->vec2<u32>{let lo=a.x+b.x;return vec2<u32>(lo,a.y+b.y+select(0u,1u,lo<a.x));}
fn shr64(a:vec2<u32>,s:u32)->vec2<u32>{return vec2<u32>((a.x>>s)|(a.y<<(32u-s)),a.y>>s);}
fn mulhi(a:u32,b:u32)->u32{
    let a0=a&65535u;let a1=a>>16u;let b0=b&65535u;let b1=b>>16u;
    let w0=a0*b0;let t=a1*b0+(w0>>16u);let w1=a0*b1+(t&65535u);
    return a1*b1+(t>>16u)+(w1>>16u);
}
fn mul64(a:vec2<u32>,b:vec2<u32>)->vec2<u32>{return vec2<u32>(a.x*b.x,mulhi(a.x,b.x)+a.x*b.y+a.y*b.x);}
fn mix64(v:vec2<u32>)->vec2<u32>{
    var z=add64(v,vec2<u32>(0x7f4a7c15u,0x9e3779b9u));
    z=mul64(z^shr64(z,30u),vec2<u32>(0x1ce4e5b9u,0xbf58476du));
    z=mul64(z^shr64(z,27u),vec2<u32>(0x133111ebu,0x94d049bbu));return z^shr64(z,31u);
}
fn rem64(v:vec2<u32>,n:u32)->u32{
    if n==1u || n==2u || n==4u { return v.x & (n-1u); }
    var r=(v.y>>16u)%n;
    r=(r*65536u+(v.y&65535u))%n;
    r=(r*65536u+(v.x>>16u))%n;
    return (r*65536u+(v.x&65535u))%n;
}
fn below(s0:vec2<u32>,n:u32)->u32{
    let remainder=rem64(vec2<u32>(0xffffffffu),n);
    let zone=vec2<u32>(0xffffffffu-remainder,0xffffffffu);
    var s=s0;
    loop {
        let v=mix64(s);s=add64(s,vec2<u32>(0x7f4a7c15u,0x9e3779b9u));
        if v.y<zone.y || (v.y==zone.y && v.x<zone.x) { return rem64(v,n); }
    }
    return 0u; // Unreachable for the finite full-period tape; satisfies WGSL return analysis.
}
fn record_hash(v:State)->vec2<u32>{
    var h=mix64(vec2<u32>(v.played,0u));h=mix64(h^vec2<u32>(0u,v.leader));
    for(var p=0u;p<v.len;p++){h=mix64(h^vec2<u32>(0x100u|((v.plays>>(5u*p))&31u),0u));}
    return h;
}

fn legal(di:u32, hand:u32, s:State)->u32 {
    if s.len==0u {return hand;}
    let q=rules[di*260u+(s.plays&31u)];
    let follows=hand&rules[di*260u+28u+q];
    return select(hand,follows,follows!=0u);
}
fn advance(di:u32,s:State,tile:u32)->State {
    var c=State(s.played|(1u<<tile),s.plays|(tile<<(5u*s.len)),s.len+1u,s.leader,s.t1,s.t0);
    if c.len==4u {
        let q=rules[di*260u+(c.plays&31u)];
        var best=0u;var winner=0u;var points=1u;
        for(var i=0u;i<4u;i++) {
            let t=(c.plays>>(5u*i))&31u;
            let strength=rules[di*260u+36u+q*28u+t];
            if i==0u || strength>best {best=strength;winner=i;}
            points+=rules[2340u+t];
        }
        c.leader=(s.leader+winner)&3u;
        if (c.leader&1u)==1u {c.t1+=points;} else {c.t0+=points;}
        c.plays=0u;c.len=0u;
    }
    return c;
}
@compute @workgroup_size(64)
fn run(@builtin(global_invocation_id) gid:vec3<u32>) {
    let n=packet[10u];let p=packet[11u];
    let lane=gid.x+gid.y*packet[12u];
    if lane>=n*p {return;}
    let sid=lane%n;let plan=lane/n;
    let scenario_base=16u+sid*6u;
    let plan_base=16u+n*6u+plan*8u;
    let di=packet[0u];let bid=packet[1u];let viewer=packet[2u];let k=packet[3u];
    let tape=vec2<u32>(packet[scenario_base+4u],packet[scenario_base+5u]);
    let out=lane*select(1u,32u,TRACE);
    results[out]=0xffffffffu;
    if TRACE {for(var i=1u;i<32u;i++){results[out+i]=0xffffffffu;}}
    var s=State(packet[4u],packet[5u],packet[6u],packet[7u],packet[8u],packet[9u]);
    for(var step=0u;step<=28u;step++) {
        if s.t1>=bid || s.t0>42u-bid {
            results[out]=select(0u,1u,(s.t1>=bid)==((viewer&1u)==1u));
            if TRACE {results[out+1u]=step;}
            return;
        }
        if step==28u {return;}
        let seat=(s.leader+s.len)&3u;
        var mask=legal(di,packet[scenario_base+seat]&~s.played,s);
        if mask==0u {return;}
        var tile=0xffffffffu;
        if seat==viewer {
            for(var i=0u;i<k;i++) {
                let candidate=packet[plan_base+i];
                if (mask&(1u<<candidate))!=0u {tile=candidate;break;}
            }
        } else {
            let choices=countOneBits(mask);
            if choices>1u {
                let skip=below(tape^record_hash(s),choices);
                for(var i=0u;i<skip;i++) {mask&=mask-1u;}
            }
            tile=firstTrailingBit(mask);
        }
        if tile>=28u {return;}
        if TRACE {results[out+2u+step]=tile;}
        s=advance(di,s,tile);
    }
}
// Eight input words and eight output words per directed arithmetic vector.
@compute @workgroup_size(64)
fn rng_check(@builtin(global_invocation_id) gid:vec3<u32>) {
    if gid.x>=arrayLength(&packet)/8u {return;}
    let b=gid.x*8u;let tape=vec2<u32>(packet[b],packet[b+1u]);
    let mixed=mix64(tape);
    let s=State(packet[b+3u],packet[b+4u],packet[b+5u],packet[b+6u],0u,0u);
    let rh=record_hash(s);
    results[b]=mixed.x;results[b+1u]=mixed.y;
    results[b+2u]=below(tape,packet[b+2u]);
    results[b+3u]=rh.x;results[b+4u]=rh.y;
}
