use proc_macro_hsm1::{hsm1, hsm1_state, handled, not_handled, transition_to};
use std::collections::VecDeque;
use state_result::*;

// Simple FSM
hsm1!(
    struct SimpleFsm {
        initial_counter: u64,
    }

    #[hsm1_state]
    fn initial(&mut self) -> StateResult {
        // Mutate the state
        self.initial_counter += 1;

        // Let the parent state handle all invocations
        handled!()
    }
);

// Simple HSM
hsm1!(
    struct SimpleHsm {
        base_counter: u64,
        initial_counter: u64,
    }

    #[hsm1_state]
    fn base(&mut self) -> StateResult {
        // Mutate the state
        self.base_counter += 1;

        // Let the parent state handle all invocations
        handled!()
    }

    #[hsm1_state(base)]
    fn initial(&mut self) -> StateResult {
        // Mutate the state
        self.initial_counter += 1;

        // Let the parent state handle all invocations
        not_handled!()
    }
);

// A more complex Hsm
hsm1!(
    struct MyHsm {
        a_i32: i32,
    }

    fn non_state_fn(&mut self) {
        self.a_i32 += 1;
        println!("non_state_fn: self.data={}", self.a_i32);
    }

    #[hsm1_state]
    fn initial_parent(&mut self) -> StateResult {
        println!("{}: never executed", self.state_name());
        handled!()
    }

    fn initial_enter(&mut self) {
        println!("{}: enter self.a_i32={}", self.state_name(), self.a_i32);
    }

    #[hsm1_state] //(initial_parent)]
    fn initial(&mut self) -> StateResult {
        self.non_state_fn();
        println!("{}: self.a_i32={}", self.state_name(), self.a_i32);
        transition_to!(do_work)
    }

    fn initial_exit(&mut self) {
        println!("{}: exit  self.a_i32={}", self.state_name(), self.a_i32);
    }

    #[hsm1_state]
    fn do_work(&mut self) -> StateResult {
        self.a_i32 += 1;
        println!("{}: self.a_i32={}", self.state_name(), self.a_i32);

        transition_to!(done)
    }

    #[hsm1_state]
    fn done(&mut self) -> StateResult {
        self.a_i32 += 1;
        println!("{}: self.a_i32={}", self.state_name(), self.a_i32);

        handled!()
    }

    #[hsm1_state]
    fn do_nothing_ret_not_handled(&mut self) -> StateResult {
        self.a_i32 += 1;
        println!("{}: self.a_i32={}", self.state_name(), self.a_i32);

        not_handled!()
    }
);

fn main() {
    let mut simple_fsm = SimpleFsm::new();
    simple_fsm.dispatch();
    assert_eq!(simple_fsm.initial_counter, 1);
    println!("main: simple_fsm.initial_counter={}", simple_fsm.initial_counter);

    let mut simple_hsm = SimpleHsm::new();
    simple_hsm.dispatch();
    assert_eq!(simple_hsm.base_counter, 1);
    assert_eq!(simple_hsm.initial_counter, 1);
    println!("main: simple_hsm.base_counter={}", simple_hsm.base_counter);
    println!("main: simple_hsm.initial_counter={}", simple_hsm.initial_counter);

    let mut my_hsm = MyHsm::new();
    assert_eq!(my_hsm.smi.current_state_fns_hdl as usize, 1); //MyHsm::initial as usize);
    assert_eq!(my_hsm.smi.previous_state_fns_hdl as usize, 1); //MyHsm::initial as usize);
    assert!(my_hsm.smi.current_state_changed);

    my_hsm.a_i32 = 123;
    println!("main: my_hsm.a_i32={}", my_hsm.a_i32);

   // Invoke initial
    my_hsm.dispatch();
    println!("main: my_hsm.a_i32={}", my_hsm.a_i32);
    assert_eq!(my_hsm.smi.current_state_fns_hdl as usize, 2); //MyHsm::do_work as usize);
    assert_eq!(my_hsm.smi.previous_state_fns_hdl as usize, 1); //MyHsm::initial as usize);
    assert!(my_hsm.smi.current_state_changed);

    // Invoke do_work
    my_hsm.dispatch();
    println!("main: my_hsm.a_i32={}", my_hsm.a_i32);
    assert_eq!(my_hsm.smi.current_state_fns_hdl as usize, 3); //MyHsm::done as usize);
    assert_eq!(my_hsm.smi.previous_state_fns_hdl as usize, 2); //MyHsm::do_work as usize);
    assert!(my_hsm.smi.current_state_changed);

    // Invoke done
    my_hsm.dispatch();
    println!("main: my_hsm.a_i32={}", my_hsm.a_i32);
    assert_eq!(my_hsm.smi.current_state_fns_hdl as usize, 3); //MyHsm::done as usize);
    assert_eq!(my_hsm.smi.previous_state_fns_hdl as usize, 2); //MyHsm::do_work as usize);
    assert!(!my_hsm.smi.current_state_changed);

    // Invoke done again
    my_hsm.dispatch();
    println!("main: my_hsm.a_i32={}", my_hsm.a_i32);
    assert_eq!(my_hsm.smi.current_state_fns_hdl as usize, 3); //MyHsm::done as usize);
    assert_eq!(my_hsm.smi.previous_state_fns_hdl as usize, 2); //MyHsm::do_work as usize);
    assert!(!my_hsm.smi.current_state_changed);
}

