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

type StateFn<SM> = fn(&mut SM, &NoMessages) -> StateResult;
type EnterFn<SM> = fn(&mut SM, &NoMessages);
type ExitFn<SM> = fn(&mut SM, &NoMessages);

pub enum StateResult {
    NotHandled,
    Handled,
    TransitionTo(usize),
}

pub struct StateInfo<SM> {
    pub name: String,
    pub parent: Option<usize>,
    pub enter: Option<EnterFn<SM>>,
    pub process: StateFn<SM>,
    pub exit: Option<ExitFn<SM>>,
    pub active: bool,
    pub enter_cnt: usize,
    pub process_cnt: usize,
    pub exit_cnt: usize,
}

impl<SM> StateInfo<SM> {
    fn new(
        name: &str,
        enter_fn: Option<EnterFn<SM>>,
        process_fn: StateFn<SM>,
        exit_fn: Option<ExitFn<SM>>,
        parent_hdl: Option<usize>,
    ) -> Self {
        StateInfo {
            name: name.to_owned(),
            parent: parent_hdl,
            enter: enter_fn,
            process: process_fn,
            exit: exit_fn,
            active: false,
            enter_cnt: 0,
            process_cnt: 0,
            exit_cnt: 0,
        }
    }
}

pub struct StateMachineExecutor<SM> {
    //pub name: String, // TODO: add StateMachineInfo::name
    pub sm: SM,
    pub state_fns: Vec<StateInfo<SM>>,
    pub enter_fns_hdls: Vec<usize>,
    pub exit_fns_hdls: std::collections::VecDeque<usize>,
    pub current_state_fns_hdl: usize,
    pub previous_state_fns_hdl: usize,
    pub current_state_changed: bool,
    //pub transition_dest_hdl: Option<usize>,
}

impl<SM> StateMachineExecutor<SM> {
    fn new(sm: SM, max_fns: usize, initial_hdl: usize) -> Self {
        StateMachineExecutor {
            sm,
            state_fns: Vec::<StateInfo<SM>>::with_capacity(max_fns),
            enter_fns_hdls: Vec::<usize>::with_capacity(max_fns),
            exit_fns_hdls: VecDeque::<usize>::with_capacity(max_fns),
            current_state_fns_hdl: initial_hdl,
            previous_state_fns_hdl: initial_hdl,
            current_state_changed: true,
            //transition_dest_hdl: Option<usize>,
        }
    }

    fn state_name(&self, hdl: usize) -> &str {
        &self.state_fns[hdl].name
    }

    fn current_state_name(&self) -> &str {
        self.state_name(self.current_state_fns_hdl)
    }

    fn get_sm(&mut self) -> &SM {
        &self.sm
    }

    fn add_state(&mut self, state_info: StateInfo<SM>) {
        self.state_fns.push(state_info);
    }

    fn get_state_fns_enter_cnt(&self, hdl: usize) -> usize {
        self.state_fns[hdl].enter_cnt
    }
    fn get_state_fns_process_cnt(&self, hdl: usize) -> usize {
        self.state_fns[hdl].process_cnt
    }

    fn get_state_fns_exit_cnt(&self, hdl: usize) -> usize {
        self.state_fns[hdl].exit_cnt
    }

