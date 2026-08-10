//! Deterministic text rendering of a lesson — its receipt, in the §16.11
//! spirit: origin conflict with its own grade and labels, verdict, the
//! lesson's grade, the declared domain, both implicants, the full widening
//! trace with every terminating witness (complete worlds, complete value
//! rows), and the measured basin. Enough is recorded for an independent
//! implementation to re-verify every step: kernels reconstruct from
//! (hand, seat, trick) by replay, fibers enumerate from kernels, and
//! values recompute from the declared operator and valuation. Byte
//! stability is the point — every formatting choice here is part of any
//! frozen fixture.

use walt_core::{DominoSet, Seat};
use walt_kernel::World;
use walt_skeleton::LumpabilityFailure;

use crate::lesson::{DropOutcome, Lesson, LessonOrigin, WideningWitness};

fn render_set(s: DominoSet) -> String {
    let tiles: Vec<String> = s.iter().map(|d| d.to_string()).collect();
    format!("{{{}}}", tiles.join(" "))
}

fn render_world(world: &World) -> String {
    Seat::ALL
        .iter()
        .map(|s| format!("{s}={}", render_set(world.hand(*s))))
        .collect::<Vec<_>>()
        .join(" ")
}

fn render_witness(w: &WideningWitness) -> String {
    match w {
        WideningWitness::World {
            hand,
            seat,
            trick_no,
            ply,
            world,
            values,
        } => {
            let vals: Vec<String> = values.iter().map(|(a, v)| format!("{a}={v}")).collect();
            format!(
                "witness h{hand} {seat} t{trick_no} p{ply} world {} values {}",
                render_world(world),
                vals.join(" ")
            )
        }
        WideningWitness::Lumpable {
            hand,
            seat,
            trick_no,
            ply,
            nodes,
            classes,
        } => format!(
            "witness h{hand} {seat} t{trick_no} p{ply}: {nodes} nodes -> {classes} classes, LUMPABLE"
        ),
    }
}

fn render_origin(origin: &LessonOrigin) -> Vec<String> {
    match origin {
        LessonOrigin::Regret(c) => {
            let better: Vec<String> = c.better.iter().map(|d| d.to_string()).collect();
            vec![
                format!(
                    "origin: regret conflict h{} {} t{} p{} fiber {} grade {}",
                    c.hand, c.seat, c.trick_no, c.ply, c.fiber, c.grade
                ),
                format!(
                    "  chosen {} better [{}] regret {}",
                    c.chosen,
                    better.join(" "),
                    c.regret
                ),
            ]
        }
        LessonOrigin::Lumpability {
            hand,
            trick_no,
            descriptor,
            failure,
        } => {
            let shape = match failure {
                LumpabilityFailure::LegalSets { node_a, node_b, .. } => {
                    format!("legal-sets witness at carrier nodes {node_a},{node_b}")
                }
                LumpabilityFailure::Kernel {
                    node_a,
                    node_b,
                    action,
                    increment,
                    mass_a,
                    mass_b,
                    ..
                } => format!(
                    "kernel witness at carrier nodes {node_a},{node_b}: action {action} increment {increment:+} mass {mass_a} != {mass_b}"
                ),
            };
            vec![format!(
                "origin: §12.6 lumpability failure h{hand} t{trick_no} descriptor {descriptor} — {shape} (carrier rebuilds from the kernel)"
            )]
        }
    }
}

/// Renders one lesson in full.
pub fn render_lesson(lesson: &Lesson) -> String {
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(&s);
        out.push('\n');
    };
    line("walt lesson (S5b) — exploratory tier".to_string());
    for l in render_origin(&lesson.origin) {
        line(l);
    }
    line(format!("verdict: {}", lesson.verdict));
    line(format!("grade: {}", lesson.grade));
    line(format!(
        "domain: {} — {} decisions, {} worlds, all fibers exhaustively enumerated",
        lesson.basin.domain, lesson.basin.decisions_total, lesson.basin.worlds_total
    ));
    line(format!(
        "initial implicant ({} cells): {}",
        lesson.initial.cells.len(),
        lesson.initial.render()
    ));
    line("trace:".to_string());
    for step in &lesson.trace {
        match &step.outcome {
            DropOutcome::Dropped => line(format!("  drop {} -> dropped", step.cell)),
            DropOutcome::LoadBearing(w) => line(format!(
                "  drop {} -> LOAD-BEARING; {}",
                step.cell,
                render_witness(w)
            )),
        }
    }
    line(format!(
        "final implicant ({} cells): {}",
        lesson.implicant.cells.len(),
        lesson.implicant.render()
    ));
    let load: Vec<String> = lesson
        .load_bearing()
        .iter()
        .map(|c| c.to_string())
        .collect();
    line(format!(
        "load-bearing: [{}]",
        if load.is_empty() {
            "none".to_string()
        } else {
            load.join(", ")
        }
    ));
    line(format!(
        "basin: decisions {}/{} worlds {}/{} strict-worlds {}",
        lesson.basin.decisions_matched,
        lesson.basin.decisions_total,
        lesson.basin.worlds_matched,
        lesson.basin.worlds_total,
        lesson.basin.strict_worlds
    ));
    for m in &lesson.basin.matched {
        line(format!(
            "  h{} {} t{} p{}: worlds {}/{} strict {}",
            m.hand, m.seat, m.trick_no, m.ply, m.worlds_matched, m.worlds_total, m.strict_worlds
        ));
    }
    out
}

/// One summary line per lesson — the pin shape for tables.
pub fn lesson_pin_line(lesson: &Lesson) -> String {
    let origin = match &lesson.origin {
        LessonOrigin::Regret(c) => {
            format!("regret h{} {} t{} p{}", c.hand, c.seat, c.trick_no, c.ply)
        }
        LessonOrigin::Lumpability {
            hand,
            trick_no,
            descriptor,
            ..
        } => {
            format!("lumpability h{hand} t{trick_no} {descriptor}")
        }
    };
    let load: Vec<String> = lesson
        .load_bearing()
        .iter()
        .map(|c| c.to_string())
        .collect();
    format!(
        "{origin}: verdict [{}] grade [{}] final [{}] load-bearing [{}] basin {}/{} decisions {}/{} worlds strict {}",
        lesson.verdict,
        lesson.grade,
        lesson.implicant.render(),
        if load.is_empty() {
            "none".to_string()
        } else {
            load.join(", ")
        },
        lesson.basin.decisions_matched,
        lesson.basin.decisions_total,
        lesson.basin.worlds_matched,
        lesson.basin.worlds_total,
        lesson.basin.strict_worlds
    )
}
