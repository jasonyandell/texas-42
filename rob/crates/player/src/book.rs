//! The contingency book: plan-tree JSON projections
//! (BRIEF_PLAYER_01 §7 trace, §8 P5; INV-P1).
//!
//! This is the display layer for [`Plan`]: deterministic JSON emission, a
//! strict parser for the round-trip receipt (`r_book_roundtrip`), and a
//! depth/breadth-capped projection for the inspector. Every number is an
//! exact integer from the plan; per-node values are values *of the plan at
//! that information set* — no per-domino scalar exists here or anywhere
//! (INV-P1).

use rob_core::{domino_from_id, domino_id, Domino, DominoId, Pip, Seat};

use crate::plan::{FrontierLeaf, LeafKind, Observation, Plan, PlanChild, PlanLeaf, PlanNode};

fn fmt_tile(id: DominoId) -> String {
    let d = domino_from_id(id);
    format!("{}-{}", d.high().value(), d.low().value())
}

fn parse_tile(name: &str) -> DominoId {
    let (h, l) = name.split_once('-').expect("tile as high-low");
    let high = Pip::new(h.parse().expect("pip digit")).expect("valid pip");
    let low = Pip::new(l.parse().expect("pip digit")).expect("valid pip");
    domino_id(Domino::new(high, low))
}

fn obs_to_json(obs: &Observation) -> String {
    let steps: Vec<String> = obs
        .iter()
        .map(|&(seat, tile)| {
            format!(
                "[{},\"{}\"]",
                seat,
                fmt_tile(
                    rob_core::all_ids()
                        .nth(tile as usize)
                        .expect("valid tile index")
                )
            )
        })
        .collect();
    format!("[{}]", steps.join(","))
}

fn leaf_to_json(leaf: &PlanLeaf) -> String {
    let kind = match leaf.kind {
        LeafKind::Settled => "settled",
        LeafKind::Frontier(FrontierLeaf::BankedPoints) => "frontier-banked",
    };
    format!(
        "{{\"leaf\":\"{kind}\",\"worlds\":{},\"value\":{}}}",
        leaf.world_count, leaf.value_total
    )
}

fn node_to_json(node: &PlanNode) -> String {
    let children: Vec<String> = node
        .children
        .iter()
        .map(|(obs, child)| {
            let child_json = match child {
                PlanChild::Node(n) => node_to_json(n),
                PlanChild::Leaf(l) => leaf_to_json(l),
            };
            format!("[{},{}]", obs_to_json(obs), child_json)
        })
        .collect();
    format!(
        "{{\"action\":\"{}\",\"worlds\":{},\"value\":{},\"children\":[{}]}}",
        fmt_tile(node.action),
        node.world_count,
        node.value_total,
        children.join(",")
    )
}

/// Deterministic JSON emission of a whole plan (the `r_book_roundtrip`
/// canonical form).
pub fn plan_to_json(plan: &Plan) -> String {
    format!(
        "{{\"format\":\"rob-plan\",\"version\":1,\"viewer\":{},\"window\":{},\"fiber\":{},\"truncated\":{},\"root\":{}}}",
        plan.viewer.index(),
        plan.window,
        plan.fiber_count,
        plan.truncated,
        node_to_json(&plan.root)
    )
}

// ---------------------------------------------------------------------------
// Strict parser for exactly the grammar emitted above (verification use:
// panics on any malformation — a receipt, not a general JSON reader).

struct Parser<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Parser<'a> {
    fn expect(&mut self, lit: &str) {
        assert!(
            self.bytes[self.at..].starts_with(lit.as_bytes()),
            "expected `{lit}` at byte {}",
            self.at
        );
        self.at += lit.len();
    }

    fn peek(&self) -> u8 {
        self.bytes[self.at]
    }

    fn integer(&mut self) -> i64 {
        let start = self.at;
        if self.peek() == b'-' {
            self.at += 1;
        }
        while self.at < self.bytes.len() && self.bytes[self.at].is_ascii_digit() {
            self.at += 1;
        }
        std::str::from_utf8(&self.bytes[start..self.at])
            .expect("ascii digits")
            .parse()
            .expect("integer literal")
    }

    fn string(&mut self) -> String {
        self.expect("\"");
        let start = self.at;
        while self.peek() != b'"' {
            self.at += 1;
        }
        let s = std::str::from_utf8(&self.bytes[start..self.at])
            .expect("utf8 string")
            .to_string();
        self.at += 1;
        s
    }

    fn boolean(&mut self) -> bool {
        if self.bytes[self.at..].starts_with(b"true") {
            self.at += 4;
            true
        } else {
            self.expect("false");
            false
        }
    }

    fn observation(&mut self) -> Observation {
        self.expect("[");
        let mut obs = Vec::new();
        while self.peek() != b']' {
            if self.peek() == b',' {
                self.at += 1;
            }
            self.expect("[");
            let seat = self.integer() as u8;
            self.expect(",");
            let tile = parse_tile(&self.string());
            self.expect("]");
            obs.push((seat, tile.index() as u8));
        }
        self.at += 1;
        obs
    }

    fn child(&mut self) -> PlanChild {
        if self.bytes[self.at..].starts_with(b"{\"leaf\":") {
            self.expect("{\"leaf\":");
            let kind = match self.string().as_str() {
                "settled" => LeafKind::Settled,
                "frontier-banked" => LeafKind::Frontier(FrontierLeaf::BankedPoints),
                other => panic!("unknown leaf kind {other}"),
            };
            self.expect(",\"worlds\":");
            let world_count = self.integer() as u64;
            self.expect(",\"value\":");
            let value_total = self.integer();
            self.expect("}");
            PlanChild::Leaf(PlanLeaf {
                kind,
                world_count,
                value_total,
            })
        } else {
            PlanChild::Node(self.node())
        }
    }

    fn node(&mut self) -> PlanNode {
        self.expect("{\"action\":");
        let action = parse_tile(&self.string());
        self.expect(",\"worlds\":");
        let world_count = self.integer() as u64;
        self.expect(",\"value\":");
        let value_total = self.integer();
        self.expect(",\"children\":[");
        let mut children = std::collections::BTreeMap::new();
        while self.peek() != b']' {
            if self.peek() == b',' {
                self.at += 1;
            }
            self.expect("[");
            let obs = self.observation();
            self.expect(",");
            let child = self.child();
            self.expect("]");
            children.insert(obs, child);
        }
        self.at += 1;
        self.expect("}");
        PlanNode {
            action,
            world_count,
            value_total,
            children,
        }
    }
}

