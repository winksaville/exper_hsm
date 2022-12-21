//use hsm1::{handled, hsm1, hsm1_state, not_handled, transition_to, StateResult};
use proc_macro_hsm1::{handled, hsm1, hsm1_initial_state, hsm1_state, transition_to, StateResult};

struct NoMessages;

#[test]
fn test_transitions_with_one_state() {
    hsm1!(
        struct Test {
            initial_enter_cnt: usize,
            initial_cnt: usize,
            initial_exit_cnt: usize,
        }

        fn initial_enter(&mut self, _msg: &NoMessages) {
            self.initial_enter_cnt += 1;
        }

        #[hsm1_initial_state]
        // This state has hdl 0
        fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
            self.initial_cnt += 1;
            transition_to!(initial)
        }

        fn initial_exit(&mut self, _msg: &NoMessages) {
            self.initial_exit_cnt += 1;
        }
    );

    let mut fsm = Test::new();
    assert_eq!(fsm.initial_enter_cnt, 0);
    assert_eq!(fsm.initial_cnt, 0);
    assert_eq!(fsm.initial_exit_cnt, 0);

    fsm.dispatch(&NoMessages);
    assert_eq!(fsm.initial_enter_cnt, 1);
    assert_eq!(fsm.initial_cnt, 1);
    assert_eq!(fsm.initial_exit_cnt, 1);

    fsm.dispatch(&NoMessages);
    assert_eq!(fsm.initial_enter_cnt, 2);
    assert_eq!(fsm.initial_cnt, 2);
    assert_eq!(fsm.initial_exit_cnt, 2);

    fsm.dispatch(&NoMessages);
    assert_eq!(fsm.initial_enter_cnt, 3);
    assert_eq!(fsm.initial_cnt, 3);
    assert_eq!(fsm.initial_exit_cnt, 3);
}

#[test]
fn test_transitions_between_two_unrelated_states() {
    hsm1!(
        struct Test {
            initial_enter_cnt: usize,
            initial_cnt: usize,
            initial_exit_cnt: usize,
            other_enter_cnt: usize,
            other_cnt: usize,
            other_exit_cnt: usize,
        }

        fn initial_enter(&mut self, _msg: &NoMessages) {
            self.initial_enter_cnt += 1;
        }

        #[hsm1_initial_state]
        // This state has hdl 0
        fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
            self.initial_cnt += 1;
            transition_to!(other)
        }

        fn initial_exit(&mut self, _msg: &NoMessages) {
            self.initial_exit_cnt += 1;
        }

        fn other_enter(&mut self, _msg: &NoMessages) {
            self.other_enter_cnt += 1;
        }

        #[hsm1_state]
        // This state has hdl 0
        fn other(&mut self, _msg: &NoMessages) -> StateResult!() {
            self.other_cnt += 1;
            transition_to!(initial)
        }

        fn other_exit(&mut self, _msg: &NoMessages) {
            self.other_exit_cnt += 1;
        }
    );

    let mut fsm = Test::new();
    assert_eq!(fsm.initial_enter_cnt, 0);
    assert_eq!(fsm.initial_cnt, 0);
    assert_eq!(fsm.initial_exit_cnt, 0);
    assert_eq!(fsm.other_enter_cnt, 0);
    assert_eq!(fsm.other_cnt, 0);
    assert_eq!(fsm.other_exit_cnt, 0);

    fsm.dispatch(&NoMessages);
    assert_eq!(fsm.initial_enter_cnt, 1);
    assert_eq!(fsm.initial_cnt, 1);
    assert_eq!(fsm.initial_exit_cnt, 1);
    assert_eq!(fsm.other_enter_cnt, 0);
    assert_eq!(fsm.other_cnt, 0);
    assert_eq!(fsm.other_exit_cnt, 0);

    fsm.dispatch(&NoMessages);
    assert_eq!(fsm.initial_enter_cnt, 1);
    assert_eq!(fsm.initial_cnt, 1);
    assert_eq!(fsm.initial_exit_cnt, 1);
    assert_eq!(fsm.other_enter_cnt, 1);
    assert_eq!(fsm.other_cnt, 1);
    assert_eq!(fsm.other_exit_cnt, 1);

    fsm.dispatch(&NoMessages);
    assert_eq!(fsm.initial_enter_cnt, 2);
    assert_eq!(fsm.initial_cnt, 2);
    assert_eq!(fsm.initial_exit_cnt, 2);
    assert_eq!(fsm.other_enter_cnt, 1);
    assert_eq!(fsm.other_cnt, 1);
    assert_eq!(fsm.other_exit_cnt, 1);

    fsm.dispatch(&NoMessages);
    assert_eq!(fsm.initial_enter_cnt, 2);
    assert_eq!(fsm.initial_cnt, 2);
    assert_eq!(fsm.initial_exit_cnt, 2);
    assert_eq!(fsm.other_enter_cnt, 2);
    assert_eq!(fsm.other_cnt, 2);
    assert_eq!(fsm.other_exit_cnt, 2);

    fsm.dispatch(&NoMessages);
    assert_eq!(fsm.initial_enter_cnt, 3);
    assert_eq!(fsm.initial_cnt, 3);
    assert_eq!(fsm.initial_exit_cnt, 3);
    assert_eq!(fsm.other_enter_cnt, 2);
    assert_eq!(fsm.other_cnt, 2);
    assert_eq!(fsm.other_exit_cnt, 2);
}

