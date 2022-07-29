use hsm1::{handled, hsm1, hsm1_state, transition_to, StateResult};

struct NoMessages;

hsm1!(
    struct Hsm {
        base_enter_cnt: usize,
        base_cnt: usize,
        base_exit_cnt: usize,
        intermediate_enter_cnt: usize,
        intermediate_cnt: usize,
        intermediate_exit_cnt: usize,
        bottom_enter_cnt: usize,
        bottom_cnt: usize,
        bottom_exit_cnt: usize,
    }

    fn initial_enter(&mut self, msg: &NoMessages) {
        self.base_enter_cnt += 1
    }

    #[hsm1_state]
    fn initial(&mut self, msg: &NoMessages) -> StateResult!() {
        self.base_cnt += 1;
        transition_to!(intermediate)
    }

    fn initial_exit(&mut self, msg: &NoMessages) {
        self.base_exit_cnt += 1;
    }

    fn intermediate_enter(&mut self, msg: &NoMessages) {
        self.intermediate_enter_cnt += 1;
    }

    #[hsm1_state(initial)]
    fn intermediate(&mut self, msg: &NoMessages) -> StateResult!() {
        self.intermediate_cnt += 1;
        handled!()
    }

    fn intermediate_exit(&mut self, msg: &NoMessages) {
        self.intermediate_exit_cnt += 1;
    }

    fn bottom_enter(&mut self, msg: &NoMessages) {
        self.bottom_enter_cnt += 1;
    }

    #[hsm1_state(intermediate)]
    fn bottom(&mut self, msg: &NoMessages) -> StateResult!() {
        self.bottom_cnt += 1;
        transition_to!(intermediate)
    }

    fn bottom_exit(&mut self, msg: &NoMessages) {
        self.bottom_exit_cnt += 1;
    }
);

fn main() {
    println!("hsm-3-states");

    // Create a sm and validate it's in the expected state
    let mut sm = Hsm::new();
    assert_eq!(sm.base_enter_cnt, 0);
    assert_eq!(sm.base_cnt, 0);
    assert_eq!(sm.base_exit_cnt, 0);
    assert_eq!(sm.intermediate_enter_cnt, 0);
    assert_eq!(sm.intermediate_cnt, 0);
    assert_eq!(sm.intermediate_exit_cnt, 0);
    assert_eq!(sm.bottom_enter_cnt, 0);
    assert_eq!(sm.bottom_cnt, 0);
    assert_eq!(sm.bottom_exit_cnt, 0);

    // base process msg returns TranitionTo intermediate, its child
    sm.dispatch(&NoMessages);
    assert_eq!(sm.base_enter_cnt, 1);
    assert_eq!(sm.base_cnt, 1);
    assert_eq!(sm.base_exit_cnt, 1); // BUG, we're going to a child shouldn't exit base??
    assert_eq!(sm.intermediate_enter_cnt, 0);
    assert_eq!(sm.intermediate_cnt, 0);
    assert_eq!(sm.intermediate_exit_cnt, 0);
    assert_eq!(sm.bottom_enter_cnt, 0);
    assert_eq!(sm.bottom_cnt, 0);
    assert_eq!(sm.bottom_exit_cnt, 0);

    // intermediate process message returns Handled
    sm.dispatch(&NoMessages);
    assert_eq!(sm.base_enter_cnt, 1);
    assert_eq!(sm.base_cnt, 1);
    assert_eq!(sm.base_exit_cnt, 1); // OK it didn't change, but should actually be 0
    assert_eq!(sm.intermediate_enter_cnt, 1);
    assert_eq!(sm.intermediate_cnt, 1);
    assert_eq!(sm.intermediate_exit_cnt, 0);
    assert_eq!(sm.bottom_enter_cnt, 0);
    assert_eq!(sm.bottom_cnt, 0);
    assert_eq!(sm.bottom_exit_cnt, 0);

    // intermediate process message returns Handled
    sm.dispatch(&NoMessages);
    assert_eq!(sm.base_enter_cnt, 1);
    assert_eq!(sm.base_cnt, 1);
    assert_eq!(sm.base_exit_cnt, 1); // OK it didn't change, but should actually be 0
    assert_eq!(sm.intermediate_enter_cnt, 1);
    assert_eq!(sm.intermediate_cnt, 2);
    assert_eq!(sm.intermediate_exit_cnt, 0);
    assert_eq!(sm.bottom_enter_cnt, 0);
    assert_eq!(sm.bottom_cnt, 0);
    assert_eq!(sm.bottom_exit_cnt, 0);
}
