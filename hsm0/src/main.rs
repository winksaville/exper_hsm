use custom_logger::env_logger_init;

use std::collections::VecDeque;

#[derive(Debug)]
#[allow(unused)]
pub struct Header<P> {
    pub tx_response: Option<std::sync::mpsc::Sender<P>>,
}

// Create a Protocol with three messages
#[derive(Debug)]
pub struct NoMessages;

type StateFn = fn(&mut StateMachine, &NoMessages) -> StateResult;
type EnterFn = fn(&mut StateMachine, &NoMessages);
type ExitFn = fn(&mut StateMachine, &NoMessages);

pub enum StateResult {
    NotHandled,
    Handled,
    TransitionTo(usize),
}

pub struct StateInfo {
    pub name: String,
    pub parent: Option<usize>,
    pub enter: Option<EnterFn>,
    pub process: StateFn,
    pub exit: Option<ExitFn>,
    pub active: bool,
}

impl StateMachine {
    fn state_name(&self) -> &str {
        &self.smi.state_fns[self.smi.current_state_fns_hdl].name
    }

    // When the state machine starts there will be no fn's to
    // exit so we initialize only the enter_fns_hdls.
    fn initial_enter_fns_hdls(&mut self) {
        let mut enter_hdl = self.smi.current_state_fns_hdl;
        loop {
            log::trace!(
                "initial_enter_fns_hdls: push enter_hdl={} {}",
                enter_hdl,
                self.smi.state_fns[enter_hdl].name
            );
            self.smi.enter_fns_hdls.push(enter_hdl);
            enter_hdl = if let Some(hdl) = self.smi.state_fns[enter_hdl].parent {
                hdl
            } else {
                break;
            };
        }
    }

    // Starting at self.smi.current_state_fns_hdl generate the
    // list of StateFns that we're going to exit. If exit_sentinel is None
    // then exit from current_state_fns_hdl and all of its parents.
    // If exit_sentinel is Some then exit from the current state_fns_hdl
    // up to but not including the exit_sentinel.
    fn setup_exit_fns_hdls(&mut self, exit_sentinel: Option<usize>) {
        let mut exit_hdl = self.smi.current_state_fns_hdl;

        // Always exit the first state, this handles the special case
        // where Some(exit_hdl) == exit_sentinel.
        log::trace!(
            "setup_exit_fns_hdls: push_back(curren_state_fns_hdl={} {})",
            exit_hdl,
            self.smi.state_fns[exit_hdl].name,
        );
        self.smi.exit_fns_hdls.push_back(exit_hdl);

        loop {
            exit_hdl = if let Some(hdl) = self.smi.state_fns[exit_hdl].parent {
                hdl
            } else {
                // No parent we're done
                log::trace!(
                    "setup_exit_fns_hdls: No parent exit_hdl={} {}, return",
                    exit_hdl,
                    self.smi.state_fns[exit_hdl].name,
                );
                return;
            };

            if Some(exit_hdl) == exit_sentinel {
                // Reached the exit sentinel so we're done
                log::trace!(
                    "setup_exit_fns_hdls: exit_hdl={} {} == exit_sentinel={} {}, reached exit_sentinel return",
                    exit_hdl,
                    self.smi.state_fns[exit_hdl].name,
                    exit_sentinel.unwrap(),
                    self.smi.state_fns[exit_sentinel.unwrap()].name,
                );
                return;
            }

            log::trace!(
                "setup_exit_fns_hdls: push_back(exit_hdl={} {})",
                exit_hdl,
                self.smi.state_fns[exit_hdl].name,
            );
            self.smi.exit_fns_hdls.push_back(exit_hdl);
        }
    }

    fn setup_exit_enter_fns_hdls(&mut self, next_state_hdl: usize) {
        let mut cur_hdl = next_state_hdl;

        // Setup the enter vector
        let exit_sentinel = loop {
            log::trace!(
                "setup_exit_enter_fns_hdls: cur_hdl={} {}, TOL",
                cur_hdl,
                self.smi.state_fns[cur_hdl].name
            );
            self.smi.enter_fns_hdls.push(cur_hdl);

            cur_hdl = if let Some(hdl) = self.smi.state_fns[cur_hdl].parent {
                hdl
            } else {
                // Exit state_fns[self.smi.current_state_fns_hdl] and all its parents
                log::trace!(
                    "setup_exit_enter_fns_hdls: cur_hdl={} {} has no parent exit_sentinel=None",
                    cur_hdl,
                    self.smi.state_fns[cur_hdl].name,
                );
                break None;
            };

            if self.smi.state_fns[cur_hdl].active {
                // Exit state_fns[self.smi.current_state_fns_hdl] and
                // parents upto but excluding state_fns[cur_hdl]
                log::trace!(
                    "setup_exit_enter_fns_hdls: cur_hdl={} {} is active so it's exit_sentinel",
                    cur_hdl,
                    self.smi.state_fns[cur_hdl].name,
                );
                break Some(cur_hdl);
            }
        };

        // Setup the exit vector
        self.setup_exit_fns_hdls(exit_sentinel);
    }

