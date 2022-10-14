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
    pub enter_cnt: usize,
    pub process_cnt: usize,
    pub exit_cnt: usize,
}

impl StateMachine {
    fn state_name(&self, hdl: usize) -> &str {
        &self.smi.state_fns[hdl].name
    }

    fn current_state_name(&self) -> &str {
        self.state_name(self.smi.current_state_fns_hdl)
    }

    // When the state machine starts there will be no fn's to
    // exit so we initialize only the enter_fns_hdls.
    fn initial_enter_fns_hdls(&mut self) {
        let mut enter_hdl = self.smi.current_state_fns_hdl;
        loop {
            log::trace!(
                "initial_enter_fns_hdls: push enter_hdl={} {}",
                enter_hdl,
                self.state_name(enter_hdl)
            );
            self.smi.enter_fns_hdls.push(enter_hdl);
            enter_hdl = if let Some(hdl) = self.smi.state_fns[enter_hdl].parent {
                hdl
            } else {
                break;
            };
        }
    }

    fn setup_exit_enter_fns_hdls(&mut self, next_state_hdl: usize) {
        let mut cur_hdl = next_state_hdl;

        // Setup the enter vector
        let exit_sentinel = loop {
            log::trace!(
                "setup_exit_enter_fns_hdls: cur_hdl={} {}, TOL",
                cur_hdl,
                self.state_name(cur_hdl)
            );
            self.smi.enter_fns_hdls.push(cur_hdl);

            cur_hdl = if let Some(hdl) = self.smi.state_fns[cur_hdl].parent {
                hdl
            } else {
                // Exit state_fns[self.smi.current_state_fns_hdl] and all its parents
                log::trace!(
                    "setup_exit_enter_fns_hdls: cur_hdl={} {} has no parent exit_sentinel=None",
                    cur_hdl,
                    self.state_name(cur_hdl)
                );
                break None;
            };

            if self.smi.state_fns[cur_hdl].active {
                // Exit state_fns[self.smi.current_state_fns_hdl] and
                // parents upto but excluding state_fns[cur_hdl]
                log::trace!(
                    "setup_exit_enter_fns_hdls: cur_hdl={} {} is active so it's exit_sentinel",
                    cur_hdl,
                    self.state_name(cur_hdl)
                );
                break Some(cur_hdl);
            }
        };

        // Starting at self.smi.current_state_fns_hdl generate the
        // list of StateFns that we're going to exit. If exit_sentinel is None
        // then exit from current_state_fns_hdl and all of its parents.
        // If exit_sentinel is Some then exit from the current state_fns_hdl
        // up to but not including the exit_sentinel.
        let mut exit_hdl = self.smi.current_state_fns_hdl;

        // Always exit the first state, this handles the special case
        // where Some(exit_hdl) == exit_sentinel.
        log::trace!(
            "setup_exit_enter_fns_hdls: push_back(curren_state_fns_hdl={} {})",
            exit_hdl,
            self.state_name(exit_hdl)
        );
        self.smi.exit_fns_hdls.push_back(exit_hdl);

        loop {
            exit_hdl = if let Some(hdl) = self.smi.state_fns[exit_hdl].parent {
                hdl
            } else {
                // No parent we're done
                log::trace!(
                    "setup_exit_enter_fns_hdls: No parent exit_hdl={} {}, return",
                    exit_hdl,
                    self.state_name(exit_hdl)
                );
                return;
            };

            if Some(exit_hdl) == exit_sentinel {
                // Reached the exit sentinel so we're done
                log::trace!(
                    "setup_exit_enter_fns_hdls: exit_hdl={} {} == exit_sentinel={} {}, reached exit_sentinel return",
                    exit_hdl,
                    self.state_name(exit_hdl),
                    exit_sentinel.unwrap(),
                    self.state_name(exit_sentinel.unwrap()),
                );
                return;
            }

            log::trace!(
                "setup_exit_enter_fns_hdls: push_back(exit_hdl={} {})",
                exit_hdl,
                self.state_name(exit_hdl)
            );
            self.smi.exit_fns_hdls.push_back(exit_hdl);
        }
    }

