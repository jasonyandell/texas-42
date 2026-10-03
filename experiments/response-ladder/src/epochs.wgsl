// Resumable fixed lawful policy x original scenario. CPU supplies only exact
// modeled field decisions; the GPU owns physical evolution and payoff lanes.
struct State { played:u32, plays:u32, len:u32, leader:u32, t1:u32, t0:u32 }
@group(0) @binding(0) var<storage,read> packet:array<u32>;
@group(0) @binding(1) var<storage,read> rules:array<u32>;
@group(0) @binding(2) var<storage,read_write> lanes:array<u32>;
@group(0) @binding(3) var<storage,read> answers:array<u32>;
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

fn policy_action(base:u32, trace:u32, count:u32, mask:u32, di:u32, viewer:u32, s:State)->u32 {
    let decisions=packet[base+8u];
    for(var d=0u;d<packet[base+9u];d++) {
        let at=decisions+d*32u;
        if packet[at]!=count {continue;}
        var equal=true;
        for(var i=0u;i<count;i++){if packet[at+2u+i]!=lanes[trace+i] {equal=false;break;}}
        let action=packet[at+1u];
        if equal && action<28u && (mask&(1u<<action))!=0u {return action;}
    }
    if packet[base+10u]!=0u {
        return compiled_action(di,packet[base+10u],viewer,s,mask);
    }
    for(var i=0u;i<packet[base];i++) {
        let tile=packet[base+1u+i];
        if (mask&(1u<<tile))!=0u {return tile;}
    }
    return firstTrailingBit(mask);
}

// The packet's optional compiled block stores four actors, each with leading
// and following [length,c0,c1,c2] records, then 28 THREAT and 8*28 BEATS masks.
// These masks are
// derived by the host from the frozen rule algebra; the shader only performs
// the bounded relational scan.
fn compiled_current_winner(di:u32,s:State)->u32 {
    if s.len==0u {return 0xffffffffu;}
    let q=rules[di*260u+(s.plays&31u)];
    var best=0u;var winner=0xffffffffu;
    for(var i=0u;i<3u;i++) {
        if i>=s.len {break;}
        let tile=(s.plays>>(5u*i))&31u;
        let strength=rules[di*260u+36u+q*28u+tile];
        if winner==0xffffffffu || strength>best {best=strength;winner=tile;}
    }
    return winner;
}

fn compiled_partner_winning(di:u32, seat:u32, s:State)->bool {
    if s.len==0u {return false;}
    let q=rules[di*260u+(s.plays&31u)];
    var best=0u;var winner_offset=0u;
    for(var i=0u;i<3u;i++) {
        if i>=s.len {break;}
        let tile=(s.plays>>(5u*i))&31u;
        let strength=rules[di*260u+36u+q*28u+tile];
        if i==0u || strength>best {best=strength;winner_offset=i;}
    }
    return ((s.leader+winner_offset)&3u)==((seat+2u)&3u);
}

fn compiled_action(di:u32, actors:u32, seat:u32, s:State, mask:u32)->u32 {
    let base=actors+seat*8u+select(4u,0u,s.len==0u);
    let live=0x0fffffffu^s.played;
    let partner=compiled_partner_winning(di,seat,s);
    var led=0xffffffffu;
    if s.len>0u {led=rules[di*260u+(s.plays&31u)];}
    for(var position=0u;position<3u;position++) {
        if position>=packet[base] {break;}
        let clause=packet[base+1u+position];
        if (clause&1u)==1u && !partner {continue;}
        let selector=clause>>1u;
        var candidates=mask;
        loop {
            if candidates==0u {break;}
            let tile=firstTrailingBit(candidates);
            candidates&=candidates-1u;
            var matches=false;
            if selector==0u {
                matches=true;
            } else if selector==1u {
                matches=rules[2340u+tile]==0u;
            } else if selector==2u {
                matches=rules[2340u+tile]==5u;
            } else if selector==3u {
                matches=rules[2340u+tile]==10u;
            } else if selector==4u {
                matches=(packet[actors+32u+tile]&live)==0u;
            } else if selector==5u {
                matches=led!=0xffffffffu && (rules[di*260u+28u+led]&(1u<<tile))!=0u;
            } else if selector==6u {
                matches=led!=0xffffffffu
                    && (rules[di*260u+28u+led]&(1u<<tile))!=0u
                    && (packet[actors+32u+28u+led*28u+tile]&live)==0u;
            } else if selector==7u {
                let winner=compiled_current_winner(di,s);
                matches=winner!=0xffffffffu && led!=0xffffffffu
                    && rules[di*260u+36u+led*28u+tile]
                        > rules[di*260u+36u+led*28u+winner];
            }
            if matches {return tile;}
        }
    }
    return firstTrailingBit(mask);
}

@compute @workgroup_size(64)
fn epoch(@builtin(global_invocation_id) gid:vec3<u32>) {
    let n=packet[10u];let p=packet[11u];let lane=gid.x+gid.y*packet[12u];
    if lane>=n*p {return;}
    let b=lane*40u;
    if packet[13u]==0u {
        lanes[b]=0u;lanes[b+1u]=packet[4u];lanes[b+2u]=packet[5u];
        lanes[b+3u]=packet[6u];lanes[b+4u]=packet[7u];
        lanes[b+5u]=packet[8u];lanes[b+6u]=packet[9u];lanes[b+7u]=0u;
        for(var i=0u;i<4u;i++){lanes[b+8u+i]=packet[16u+i];}
    }
    if lanes[b]==2u || lanes[b]==3u {return;}
    let sb=20u+(lane%n)*6u;let pb=20u+n*6u+(lane/n)*12u;
    let di=packet[0u];let bid=packet[1u];let viewer=packet[2u];
    var s=State(lanes[b+1u],lanes[b+2u],lanes[b+3u],lanes[b+4u],lanes[b+5u],lanes[b+6u]);
    var step=lanes[b+7u];
    var supplied=lanes[b]==1u;
    for(var turn=0u;turn<=28u;turn++) {
        if s.t1>=bid || s.t0>42u-bid {lanes[b]=2u;break;}
        if step>=28u {lanes[b]=3u;break;}
        let seat=(s.leader+s.len)&3u;
        let mask=legal(di,packet[sb+seat]&~s.played,s);
        if mask==0u {lanes[b]=3u;break;}
        var tile=0xffffffffu;
        if seat==viewer {tile=policy_action(pb,b+12u,step,mask,di,viewer,s);}
        else if packet[14u]!=0u {
            tile=compiled_action(di,packet[14u],seat,s,mask);
        }
        else if countOneBits(mask)==1u {tile=firstTrailingBit(mask);}
        else if supplied {tile=answers[lane];supplied=false;}
        else {lanes[b]=1u;break;}
        if tile>=28u || (mask&(1u<<tile))==0u {lanes[b]=3u;break;}
        if s.len>0u {
            let q=rules[di*260u+(s.plays&31u)];
            let suit=rules[di*260u+28u+q];
            if (suit&(1u<<tile))==0u {lanes[b+8u+seat]|=suit;}
        }
        lanes[b+12u+step]=tile;step++;
        s=advance(di,s,tile);
    }
    lanes[b+1u]=s.played;lanes[b+2u]=s.plays;lanes[b+3u]=s.len;
    lanes[b+4u]=s.leader;lanes[b+5u]=s.t1;lanes[b+6u]=s.t0;lanes[b+7u]=step;
}
