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

//#[derive(Debug)]
pub struct StateMachineInfo {
    //pub name: String, // TODO: add StateMachineInfo::name
    pub state_fns: [StateInfo; StateMachine::MAX_STATE_FNS],
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
                // STATE_BASE_HDL
                StateInfo {
                    name: "base".to_owned(),
                    parent: None,
                    enter: Some(StateMachine::base_enter),
                    process: StateMachine::base,
                    exit: Some(StateMachine::base_exit),
                    active: false,
                },
                // STATE_INTERMEDIATE_HDL
                StateInfo {
                    name: "intermediate".to_owned(),
                    parent: Some(0),
                    enter: Some(StateMachine::intermediate_enter),
                    process: StateMachine::intermediate,
                    exit: Some(StateMachine::intermediate_exit),
                    active: false,
                },
                // STATE_ADD_HDL
                StateInfo {
                    name: "bottom".to_owned(),
                    parent: Some(1),
                    enter: Some(StateMachine::bottom_enter),
                    process: StateMachine::bottom,
                    exit: Some(StateMachine::bottom_exit),
                    active: false,
                },
            ],
            enter_fns_hdls: Vec::<usize>::with_capacity(StateMachine::MAX_STATE_FNS),
            exit_fns_hdls: VecDeque::<usize>::with_capacity(StateMachine::MAX_STATE_FNS),
            current_state_fns_hdl: StateMachine::INITIAL_HDL,
            previous_state_fns_hdl: StateMachine::INITIAL_HDL,
            current_state_changed: true,
            //transition_dest_hdl: Option<usize>,
        }
    }
}

pub struct StateMachine {
    pub smi: StateMachineInfo,
    pub base_enter_cnt: u64,
    pub base_cnt: u64,
    pub base_exit_cnt: u64,
    pub intermediate_enter_cnt: u64,
    pub intermediate_cnt: u64,
    pub intermediate_exit_cnt: u64,
    pub bottom_enter_cnt: u64,
    pub bottom_cnt: u64,
    pub bottom_exit_cnt: u64,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine {
    const MAX_STATE_FNS: usize = 3;
    #[allow(unused)]
    const BASE_HDL: usize = 0;
    #[allow(unused)]
    const INTERMEDIATE_HDL: usize = 1;
    #[allow(unused)]
    const BOTTOM_HDL: usize = 2;

    const INITIAL_HDL: usize = StateMachine::BASE_HDL;

    pub fn new() -> Self {
        let mut sm = StateMachine {
            smi: StateMachineInfo::new(),
            base_enter_cnt: 0,
            base_cnt: 0,
            base_exit_cnt: 0,
            intermediate_enter_cnt: 0,
            intermediate_cnt: 0,
            intermediate_exit_cnt: 0,
            bottom_enter_cnt: 0,
            bottom_cnt: 0,
            bottom_exit_cnt: 0,
        };

        let name = sm.state_name();
        println!("new: inital state={}", name);

        // Initialize so transition to initial state works
        sm.initial_enter_fns_hdls();

        sm
    }

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
        loop {
            log::trace!(
                "setup_exit_fns_hdls: push_back exit_hdl={} {}",
                exit_hdl,
                self.smi.state_fns[exit_hdl].name
            );
            self.smi.exit_fns_hdls.push_back(exit_hdl);

            if Some(exit_hdl) == exit_sentinel {
                // This handles the special case there we're transition_to yourself.smi
                return;
            }

            exit_hdl = if let Some(hdl) = self.smi.state_fns[exit_hdl].parent {
                hdl
            } else {
                // No parent we're done
                return;
            };

            if Some(exit_hdl) == exit_sentinel {
                // Reached the exit sentinel so we're done
                return;
            }
        }
    }

    fn setup_exit_enter_fns_hdls(&mut self, next_state_hdl: usize) {
        let mut cur_hdl = next_state_hdl;

        // Setup the enter vector
        let exit_sentinel = loop {
            log::trace!(
                "setup_exit_enter_fns_hdls: enter_hdl={} {}",
                cur_hdl,
                self.smi.state_fns[cur_hdl].name
            );
            self.smi.enter_fns_hdls.push(cur_hdl);

            cur_hdl = if let Some(hdl) = self.smi.state_fns[cur_hdl].parent {
                hdl
            } else {
                // Exit state_fns[self.smi.current_state_fns_hdl] and all its parents
                break None;
            };

            if self.smi.state_fns[cur_hdl].active {
                // Exit state_fns[self.smi.current_state_fns_hdl] and
                // parents upto but excluding state_fns[cur_hdl]
                break Some(cur_hdl);
            }
        };

        // Setup the exit vector
        self.setup_exit_fns_hdls(exit_sentinel);
    }

