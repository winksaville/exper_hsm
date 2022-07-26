use criterion::{black_box, criterion_group, criterion_main, Criterion};

use hsm1::{handled, hsm1, hsm1_state, not_handled, transition_to};
use state_result::*;
use std::collections::VecDeque;

struct NoMessages;

pub fn bench_minimal_fsm_returning_handled(c: &mut Criterion) {
    hsm1!(
        struct Fsm {}

        #[hsm1_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult {
            handled!()
        }
    );

    c.bench_function("bench_minimal_fsm_returning_handled", |b| {
        let mut sm = Fsm::new();
        let msg = NoMessages;
        b.iter(|| {
            sm.dispatch(black_box(&msg));
        });
    });
}

pub fn bench_minimal_fsm_returning_not_handled(c: &mut Criterion) {
    hsm1!(
        struct Fsm {}

        #[hsm1_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult {
            not_handled!()
        }
    );

    c.bench_function("bench_minimal_fsm_returning_not_handled", |b| {
        let mut sm = Fsm::new();
        let msg = NoMessages;
        b.iter(|| {
            sm.dispatch(black_box(&msg));
        });
    });
}

pub fn bench_minimal_fsm_returning_transition_to_self(c: &mut Criterion) {
    hsm1!(
        struct Fsm {}

        #[hsm1_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult {
            transition_to!(initial)
        }
    );

    c.bench_function("bench_minimal_fsm_returning_transition_to_self", |b| {
        let mut sm = Fsm::new();
        let msg = NoMessages;
        b.iter(|| {
            sm.dispatch(black_box(&msg));
        });
    });
}

pub fn bench_minimal_fsm_returning_transition_to_self_with_enter(c: &mut Criterion) {
    hsm1!(
        struct Fsm {
            initial_enter_cnt: u64,
            initial_cnt: u64,
            initial_exit_cnt: u64,
        }

        fn initial_enter(&mut self, _msg: &NoMessages) {}

        #[hsm1_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult {
            transition_to!(initial)
        }
    );

    c.bench_function(
        "bench_minimal_fsm_returning_transition_to_self_with_enter",
        |b| {
            let mut sm = Fsm::new();
            let msg = NoMessages;
            b.iter(|| {
                sm.dispatch(black_box(&msg));
            });
        },
    );
}

pub fn bench_minimal_fsm_returning_transition_to_self_with_exit(c: &mut Criterion) {
    hsm1!(
        struct Fsm {
            initial_enter_cnt: u64,
            initial_cnt: u64,
            initial_exit_cnt: u64,
        }

        fn initial_exit(&mut self, _msg: &NoMessages) {}

        #[hsm1_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult {
            transition_to!(initial)
        }
    );

    c.bench_function(
        "bench_minimal_fsm_returning_transition_to_self_with_exit",
        |b| {
            let mut sm = Fsm::new();
            let msg = NoMessages;
            b.iter(|| {
                sm.dispatch(black_box(&msg));
            });
        },
    );
}

pub fn bench_minimal_fsm_returning_transition_to_self_with_ee(c: &mut Criterion) {
    hsm1!(
        struct Fsm {
            initial_enter_cnt: u64,
            initial_cnt: u64,
            initial_exit_cnt: u64,
        }

        fn initial_enter(&mut self, _msg: &NoMessages) {
            self.initial_enter_cnt += 1;
        }

        #[hsm1_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult {
            self.initial_cnt += 1;
            transition_to!(initial)
        }

        fn initial_exit(&mut self, _msg: &NoMessages) {
            self.initial_exit_cnt += 1;
        }
    );

    c.bench_function(
        "bench_minimal_fsm_returning_transition_to_self_with_ee",
        |b| {
            let mut sm = Fsm::new();
            let msg = NoMessages;
            b.iter(|| {
                sm.dispatch(black_box(&msg));
            });
            //assert!(sm.initial_enter_cnt, 0);
            //assert!(sm.initial_cnt > 0);
            //assert!(sm.initial_exit_cnt > 0);
            //println!("sm: initial_enter_cnt={} initial_cnt={} initial_exit_cnt={}",
            //    sm.initial_enter_cnt, sm.initial_cnt, sm.initial_exit_cnt);
        },
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.05).sample_size(5000);
    targets = bench_minimal_fsm_returning_handled,
    bench_minimal_fsm_returning_not_handled,
    bench_minimal_fsm_returning_transition_to_self,
    bench_minimal_fsm_returning_transition_to_self_with_enter,
    bench_minimal_fsm_returning_transition_to_self_with_exit,
    bench_minimal_fsm_returning_transition_to_self_with_ee,
}
criterion_main!(benches);
