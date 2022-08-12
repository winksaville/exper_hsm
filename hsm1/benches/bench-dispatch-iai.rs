use iai::black_box;

use hsm1::{
    handled, hsm1, hsm1_initial_state, hsm1_state, not_handled, transition_to, StateResult,
};

struct NoMessages;

pub fn bench_fsm_setup() {
    hsm1!(
        struct Fsm {}

        #[hsm1_initial_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
            handled!()
        }
    );

    let _sm = black_box(Fsm::new());
    let _msg = black_box(NoMessages);
}

pub fn bench_minimal_fsm_returning_handled() {
    hsm1!(
        struct Fsm {}

        #[hsm1_initial_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
            handled!()
        }
    );

    let mut sm = black_box(Fsm::new());
    let msg = black_box(NoMessages);
    sm.dispatch(black_box(&msg));
}

pub fn bench_minimal_fsm_returning_not_handled() {
    hsm1!(
        struct Fsm {}

        #[hsm1_initial_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
            not_handled!()
        }
    );

    let mut sm = black_box(Fsm::new());
    let msg = black_box(NoMessages);
    sm.dispatch(black_box(&msg));
}

pub fn bench_minimal_fsm_returning_transition_to_self() {
    hsm1!(
        struct Fsm {}

        #[hsm1_initial_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
            transition_to!(initial)
        }
    );

    let mut sm = black_box(Fsm::new());
    let msg = black_box(NoMessages);
    sm.dispatch(black_box(&msg));
}

pub fn bench_minimal_fsm_returning_transition_to_self_with_enter() {
    hsm1!(
        struct Fsm {
            initial_enter_cnt: u64,
            initial_cnt: u64,
            initial_exit_cnt: u64,
        }

        fn initial_enter(&mut self, _msg: &NoMessages) {}

        #[hsm1_initial_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
            transition_to!(initial)
        }
    );

    let mut sm = black_box(Fsm::new());
    let msg = black_box(NoMessages);
    sm.dispatch(black_box(&msg));
}

pub fn bench_minimal_fsm_returning_transition_to_self_with_exit() {
    hsm1!(
        struct Fsm {
            initial_enter_cnt: u64,
            initial_cnt: u64,
            initial_exit_cnt: u64,
        }

        fn initial_exit(&mut self, _msg: &NoMessages) {}

        #[hsm1_initial_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
            transition_to!(initial)
        }
    );

    let mut sm = black_box(Fsm::new());
    let msg = black_box(NoMessages);
    sm.dispatch(black_box(&msg));
}

pub fn bench_minimal_fsm_returning_transition_to_self_with_ee() {
    hsm1!(
        struct Fsm {
            initial_enter_cnt: u64,
            initial_cnt: u64,
            initial_exit_cnt: u64,
        }

        fn initial_enter(&mut self, _msg: &NoMessages) {
            self.initial_enter_cnt += 1;
        }

        #[hsm1_initial_state]
        fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
            self.initial_cnt += 1;
            transition_to!(initial)
        }

        fn initial_exit(&mut self, _msg: &NoMessages) {
            self.initial_exit_cnt += 1;
        }
    );

    let mut sm = black_box(Fsm::new());
    let msg = black_box(NoMessages);
    sm.dispatch(black_box(&msg));
    //assert_eq!(sm.initial_enter_cnt, 1);
    //assert_eq!(sm.initial_cnt, 1);
    //assert_eq!(sm.initial_exit_cnt, 1);
    //println!("sm: initial_enter_cnt={} initial_cnt={} initial_exit_cnt={}",
    //    sm.initial_enter_cnt, sm.initial_cnt, sm.initial_exit_cnt);
}

iai::main!(
    bench_fsm_setup,
    bench_minimal_fsm_returning_handled,
    bench_minimal_fsm_returning_not_handled,
    bench_minimal_fsm_returning_transition_to_self,
    bench_minimal_fsm_returning_transition_to_self_with_enter,
    bench_minimal_fsm_returning_transition_to_self_with_exit,
    bench_minimal_fsm_returning_transition_to_self_with_ee,
);