    pub fn dispatch_hdl(&mut self, msg: &NoMessages, hdl: usize) {
        log::trace!(
            "dispatch_hdl:+ hdl={} {}",
            hdl,
            self.state_name(hdl)
        );

        if self.smi.current_state_changed {
            // Execute the enter functions
            while let Some(enter_hdl) = self.smi.enter_fns_hdls.pop() {
                if let Some(state_enter) = self.smi.state_fns[enter_hdl].enter {
                    log::trace!(
                        "dispatch_hdl: entering hdl={} {}",
                        enter_hdl,
                        self.state_name(enter_hdl)
                    );
                    self.smi.state_fns[enter_hdl].enter_cnt += 1;
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
            self.state_name(hdl)
        );

        self.smi.state_fns[hdl].process_cnt += 1;
        match (self.smi.state_fns[hdl].process)(self, msg) {
            StateResult::NotHandled => {
                if let Some(parent_hdl) = self.smi.state_fns[hdl].parent {
                    log::trace!(
                        "dispatch_hdl: hdl={} {} NotHandled, recurse into dispatch_hdl",
                        hdl,
                        self.state_name(hdl)
                    );
                    self.dispatch_hdl(msg, parent_hdl);
                } else {
                    log::trace!(
                        "dispatch_hdl: hdl={} {}, NotHandled, no parent, ignoring messages",
                        hdl,
                        self.state_name(hdl)
                    );
                }
            }
            StateResult::Handled => {
                // Nothing to do
                log::trace!(
                    "dispatch_hdl: hdl={} {} Handled",
                    hdl,
                    self.state_name(hdl)
                );
            }
            StateResult::TransitionTo(next_state_hdl) => {
                log::trace!(
                    "dispatch_hdl: transition_to hdl={} {}",
                    next_state_hdl,
                    self.state_name(next_state_hdl)
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
                        self.state_name(exit_hdl)
                    );
                    self.smi.state_fns[exit_hdl].exit_cnt += 1;
                    (state_exit)(self, msg);
                    self.smi.state_fns[exit_hdl].active = false;
                }
            }
        }

        log::trace!(
            "dispatch_hdl:- hdl={} {}",
            hdl,
            self.state_name(hdl)
        );
    }

    pub fn dispatch(&mut self, msg: &NoMessages) {
        log::trace!(
            "dispatch:+ current_state_fns_hdl={} {}",
            self.smi.current_state_fns_hdl,
            self.current_state_name()
        );
        self.dispatch_hdl(msg, self.smi.current_state_fns_hdl);
        log::trace!(
            "dispatch:- current_state_fns_hdl={} {}",
            self.smi.current_state_fns_hdl,
            self.current_state_name()
        );
    }
}

//#[derive(Debug)]
pub struct StateMachineInfo {
    //pub name: String, // TODO: add StateMachineInfo::name
    pub state_fns: Vec<StateInfo>,
    pub enter_fns_hdls: Vec<usize>,
    pub exit_fns_hdls: std::collections::VecDeque<usize>,
    pub current_state_fns_hdl: usize,
    pub previous_state_fns_hdl: usize,
    pub current_state_changed: bool,
    //pub transition_dest_hdl: Option<usize>,
}

impl StateMachineInfo {
    fn new(max_fns: usize, initial_hdl: usize) -> Self {
        StateMachineInfo {
            state_fns: Vec::<StateInfo>::with_capacity(max_fns),
            enter_fns_hdls: Vec::<usize>::with_capacity(max_fns),
            exit_fns_hdls: VecDeque::<usize>::with_capacity(max_fns),
            current_state_fns_hdl: initial_hdl,
            previous_state_fns_hdl: initial_hdl,
            current_state_changed: true,
            //transition_dest_hdl: Option<usize>,
        }
    }

    fn add_state(&mut self, state_info: StateInfo) {
        self.state_fns.push(state_info);
    }
}

pub struct StateMachine {
    pub smi: StateMachineInfo,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

// StateMachine simply transitions back and forth
// between initial and other.
//
//                base=0
//        --------^  ^-------
//       /                   \
//      /                     \
//    other=2   <======>   initial=1

const MAX_STATE_FNS: usize = 3;
const BASE_HDL: usize = 0;
const INITIAL_HDL: usize = 1;
const OTHER_HDL: usize = 2;

impl StateMachine {
    pub fn new() -> Self {
        let mut sm = StateMachine {
            smi: StateMachineInfo::new(MAX_STATE_FNS, INITIAL_HDL),
        };

        let base_si = StateInfo {
            name: "base".to_owned(),
            parent: None,
            enter: Some(Self::base_enter),
            process: Self::base,
            exit: Some(Self::base_exit),
            active: false,
            enter_cnt: 0,
            process_cnt: 0,
            exit_cnt: 0,
        };
        sm.smi.add_state(base_si);

        let initial_si = StateInfo {
            name: "initial".to_owned(),
            parent: Some(BASE_HDL),
            enter: Some(Self::initial_enter),
            process: Self::initial,
            exit: Some(Self::initial_exit),
            active: false,
            enter_cnt: 0,
            process_cnt: 0,
            exit_cnt: 0,
        };
        sm.smi.add_state(initial_si);

        let other_si = StateInfo {
            name: "other".to_owned(),
            parent: Some(BASE_HDL),
            enter: Some(Self::other_enter),
            process: Self::other,
            exit: Some(Self::other_exit),
            active: false,
            enter_cnt: 0,
            process_cnt: 0,
            exit_cnt: 0,
        };
        sm.smi.add_state(other_si);

        // Initialize so transition to initial state works
        sm.initial_enter_fns_hdls();

        log::trace!("new: inital state={} enter_fnss_hdls={:?}", sm.current_state_name(), sm.smi.enter_fns_hdls);

        sm
    }

