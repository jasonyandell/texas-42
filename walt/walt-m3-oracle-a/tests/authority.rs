use std::sync::OnceLock;

use walt_geom::q;
use walt_m3_carrier::{M3Carrier, OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL, ROOTS};
use walt_m3_oracle_a::{
    evaluate_all_m3a, evaluate_m3a_task, M3aAuthorityRun, M3aTask, M3A_AUTHORITY_TASK_COUNT,
    M3A_ROOT_TREE_V0_CAP,
};

const RAW_RECEIPT: &[u8] = include_bytes!("../../../rob/receipts/verify_player.txt");

fn carrier() -> &'static M3Carrier {
    static CARRIER: OnceLock<M3Carrier> = OnceLock::new();
    CARRIER.get_or_init(|| M3Carrier::from_receipt_bytes(RAW_RECEIPT).expect("frozen carrier"))
}

fn authority() -> &'static M3aAuthorityRun {
    static AUTHORITY: OnceLock<M3aAuthorityRun> = OnceLock::new();
    AUTHORITY.get_or_init(|| evaluate_all_m3a(carrier()).expect("all four M3A roots fit their cap"))
}

#[test]
fn typed_tasks_are_exact_and_cannot_name_m3b() {
    assert_eq!(M3A_AUTHORITY_TASK_COUNT, 4);
    assert_eq!(M3A_ROOT_TREE_V0_CAP, 1u64 << 26);
    assert_eq!(M3aTask::ALL.map(M3aTask::task_ordinal), [0, 1, 2, 3]);
    assert_eq!(M3aTask::ALL.map(M3aTask::root_index), [4, 7, 9, 20]);
    assert_eq!(M3aTask::ALL.map(M3aTask::root), ROOTS);
}

#[test]
fn all_roots_return_exact_values_and_fresh_visit_partitions() {
    let run = authority();
    for (ordinal, record) in run.records().iter().enumerate() {
        let task = M3aTask::ALL[ordinal];
        assert_eq!(record.task(), task);
        assert_eq!(record.task_ordinal(), ordinal as u32);
        assert_eq!(record.objective(), OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL);
        assert_eq!(record.root_index(), task.root_index());
        assert_eq!(record.root(), task.root());
        assert_eq!(run.record(task), record);

        let exact = record.exact_q_fields();
        assert!(exact.denominator > 0);
        assert_eq!(record.value(), &q(exact.numerator, exact.denominator));
        assert!(record.value() >= &q(-4, 1));
        assert!(record.value() <= &q(4, 1));

        assert!(record.visits() > 0);
        assert!(record.visits() <= M3A_ROOT_TREE_V0_CAP);
        assert_eq!(
            record.epoch_visits().into_iter().sum::<u64>(),
            record.visits()
        );
        assert_eq!(
            record.budget_remaining(),
            M3A_ROOT_TREE_V0_CAP - record.visits()
        );
    }
}

#[test]
fn record_projection_is_deterministic_without_value_pins() {
    let run = authority();
    let projected_once: Vec<_> = run
        .records()
        .iter()
        .map(|record| {
            (
                record.task_ordinal(),
                record.objective(),
                record.root_index(),
                record.exact_q_fields(),
                record.visits(),
                record.epoch_visits(),
            )
        })
        .collect();
    let projected_twice: Vec<_> = run
        .records()
        .iter()
        .map(|record| {
            (
                record.task_ordinal(),
                record.objective(),
                record.root_index(),
                record.exact_q_fields(),
                record.visits(),
                record.epoch_visits(),
            )
        })
        .collect();
    assert_eq!(projected_once, projected_twice);
}

#[test]
fn a_fresh_single_root_run_reproduces_its_typed_record() {
    let fresh = evaluate_m3a_task(carrier(), M3aTask::Root31)
        .expect("the isolated root-31 solve fits its fresh cap");
    assert_eq!(&fresh, authority().record(M3aTask::Root31));
}
