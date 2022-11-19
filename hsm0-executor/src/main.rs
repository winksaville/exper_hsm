#![allow(unused)]
use std::{cell::RefCell, rc::Rc};

use hsm0_executor::{Executor, StateResult, Handled, StateInfo};

pub struct StateMachine {
    state: i32,
}

// Create a Protocol
pub struct NoMessages;

const MAX_STATES: usize = 1;
const IDX_STATE1: usize = 0;

impl StateMachine {
    fn new() -> Executor<Self, NoMessages> {
        let sm = StateMachine {
            state: 0,
        };

        let sme = Executor::new(Rc::clone(&sm), MAX_STATES);

        sme.borrow_mut()
            .state(StateInfo::new("state1", None, Self::state1, None, None))
            .initialize(IDX_STATE1)
            .expect("Unexpected error initializing");

        let x = sm.borrow().get_sme();

        // Validate initialization complete, panic if not
        if sm.borrow_mut().sme.is_none() {
            panic!("StateMacine.sme is not initialized");
        }

        sme
    }

    fn get_sme(&self) -> Rc<RefCell<Executor<Self, NoMessages>>> {
        match &self.sme {
            Some(sme) => Rc::clone(&sme),
            None => panic!("StateMachine.sme is not initialized"),
        }
    }

    fn state1(&mut self, e: &Executor<Self, NoMessages>, _msg: &NoMessages) -> StateResult {
        // Enabling the first println! below causes:
        //  thread 'main' panicked at 'already mutably borrowed: BorrowError', hsm0-executor/src/main.rs:54:41
        //println!("{}:+", self.get_sme().borrow().get_state_name(IDX_STATE1));
        println!("{}:+", e.get_state_name(IDX_STATE1));

        self.state += 1;

        println!("{}:-", "state1");
        (Handled::Yes, None)
    }
}


#[allow(unused)]
fn main() {
    println!("main:+");

    // Create a sme and validate it's in the expected state
    let sme = StateMachine::new();
    assert_eq!(std::mem::size_of_val(sme.borrow().get_sm()), 8);
    assert_eq!(sme.borrow().get_state_enter_cnt(IDX_STATE1), 0);
    assert_eq!(sme.borrow().get_state_process_cnt(IDX_STATE1), 0);
    assert_eq!(sme.borrow().get_state_exit_cnt(IDX_STATE1), 0);
    assert_eq!(sme.borrow().get_sm().borrow().state, 0);

    sme.borrow_mut().dispatch(&NoMessages);
    assert_eq!(sme.borrow().get_state_enter_cnt(IDX_STATE1), 0);
    assert_eq!(sme.borrow().get_state_process_cnt(IDX_STATE1), 1);
    assert_eq!(sme.borrow().get_state_exit_cnt(IDX_STATE1), 0);
    assert_eq!(sme.borrow().get_sm().borrow().state, 1);

    sme.borrow_mut().dispatch(&NoMessages);
    assert_eq!(sme.borrow().get_state_enter_cnt(IDX_STATE1), 0);
    assert_eq!(sme.borrow().get_state_process_cnt(IDX_STATE1), 2);
    assert_eq!(sme.borrow().get_state_exit_cnt(IDX_STATE1), 0);
    assert_eq!(sme.borrow().get_sm().borrow().state, 2);

    println!("main:-");
}
