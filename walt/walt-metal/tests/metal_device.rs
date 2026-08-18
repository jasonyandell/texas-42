use core::convert::Infallible;

use walt_gpu_ref::{
    M2ArithmeticCorpusV1, M2GlobalParityAccumulatorV1, OpeningChooseTableV1,
    M2_CONTEXT_TASK_COUNT_V1,
};
use walt_metal::{CommandEvent, CommandState, CommandTerminal, MetalError, MetalRuntime};

fn exit_timeout(state: CommandState) -> Infallible {
    eprintln!("Metal command exceeded 120 seconds in {state:?}");
    std::process::exit(124)
}

#[derive(Default)]
struct EventCensus {
    committed: usize,
    completed: usize,
}

impl EventCensus {
    fn observe(&mut self, event: CommandEvent) {
        match event {
            CommandEvent::Committed => self.committed += 1,
            CommandEvent::Terminal(CommandTerminal::Completed) => self.completed += 1,
            CommandEvent::Terminal(other) => panic!("unexpected terminal event {other:?}"),
        }
    }
}

#[test]
#[ignore = "requires elevated access to the local Apple GPU; run in release mode"]
fn canonical_arithmetic_and_negative_controls_device_gate() {
    let mut runtime = MetalRuntime::new().expect("initialize frozen Metal runtime");
    let mut census = EventCensus::default();
    let mut timeout = exit_timeout;
    {
        let mut observer = |event| census.observe(event);

        assert!(matches!(
            runtime.run_arithmetic_negative(&mut observer, &mut timeout),
            Err(MetalError::Gate0Required)
        ));

        runtime
            .run_gate0(&mut observer, &mut timeout)
            .expect("Gate 0 empty command");
        assert!(matches!(
            runtime.run_gate0(&mut observer, &mut timeout),
            Err(MetalError::Gate0AlreadyPassed)
        ));
        assert!(runtime.device_profile().gate0_passed);
        assert_eq!(
            runtime.device_profile().allocations.projector_logical_bytes,
            5_109_296
        );
        assert_eq!(
            runtime
                .device_profile()
                .allocations
                .arithmetic_logical_bytes,
            2_359_424
        );

        let arithmetic_negative = runtime
            .run_arithmetic_negative(&mut observer, &mut timeout)
            .expect("exact arithmetic negative command");
        assert_eq!(arithmetic_negative.integrity().hard_count, 13);
        assert_eq!(
            arithmetic_negative.integrity().cpu_output_digest,
            arithmetic_negative.integrity().gpu_output_digest
        );

        let corpus = M2ArithmeticCorpusV1::canonical().expect("portable BigUint corpus");
        let arithmetic = runtime
            .run_official_arithmetic(&corpus, &mut observer, &mut timeout)
            .expect("exact 16,384-case U256 Metal grid");
        assert_eq!(arithmetic.integrity().accepted_count, 16_384);
        assert_eq!(
            arithmetic.integrity().cpu_output_digest,
            arithmetic.integrity().gpu_output_digest
        );

        let choose = OpeningChooseTableV1::canonical().expect("portable canonical choose table");
        for ordinal in 0..13 {
            let control = runtime
                .run_opening_negative(ordinal, &choose, &mut observer, &mut timeout)
                .expect("exact one-thread opening negative command");
            assert_eq!(control.integrity().ordinal, ordinal as u32);
        }
    }

    assert_eq!(census.committed, 16);
    assert_eq!(census.completed, 16);
}

#[test]
#[ignore = "requires elevated access to the local Apple GPU; run in release mode"]
fn canonical_maximum_smoke_device_gate() {
    let mut runtime = MetalRuntime::new().expect("initialize frozen Metal runtime");
    let mut census = EventCensus::default();
    let mut timeout = exit_timeout;
    {
        let mut observer = |event| census.observe(event);
        runtime
            .run_gate0(&mut observer, &mut timeout)
            .expect("smoke Gate 0 empty command");
        let smoke = runtime
            .run_maximum_smoke(&mut observer, &mut timeout)
            .expect("maximum grade-seven/matching-six portable parity smoke");
        assert_eq!(smoke.integrity().response_count, 7_980);
        assert_eq!(smoke.integrity().candidate_slot_count, 79_800);
        assert_eq!(smoke.integrity().valid_cell_count, 11_730);
    }

    assert_eq!(census.committed, 2);
    assert_eq!(census.completed, 2);
}

#[test]
#[ignore = "requires elevated access to the local Apple GPU; run in release mode"]
fn canonical_full_opening_carrier_device_gate() {
    let mut runtime = MetalRuntime::new().expect("initialize frozen Metal runtime");
    let mut census = EventCensus::default();
    let mut timeout = exit_timeout;
    let mut accumulator =
        M2GlobalParityAccumulatorV1::canonical().expect("opaque global parity accumulator");
    let choose = OpeningChooseTableV1::canonical().expect("portable canonical choose table");
    {
        let mut observer = |event| census.observe(event);
        runtime
            .run_gate0(&mut observer, &mut timeout)
            .expect("full-carrier Gate 0 empty command");
        for ordinal in 0..M2_CONTEXT_TASK_COUNT_V1 {
            let accepted = runtime
                .run_next_opening(&mut accumulator, &choose, &mut observer, &mut timeout)
                .expect("canonical opening task portable join");
            assert_eq!(
                accepted.bound_task().checked_payload().task_ordinal(),
                ordinal as u32
            );
            assert_eq!(accumulator.accepted_task_count(), ordinal as u32 + 1);
        }
    }

    let digests = accumulator
        .finish()
        .expect("complete global parity streams");
    assert_eq!(digests.cpu_raw_sha256(), digests.gpu_raw_sha256());
    assert_eq!(digests.cpu_payload_sha256(), digests.gpu_payload_sha256());
    assert_eq!(
        digests.cpu_aggregate_sha256(),
        digests.gpu_aggregate_sha256()
    );
    assert_eq!(census.committed, M2_CONTEXT_TASK_COUNT_V1 + 1);
    assert_eq!(census.completed, M2_CONTEXT_TASK_COUNT_V1 + 1);
}
