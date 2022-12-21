use proc_macro_hsm1::{
    handled, hsm1, hsm1_initial_state, hsm1_state, not_handled, transition_to, StateResult,
};

struct NoMessages;

// Simple FSM
hsm1!(
    struct SimpleFsm {
        initial_counter: u64,
    }

    #[hsm1_initial_state]
    fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
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
    fn base(&mut self, _msg: &NoMessages) -> StateResult!() {
        // Mutate the state
        self.base_counter += 1;

        // Let the parent state handle all invocations
        handled!()
    }

    #[hsm1_initial_state(base)]
    fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
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
    fn initial_parent(&mut self, _msg: &NoMessages) -> StateResult!() {
        println!("{}: never executed", self.state_name());
        handled!()
    }

    fn initial_enter(&mut self, _msg: &NoMessages) {
        println!("{}: enter self.a_i32={}", self.state_name(), self.a_i32);
    }

    #[hsm1_initial_state]
    fn initial(&mut self, _msg: &NoMessages) -> StateResult!() {
        self.non_state_fn();
        println!("{}: self.a_i32={}", self.state_name(), self.a_i32);
        transition_to!(do_work)
    }

    fn initial_exit(&mut self, _msg: &NoMessages) {
        println!("{}: exit  self.a_i32={}", self.state_name(), self.a_i32);
    }

    #[hsm1_state]
    fn do_work(&mut self, _msg: &NoMessages) -> StateResult!() {
        self.a_i32 += 1;
        println!("{}: self.a_i32={}", self.state_name(), self.a_i32);

        transition_to!(done)
    }

    #[hsm1_state]
    fn done(&mut self, _msg: &NoMessages) -> StateResult!() {
        self.a_i32 += 1;
        println!("{}: self.a_i32={}", self.state_name(), self.a_i32);

        handled!()
    }

    #[hsm1_state]
    fn do_nothing_ret_not_handled(&mut self, _msg: &NoMessages) -> StateResult!() {
        self.a_i32 += 1;
        println!("{}: self.a_i32={}", self.state_name(), self.a_i32);

        not_handled!()
    }
);

#[test]
fn multiple_state_machines() {
    let msg = NoMessages;

    let mut simple_fsm = SimpleFsm::new();
    simple_fsm.dispatch(&msg);
    assert_eq!(simple_fsm.initial_counter, 1);
    println!(
        "main: simple_fsm.initial_counter={}",
        simple_fsm.initial_counter
    );

    let mut simple_hsm = SimpleHsm::new();
    simple_hsm.dispatch(&msg);
    assert_eq!(simple_hsm.base_counter, 1);
    assert_eq!(simple_hsm.initial_counter, 1);
    println!("main: simple_hsm.base_counter={}", simple_hsm.base_counter);
    println!(
        "main: simple_hsm.initial_counter={}",
        simple_hsm.initial_counter
    );

    let mut my_hsm = MyHsm::new();
    assert_eq!(my_hsm.smi.current_state_fns_hdl as usize, 1); //MyHsm::initial as usize);
    assert_eq!(my_hsm.smi.previous_state_fns_hdl as usize, 1); //MyHsm::initial as usize);
    assert!(my_hsm.smi.current_state_changed);

    my_hsm.a_i32 = 123;
    println!("main: my_hsm.a_i32={}", my_hsm.a_i32);

    // Invoke initial
    my_hsm.dispatch(&msg);
    println!("main: my_hsm.a_i32={}", my_hsm.a_i32);
    assert_eq!(my_hsm.smi.current_state_fns_hdl as usize, 2); //MyHsm::do_work as usize);
    assert_eq!(my_hsm.smi.previous_state_fns_hdl as usize, 1); //MyHsm::initial as usize);
    assert!(my_hsm.smi.current_state_changed);

    // Invoke do_work
    my_hsm.dispatch(&msg);
    println!("main: my_hsm.a_i32={}", my_hsm.a_i32);
    assert_eq!(my_hsm.smi.current_state_fns_hdl as usize, 3); //MyHsm::done as usize);
    assert_eq!(my_hsm.smi.previous_state_fns_hdl as usize, 2); //MyHsm::do_work as usize);
    assert!(my_hsm.smi.current_state_changed);

    // Invoke done
    my_hsm.dispatch(&msg);
    println!("main: my_hsm.a_i32={}", my_hsm.a_i32);
    assert_eq!(my_hsm.smi.current_state_fns_hdl as usize, 3); //MyHsm::done as usize);
    assert_eq!(my_hsm.smi.previous_state_fns_hdl as usize, 2); //MyHsm::do_work as usize);
    assert!(!my_hsm.smi.current_state_changed);

    // Invoke done again
    my_hsm.dispatch(&msg);
    println!("main: my_hsm.a_i32={}", my_hsm.a_i32);
    assert_eq!(my_hsm.smi.current_state_fns_hdl as usize, 3); //MyHsm::done as usize);
    assert_eq!(my_hsm.smi.previous_state_fns_hdl as usize, 2); //MyHsm::do_work as usize);
    assert!(!my_hsm.smi.current_state_changed);
}