    // When the state machine starts there will be no fn's to
    // exit so we initialize only the enter_fns_hdls.
    fn initial_enter_fns_hdls(&mut self) {
        let mut enter_hdl = self.current_state_fns_hdl;
        loop {
            log::trace!(
                "initial_enter_fns_hdls: push enter_hdl={} {}",
                enter_hdl,
                self.state_name(enter_hdl)
            );
            self.enter_fns_hdls.push(enter_hdl);
            enter_hdl = if let Some(hdl) = self.state_fns[enter_hdl].parent {
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
            self.enter_fns_hdls.push(cur_hdl);

            cur_hdl = if let Some(hdl) = self.state_fns[cur_hdl].parent {
                hdl
            } else {
                // Exit state_fns[self.current_state_fns_hdl] and all its parents
                log::trace!(
                    "setup_exit_enter_fns_hdls: cur_hdl={} {} has no parent exit_sentinel=None",
                    cur_hdl,
                    self.state_name(cur_hdl)
                );
                break None;
            };

            if self.state_fns[cur_hdl].active {
                // Exit state_fns[self.current_state_fns_hdl] and
                // parents upto but excluding state_fns[cur_hdl]
                log::trace!(
                    "setup_exit_enter_fns_hdls: cur_hdl={} {} is active so it's exit_sentinel",
                    cur_hdl,
                    self.state_name(cur_hdl)
                );
                break Some(cur_hdl);
            }
        };

        // Starting at self.current_state_fns_hdl generate the
        // list of StateFns that we're going to exit. If exit_sentinel is None
        // then exit from current_state_fns_hdl and all of its parents.
        // If exit_sentinel is Some then exit from the current state_fns_hdl
        // up to but not including the exit_sentinel.
        let mut exit_hdl = self.current_state_fns_hdl;

        // Always exit the first state, this handles the special case
        // where Some(exit_hdl) == exit_sentinel.
        log::trace!(
            "setup_exit_enter_fns_hdls: push_back(curren_state_fns_hdl={} {})",
            exit_hdl,
            self.state_name(exit_hdl)
        );
        self.exit_fns_hdls.push_back(exit_hdl);

        loop {
            exit_hdl = if let Some(hdl) = self.state_fns[exit_hdl].parent {
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
            self.exit_fns_hdls.push_back(exit_hdl);
        }
    }

    pub fn dispatch_hdl(&mut self, msg: &NoMessages, hdl: usize) {
        log::trace!("dispatch_hdl:+ hdl={} {}", hdl, self.state_name(hdl));

        if self.current_state_changed {
            // Execute the enter functions
            while let Some(enter_hdl) = self.enter_fns_hdls.pop() {
                if let Some(state_enter) = self.state_fns[enter_hdl].enter {
                    log::trace!(
                        "dispatch_hdl: entering hdl={} {}",
                        enter_hdl,
                        self.state_name(enter_hdl)
                    );
                    self.state_fns[enter_hdl].enter_cnt += 1;
                    (state_enter)(&mut self.sm, msg);
                    self.state_fns[enter_hdl].active = true;
                }
            }
            self.current_state_changed = false;
        }

        // Invoke the current state funtion processing the result
        log::trace!(
            "dispatch_hdl: processing hdl={} {}",
            hdl,
            self.state_name(hdl)
        );

        self.state_fns[hdl].process_cnt += 1;
        match (self.state_fns[hdl].process)(&mut self.sm, msg) {
            StateResult::NotHandled => {
                if let Some(parent_hdl) = self.state_fns[hdl].parent {
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
                log::trace!("dispatch_hdl: hdl={} {} Handled", hdl, self.state_name(hdl));
            }
            StateResult::TransitionTo(next_state_hdl) => {
                log::trace!(
                    "dispatch_hdl: transition_to hdl={} {}",
                    next_state_hdl,
                    self.state_name(next_state_hdl)
                );
                self.setup_exit_enter_fns_hdls(next_state_hdl);

                self.previous_state_fns_hdl = self.current_state_fns_hdl;
                self.current_state_fns_hdl = next_state_hdl;
                self.current_state_changed = true;
            }
        }

        if self.current_state_changed {
            while let Some(exit_hdl) = self.exit_fns_hdls.pop_front() {
                if let Some(state_exit) = self.state_fns[exit_hdl].exit {
                    log::trace!(
                        "dispatch_hdl: exiting hdl={} {}",
                        exit_hdl,
                        self.state_name(exit_hdl)
                    );
                    self.state_fns[exit_hdl].exit_cnt += 1;
                    (state_exit)(&mut self.sm, msg);
                    self.state_fns[exit_hdl].active = false;
                }
            }
        }

        log::trace!("dispatch_hdl:- hdl={} {}", hdl, self.state_name(hdl));
    }

    pub fn dispatch(&mut self, msg: &NoMessages) {
        log::trace!(
            "dispatch:+ current_state_fns_hdl={} {}",
            self.current_state_fns_hdl,
            self.current_state_name()
        );
        self.dispatch_hdl(msg, self.current_state_fns_hdl);
        log::trace!(
            "dispatch:- current_state_fns_hdl={} {}",
            self.current_state_fns_hdl,
            self.current_state_name()
        );
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

#[derive(Default)]
pub struct StateMachine;

const MAX_STATE_FNS: usize = 3;
const BASE_HDL: usize = 0;
const INITIAL_HDL: usize = 1;
const OTHER_HDL: usize = 2;

impl StateMachine {
    pub fn create() -> StateMachineExecutor<Self> {
        let sm = StateMachine::default();
        let mut sme = StateMachineExecutor::new(sm, MAX_STATE_FNS, INITIAL_HDL);

        let base_si = StateInfo::new(
            "base",
            Some(Self::base_enter),
            Self::base,
            Some(Self::base_exit),
            None,
        );
        sme.add_state(base_si);

        let initial_si = StateInfo::new(
            "initial",
            Some(Self::initial_enter),
            Self::initial,
            Some(Self::initial_exit),
            Some(BASE_HDL),
        );
        sme.add_state(initial_si);

        let other_si = StateInfo::new(
            "other",
            Some(Self::other_enter),
            Self::other,
            Some(Self::other_exit),
            Some(BASE_HDL),
        );
        sme.add_state(other_si);

        // Initialize so transition to initial state works
        sme.initial_enter_fns_hdls();

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
    let mut sme = StateMachine::create();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leaf_transitions_in_a_tree() {
        test_transition_between_leafs_in_a_tree();
    }
}