#[cfg(test)]
mod tests {
    use proc_macro_hsm1::{hsm1, hsm1_state, handled, not_handled, transition_to};
    use std::collections::VecDeque;
    use state_result::*;

    #[test]
    fn test_initialization_via_default() {
        hsm1!(
            struct Test {}

            #[hsm1_state]
            fn initial(&mut self) -> StateResult {
                StateResult::Handled
            }
        );

        let fsm: Test = Default::default();
        assert_eq!(fsm.smi.current_state_fns_hdl as usize, 0); //Test::initial as usize);
        assert_eq!(fsm.smi.previous_state_fns_hdl as usize, 0); //Test::initial as usize);
        assert!(fsm.smi.current_state_changed);
    }

    #[test]
    fn test_dispatch() {
        hsm1!(
            struct TestDispatch {}

            #[hsm1_state]
            fn initial(&mut self) -> StateResult {
                StateResult::TransitionTo(1usize) //TestDispatch::done)
            }

            #[hsm1_state]
            fn done(&mut self) -> StateResult {
                StateResult::Handled
            }
        );

        let mut fsm = TestDispatch::new();
        assert_eq!(fsm.smi.current_state_fns_hdl as usize, 0); //TestDispatch::initial as usize);
        assert_eq!(fsm.smi.previous_state_fns_hdl as usize, 0); //TestDispatch::initial as usize);
        assert!(fsm.smi.current_state_changed);

        fsm.dispatch();
        assert_eq!(fsm.smi.current_state_fns_hdl as usize, 1); //TestDispatch::done as usize);
        assert_eq!(fsm.smi.previous_state_fns_hdl as usize, 0); //TestDispatch::initial as usize);
        assert!(fsm.smi.current_state_changed);

        fsm.dispatch();
        assert_eq!(fsm.smi.current_state_fns_hdl as usize, 1); //TestDispatch::done as usize);
        assert_eq!(fsm.smi.previous_state_fns_hdl as usize, 0); //TestDispatch::initial as usize);
        assert!(!fsm.smi.current_state_changed);

        fsm.dispatch();
        assert_eq!(fsm.smi.current_state_fns_hdl as usize, 1); //TestDispatch::done as usize);
        assert_eq!(fsm.smi.previous_state_fns_hdl as usize, 0); //TestDispatch::initial as usize);
        assert!(!fsm.smi.current_state_changed);
    }

