#![allow(unused)]
use std::{cell::RefCell, rc::Rc};

use hsm0_executor::{Executor, Handled, StateInfo, StateResult};

#[derive(Debug)]
pub struct StateMachine {
    state: i32,
}

// Create a Protocol
#[derive(Clone, Debug)]
pub enum Messages {
    Val { val: i32 },
}

const MAX_STATES: usize = 2;
const IDX_STATE1: usize = 0;
const IDX_STATE2: usize = 1;

impl StateMachine {
    fn new() -> Executor<Self, Messages> {
        let sm = Rc::new(RefCell::new(StateMachine { state: 0 }));

        let mut sme = Executor::new(Rc::clone(&sm), MAX_STATES);

        sme.state(StateInfo::new("state1", None, Self::state1, None, None))
            .state(StateInfo::new("state2", None, Self::state2, None, None))
            .initialize(IDX_STATE1)
            .expect("Unexpected error initializing");

        sme
    }

    fn state1(&mut self, e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
        println!("{}:+", e.get_state_name(IDX_STATE1));

        // Defer messages
        e.defer_send(msg.clone());

        println!("{}:-", e.get_state_name(IDX_STATE1));
        (Handled::Yes, Some(IDX_STATE2))
    }

    fn state2(&mut self, e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
        println!("{}:+", e.get_state_name(IDX_STATE2));

        match msg {
            Messages::Val { val } => {
                self.state -= val;
            }
        }

        println!("{}:-", e.get_state_name(IDX_STATE2));
        (Handled::Yes, Some(IDX_STATE1))
    }
}

#[allow(unused)]
fn main() {
    println!("main:+");

    // Create a sme and validate it's in the expected state
    let mut sme = StateMachine::new();
    assert_eq!(std::mem::size_of_val(sme.get_sm()), 8);
    assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 0);
    assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 0);
    assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
    assert_eq!(sme.get_state_enter_cnt(IDX_STATE2), 0);
    assert_eq!(sme.get_state_process_cnt(IDX_STATE2), 0);
    assert_eq!(sme.get_state_exit_cnt(IDX_STATE2), 0);
    assert_eq!(sme.get_sm().borrow().state, 0);

    // msg.val == 1 will be deferred and processed in state2
    let msg = Messages::Val { val: 1 };
    sme.dispatcher(&msg);
    assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 0);
    assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 1);
    assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
    assert_eq!(sme.get_state_enter_cnt(IDX_STATE2), 0);
    assert_eq!(sme.get_state_process_cnt(IDX_STATE2), 1);
    assert_eq!(sme.get_state_exit_cnt(IDX_STATE2), 0);

    // msg.val == 1 was deferred and processed in state2
    assert_eq!(sme.get_sm().borrow().state, -1);
    // which transitioned to "state2" and which transitioned back to "state1"
    assert_eq!(sme.get_current_state_name(), "state1");

    // msg.val == 2 will be deferred and processed in state2
    let msg = Messages::Val { val: 2 };
    sme.dispatcher(&msg);
    assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 0);
    assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 2);
    assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
    assert_eq!(sme.get_state_enter_cnt(IDX_STATE2), 0);
    assert_eq!(sme.get_state_process_cnt(IDX_STATE2), 2);
    assert_eq!(sme.get_state_exit_cnt(IDX_STATE2), 0);

    // msg.val == 2 was deferred and processed in state2
    assert_eq!(sme.get_sm().borrow().state, -3);

    // which transitioned to "state2" and which transitioned back to "state1"
    assert_eq!(sme.get_current_state_name(), "state1");

    println!("main:-");
}