    pub fn dispatch_hdl(&mut self, msg: &NoMessages, hdl: usize) {
        log::trace!(
            "dispatch_hdl:+ hdl={} {}",
            hdl,
            self.smi.state_fns[hdl].name
        );

        if self.smi.current_state_changed {
            // Execute the enter functions
            while let Some(enter_hdl) = self.smi.enter_fns_hdls.pop() {
                if let Some(state_enter) = self.smi.state_fns[enter_hdl].enter {
                    log::trace!(
                        "dispatch_hdl: entering hdl={} {}",
                        enter_hdl,
                        self.smi.state_fns[enter_hdl].name
                    );
                    (state_enter)(self, msg);
                    self.smi.state_fns[enter_hdl].active = true;
                }
            }
            self.smi.current_state_changed = false;
        }

        // Invoke the current state funtion processing the result
        log::trace!(
            "dispatch_hdl: processing hdl={} {}",
            hdl,
            self.smi.state_fns[hdl].name
        );
        match (self.smi.state_fns[hdl].process)(self, msg) {
            StateResult::NotHandled => {
                if let Some(parent_hdl) = self.smi.state_fns[hdl].parent {
                    log::trace!(
                        "dispatch_hdl: hdl={} {} NotHandled, recurse into dispatch_hdl",
                        hdl,
                        self.smi.state_fns[hdl].name
                    );
                    self.dispatch_hdl(msg, parent_hdl);
                } else {
                    log::trace!(
                        "dispatch_hdl: hdl={} {}, NotHandled, no parent, ignoring messages",
                        hdl,
                        self.smi.state_fns[hdl].name
                    );
                }
            }
            StateResult::Handled => {
                // Nothing to do
                log::trace!(
                    "dispatch_hdl: hdl={} {} Handled",
                    hdl,
                    self.smi.state_fns[hdl].name
                );
            }
            StateResult::TransitionTo(next_state_hdl) => {
                log::trace!(
                    "dispatch_hdl: transition_to hdl={} {}",
                    next_state_hdl,
                    self.smi.state_fns[next_state_hdl].name
                );
                self.setup_exit_enter_fns_hdls(next_state_hdl);

                self.smi.previous_state_fns_hdl = self.smi.current_state_fns_hdl;
                self.smi.current_state_fns_hdl = next_state_hdl;
                self.smi.current_state_changed = true;
            }
        }

        if self.smi.current_state_changed {
            while let Some(exit_hdl) = self.smi.exit_fns_hdls.pop_front() {
                if let Some(state_exit) = self.smi.state_fns[exit_hdl].exit {
                    log::trace!(
                        "dispatch_hdl: exiting hdl={} {}",
                        exit_hdl,
                        self.smi.state_fns[exit_hdl].name
                    );
                    (state_exit)(self, msg);
                    self.smi.state_fns[exit_hdl].active = false;
                }
            }
        }

        log::trace!(
            "dispatch_hdl:- hdl={} {}",
            hdl,
            self.smi.state_fns[hdl].name
        );
    }

    pub fn dispatch(&mut self, msg: &NoMessages) {
        log::trace!(
            "dispatch:+ current_state_fns_hdl={} {}",
            self.smi.current_state_fns_hdl,
            self.smi.state_fns[self.smi.current_state_fns_hdl].name
        );
        self.dispatch_hdl(msg, self.smi.current_state_fns_hdl);
        log::trace!(
            "dispatch:- current_state_fns_hdl={} {}",
            self.smi.current_state_fns_hdl,
            self.smi.state_fns[self.smi.current_state_fns_hdl].name
        );
    }
}

//#[derive(Debug)]
pub struct StateMachineInfo {
    //pub name: String, // TODO: add StateMachineInfo::name
    pub state_fns: [StateInfo; MAX_STATE_FNS],
    pub enter_fns_hdls: Vec<usize>,
    pub exit_fns_hdls: std::collections::VecDeque<usize>,
    pub current_state_fns_hdl: usize,
    pub previous_state_fns_hdl: usize,
    pub current_state_changed: bool,
    //pub transition_dest_hdl: Option<usize>,
}

