use proc_macro_hsm1::{handled, hsm1, hsm1_state};
use state_result::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum SimpleFsmProtocol {
    Add {
        tx_response: Option<std::sync::mpsc::Sender<SimpleFsmProtocol>>,
        f1: i32,
    },
    Get {
        tx_response: Option<std::sync::mpsc::Sender<SimpleFsmProtocol>>,
        data: i32,
    },
}

// Simple FSM
hsm1!(
    struct SimpleFsm {
        initial_enter_counter: u64,
        initial_counter: u64,
        initial_exit_counter: u64,
        data: i32,
    }

    fn initial_enter(&mut self, _msg: &mut SimpleFsmProtocol) {
        self.initial_enter_counter += 1;
    }

    #[hsm1_state]
    fn initial(&mut self, msg: &mut SimpleFsmProtocol) -> StateResult {
        self.initial_counter += 1;
        // Mutate the state
        let sr: StateResult = match msg {
            SimpleFsmProtocol::Add { tx_response: _, f1 } => {
                println!("SimpleFsm::initial: msg Add");
                self.data += *f1;

                handled!()
            }
            SimpleFsmProtocol::Get {
                tx_response: _,
                data,
            } => {
                println!("SimpleFsm::initial: msg Get");
                *data += self.data; // Enable if mst: &mut SimpleFsmProtocol

                handled!()
            }
        };
        println!("initial: data={}", self.data);

        // Let the parent state handle all invocations
        sr
    }

    fn initial_exit(&mut self, _msg: &mut SimpleFsmProtocol) {
        self.initial_exit_counter += 1;
    }
);

fn main() {
    let mut simple_fsm = SimpleFsm::new();
    assert_eq!(simple_fsm.initial_enter_counter, 0);
    assert_eq!(simple_fsm.initial_counter, 0);
    assert_eq!(simple_fsm.initial_exit_counter, 0);

    let mut msg = SimpleFsmProtocol::Add {
        tx_response: None,
        f1: 15,
    };
    simple_fsm.dispatch(&mut msg);
    assert_eq!(simple_fsm.data, 15);
    assert_eq!(simple_fsm.initial_enter_counter, 1);
    assert_eq!(simple_fsm.initial_counter, 1);
    assert_eq!(simple_fsm.initial_exit_counter, 0);
    println!(
        "main: simple_fsm.initial_counter={}",
        simple_fsm.initial_counter
    );

    let mut msg = SimpleFsmProtocol::Get {
        tx_response: None,
        data: 0,
    };
    simple_fsm.dispatch(&mut msg);
    match msg {
        SimpleFsmProtocol::Get {
            tx_response: _,
            data,
        } => {
            assert_eq!(simple_fsm.data, data);
            assert_eq!(data, 15);
            assert_eq!(simple_fsm.data, 15);
        }
        _ => panic!("Should Never Happen"),
    }
    assert_eq!(simple_fsm.data, 15);
    assert_eq!(simple_fsm.initial_enter_counter, 1);
    assert_eq!(simple_fsm.initial_counter, 2);
    //assert_eq!(simple_fsm.initial_exit_counter, 0);
    println!(
        "main: simple_fsm.initial_counter={}",
        simple_fsm.initial_counter
    );
}
