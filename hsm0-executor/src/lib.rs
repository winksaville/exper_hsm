use std::collections::VecDeque;

type ProcessFn<SM, P> = fn(&mut SM, &P) -> StateResult;
type EnterFn<SM, P> = fn(&mut SM, &P);
type ExitFn<SM, P> = fn(&mut SM, &P);

pub enum StateResult {
    NotHandled,
    Handled,
    TransitionTo(usize),
}

pub struct StateInfo<SM, P> {
    pub name: String,
    pub parent: Option<usize>,
    pub enter: Option<EnterFn<SM, P>>,
    pub process: ProcessFn<SM, P>,
    pub exit: Option<ExitFn<SM, P>>,
    pub active: bool,
    pub enter_cnt: usize,
    pub process_cnt: usize,
    pub exit_cnt: usize,
}

impl<SM, P> StateInfo<SM, P> {
    pub fn new(
        name: &str,
        enter_fn: Option<EnterFn<SM, P>>,
        process_fn: ProcessFn<SM, P>,
        exit_fn: Option<ExitFn<SM, P>>,
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

pub struct StateMachineExecutor<SM, P> {
    //pub name: String, // TODO: add StateMachineInfo::name
    pub sm: SM,
    pub states: Vec<StateInfo<SM, P>>,
    pub current_state_changed: bool,
    pub idx_current_state: usize,
    pub idx_previous_state: usize,
    pub idxs_enter_fns: Vec<usize>,
    pub idxs_exit_fns: std::collections::VecDeque<usize>,
    //pub transition_dest_hdl: Option<usize>,
}

impl<SM, P> StateMachineExecutor<SM, P> {
    // Begin building an executor.
    //
    // You must call add_state to add one or more states
    pub fn build(sm: SM, max_states: usize, idx_initial_state: usize) -> Self {
        StateMachineExecutor {
            sm,
            states: Vec::<StateInfo<SM, P>>::with_capacity(max_states),
            idxs_enter_fns: Vec::<usize>::with_capacity(max_states),
            idxs_exit_fns: VecDeque::<usize>::with_capacity(max_states),
            idx_current_state: idx_initial_state,
            idx_previous_state: idx_initial_state,
            current_state_changed: true,
            //transition_dest_hdl: Option<usize>,
        }
    }

    // Add a state to the the executor
    pub fn add_state(&mut self, state_info: StateInfo<SM, P>) -> &mut Self {
        self.states.push(state_info);

        self
    }

    // Initialize and so the executor is ready to dispatch messages.
    //
    // The first state will be the initial state as identified by the
    // initial_hdl parameter in build.
    pub fn initialize(&mut self) {
        let mut enter_hdl = self.idx_current_state;
        loop {
            log::trace!(
                "initial_enter_fns_hdls: push enter_hdl={} {}",
                enter_hdl,
                self.state_name(enter_hdl)
            );
            self.idxs_enter_fns.push(enter_hdl);
            enter_hdl = if let Some(hdl) = self.states[enter_hdl].parent {
                hdl
            } else {
                break;
            };
        }
    }

    pub fn state_name(&self, hdl: usize) -> &str {
        &self.states[hdl].name
    }

    pub fn current_state_name(&self) -> &str {
        self.state_name(self.idx_current_state)
    }

    pub fn get_sm(&mut self) -> &SM {
        &self.sm
    }

    pub fn get_state_enter_cnt(&self, hdl: usize) -> usize {
        self.states[hdl].enter_cnt
    }
    pub fn get_state_process_cnt(&self, hdl: usize) -> usize {
        self.states[hdl].process_cnt
    }

    pub fn get_state_exit_cnt(&self, hdl: usize) -> usize {
        self.states[hdl].exit_cnt
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
            self.idxs_enter_fns.push(cur_hdl);

            cur_hdl = if let Some(hdl) = self.states[cur_hdl].parent {
                hdl
            } else {
                // Exit state_infos[self.current_state_infos_hdl] and all its parents
                log::trace!(
                    "setup_exit_enter_fns_hdls: cur_hdl={} {} has no parent exit_sentinel=None",
                    cur_hdl,
                    self.state_name(cur_hdl)
                );
                break None;
            };

            if self.states[cur_hdl].active {
                // Exit state_infos[self.current_state_infos_hdl] and
                // parents upto but excluding state_infos[cur_hdl]
                log::trace!(
                    "setup_exit_enter_fns_hdls: cur_hdl={} {} is active so it's exit_sentinel",
                    cur_hdl,
                    self.state_name(cur_hdl)
                );
                break Some(cur_hdl);
            }
        };

        // Starting at self.current_state_infos_hdl generate the
        // list of StateFns that we're going to exit. If exit_sentinel is None
        // then exit from current_state_infos_hdl and all of its parents.
        // If exit_sentinel is Some then exit from the current state_infos_hdl
        // up to but not including the exit_sentinel.
        let mut exit_hdl = self.idx_current_state;

        // Always exit the first state, this handles the special case
        // where Some(exit_hdl) == exit_sentinel.
        log::trace!(
            "setup_exit_enter_fns_hdls: push_back(curren_state_infos_hdl={} {})",
            exit_hdl,
            self.state_name(exit_hdl)
        );
        self.idxs_exit_fns.push_back(exit_hdl);

        loop {
            exit_hdl = if let Some(hdl) = self.states[exit_hdl].parent {
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
            self.idxs_exit_fns.push_back(exit_hdl);
        }
    }

    pub fn dispatch_hdl(&mut self, msg: &P, hdl: usize) {
        log::trace!("dispatch_hdl:+ hdl={} {}", hdl, self.state_name(hdl));

        if self.current_state_changed {
            // Execute the enter functions
            while let Some(enter_hdl) = self.idxs_enter_fns.pop() {
                if let Some(state_enter) = self.states[enter_hdl].enter {
                    log::trace!(
                        "dispatch_hdl: entering hdl={} {}",
                        enter_hdl,
                        self.state_name(enter_hdl)
                    );
                    self.states[enter_hdl].enter_cnt += 1;
                    (state_enter)(&mut self.sm, msg);
                    self.states[enter_hdl].active = true;
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

        self.states[hdl].process_cnt += 1;
        match (self.states[hdl].process)(&mut self.sm, msg) {
            StateResult::NotHandled => {
                if let Some(parent_hdl) = self.states[hdl].parent {
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

                self.idx_previous_state = self.idx_current_state;
                self.idx_current_state = next_state_hdl;
                self.current_state_changed = true;
            }
        }

        if self.current_state_changed {
            while let Some(exit_hdl) = self.idxs_exit_fns.pop_front() {
                if let Some(state_exit) = self.states[exit_hdl].exit {
                    log::trace!(
                        "dispatch_hdl: exiting hdl={} {}",
                        exit_hdl,
                        self.state_name(exit_hdl)
                    );
                    self.states[exit_hdl].exit_cnt += 1;
                    (state_exit)(&mut self.sm, msg);
                    self.states[exit_hdl].active = false;
                }
            }
        }

        log::trace!("dispatch_hdl:- hdl={} {}", hdl, self.state_name(hdl));
    }

    pub fn dispatch(&mut self, msg: &P) {
        log::trace!(
            "dispatch:+ current_state_infos_hdl={} {}",
            self.idx_current_state,
            self.current_state_name()
        );
        self.dispatch_hdl(msg, self.idx_current_state);
        log::trace!(
            "dispatch:- current_state_infos_hdl={} {}",
            self.idx_current_state,
            self.current_state_name()
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // StateMachine simply transitions back and forth
    // between initial and other.
    //
    //                base=0
    //        --------^  ^-------
    //       /                   \
    //      /                     \
    //    other=2   <======>   initial=1

    #[derive(Default)]
    struct StateMachine;

    // Create a Protocol with no messages
    struct NoMessages;

    const MAX_STATES: usize = 3;
    const BASE_HDL: usize = 0;
    const INITIAL_HDL: usize = 1;
    const OTHER_HDL: usize = 2;

    impl StateMachine {
        pub fn new() -> StateMachineExecutor<Self, NoMessages> {
            let sm = StateMachine::default();
            let mut sme = StateMachineExecutor::build(sm, MAX_STATES, INITIAL_HDL);

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
                sme.idxs_enter_fns
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
        assert_eq!(sme.get_state_enter_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_process_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_exit_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_enter_cnt(INITIAL_HDL), 0);
        assert_eq!(sme.get_state_process_cnt(INITIAL_HDL), 0);
        assert_eq!(sme.get_state_exit_cnt(INITIAL_HDL), 0);
        assert_eq!(sme.get_state_enter_cnt(OTHER_HDL), 0);
        assert_eq!(sme.get_state_process_cnt(OTHER_HDL), 0);
        assert_eq!(sme.get_state_exit_cnt(OTHER_HDL), 0);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(BASE_HDL), 1);
        assert_eq!(sme.get_state_process_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_exit_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_enter_cnt(INITIAL_HDL), 1);
        assert_eq!(sme.get_state_process_cnt(INITIAL_HDL), 1);
        assert_eq!(sme.get_state_exit_cnt(INITIAL_HDL), 1);
        assert_eq!(sme.get_state_enter_cnt(OTHER_HDL), 0);
        assert_eq!(sme.get_state_process_cnt(OTHER_HDL), 0);
        assert_eq!(sme.get_state_exit_cnt(OTHER_HDL), 0);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(BASE_HDL), 1);
        assert_eq!(sme.get_state_process_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_exit_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_enter_cnt(INITIAL_HDL), 1);
        assert_eq!(sme.get_state_process_cnt(INITIAL_HDL), 1);
        assert_eq!(sme.get_state_exit_cnt(INITIAL_HDL), 1);
        assert_eq!(sme.get_state_enter_cnt(OTHER_HDL), 1);
        assert_eq!(sme.get_state_process_cnt(OTHER_HDL), 1);
        assert_eq!(sme.get_state_exit_cnt(OTHER_HDL), 1);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(BASE_HDL), 1);
        assert_eq!(sme.get_state_process_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_exit_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_enter_cnt(INITIAL_HDL), 2);
        assert_eq!(sme.get_state_process_cnt(INITIAL_HDL), 2);
        assert_eq!(sme.get_state_exit_cnt(INITIAL_HDL), 2);
        assert_eq!(sme.get_state_enter_cnt(OTHER_HDL), 1);
        assert_eq!(sme.get_state_process_cnt(OTHER_HDL), 1);
        assert_eq!(sme.get_state_exit_cnt(OTHER_HDL), 1);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(BASE_HDL), 1);
        assert_eq!(sme.get_state_process_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_exit_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_enter_cnt(INITIAL_HDL), 2);
        assert_eq!(sme.get_state_process_cnt(INITIAL_HDL), 2);
        assert_eq!(sme.get_state_exit_cnt(INITIAL_HDL), 2);
        assert_eq!(sme.get_state_enter_cnt(OTHER_HDL), 2);
        assert_eq!(sme.get_state_process_cnt(OTHER_HDL), 2);
        assert_eq!(sme.get_state_exit_cnt(OTHER_HDL), 2);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(BASE_HDL), 1);
        assert_eq!(sme.get_state_process_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_exit_cnt(BASE_HDL), 0);
        assert_eq!(sme.get_state_enter_cnt(INITIAL_HDL), 3);
        assert_eq!(sme.get_state_process_cnt(INITIAL_HDL), 3);
        assert_eq!(sme.get_state_exit_cnt(INITIAL_HDL), 3);
        assert_eq!(sme.get_state_enter_cnt(OTHER_HDL), 2);
        assert_eq!(sme.get_state_process_cnt(OTHER_HDL), 2);
        assert_eq!(sme.get_state_exit_cnt(OTHER_HDL), 2);
    }

    #[test]
    fn test_leaf_transitions_in_a_tree() {
        test_transition_between_leafs_in_a_tree();
    }
}