impl StateMachineInfo {
    fn new() -> Self {
        StateMachineInfo {
            state_fns: [
                StateInfo {
                    name: "initial_base".to_owned(),
                    parent: None,
                    enter: Some(StateMachine::initial_base_enter),
                    process: StateMachine::initial_base,
                    exit: Some(StateMachine::initial_base_exit),
                    active: false,
                },
                StateInfo {
                    name: "initial".to_owned(),
                    parent: Some(INITIAL_BASE_HDL),
                    enter: Some(StateMachine::initial_enter),
                    process: StateMachine::initial,
                    exit: Some(StateMachine::initial_exit),
                    active: false,
                },
                StateInfo {
                    name: "other_base".to_owned(),
                    parent: None,
                    enter: Some(StateMachine::other_base_enter),
                    process: StateMachine::other_base,
                    exit: Some(StateMachine::other_base_exit),
                    active: false,
                },
                StateInfo {
                    name: "other".to_owned(),
                    parent: Some(OTHER_BASE_HDL),
                    enter: Some(StateMachine::other_enter),
                    process: StateMachine::other,
                    exit: Some(StateMachine::other_exit),
                    active: false,
                },
            ],
            enter_fns_hdls: Vec::<usize>::with_capacity(MAX_STATE_FNS),
            exit_fns_hdls: VecDeque::<usize>::with_capacity(MAX_STATE_FNS),
            current_state_fns_hdl: INITIAL_HDL,
            previous_state_fns_hdl: INITIAL_HDL,
            current_state_changed: true,
            //transition_dest_hdl: Option<usize>,
        }
    }
}

pub struct StateMachine {
    pub smi: StateMachineInfo,
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

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine {
    pub fn new() -> Self {
        let mut sm = StateMachine {
            smi: StateMachineInfo::new(),
            initial_base_enter_cnt: 0,
            initial_base_cnt: 0,
            initial_base_exit_cnt: 0,
            initial_enter_cnt: 0,
            initial_cnt: 0,
            initial_exit_cnt: 0,
            other_base_enter_cnt: 0,
            other_base_cnt: 0,
            other_base_exit_cnt: 0,
            other_enter_cnt: 0,
            other_cnt: 0,
            other_exit_cnt: 0,
        };

        let name = sm.state_name();
        log::trace!("new: inital state={}", name);

        // Initialize so transition to initial state works
        sm.initial_enter_fns_hdls();

        sm
    }
}

const MAX_STATE_FNS: usize = 4;
const INITIAL_BASE_HDL: usize = 0;
const INITIAL_HDL: usize = 1;
const OTHER_BASE_HDL: usize = 2;
const OTHER_HDL: usize = 3;

impl StateMachine {
    fn initial_base_enter(&mut self, _msg: &NoMessages) {
        self.initial_base_enter_cnt += 1;
    }

    // This state has hdl 0
    fn initial_base(&mut self, _msg: &NoMessages) -> StateResult {
        self.initial_base_cnt += 1;
        StateResult::Handled
    }

    fn initial_base_exit(&mut self, _msg: &NoMessages) {
        self.initial_base_exit_cnt += 1;
    }

    fn initial_enter(&mut self, _msg: &NoMessages) {
        self.initial_enter_cnt += 1;
    }

    // This state has hdl 0
    fn initial(&mut self, _msg: &NoMessages) -> StateResult {
        self.initial_cnt += 1;
        StateResult::TransitionTo(OTHER_HDL)
    }

    fn initial_exit(&mut self, _msg: &NoMessages) {
        self.initial_exit_cnt += 1;
    }

    fn other_base_enter(&mut self, _msg: &NoMessages) {
        self.other_base_enter_cnt += 1;
    }

    // This state has hdl 0
    fn other_base(&mut self, _msg: &NoMessages) -> StateResult {
        self.other_base_cnt += 1;
        StateResult::Handled
    }

    fn other_base_exit(&mut self, _msg: &NoMessages) {
        self.other_base_exit_cnt += 1;
    }

    fn other_enter(&mut self, _msg: &NoMessages) {
        self.other_enter_cnt += 1;
    }

    // This state has hdl 0
    fn other(&mut self, _msg: &NoMessages) -> StateResult {
        self.other_cnt += 1;
        StateResult::TransitionTo(INITIAL_HDL)
    }

    fn other_exit(&mut self, _msg: &NoMessages) {
        self.other_exit_cnt += 1;
    }
}

fn test_transition_to_between_leafs_of_trees() {
    // Create a sm and validate it's in the expected state
    let mut sm = StateMachine::new();
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

fn main() {
    println!("main");
    env_logger_init("info");
    log::debug!("main:+");

    test_transition_to_between_leafs_of_trees();

    log::debug!("main:-");
}
