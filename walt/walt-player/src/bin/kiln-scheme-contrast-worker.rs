//! Query supplied contrast worlds; never invoke a player or consume outcomes.
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};
use walt::rules::{Domino, DominoSet, Seat};
use walt::scheme::{Atom, Budget, CompiledFix, Fix, Term, Value};
use walt::{gym,solver};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request { decl:usize,bid:usize,bidder:usize,seat:usize,hand:Vec<usize>,plays:Vec<usize>,seed:u32 }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Row { request:Request,hands:[[usize;7];4],baseline:usize,alternative:usize }
fn tile(n:usize)->Result<Domino,String>{Domino::from_index(n).ok_or("tile outside range".into())}
fn seat(n:usize)->Result<Seat,String>{Seat::from_index(n).ok_or("chair outside range".into())}

fn run(text:&str, queries:&[Fix], cache:&mut BTreeMap<(usize,usize,usize),CompiledFix>) -> Result<serde_json::Value,String> {
    let row:Row=serde_json::from_str(text).map_err(|e|e.to_string())?;
    let r=&row.request;
    if r.bid!=30 || ![0,1,2,3,4,5,6,7,9].contains(&r.decl) || r.plays.len()%2!=0
        || r.plays.len()>52 || r.hand.len()!=7 || row.baseline==row.alternative {
        return Err("invalid contrast request".into());
    }
    let _policy_seed=r.seed; // Identity only; never an input to membership.
    let mut partition=row.hands.iter().flatten().copied().collect::<Vec<_>>();partition.sort_unstable();
    if partition!=(0..28).collect::<Vec<_>>() || row.hands[r.seat.min(3)].to_vec()!=r.hand {
        return Err("hands must partition the deck and preserve the actor hand".into());
    }
    let history=r.plays.chunks_exact(2).map(|x|Ok((seat(x[0])?,tile(x[1])?)))
        .collect::<Result<Vec<_>,String>>()?;
    let mut hands=row.hands.map(|h|h.into_iter().map(tile).collect::<Result<Vec<_>,_>>().unwrap().into_iter().collect::<DominoSet>());
    let original=hands[r.seat.min(3)];
    for &(s,t) in &history {if !hands[s.index()].remove(t) {return Err("history/holder mismatch".into());}}
    let ex=gym::from_request(solver::decl_of(r.decl),seat(r.bidder)?,seat(r.seat)?,original,&history)?;
    let kernel=ex.root.kernel();
    let world=kernel.world(std::array::from_fn(|i|hands[kernel.hidden()[i].seat.index()]));
    if !kernel.contains(&world) {return Err("incompatible supplied world".into());}
    let legal=walt::rules::legal_plays(kernel.decl(),kernel.viewer_hand(),ex.frame.led_context());
    if !legal.contains(tile(row.baseline)?) || !legal.contains(tile(row.alternative)?) {return Err("illegal bound action".into());}
    let mut memberships=Vec::new();
    for (i,query) in queries.iter().enumerate() {
        let key=(i,row.baseline,row.alternative);
        if !cache.contains_key(&key) {
            let mut bound=query.clone();
            for case in &mut bound.cases {
                for (name,n) in [("baseline",row.baseline),("alternative",row.alternative)] {
                    case.atoms.insert(0,Atom{predicate:"tile".into(),args:vec![Term::Role(name.into()),Term::Literal(Value::Domino(tile(n)?))],negated:false});
                }
            }
            cache.insert(key,bound.compile(&gym::registry()).map_err(|e|e.to_string())?);
        }
        let answers=cache[&key].evaluate(&ex.frame,&world,&mut Budget::new(2_000_000)).map_err(|e|e.to_string())?;
        for a in &answers {
            if a.0!=vec![Value::Domino(tile(row.alternative)?),Value::Domino(tile(row.baseline)?)] {
                return Err("query must output the bound alternative/baseline pair".into());
            }
        }
        memberships.push(!answers.is_empty());
    }
    Ok(json!({"memberships":memberships}))
}
fn main(){
    let queries=std::env::args().skip(1).map(|p|std::fs::read_to_string(p).expect("query file").parse::<Fix>().expect("Scheme syntax")).collect::<Vec<_>>();
    assert!(!queries.is_empty() && queries.len()<=64,"pass 1..64 query files");
    let mut cache=BTreeMap::new();
    for line in io::stdin().lock().lines(){
        let line=line.expect("contrast row");
        let result=if line.len()>100_000 {Err("row too large".into())} else {run(&line,&queries,&mut cache)};
        println!("{}",result.unwrap_or_else(|error|json!({"error":error})));
        io::stdout().flush().expect("flush memberships");
    }
}