    #[test]
    fn test_initialization_via_new() {
        hsm1!(
            struct Test {}

            #[hsm1_state]
            fn initial(&mut self) -> StateResult {
                StateResult::Handled
            }
        );

        let mut fsm = Test::new();
        assert_eq!(fsm.smi.current_state_fns_hdl as usize, 0); //Test::initial as usize);
        assert_eq!(fsm.smi.previous_state_fns_hdl as usize, 0); //Test::initial as usize);
        assert!(fsm.smi.current_state_changed);

        fsm.dispatch();
        assert_eq!(fsm.smi.current_state_fns_hdl as usize, 0); //Test::initial as usize);
        assert_eq!(fsm.smi.previous_state_fns_hdl as usize, 0); //Test::initial as usize);
        assert!(!fsm.smi.current_state_changed);
    }

    #[test]
    fn test_transition_to() {
        hsm1!(
            struct Test {}

            #[hsm1_state]
            fn initial(&mut self) -> StateResult {
                StateResult::TransitionTo(1usize) //Test::done)
            }

            #[hsm1_state]
            fn done(&mut self) -> StateResult {
                StateResult::Handled
            }
        );

        let mut fsm = Test::new();
        assert_eq!(fsm.smi.current_state_fns_hdl as usize, 0); //Test::initial as usize);
        assert_eq!(fsm.smi.previous_state_fns_hdl as usize, 0); //Test::initial as usize);
        assert!(fsm.smi.current_state_changed);

        fsm.dispatch();
        assert_eq!(fsm.smi.current_state_fns_hdl as usize, 1); //Test::done as usize);
        assert_eq!(fsm.smi.previous_state_fns_hdl as usize, 0); //Test::initial as usize);
        assert!(fsm.smi.current_state_changed);
    }

