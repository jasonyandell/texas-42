//! Native experiment host. Full deals stay here; the unchanged deployed
//! decision procedure receives exactly one original hand and public history.
use serde::Deserialize;
use serde_json::{json, Value};
use walt::{
    rules::{legal_plays, Domino},
    solver,
};

pub const PROFILE: &str = "walt-table-v2-opening160-ordinary40-partner-bid30-v1";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Game {
    profile: String,
    hands: [[usize; 7]; 4],
    bidder: usize,
    decl: usize,
    seed: u32,
}

/// One complete game per request. A killed host loses only this unfinished game.
pub fn run(text: &str) -> Result<Value, String> {
    play(text, |call| {
        crate::decide(
            serde_json::from_value(call).map_err(|e| e.to_string())?,
            |_| {},
        )
    })
}

fn play(
    text: &str,
    mut choose: impl FnMut(Value) -> Result<Value, String>,
) -> Result<Value, String> {
    let start = std::time::Instant::now();
    let game: Game = serde_json::from_str(text).map_err(|e| e.to_string())?;
    if game.profile != PROFILE
        || game.bidder > 3
        || ![0, 1, 2, 3, 4, 5, 6, 7, 9].contains(&game.decl)
    {
        return Err("invalid game profile, bidder or declaration".into());
    }
    let mut tiles: Vec<_> = game.hands.iter().flatten().copied().collect();
    tiles.sort_unstable();
    if tiles != (0..28).collect::<Vec<_>>() {
        return Err("hands must partition the 28 dominoes".into());
    }
    let dcl = solver::decl_of(game.decl);
    let mut pairs = Vec::with_capacity(28);
    let mut decisions = Vec::with_capacity(28);
    for ply in 0..28 {
        let state = solver::replay(dcl, game.bidder, &pairs);
        let leader = (state.leader as usize + 4 - state.r) % 4;
        let actor = (leader + state.plays.len()) % 4;
        let points = if state.r == 0 {
            [state.banked_t0, state.banked_t1]
        } else {
            [state.banked_t1, state.banked_t0]
        };
        let mask = game.hands[actor].iter().fold(0u32, |m, &t| m | (1 << t)) & !state.played;
        let led = state
            .plays
            .first()
            .map(|&t| dcl.led_context(Domino::from_index(t as usize).unwrap()));
        let legal = solver::mask_of(legal_plays(dcl, solver::set_of(mask), led));
        let call = json!({"request":{"decl":game.decl,"bid":30,"bidder":game.bidder,
            "seat":actor,"hand":game.hands[actor],"plays":pairs.iter().flat_map(|&(a,t)| [a,t]).collect::<Vec<_>>(),
            "seed":game.seed},"worlds":if ply == 0 {160} else {40},
            "partner":ply != 0,"budget_ms":if ply == 0 {20_000} else {14_000}});
        let response = choose(call.clone())?;
        let tile = response["choice"].as_u64().ok_or("missing choice")?;
        if tile >= 28
            || legal & (1 << tile) == 0
            || response["leader"] != json!(leader)
            || response["points"] != json!(points)
        {
            return Err("player response disagrees with game host".into());
        }
        pairs.push((actor, tile as usize));
        decisions.push(json!({"call":call,"response":response}));
    }
    let state = solver::replay(dcl, game.bidder, &pairs);
    let points = if state.r == 0 {
        [state.banked_t0, state.banked_t1]
    } else {
        [state.banked_t1, state.banked_t0]
    };
    Ok(
        json!({"schema":"kiln-played-game-v1","profile":PROFILE,"play_bid":30,
        "hands":game.hands,"bidder":game.bidder,"decl":game.decl,"seed":game.seed,
        "record":pairs.iter().flat_map(|&(a,t)| [a,t]).collect::<Vec<_>>(),
        "decisions":decisions,"points":points,"score":points[game.bidder % 2],
        "elapsed_us":start.elapsed().as_micros() as u64}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_finishes_all_tricks_and_exposes_only_own_hand() {
        for decl in [0, 1, 2, 3, 4, 5, 6, 7, 9] {
            for bidder in 0..4 {
                let hands = [
                    [0, 1, 2, 3, 4, 5, 6],
                    [7, 8, 9, 10, 11, 12, 13],
                    [14, 15, 16, 17, 18, 19, 20],
                    [21, 22, 23, 24, 25, 26, 27],
                ];
                let input =
                    json!({"profile":PROFILE,"hands":hands,"bidder":bidder,"decl":decl,"seed":42});
                let result = play(&input.to_string(), |call| {
                    let req = &call["request"];
                    assert_eq!(req.as_object().unwrap().len(), 7);
                    let actor = req["seat"].as_u64().unwrap() as usize;
                    assert_eq!(req["hand"], json!(hands[actor]));
                    assert_eq!(req["bid"], 30);
                    assert_eq!(req["plays"].as_array().unwrap().len() % 2, 0);
                    let request: crate::Call = serde_json::from_value(call).unwrap();
                    let mut status: Value = serde_json::from_str(
                        &solver::partnership_wire::run(&format!(
                            "status\n{}",
                            request.request.text().unwrap()
                        ))
                        .unwrap(),
                    )
                    .unwrap();
                    status["choice"] = status["legal"][0].clone();
                    Ok(status)
                })
                .unwrap();
                assert_eq!(result["record"].as_array().unwrap().len(), 56);
                assert_eq!(result["decisions"].as_array().unwrap().len(), 28);
                assert_eq!(
                    result["points"][0].as_u64().unwrap() + result["points"][1].as_u64().unwrap(),
                    42
                );
            }
        }
    }

    #[test]
    fn malformed_game_never_reaches_player() {
        let hands = [[0; 7]; 4];
        let input = json!({"profile":PROFILE,"hands":hands,"bidder":0,"decl":6,"seed":1});
        assert!(play(&input.to_string(), |_| panic!(
            "invalid game reached player"
        ))
        .is_err());
    }
}
