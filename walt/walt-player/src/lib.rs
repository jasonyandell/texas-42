//! EXPLORATORY deployed Walt. Native and browser adapters call this same
//! budgeted decision procedure. Only complete comparisons may replace the
//! current move; checkpoints let a host retain that move after interruption.
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::Duration;
use walt::{
    clock::Instant,
    policy_search::{partner_rollout as r, request},
    solver,
};

pub const PLAYER_ID: &str = "walt-table-v2";
mod auction;

#[derive(Deserialize)]
#[serde(untagged)]
enum Seed {
    Integer(u64),
    Decimal(String),
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    decl: u64,
    bid: u64,
    bidder: u64,
    seat: u64,
    hand: Vec<u64>,
    plays: Vec<u64>,
    seed: Seed,
}
impl Request {
    fn text(&self) -> Result<String, String> {
        let seed = match &self.seed {
            Seed::Integer(n) => *n,
            Seed::Decimal(s) => s.parse::<u64>().map_err(|_| "invalid seed")?,
        };
        if !(30..=42).contains(&self.bid) {
            return Err("the table player requires a straight bid from 30 through 42".into());
        }
        let words = |xs: &[u64]| xs.iter().map(u64::to_string).collect::<Vec<_>>().join(" ");
        Ok(format!(
            "decl {}\nbid {}\nbidder {}\nseat {}\nhand {}\nplays {}\nseed {seed}\n",
            self.decl,
            self.bid,
            self.bidder,
            self.seat,
            words(&self.hand),
            words(&self.plays)
        ))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Call {
    request: Request,
    #[serde(default = "default_worlds")]
    worlds: usize,
    #[serde(default = "default_partner")]
    partner: bool,
    #[serde(default = "default_budget")]
    budget_ms: u64,
}
fn default_worlds() -> usize {
    40
}
fn default_partner() -> bool {
    true
}
fn default_budget() -> u64 {
    14_000
}

fn evaluation(text: &str, n: usize, n0: usize, ms: u64) -> Result<Value, String> {
    let wire = format!("baseline\n{text}n {n}\nn0 {n0}\nn1 2\nbudget_ms {ms}\ninner_belief 0\nselection 0\nmodeled_selection 0\n");
    let value = solver::partnership_wire::run(&wire)?;
    serde_json::from_str(&value).map_err(|e| e.to_string())
}

fn review_value(value: &r::Review, baseline: usize) -> Value {
    json!({"schema":r::ID,"field":r::FIELD,"baseline":baseline,
        "choice":value.choice,"status":value.status,"stop":value.stop,
        "offers":value.offers,"legal":value.legal,"values":value.legal.iter().zip(&value.values).map(|(a,n)| [*a,*n]).collect::<Vec<_>>(),
        "paired":value.paired,"support":value.support,"samples":value.samples,
        "requested":value.requested,"coverage":value.coverage,"calls":value.decisions.len(),
        "cache_hits":value.cache_hits,"elapsed_us":value.elapsed_us})
}

fn emit(value: &mut Value, start: Instant, budget_ms: u64, checkpoint: &mut impl FnMut(&Value)) {
    let elapsed = start.elapsed().as_micros() as u64;
    value["elapsed_us"] = json!(elapsed);
    value["over_budget"] = json!(elapsed > budget_ms * 1000);
    checkpoint(value);
}

/// Caller validates the returned position/choice against its independent game
/// engine. The host receives a legal checkpoint before expensive evaluation.
pub fn decide(call: Call, mut checkpoint: impl FnMut(&Value)) -> Result<Value, String> {
    let start = Instant::now();
    if !(100..=14_000).contains(&call.budget_ms) || !(1..=640).contains(&call.worlds) {
        return Err("invalid time or sampling budget".into());
    }
    if call.partner && call.worlds != 40 {
        return Err("partner check models default L1 40/8".into());
    }
    let text = call.request.text()?;
    let status: Value =
        serde_json::from_str(&solver::partnership_wire::run(&format!("status\n{text}"))?)
            .map_err(|e| e.to_string())?;
    let legal = status["legal"].as_array().ok_or("missing legal choices")?;
    let first = legal.first().ok_or("no legal play")?.clone();
    let forced = legal.len() == 1;
    let mut value = json!({"schema":"partnership-decision-v1","player_version":PLAYER_ID,
        "choice":first,"legal":legal,"leader":status["leader"],"points":status["points"],"trick":status["trick"],
        "route":if forced {"forced"} else {"legal-fallback"},"mode":"baseline",
        "inner_belief":"voidless","selection":"fixed","modeled_selection":"fixed",
        "n":call.worlds,"n0":8,"n1":2,"budget_ms":call.budget_ms,
        "review":if call.partner {"partner-rollout"} else {"off"},"review_result":null,
        "evaluation":null,"fallback_evaluation":null,"elapsed_us":0,"over_budget":false,
        "phases":[{"name":"status-check","status":"completed"}]});
    emit(&mut value, start, call.budget_ms, &mut checkpoint);
    if forced {
        return Ok(value);
    }
    let remaining = || {
        call.budget_ms
            .saturating_sub(start.elapsed().as_millis() as u64)
            .saturating_sub(100)
    };
    // Preserve the native deployed sequence: cheap complete L1, full L1, then
    // at most 500 ms for the optional count-offer continuation check.
    for (name, n, n0, ms) in [
        (
            "fallback-l1",
            8,
            2,
            remaining()
                .checked_div(4)
                .unwrap_or(0)
                .min(1500)
                .saturating_sub(40),
        ),
        ("baseline", call.worlds, 8, 0),
    ] {
        let ms = if name == "baseline" {
            remaining().saturating_sub(40)
        } else {
            ms
        };
        let phase_start = Instant::now();
        let result = if ms == 0 {
            Err("no-time".into())
        } else {
            evaluation(&text, n, n0, ms)
        };
        let phase_status = match result {
            Ok(report) => {
                value["choice"] = report["choice"].clone();
                let field = if name == "baseline" {
                    "evaluation"
                } else {
                    "fallback_evaluation"
                };
                value[field] = report;
                value["route"] = json!(if name == "baseline" {
                    "baseline"
                } else {
                    "l1-fallback"
                });
                "completed".to_owned()
            }
            Err(error) => error,
        };
        value["phases"].as_array_mut().unwrap().push(json!({"name":name,"status":phase_status,"elapsed_us":phase_start.elapsed().as_micros() as u64}));
        emit(&mut value, start, call.budget_ms, &mut checkpoint);
    }
    if call.partner && value["route"] == "baseline" {
        let baseline = value["choice"].as_u64().unwrap() as usize;
        let ms = remaining().min(500);
        let result = if ms < 50 {
            json!({"schema":r::ID,"baseline":baseline,"choice":baseline,"status":"unresolved-no-time"})
        } else {
            let deadline = solver::Deadline::after(Duration::from_millis(ms.saturating_sub(35)));
            match request::from_text_with_seed(&text).and_then(|(f, seed)| {
                r::review(&f, seed, baseline, deadline, r::MAX_SAMPLES, false)
            }) {
                Ok(report) => review_value(&report, baseline),
                Err(error) => {
                    json!({"schema":r::ID,"baseline":baseline,"choice":baseline,"status":"unresolved","reason":error})
                }
            }
        };
        if result["status"] == "changed" {
            value["choice"] = result["choice"].clone();
            value["route"] = json!("baseline-reviewed");
        }
        value["phases"]
            .as_array_mut()
            .unwrap()
            .push(json!({"name":"partner-rollout-review","status":result["status"]}));
        value["review_result"] = result;
    }
    emit(&mut value, start, call.budget_ms, &mut checkpoint);
    Ok(value)
}

pub fn handle(text: &str, checkpoint: impl FnMut(&Value)) -> Value {
    if text.len() > 16_384 {
        return json!({"error":"request too large"});
    }
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Input {
        Play(Call),
        Auction(auction::Call),
        AuctionPrice(auction::PriceCall),
        AuctionMerge(auction::MergeCall),
    }
    match serde_json::from_str::<Input>(text)
        .map_err(|e| e.to_string())
        .and_then(|call| match call {
            Input::Play(call) => decide(call, checkpoint),
            Input::Auction(call) => auction::decide(call, checkpoint),
            Input::AuctionPrice(call) => auction::price_call(call),
            Input::AuctionMerge(call) => auction::merge(call),
        }) {
        Ok(value) => value,
        Err(error) => json!({"error":error}),
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(unsafe_code)]
mod abi {
    use std::{cell::RefCell, time::Duration};
    // Only foreign calls in the player. JS provides monotonic microseconds
    // and synchronously copies an immutable checkpoint from linear memory.
    #[link(wasm_import_module = "walt_host")]
    extern "C" {
        fn now_us() -> u64;
        fn checkpoint(ptr: u32, len: u32);
    }
    fn now() -> Duration {
        Duration::from_micros(unsafe { now_us() })
    }
    thread_local! {
        static INPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
        static OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }
    #[no_mangle]
    pub extern "C" fn walt_in_prepare(len: u32) -> u32 {
        assert!(len <= 16_384);
        INPUT.with(|b| {
            let mut v = b.borrow_mut();
            v.resize(len as usize, 0);
            v.as_ptr() as u32
        })
    }
    #[no_mangle]
    pub extern "C" fn walt_call() -> u32 {
        walt::clock::install(now);
        let text = INPUT.with(|b| String::from_utf8_lossy(&b.borrow()).into_owned());
        let value = super::handle(&text, |v| {
            let s = v.to_string();
            // Host must copy before returning; it may not reenter the solver.
            unsafe {
                checkpoint(s.as_ptr() as u32, s.len() as u32);
            }
        });
        OUTPUT.with(|b| {
            let mut v = b.borrow_mut();
            *v = value.to_string().into_bytes();
            v.len() as u32
        })
    }
    #[no_mangle]
    pub extern "C" fn walt_out_ptr() -> u32 {
        OUTPUT.with(|b| b.borrow().as_ptr() as u32)
    }
}