    fn base_enter(&mut self, _msg: &NoMessages) {
    }

    // This state has hdl 0
    fn base(&mut self, _msg: &NoMessages) -> StateResult {
        StateResult::Handled
    }

    fn base_exit(&mut self, _msg: &NoMessages) {
    }

    fn initial_enter(&mut self, _msg: &NoMessages) {
    }

    // This state has hdl 0
    fn initial(&mut self, _msg: &NoMessages) -> StateResult {
        StateResult::TransitionTo(OTHER_HDL)
    }

    fn initial_exit(&mut self, _msg: &NoMessages) {
    }

    fn other_enter(&mut self, _msg: &NoMessages) {
    }

    // This state has hdl 0
    fn other(&mut self, _msg: &NoMessages) -> StateResult {
        StateResult::TransitionTo(INITIAL_HDL)
    }

    fn other_exit(&mut self, _msg: &NoMessages) {
    }
}

fn test_transition_between_leafs_in_a_tree() {
    // Create a sm and validate it's in the expected state
    let mut sm = StateMachine::new();
    assert_eq!(sm.smi.state_fns[BASE_HDL].enter_cnt, 0);
    assert_eq!(sm.smi.state_fns[BASE_HDL].process_cnt, 0);
    assert_eq!(sm.smi.state_fns[BASE_HDL].exit_cnt, 0);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].enter_cnt, 0);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].process_cnt, 0);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].exit_cnt, 0);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].enter_cnt, 0);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].process_cnt, 0);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].exit_cnt, 0);

    sm.dispatch(&NoMessages);
    assert_eq!(sm.smi.state_fns[BASE_HDL].enter_cnt, 1);
    assert_eq!(sm.smi.state_fns[BASE_HDL].process_cnt, 0);
    assert_eq!(sm.smi.state_fns[BASE_HDL].exit_cnt, 0);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].enter_cnt, 1);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].process_cnt, 1);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].exit_cnt, 1);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].enter_cnt, 0);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].process_cnt, 0);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].exit_cnt, 0);

    sm.dispatch(&NoMessages);
    assert_eq!(sm.smi.state_fns[BASE_HDL].enter_cnt, 1);
    assert_eq!(sm.smi.state_fns[BASE_HDL].process_cnt, 0);
    assert_eq!(sm.smi.state_fns[BASE_HDL].exit_cnt, 0);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].enter_cnt, 1);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].process_cnt, 1);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].exit_cnt, 1);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].enter_cnt, 1);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].process_cnt, 1);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].exit_cnt, 1);

    sm.dispatch(&NoMessages);
    assert_eq!(sm.smi.state_fns[BASE_HDL].enter_cnt, 1);
    assert_eq!(sm.smi.state_fns[BASE_HDL].process_cnt, 0);
    assert_eq!(sm.smi.state_fns[BASE_HDL].exit_cnt, 0);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].enter_cnt, 2);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].process_cnt, 2);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].exit_cnt, 2);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].enter_cnt, 1);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].process_cnt, 1);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].exit_cnt, 1);

    sm.dispatch(&NoMessages);
    assert_eq!(sm.smi.state_fns[BASE_HDL].enter_cnt, 1);
    assert_eq!(sm.smi.state_fns[BASE_HDL].process_cnt, 0);
    assert_eq!(sm.smi.state_fns[BASE_HDL].exit_cnt, 0);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].enter_cnt, 2);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].process_cnt, 2);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].exit_cnt, 2);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].enter_cnt, 2);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].process_cnt, 2);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].exit_cnt, 2);

    sm.dispatch(&NoMessages);
    assert_eq!(sm.smi.state_fns[BASE_HDL].enter_cnt, 1);
    assert_eq!(sm.smi.state_fns[BASE_HDL].process_cnt, 0);
    assert_eq!(sm.smi.state_fns[BASE_HDL].exit_cnt, 0);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].enter_cnt, 3);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].process_cnt, 3);
    assert_eq!(sm.smi.state_fns[INITIAL_HDL].exit_cnt, 3);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].enter_cnt, 2);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].process_cnt, 2);
    assert_eq!(sm.smi.state_fns[OTHER_HDL].exit_cnt, 2);
}

fn main() {
    println!("main");
    env_logger_init("info");
    log::info!("main:+");

    test_transition_between_leafs_in_a_tree();

    log::info!("main:-");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leaf_transitions_in_a_tree() {
        test_transition_between_leafs_in_a_tree();
    }
}