/// Parse the canonical plan JSON back into a [`Plan`] (round-trip receipt
/// surface; panics on malformation).
pub fn plan_from_json(text: &str) -> Plan {
    let mut p = Parser {
        bytes: text.as_bytes(),
        at: 0,
    };
    p.expect("{\"format\":\"rob-plan\",\"version\":1,\"viewer\":");
    let viewer = Seat::ALL[p.integer() as usize];
    p.expect(",\"window\":");
    let window = p.integer() as usize;
    p.expect(",\"fiber\":");
    let fiber_count = p.integer() as u64;
    p.expect(",\"truncated\":");
    let truncated = p.boolean();
    p.expect(",\"root\":");
    let root = p.node();
    p.expect("}");
    assert_eq!(p.at, p.bytes.len(), "trailing bytes after plan JSON");
    Plan {
        viewer,
        window,
        fiber_count,
        truncated,
        root,
    }
}

// ---------------------------------------------------------------------------
// Display projection (inspector): depth- and breadth-capped book.

fn project_node(node: &PlanNode, depth: usize, breadth: usize) -> String {
    if depth == 0 {
        return format!(
            "{{\"action\":\"{}\",\"worlds\":{},\"value\":{},\"elided\":{}}}",
            fmt_tile(node.action),
            node.world_count,
            node.value_total,
            node.children.len()
        );
    }
    let shown: Vec<String> = node
        .children
        .iter()
        .take(breadth)
        .map(|(obs, child)| {
            let child_json = match child {
                PlanChild::Node(n) => project_node(n, depth - 1, breadth),
                PlanChild::Leaf(l) => leaf_to_json(l),
            };
            format!("[{},{}]", obs_to_json(obs), child_json)
        })
        .collect();
    let elided = node.children.len().saturating_sub(breadth);
    format!(
        "{{\"action\":\"{}\",\"worlds\":{},\"value\":{},\"children\":[{}],\"elided\":{}}}",
        fmt_tile(node.action),
        node.world_count,
        node.value_total,
        shown.join(","),
        elided
    )
}

/// The inspector's capped contingency-book projection: at most `depth`
/// decision levels and `breadth` branches per node, with exact elision
/// counts. Display data only (INV-P1); the full canonical form is
/// [`plan_to_json`].
pub fn plan_book_projection(plan: &Plan, depth: usize, breadth: usize) -> String {
    book_projection_impl(plan, &[], depth, breadth)
}

/// [`plan_book_projection`] with the root opening values — the exact
/// best-plan value for every legal opening, chosen and rejected alike
/// (plan projections, INV-P1).
pub fn plan_book_projection_with_openings(
    plan: &Plan,
    openings: &[crate::solver::OpeningValue],
    depth: usize,
    breadth: usize,
) -> String {
    book_projection_impl(plan, openings, depth, breadth)
}

fn book_projection_impl(
    plan: &Plan,
    openings: &[crate::solver::OpeningValue],
    depth: usize,
    breadth: usize,
) -> String {
    let openings_json: Vec<String> = openings
        .iter()
        .map(|o| format!("[\"{}\",{}]", fmt_tile(o.action), o.value_total))
        .collect();
    format!(
        "{{\"format\":\"rob-plan-book\",\"version\":1,\"viewer\":{},\"window\":{},\"fiber\":{},\"truncated\":{},\"openings\":[{}],\"root\":{}}}",
        plan.viewer.index(),
        plan.window,
        plan.fiber_count,
        plan.truncated,
        openings_json.join(","),
        project_node(&plan.root, depth, breadth)
    )
}