    #[test]
    fn test_no_enter_exit() {
        hsm1!(
            struct Test {
                initial_enter_cnt: usize,
                initial_cnt: usize,
                initial_exit_cnt: usize,
                done_enter_cnt: usize,
                done_cnt: usize,
                done_exit_cnt: usize,
            }

            #[hsm1_state]
            fn initial(&mut self) -> StateResult {
                self.initial_cnt += 1;
                StateResult::TransitionTo(1usize) //Test::done)
            }

            #[hsm1_state]
            fn done(&mut self) -> StateResult {
                self.done_cnt += 1;
                StateResult::Handled
            }
        );

        let mut fsm = Test::new();
        assert_eq!(fsm.initial_enter_cnt, 0);
        assert_eq!(fsm.initial_cnt, 0);
        assert_eq!(fsm.initial_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 0);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 0);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 1);
        assert_eq!(fsm.done_exit_cnt, 0);
    }

    #[test]
    fn test_enter() {
        hsm1!(
            struct Test {
                initial_enter_cnt: usize,
                initial_cnt: usize,
                initial_exit_cnt: usize,
                done_enter_cnt: usize,
                done_cnt: usize,
                done_exit_cnt: usize,
            }

            fn initial_enter(&mut self) {
                println!("test_enter: initial_enter");
                self.initial_enter_cnt += 1;
            }

            #[hsm1_state]
            fn initial(&mut self) -> StateResult {
                println!("test_enter: initial");
                self.initial_cnt += 1;
                StateResult::TransitionTo(1usize) //Test::done)
            }

            #[hsm1_state]
            fn done(&mut self) -> StateResult {
                println!("test_enter: done");
                self.done_cnt += 1;
                StateResult::Handled
            }

            fn done_enter(&mut self) {
                println!("test_enter: done_enter");
                self.done_enter_cnt += 1;
            }
        );

        let mut fsm = Test::new();
        assert_eq!(fsm.initial_enter_cnt, 0);
        assert_eq!(fsm.initial_cnt, 0);
        assert_eq!(fsm.initial_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 1);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 1);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 1);
        assert_eq!(fsm.done_cnt, 1);
        assert_eq!(fsm.done_exit_cnt, 0);
    }

    #[test]
    fn test_exit() {
        hsm1!(
            struct Test {
                initial_enter_cnt: usize,
                initial_cnt: usize,
                initial_exit_cnt: usize,
                done_enter_cnt: usize,
                done_cnt: usize,
                done_exit_cnt: usize,
            }

            #[hsm1_state]
            fn initial(&mut self) -> StateResult {
                self.initial_cnt += 1;
                StateResult::TransitionTo(1usize) //Test::done)
            }

            fn initial_exit(&mut self) {
                self.initial_exit_cnt += 1;
            }

            fn done_exit(&mut self) {
                self.done_exit_cnt += 1;
            }

            #[hsm1_state]
            fn done(&mut self) -> StateResult {
                self.done_cnt += 1;
                StateResult::Handled
            }
        );

        let mut fsm = Test::new();
        assert_eq!(fsm.initial_enter_cnt, 0);
        assert_eq!(fsm.initial_cnt, 0);
        assert_eq!(fsm.initial_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 0);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 1);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 0);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 1);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 1);
        assert_eq!(fsm.done_exit_cnt, 0);
    }

    #[test]
    fn test_both_enter_exit() {
        hsm1!(
            struct Test {
                initial_enter_cnt: usize,
                initial_cnt: usize,
                initial_exit_cnt: usize,
                do_work_enter_cnt: usize,
                do_work_cnt: usize,
                do_work_exit_cnt: usize,
                done_enter_cnt: usize,
                done_cnt: usize,
                done_exit_cnt: usize,
            }

            fn initial_enter(&mut self) {
                self.initial_enter_cnt += 1;
            }

            #[hsm1_state]
            fn initial(&mut self) -> StateResult {
                self.initial_cnt += 1;
                StateResult::TransitionTo(1) //Test::do_work)
            }

            fn initial_exit(&mut self) {
                self.initial_exit_cnt += 1;
            }

            fn do_work_exit(&mut self) {
                self.do_work_exit_cnt += 1;
            }

            #[hsm1_state]
            fn do_work(&mut self) -> StateResult {
                self.do_work_cnt += 1;
                if self.do_work_cnt < 3 {
                    StateResult::Handled
                } else {
                    StateResult::TransitionTo(2) //Test::done
                }
            }

            fn do_work_enter(&mut self) {
                self.do_work_enter_cnt += 1;
            }

            fn done_exit(&mut self) {
                self.done_exit_cnt += 1;
            }

            #[hsm1_state]
            fn done(&mut self) -> StateResult {
                self.done_cnt += 1;
                StateResult::Handled
            }

            fn done_enter(&mut self) {
                self.done_enter_cnt += 1;
            }
        );

        let mut fsm = Test::new();
        assert_eq!(fsm.initial_enter_cnt, 0);
        assert_eq!(fsm.initial_cnt, 0);
        assert_eq!(fsm.initial_exit_cnt, 0);
        assert_eq!(fsm.do_work_enter_cnt, 0);
        assert_eq!(fsm.do_work_cnt, 0);
        assert_eq!(fsm.do_work_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 1);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 1);
        assert_eq!(fsm.do_work_enter_cnt, 0);
        assert_eq!(fsm.do_work_cnt, 0);
        assert_eq!(fsm.do_work_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 1);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 1);
        assert_eq!(fsm.do_work_enter_cnt, 1);
        assert_eq!(fsm.do_work_cnt, 1);
        assert_eq!(fsm.do_work_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 1);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 1);
        assert_eq!(fsm.do_work_enter_cnt, 1);
        assert_eq!(fsm.do_work_cnt, 2);
        assert_eq!(fsm.do_work_exit_cnt, 0);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 1);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 1);
        assert_eq!(fsm.do_work_enter_cnt, 1);
        assert_eq!(fsm.do_work_cnt, 3);
        assert_eq!(fsm.do_work_exit_cnt, 1);
        assert_eq!(fsm.done_enter_cnt, 0);
        assert_eq!(fsm.done_cnt, 0);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 1);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 1);
        assert_eq!(fsm.do_work_enter_cnt, 1);
        assert_eq!(fsm.do_work_cnt, 3);
        assert_eq!(fsm.do_work_exit_cnt, 1);
        assert_eq!(fsm.done_enter_cnt, 1);
        assert_eq!(fsm.done_cnt, 1);
        assert_eq!(fsm.done_exit_cnt, 0);

        fsm.dispatch();
        assert_eq!(fsm.initial_enter_cnt, 1);
        assert_eq!(fsm.initial_cnt, 1);
        assert_eq!(fsm.initial_exit_cnt, 1);
        assert_eq!(fsm.do_work_enter_cnt, 1);
        assert_eq!(fsm.do_work_cnt, 3);
        assert_eq!(fsm.do_work_exit_cnt, 1);
        assert_eq!(fsm.done_enter_cnt, 1);
        assert_eq!(fsm.done_cnt, 2);
        assert_eq!(fsm.done_exit_cnt, 0);
    }

    #[test]
    fn test_parent() {
        hsm1!(
            struct Test {
                parent_enter_cnt: usize,
                parent_cnt: usize,
                parent_exit_cnt: usize,
                initial_enter_cnt: usize,
                initial_cnt: usize,
                initial_exit_cnt: usize,
            }

            #[hsm1_state]
            fn parent(&mut self) -> StateResult {
                self.parent_cnt += 1;
                handled!()
            }

            #[hsm1_state(parent)]
            fn initial(&mut self) -> StateResult {
                self.initial_cnt += 1;
                not_handled!()
            }
        );

        let mut hsm = Test::new();
        assert_eq!(hsm.parent_enter_cnt, 0);
        assert_eq!(hsm.parent_cnt, 0);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 0);
        assert_eq!(hsm.initial_cnt, 0);
        assert_eq!(hsm.initial_exit_cnt, 0);

        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 0);
        assert_eq!(hsm.parent_cnt, 1);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 0);
        assert_eq!(hsm.initial_cnt, 1);
        assert_eq!(hsm.initial_exit_cnt, 0);
    }

    #[test]
    fn test_parent_with_enter_exit() {
        hsm1!(
            struct Test {
                parent_enter_cnt: usize,
                parent_cnt: usize,
                parent_exit_cnt: usize,
                initial_enter_cnt: usize,
                initial_cnt: usize,
                initial_exit_cnt: usize,
            }

            fn parent_enter(&mut self) {
                self.parent_enter_cnt += 1;
            }

            #[hsm1_state]
            fn parent(&mut self) -> StateResult {
                self.parent_cnt += 1;
                handled!()
            }

            fn parent_exit(&mut self) {
                self.parent_exit_cnt += 1;
            }

            #[hsm1_state(parent)]
            fn initial(&mut self) -> StateResult {
                self.initial_cnt += 1;
                not_handled!()
            }
        );

        let mut hsm = Test::new();
        assert_eq!(hsm.parent_enter_cnt, 0);
        assert_eq!(hsm.parent_cnt, 0);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 0);
        assert_eq!(hsm.initial_cnt, 0);
        assert_eq!(hsm.initial_exit_cnt, 0);

        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 1);
        assert_eq!(hsm.parent_cnt, 1);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 0);
        assert_eq!(hsm.initial_cnt, 1);
        assert_eq!(hsm.initial_exit_cnt, 0);
    }

    #[test]
    fn test_one_tree() {
        hsm1!(
            struct Test {
                parent_enter_cnt: usize,
                parent_cnt: usize,
                parent_exit_cnt: usize,
                initial_enter_cnt: usize,
                initial_cnt: usize,
                initial_exit_cnt: usize,
                do_work_enter_cnt: usize,
                do_work_cnt: usize,
                do_work_exit_cnt: usize,
                done_enter_cnt: usize,
                done_cnt: usize,
                done_exit_cnt: usize,
            }

            fn parent_enter(&mut self) {
                self.parent_enter_cnt += 1;
            }

            #[hsm1_state]
            fn parent(&mut self) -> StateResult {
                self.parent_cnt += 1;
                handled!()
            }

            fn parent_exit(&mut self) {
                self.parent_exit_cnt += 1;
            }

            fn initial_enter(&mut self) {
                self.initial_enter_cnt += 1;
            }

            #[hsm1_state(parent)]
            fn initial(&mut self) -> StateResult {
                self.initial_cnt += 1;
                match self.initial_cnt {
                    1 => not_handled!(),
                    2 => handled!(),
                    _ => transition_to!(do_work),
                }
            }

            fn initial_exit(&mut self) {
                self.initial_exit_cnt += 1;
            }

            fn do_work_enter(&mut self) {
                self.do_work_enter_cnt += 1;
            }

            #[hsm1_state(parent)]
            fn do_work(&mut self) -> StateResult {
                self.do_work_cnt += 1;
                match self.do_work_cnt {
                    1 => handled!(),
                    2 => not_handled!(),
                    _ => transition_to!(done),
                }
            }

            fn do_work_exit(&mut self) {
                self.do_work_exit_cnt += 1;
            }

            fn done_enter(&mut self) {
                self.done_enter_cnt += 1;
            }

            #[hsm1_state(parent)]
            fn done(&mut self) -> StateResult {
                self.done_cnt += 1;
                transition_to!(parent)
            }

            fn done_exit(&mut self) {
                self.done_exit_cnt += 1;
            }
        );

        let mut hsm = Test::new();
        assert_eq!(hsm.parent_enter_cnt, 0);
        assert_eq!(hsm.parent_cnt, 0);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 0);
        assert_eq!(hsm.initial_cnt, 0);
        assert_eq!(hsm.initial_exit_cnt, 0);
        assert_eq!(hsm.do_work_enter_cnt, 0);
        assert_eq!(hsm.do_work_cnt, 0);
        assert_eq!(hsm.do_work_exit_cnt, 0);
        assert_eq!(hsm.done_enter_cnt, 0);
        assert_eq!(hsm.done_cnt, 0);
        assert_eq!(hsm.done_exit_cnt, 0);

        // Into initial which returned not_handled!()
        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 1);
        assert_eq!(hsm.parent_cnt, 1);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 1);
        assert_eq!(hsm.initial_cnt, 1);
        assert_eq!(hsm.initial_exit_cnt, 0);
        assert_eq!(hsm.do_work_enter_cnt, 0);
        assert_eq!(hsm.do_work_cnt, 0);
        assert_eq!(hsm.do_work_exit_cnt, 0);
        assert_eq!(hsm.done_enter_cnt, 0);
        assert_eq!(hsm.done_cnt, 0);
        assert_eq!(hsm.done_exit_cnt, 0);

        // In initial which returned handled!()
        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 1);
        assert_eq!(hsm.parent_cnt, 1);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 1);
        assert_eq!(hsm.initial_cnt, 2);
        assert_eq!(hsm.initial_exit_cnt, 0);
        assert_eq!(hsm.do_work_enter_cnt, 0);
        assert_eq!(hsm.do_work_cnt, 0);
        assert_eq!(hsm.do_work_exit_cnt, 0);
        assert_eq!(hsm.done_enter_cnt, 0);
        assert_eq!(hsm.done_cnt, 0);
        assert_eq!(hsm.done_exit_cnt, 0);

        // In initial which returned transition_to!(do_work)
        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 1);
        assert_eq!(hsm.parent_cnt, 1);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 1);
        assert_eq!(hsm.initial_cnt, 3);
        assert_eq!(hsm.initial_exit_cnt, 1);
        assert_eq!(hsm.do_work_enter_cnt, 0);
        assert_eq!(hsm.do_work_cnt, 0);
        assert_eq!(hsm.do_work_exit_cnt, 0);
        assert_eq!(hsm.done_enter_cnt, 0);
        assert_eq!(hsm.done_cnt, 0);
        assert_eq!(hsm.done_exit_cnt, 0);

        // Into do_work returned handled!()
        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 1);
        assert_eq!(hsm.parent_cnt, 1);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 1);
        assert_eq!(hsm.initial_cnt, 3);
        assert_eq!(hsm.initial_exit_cnt, 1);
        assert_eq!(hsm.do_work_enter_cnt, 1);
        assert_eq!(hsm.do_work_cnt, 1);
        assert_eq!(hsm.do_work_exit_cnt, 0);
        assert_eq!(hsm.done_enter_cnt, 0);
        assert_eq!(hsm.done_cnt, 0);
        assert_eq!(hsm.done_exit_cnt, 0);

        // In do_work returned not_handled!()
        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 1);
        assert_eq!(hsm.parent_cnt, 2);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 1);
        assert_eq!(hsm.initial_cnt, 3);
        assert_eq!(hsm.initial_exit_cnt, 1);
        assert_eq!(hsm.do_work_enter_cnt, 1);
        assert_eq!(hsm.do_work_cnt, 2);
        assert_eq!(hsm.do_work_exit_cnt, 0);
        assert_eq!(hsm.done_enter_cnt, 0);
        assert_eq!(hsm.done_cnt, 0);
        assert_eq!(hsm.done_exit_cnt, 0);

        // In do_work returned transition_to!(done)
        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 1);
        assert_eq!(hsm.parent_cnt, 2);
        assert_eq!(hsm.parent_exit_cnt, 0);
        assert_eq!(hsm.initial_enter_cnt, 1);
        assert_eq!(hsm.initial_cnt, 3);
        assert_eq!(hsm.initial_exit_cnt, 1);
        assert_eq!(hsm.do_work_enter_cnt, 1);
        assert_eq!(hsm.do_work_cnt, 3);
        assert_eq!(hsm.do_work_exit_cnt, 1);
        assert_eq!(hsm.done_enter_cnt, 0);
        assert_eq!(hsm.done_cnt, 0);
        assert_eq!(hsm.done_exit_cnt, 0);

        // Into done always returns transition_to!(parent)
        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 1);
        assert_eq!(hsm.parent_cnt, 2);
        assert_eq!(hsm.parent_exit_cnt, 1);
        assert_eq!(hsm.initial_enter_cnt, 1);
        assert_eq!(hsm.initial_cnt, 3);
        assert_eq!(hsm.initial_exit_cnt, 1);
        assert_eq!(hsm.do_work_enter_cnt, 1);
        assert_eq!(hsm.do_work_cnt, 3);
        assert_eq!(hsm.do_work_exit_cnt, 1);
        assert_eq!(hsm.done_enter_cnt, 1);
        assert_eq!(hsm.done_cnt, 1);
        assert_eq!(hsm.done_exit_cnt, 1);

        // Into parent always returns handled!()
        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 2);
        assert_eq!(hsm.parent_cnt, 3);
        assert_eq!(hsm.parent_exit_cnt, 1);
        assert_eq!(hsm.initial_enter_cnt, 1);
        assert_eq!(hsm.initial_cnt, 3);
        assert_eq!(hsm.initial_exit_cnt, 1);
        assert_eq!(hsm.do_work_enter_cnt, 1);
        assert_eq!(hsm.do_work_cnt, 3);
        assert_eq!(hsm.do_work_exit_cnt, 1);
        assert_eq!(hsm.done_enter_cnt, 1);
        assert_eq!(hsm.done_cnt, 1);
        assert_eq!(hsm.done_exit_cnt, 1);

        // Into parent always returns handled!()
        hsm.dispatch();
        assert_eq!(hsm.parent_enter_cnt, 2);
        assert_eq!(hsm.parent_cnt, 4);
        assert_eq!(hsm.parent_exit_cnt, 1);
        assert_eq!(hsm.initial_enter_cnt, 1);
        assert_eq!(hsm.initial_cnt, 3);
        assert_eq!(hsm.initial_exit_cnt, 1);
        assert_eq!(hsm.do_work_enter_cnt, 1);
        assert_eq!(hsm.do_work_cnt, 3);
        assert_eq!(hsm.do_work_exit_cnt, 1);
        assert_eq!(hsm.done_enter_cnt, 1);
        assert_eq!(hsm.done_cnt, 1);
        assert_eq!(hsm.done_exit_cnt, 1);
    }
}