#[test]
fn test_transitions_between_leafs_of_trees() {
    hsm1!(
        struct Test {
            initial_base_enter_cnt: usize,
            initial_base_cnt: usize,
            initial_base_exit_cnt: usize,
            initial_enter_cnt: usize,
            initial_cnt: usize,
            initial_exit_cnt: usize,
            other_base_enter_cnt: usize,
            other_base_cnt: usize,
            other_base_exit_cnt: usize,
            other_enter_cnt: usize,
            other_cnt: usize,
            other_exit_cnt: usize,
        }

        fn initial_base_enter(&mut self, _msg: &NoMessages) {
            self.initial_base_enter_cnt += 1;
        }

        #[hsm1_state]
        // This state has hdl 0
        fn initial_base(&mut self, _msg: &NoMessages) -> StateResult!() {
            self.initial_base_cnt += 1;
            handled!()
        }

        fn initial_base_exit(&mut self, _msg: &NoMessages) {
            self.initial_base_exit_cnt += 1;
        }

        fn initial_enter(&mut self, _msg: &NoMessages) {
            self.initial_enter_cnt += 1;
        }

        #[hsm1_initial_state(initial_base)]
        // This state has hdl 0
        fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
            self.initial_cnt += 1;
            transition_to!(other)
        }

        fn initial_exit(&mut self, _msg: &NoMessages) {
            self.initial_exit_cnt += 1;
        }

        fn other_base_enter(&mut self, _msg: &NoMessages) {
            self.other_base_enter_cnt += 1;
        }

        #[hsm1_state]
        // This state has hdl 0
        fn other_base(&mut self, _msg: &NoMessages) -> StateResult!() {
            self.other_base_cnt += 1;
            handled!()
        }

        fn other_base_exit(&mut self, _msg: &NoMessages) {
            self.other_base_exit_cnt += 1;
        }

        fn other_enter(&mut self, _msg: &NoMessages) {
            self.other_enter_cnt += 1;
        }

        #[hsm1_state(other_base)]
        // This state has hdl 0
        fn other(&mut self, _msg: &NoMessages) -> StateResult!() {
            self.other_cnt += 1;
            transition_to!(initial)
        }

        fn other_exit(&mut self, _msg: &NoMessages) {
            self.other_exit_cnt += 1;
        }
    );

    let mut sm = Test::new();
    assert_eq!(sm.initial_base_enter_cnt, 0);
    assert_eq!(sm.initial_base_cnt, 0);
    assert_eq!(sm.initial_base_exit_cnt, 0);
    assert_eq!(sm.initial_enter_cnt, 0);
    assert_eq!(sm.initial_cnt, 0);
    assert_eq!(sm.initial_exit_cnt, 0);
    assert_eq!(sm.other_base_enter_cnt, 0);
    assert_eq!(sm.other_base_cnt, 0);
    assert_eq!(sm.other_base_exit_cnt, 0);
    assert_eq!(sm.other_enter_cnt, 0);
    assert_eq!(sm.other_cnt, 0);
    assert_eq!(sm.other_exit_cnt, 0);

    sm.dispatch(&NoMessages);
    assert_eq!(sm.initial_base_enter_cnt, 1);
    assert_eq!(sm.initial_base_cnt, 0);
    assert_eq!(sm.initial_base_exit_cnt, 1);
    assert_eq!(sm.initial_enter_cnt, 1);
    assert_eq!(sm.initial_cnt, 1);
    assert_eq!(sm.initial_exit_cnt, 1);
    assert_eq!(sm.other_base_enter_cnt, 0);
    assert_eq!(sm.other_base_cnt, 0);
    assert_eq!(sm.other_base_exit_cnt, 0);
    assert_eq!(sm.other_enter_cnt, 0);
    assert_eq!(sm.other_cnt, 0);
    assert_eq!(sm.other_exit_cnt, 0);

    sm.dispatch(&NoMessages);
    assert_eq!(sm.initial_base_enter_cnt, 1);
    assert_eq!(sm.initial_base_cnt, 0);
    assert_eq!(sm.initial_base_exit_cnt, 1);
    assert_eq!(sm.initial_enter_cnt, 1);
    assert_eq!(sm.initial_cnt, 1);
    assert_eq!(sm.initial_exit_cnt, 1);
    assert_eq!(sm.other_base_enter_cnt, 1);
    assert_eq!(sm.other_base_cnt, 0);
    assert_eq!(sm.other_base_exit_cnt, 1);
    assert_eq!(sm.other_enter_cnt, 1);
    assert_eq!(sm.other_cnt, 1);
    assert_eq!(sm.other_exit_cnt, 1);

    sm.dispatch(&NoMessages);
    assert_eq!(sm.initial_base_enter_cnt, 2);
    assert_eq!(sm.initial_base_cnt, 0);
    assert_eq!(sm.initial_base_exit_cnt, 2);
    assert_eq!(sm.initial_enter_cnt, 2);
    assert_eq!(sm.initial_cnt, 2);
    assert_eq!(sm.initial_exit_cnt, 2);
    assert_eq!(sm.other_base_enter_cnt, 1);
    assert_eq!(sm.other_base_cnt, 0);
    assert_eq!(sm.other_base_exit_cnt, 1);
    assert_eq!(sm.other_enter_cnt, 1);
    assert_eq!(sm.other_cnt, 1);
    assert_eq!(sm.other_exit_cnt, 1);

    sm.dispatch(&NoMessages);
    assert_eq!(sm.initial_base_enter_cnt, 2);
    assert_eq!(sm.initial_base_cnt, 0);
    assert_eq!(sm.initial_base_exit_cnt, 2);
    assert_eq!(sm.initial_enter_cnt, 2);
    assert_eq!(sm.initial_cnt, 2);
    assert_eq!(sm.initial_exit_cnt, 2);
    assert_eq!(sm.other_base_enter_cnt, 2);
    assert_eq!(sm.other_base_cnt, 0);
    assert_eq!(sm.other_base_exit_cnt, 2);
    assert_eq!(sm.other_enter_cnt, 2);
    assert_eq!(sm.other_cnt, 2);
    assert_eq!(sm.other_exit_cnt, 2);

    sm.dispatch(&NoMessages);
    assert_eq!(sm.initial_base_enter_cnt, 3);
    assert_eq!(sm.initial_base_cnt, 0);
    assert_eq!(sm.initial_base_exit_cnt, 3);
    assert_eq!(sm.initial_enter_cnt, 3);
    assert_eq!(sm.initial_cnt, 3);
    assert_eq!(sm.initial_exit_cnt, 3);
    assert_eq!(sm.other_base_enter_cnt, 2);
    assert_eq!(sm.other_base_cnt, 0);
    assert_eq!(sm.other_base_exit_cnt, 2);
    assert_eq!(sm.other_enter_cnt, 2);
    assert_eq!(sm.other_cnt, 2);
    assert_eq!(sm.other_exit_cnt, 2);
}