    pub fn dispatch_msg_hdl(&mut self, msg: &NoMessages, hdl: usize) {
        log::trace!(
            "dispatch_msg_hdl:+ hdl={} {}",
            hdl,
            self.smi.state_fns[hdl].name
        );

        if self.smi.current_state_changed {
            // Execute the enter functions
            while let Some(enter_hdl) = self.smi.enter_fns_hdls.pop() {
                if let Some(state_enter) = self.smi.state_fns[enter_hdl].enter {
                    log::trace!(
                        "dispatch_msg_hdl: entering hdl={} {}",
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
            "dispatch_msg_hdl: processing hdl={} {}",
            hdl,
            self.smi.state_fns[hdl].name
        );
        match (self.smi.state_fns[hdl].process)(self, msg) {
            StateResult::NotHandled => {
                if let Some(parent_hdl) = self.smi.state_fns[hdl].parent {
                    log::trace!(
                        "dispatch_msg_hdl: hdl={} {} NotHandled, recurse into dispatch_msg_hdl",
                        hdl,
                        self.smi.state_fns[hdl].name
                    );
                    self.dispatch_msg_hdl(msg, parent_hdl);
                } else {
                    log::trace!(
                        "dispatch_msg_hdl: hdl={} {}, NotHandled, no parent, ignoring messages",
                        hdl,
                        self.smi.state_fns[hdl].name
                    );
                }
            }
            StateResult::Handled => {
                // Nothing to do
                log::trace!(
                    "dispatch_msg_hdl: hdl={} {} Handled",
                    hdl,
                    self.smi.state_fns[hdl].name
                );
            }
            StateResult::TransitionTo(next_state_hdl) => {
                log::trace!(
                    "dispatch_msg_hdl: transition_to hdl={} {}",
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
                        "dispatch_msg_hdl: exiting hdl={} {}",
                        exit_hdl,
                        self.smi.state_fns[exit_hdl].name
                    );
                    (state_exit)(self, msg)
                }
            }
        }

        log::trace!(
            "dispatch_msg_hdl:- hdl={} {}",
            hdl,
            self.smi.state_fns[hdl].name
        );
    }

    pub fn dispatch_msg(&mut self, msg: &NoMessages) {
        log::trace!(
            "dispatch_msg:+ current_state_fns_hdl={} {}",
            self.smi.current_state_fns_hdl,
            self.smi.state_fns[self.smi.current_state_fns_hdl].name
        );
        self.dispatch_msg_hdl(msg, self.smi.current_state_fns_hdl);
        log::trace!(
            "dispatch_msg:- current_state_fns_hdl={} {}",
            self.smi.current_state_fns_hdl,
            self.smi.state_fns[self.smi.current_state_fns_hdl].name
        );
    }

    pub fn base_enter(&mut self, _msg: &NoMessages) {
        self.base_enter_cnt += 1;
        log::trace!("base: base_enter_cnt={:?}", self.base_enter_cnt);
    }

    pub fn base(&mut self, _msg: &NoMessages) -> StateResult {
        self.base_cnt += 1;
        log::trace!("base: base_cnt={:?}", self.base_cnt);
        StateResult::TransitionTo(StateMachine::INTERMEDIATE_HDL)
    }

    pub fn base_exit(&mut self, _msg: &NoMessages) {
        self.base_exit_cnt += 1;
        log::trace!("base: base_exit_cnt={:?}", self.base_exit_cnt);
    }

    pub fn intermediate_enter(&mut self, _msg: &NoMessages) {
        self.intermediate_enter_cnt += 1;
        log::trace!(
            "intermediate: intermediate_enter_cnt={:?}",
            self.intermediate_enter_cnt
        );
    }

    pub fn intermediate(&mut self, _msg: &NoMessages) -> StateResult {
        self.intermediate_cnt += 1;
        log::trace!("intermediate: intermediate_cnt={:?}", self.intermediate_cnt);
        StateResult::Handled
    }

    pub fn intermediate_exit(&mut self, _msg: &NoMessages) {
        self.intermediate_exit_cnt += 1;
        log::trace!(
            "intermediate: intermediate_exit_cnt={:?}",
            self.intermediate_exit_cnt
        );
    }

    pub fn bottom_enter(&mut self, _msg: &NoMessages) {
        self.bottom_enter_cnt += 1;
        log::trace!("bottom: bottom_enter_cnt={:?}", self.bottom_enter_cnt);
    }

    pub fn bottom(&mut self, _msg: &NoMessages) -> StateResult {
        self.bottom_cnt += 1;
        log::trace!("bottom: bottom_cnt={:?}", self.bottom_cnt);
        StateResult::TransitionTo(StateMachine::INTERMEDIATE_HDL)
    }

    pub fn bottom_exit(&mut self, _msg: &NoMessages) {
        self.bottom_exit_cnt += 1;
        log::trace!("bottom: bottom_exit_cnt={:?}", self.bottom_exit_cnt);
    }
}
