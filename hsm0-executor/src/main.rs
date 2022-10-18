use custom_logger::env_logger_init;

use hsm0_executor::{StateInfo, StateMachineExecutor, StateResult};

// StateMachine simply transitions back and forth
// between initial and other.
//
//                base=0
//        --------^  ^-------
//       /                   \
//      /                     \
//    other=2   <======>   initial=1

#[derive(Default)]
pub struct StateMachine;

// Create a Protocol with no messages
pub struct NoMessages;

const MAX_STATE_FNS: usize = 3;
const BASE_HDL: usize = 0;
const INITIAL_HDL: usize = 1;
const OTHER_HDL: usize = 2;

impl StateMachine {
    pub fn new() -> StateMachineExecutor<Self, NoMessages> {
        let sm = StateMachine::default();
        let mut sme = StateMachineExecutor::build(sm, MAX_STATE_FNS, INITIAL_HDL);

        sme.add_state(StateInfo::new(
            "base",
            Some(Self::base_enter),
            Self::base,
            Some(Self::base_exit),
            None,
        ))
        .add_state(StateInfo::new(
            "initial",
            Some(Self::initial_enter),
            Self::initial,
            Some(Self::initial_exit),
            Some(BASE_HDL),
        ))
        .add_state(StateInfo::new(
            "other",
            Some(Self::other_enter),
            Self::other,
            Some(Self::other_exit),
            Some(BASE_HDL),
        ))
        .initialize();

        log::trace!(
            "new: inital state={} enter_fnss_hdls={:?}",
            sme.current_state_name(),
            sme.enter_fns_hdls
        );

        sme
    }

    fn base_enter(&mut self, _msg: &NoMessages) {}

    // This state has hdl 0
    fn base(&mut self, _msg: &NoMessages) -> StateResult {
        StateResult::Handled
    }

    fn base_exit(&mut self, _msg: &NoMessages) {}

    fn initial_enter(&mut self, _msg: &NoMessages) {}

    // This state has hdl 0
    fn initial(&mut self, _msg: &NoMessages) -> StateResult {
        StateResult::TransitionTo(OTHER_HDL)
    }

    fn initial_exit(&mut self, _msg: &NoMessages) {}

    fn other_enter(&mut self, _msg: &NoMessages) {}

    // This state has hdl 0
    fn other(&mut self, _msg: &NoMessages) -> StateResult {
        StateResult::TransitionTo(INITIAL_HDL)
    }

    fn other_exit(&mut self, _msg: &NoMessages) {}
}

fn test_transition_between_leafs_in_a_tree() {
    // Create a sme and validate it's in the expected state
    let mut sme = StateMachine::new();
    assert_eq!(std::mem::size_of_val(sme.get_sm()), 0);
    assert_eq!(sme.get_state_fns_enter_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_process_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_exit_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_enter_cnt(INITIAL_HDL), 0);
    assert_eq!(sme.get_state_fns_process_cnt(INITIAL_HDL), 0);
    assert_eq!(sme.get_state_fns_exit_cnt(INITIAL_HDL), 0);
    assert_eq!(sme.get_state_fns_enter_cnt(OTHER_HDL), 0);
    assert_eq!(sme.get_state_fns_process_cnt(OTHER_HDL), 0);
    assert_eq!(sme.get_state_fns_exit_cnt(OTHER_HDL), 0);

    sme.dispatch(&NoMessages);
    assert_eq!(sme.get_state_fns_enter_cnt(BASE_HDL), 1);
    assert_eq!(sme.get_state_fns_process_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_exit_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_enter_cnt(INITIAL_HDL), 1);
    assert_eq!(sme.get_state_fns_process_cnt(INITIAL_HDL), 1);
    assert_eq!(sme.get_state_fns_exit_cnt(INITIAL_HDL), 1);
    assert_eq!(sme.get_state_fns_enter_cnt(OTHER_HDL), 0);
    assert_eq!(sme.get_state_fns_process_cnt(OTHER_HDL), 0);
    assert_eq!(sme.get_state_fns_exit_cnt(OTHER_HDL), 0);

    sme.dispatch(&NoMessages);
    assert_eq!(sme.get_state_fns_enter_cnt(BASE_HDL), 1);
    assert_eq!(sme.get_state_fns_process_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_exit_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_enter_cnt(INITIAL_HDL), 1);
    assert_eq!(sme.get_state_fns_process_cnt(INITIAL_HDL), 1);
    assert_eq!(sme.get_state_fns_exit_cnt(INITIAL_HDL), 1);
    assert_eq!(sme.get_state_fns_enter_cnt(OTHER_HDL), 1);
    assert_eq!(sme.get_state_fns_process_cnt(OTHER_HDL), 1);
    assert_eq!(sme.get_state_fns_exit_cnt(OTHER_HDL), 1);

    sme.dispatch(&NoMessages);
    assert_eq!(sme.get_state_fns_enter_cnt(BASE_HDL), 1);
    assert_eq!(sme.get_state_fns_process_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_exit_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_enter_cnt(INITIAL_HDL), 2);
    assert_eq!(sme.get_state_fns_process_cnt(INITIAL_HDL), 2);
    assert_eq!(sme.get_state_fns_exit_cnt(INITIAL_HDL), 2);
    assert_eq!(sme.get_state_fns_enter_cnt(OTHER_HDL), 1);
    assert_eq!(sme.get_state_fns_process_cnt(OTHER_HDL), 1);
    assert_eq!(sme.get_state_fns_exit_cnt(OTHER_HDL), 1);

    sme.dispatch(&NoMessages);
    assert_eq!(sme.get_state_fns_enter_cnt(BASE_HDL), 1);
    assert_eq!(sme.get_state_fns_process_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_exit_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_enter_cnt(INITIAL_HDL), 2);
    assert_eq!(sme.get_state_fns_process_cnt(INITIAL_HDL), 2);
    assert_eq!(sme.get_state_fns_exit_cnt(INITIAL_HDL), 2);
    assert_eq!(sme.get_state_fns_enter_cnt(OTHER_HDL), 2);
    assert_eq!(sme.get_state_fns_process_cnt(OTHER_HDL), 2);
    assert_eq!(sme.get_state_fns_exit_cnt(OTHER_HDL), 2);

    sme.dispatch(&NoMessages);
    assert_eq!(sme.get_state_fns_enter_cnt(BASE_HDL), 1);
    assert_eq!(sme.get_state_fns_process_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_exit_cnt(BASE_HDL), 0);
    assert_eq!(sme.get_state_fns_enter_cnt(INITIAL_HDL), 3);
    assert_eq!(sme.get_state_fns_process_cnt(INITIAL_HDL), 3);
    assert_eq!(sme.get_state_fns_exit_cnt(INITIAL_HDL), 3);
    assert_eq!(sme.get_state_fns_enter_cnt(OTHER_HDL), 2);
    assert_eq!(sme.get_state_fns_process_cnt(OTHER_HDL), 2);
    assert_eq!(sme.get_state_fns_exit_cnt(OTHER_HDL), 2);
}

fn main() {
    println!("main");
    env_logger_init("info");
    log::info!("main:+");

    test_transition_between_leafs_in_a_tree();

    log::info!("main:-");
}
