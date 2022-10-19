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
        idx_parent: Option<usize>,
    ) -> Self {
        StateInfo {
            name: name.to_owned(),
            parent: idx_parent,
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
    //pub transition_dest_idx: Option<usize>,
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
            //transition_dest_idx: Option<usize>,
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
    // idx_initial_state parameter in build.
    pub fn initialize(&mut self) {
        let mut idx_enter = self.idx_current_state;
        loop {
            //log::trace!("initial_enter_fns_idxs: push idx_enter={} {}", idx_enter, self.state_name(idx_enter));
            self.idxs_enter_fns.push(idx_enter);
            idx_enter = if let Some(idx) = self.states[idx_enter].parent {
                idx
            } else {
                break;
            };
        }
    }

    pub fn get_state_name(&self, idx: usize) -> &str {
        &self.states[idx].name
    }

    pub fn get_current_state_name(&self) -> &str {
        self.get_state_name(self.idx_current_state)
    }

    pub fn get_sm(&mut self) -> &SM {
        &self.sm
    }

    pub fn get_state_enter_cnt(&self, idx: usize) -> usize {
        self.states[idx].enter_cnt
    }
    pub fn get_state_process_cnt(&self, idx: usize) -> usize {
        self.states[idx].process_cnt
    }

    pub fn get_state_exit_cnt(&self, idx: usize) -> usize {
        self.states[idx].exit_cnt
    }

    fn setup_exit_enter_fns_idxs(&mut self, idx_next_state: usize) {
        let mut cur_idx = idx_next_state;

        // Setup the enter vector
        let exit_sentinel = loop {
            //log::trace!("setup_exit_enter_fns_idxs: cur_idx={} {}, TOL", cur_idx, self.state_name(cur_idx));
            self.idxs_enter_fns.push(cur_idx);

            cur_idx = if let Some(idx) = self.states[cur_idx].parent {
                idx
            } else {
                // Exit state_infos[self.current_state_infos_idx] and all its parents
                //log::trace!("setup_exit_enter_fns_idxs: cur_idx={} {} has no parent exit_sentinel=None", cur_dx, self.state_name(cur_idx));
                break None;
            };

            if self.states[cur_idx].active {
                // Exit state_infos[self.current_state_infos_idx] and
                // parents upto but excluding state_infos[cur_idx]
                //log::trace!("setup_exit_enter_fns_idxs: cur_idx={} {} is active so it's exit_sentinel", cur_idx, self.state_name(cur_idx));
                break Some(cur_idx);
            }
        };

        // Starting at self.idx_current_state generate the
        // list of StateFns that we're going to exit. If exit_sentinel is None
        // then exit from idx_current_state and all of its parents.
        // If exit_sentinel is Some then exit from the idx_current_state
        // up to but not including the exit_sentinel.
        let mut idx_exit = self.idx_current_state;

        // Always exit the first state, this handles the special case
        // where Some(idx_exit) == exit_sentinel.
        //log::trace!("setup_exit_enter_fns_idxs: push_back(idx_exit={} {})", idx_exit, self.state_name(idx_exit));
        self.idxs_exit_fns.push_back(idx_exit);

        loop {
            idx_exit = if let Some(idx) = self.states[idx_exit].parent {
                idx
            } else {
                // No parent we're done
                //log::trace!("setup_exit_enter_fns_idxs: No parent idx_exit={} {}, return", idx_exit, self.state_name(idx_exit));
                return;
            };

            if Some(idx_exit) == exit_sentinel {
                // Reached the exit sentinel so we're done
                //log::trace!("setup_exit_enter_fns_idxs: idx_exit={} {} == exit_sentinel={} {}, reached exit_sentinel return", idx_exit, self.state_name(idx_exit), exit_sentinel.unwrap(), self.state_name(exit_sentinel.unwrap()));
                return;
            }

            //log::trace!( "setup_exit_enter_fns_idxs: push_back(idx_exit={} {})", idx_exit, self.state_name(idx_exit));
            self.idxs_exit_fns.push_back(idx_exit);
        }
    }

    pub fn dispatch_idx(&mut self, msg: &P, idx: usize) {
        //log::trace!("dispatch_idx:+ idx={} {}", idx, self.state_name(idx));

        if self.current_state_changed {
            // Execute the enter functions
            while let Some(idx_enter) = self.idxs_enter_fns.pop() {
                if let Some(state_enter) = self.states[idx_enter].enter {
                    //log::trace!("dispatch_idx: entering idx={} {}", idx_enter, self.state_name(idx_enter));
                    self.states[idx_enter].enter_cnt += 1;
                    (state_enter)(&mut self.sm, msg);
                    self.states[idx_enter].active = true;
                }
            }
            self.current_state_changed = false;
        }

        // Invoke the current state funtion processing the result
        //log::trace!("dispatch_idx: processing idx={} {}", idx, self.state_name(idx));

        self.states[idx].process_cnt += 1;
        match (self.states[idx].process)(&mut self.sm, msg) {
            StateResult::NotHandled => {
                if let Some(idx_parent) = self.states[idx].parent {
                    //log::trace!("dispatch_idx: idx={} {} NotHandled, recurse into dispatch_idx", idx, self.state_name(idx));
                    self.dispatch_idx(msg, idx_parent);
                } else {
                    //log::trace!("dispatch_idx: idx={} {}, NotHandled, no parent, ignoring messages", idx, self.state_name(idx));
                }
            }
            StateResult::Handled => {
                // Nothing to do
                //log::trace!("dispatch_idx: idx={} {} Handled", idx, self.state_name(idx));
            }
            StateResult::TransitionTo(idx_next_state) => {
                //log::trace!("dispatch_idx: transition_to idx={} {}", idx_next_state, self.state_name(idx_next_state));
                self.setup_exit_enter_fns_idxs(idx_next_state);

                self.idx_previous_state = self.idx_current_state;
                self.idx_current_state = idx_next_state;
                self.current_state_changed = true;
            }
        }

        if self.current_state_changed {
            while let Some(idx_exit) = self.idxs_exit_fns.pop_front() {
                if let Some(state_exit) = self.states[idx_exit].exit {
                    //log::trace!("dispatch_idx: exiting idx={} {}", idx_exit, self.state_name(idx_exit));
                    self.states[idx_exit].exit_cnt += 1;
                    (state_exit)(&mut self.sm, msg);
                    self.states[idx_exit].active = false;
                }
            }
        }

        //log::trace!("dispatch_idx:- idx={} {}", idx, self.state_name(idx));
    }

    pub fn dispatch(&mut self, msg: &P) {
        //log::trace!( "dispatch:+ current_state_infos_idx={} {}", self.idx_current_state, self.current_state_name());
        self.dispatch_idx(msg, self.idx_current_state);
        //log::trace!( "dispatch:- current_state_infos_idx={} {}", self.idx_current_state, self.current_state_name());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // Test SM with one state with one field
    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test_sm_1s_no_enter_no_exit() {
        pub struct StateMachine {
            state: i32,
        }

        // Create a Protocol
        pub struct NoMessages;

        const MAX_STATES: usize = 1;
        const IDX_STATE1: usize = 0;

        impl StateMachine {
            pub fn new() -> StateMachineExecutor<Self, NoMessages> {
                let sm = StateMachine { state: 0 };
                let mut sme = StateMachineExecutor::build(sm, MAX_STATES, IDX_STATE1);

                sme.add_state(StateInfo::new("state1", None, Self::state1, None, None))
                    .initialize();

                sme
            }

            fn state1(&mut self, _msg: &NoMessages) -> StateResult {
                self.state += 1;

                StateResult::Handled
            }
        }

        // Create a sme and validate it's in the expected state
        let mut sme = StateMachine::new();
        assert_eq!(std::mem::size_of_val(sme.get_sm()), 4);
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_sm().state, 0);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_sm().state, 1);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 2);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_sm().state, 2);
    }

    // Test SM with one state getting names
    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test_sm_1s_get_names() {
        pub struct StateMachine {
            state: i32,
        }

        // Create a Protocol
        pub struct NoMessages;

        const MAX_STATES: usize = 1;
        const IDX_STATE1: usize = 0;

        impl StateMachine {
            pub fn new() -> StateMachineExecutor<Self, NoMessages> {
                let sm = StateMachine { state: 0 };
                let mut sme = StateMachineExecutor::build(sm, MAX_STATES, IDX_STATE1);

                sme.add_state(StateInfo::new("state1", None, Self::state1, None, None))
                    .initialize();

                sme
            }

            fn state1(&mut self, _msg: &NoMessages) -> StateResult {
                self.state += 1;

                StateResult::Handled
            }
        }

        // Create a sme and validate it's in the expected state
        let mut sme = StateMachine::new();
        assert_eq!(sme.get_sm().state, 0);
        assert_eq!(sme.get_state_name(IDX_STATE1), "state1");
        assert_eq!(sme.get_current_state_name(), "state1");

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_sm().state, 1);
        assert_eq!(sme.get_state_name(IDX_STATE1), "state1");
        assert_eq!(sme.get_current_state_name(), "state1");

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_sm().state, 2);
        assert_eq!(sme.get_state_name(IDX_STATE1), "state1");
        assert_eq!(sme.get_current_state_name(), "state1");
    }

    // Test SM with one state getting names
    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test_sm_2s_get_names() {
        pub struct StateMachine {
            state: i32,
        }

        // Create a Protocol
        pub struct NoMessages;

        const MAX_STATES: usize = 2;
        const IDX_STATE1: usize = 0;
        const IDX_STATE2: usize = 1;

        impl StateMachine {
            pub fn new() -> StateMachineExecutor<Self, NoMessages> {
                let sm = StateMachine { state: 0 };
                let mut sme = StateMachineExecutor::build(sm, MAX_STATES, IDX_STATE1);

                sme
                    .add_state(StateInfo::new("state1", None, Self::state1, None, None))
                    .add_state(StateInfo::new("state2", None, Self::state2, None, None))
                    .initialize();

                sme
            }

            fn state1(&mut self, _msg: &NoMessages) -> StateResult {
                self.state += 1;

                StateResult::TransitionTo(IDX_STATE2)
            }

            fn state2(&mut self, _msg: &NoMessages) -> StateResult {
                self.state -= 1;

                StateResult::TransitionTo(IDX_STATE1)
            }
        }

        // Create a sme and validate it's in the expected state
        let mut sme = StateMachine::new();
        assert_eq!(sme.get_sm().state, 0);
        assert_eq!(sme.get_state_name(IDX_STATE1), "state1");
        assert_eq!(sme.get_current_state_name(), "state1");

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_sm().state, 1);
        assert_eq!(sme.get_state_name(IDX_STATE2), "state2");
        assert_eq!(sme.get_current_state_name(), "state2");

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_sm().state, 0);
        assert_eq!(sme.get_state_name(IDX_STATE1), "state1");
        assert_eq!(sme.get_current_state_name(), "state1");
    }

    // Test SM with one state with one field
    // plus derive Default
    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test_sm_1s_enter_no_exit() {
        #[derive(Default)]
        pub struct StateMachine {
            state: i32,
        }

        // Create a Protocol
        pub enum Message {
            Add { val: i32 },
            Sub { val: i32 },
        }

        const MAX_STATES: usize = 1;
        const IDX_STATE1: usize = 0;

        impl StateMachine {
            pub fn new() -> StateMachineExecutor<Self, Message> {
                let sm = StateMachine::default();
                let mut sme = StateMachineExecutor::build(sm, MAX_STATES, IDX_STATE1);

                sme.add_state(StateInfo::new(
                    "state1",
                    Some(Self::state1_enter),
                    Self::state1,
                    None,
                    None,
                ))
                .initialize();

                sme
            }

            fn state1_enter(&mut self, _msg: &Message) {
                self.state = 100;
            }

            fn state1(&mut self, msg: &Message) -> StateResult {
                match msg {
                    Message::Add { val } => self.state += val,
                    Message::Sub { val } => self.state -= val,
                }
                StateResult::Handled
            }
        }

        // Create a sme and validate it's in the expected state
        let mut sme = StateMachine::new();
        assert_eq!(std::mem::size_of_val(sme.get_sm()), 4);
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_sm().state, 0);

        sme.dispatch(&Message::Add { val: 2 });
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_sm().state, 102);

        sme.dispatch(&Message::Sub { val: 1 });
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 2);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_sm().state, 101);
    }

    // Test SM with twos state with one field
    // plus derive Default
    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test_sm_2s_no_enter_no_exit() {
        #[derive(Default)]
        pub struct StateMachine {
            state: i32,
        }

        // Create a Protocol
        pub enum Message {
            Add { val: i32 },
            Sub { val: i32 },
        }

        const MAX_STATES: usize = 2;
        const IDX_STATE1: usize = 0;
        const IDX_STATE2: usize = 1;

        impl StateMachine {
            pub fn new() -> StateMachineExecutor<Self, Message> {
                let sm = StateMachine::default();
                let mut sme = StateMachineExecutor::build(sm, MAX_STATES, IDX_STATE1);

                sme.add_state(StateInfo::new("state1", None, Self::state1, None, None))
                    .add_state(StateInfo::new("state1", None, Self::state2, None, None))
                    .initialize();

                sme
            }

            fn state1(&mut self, msg: &Message) -> StateResult {
                match msg {
                    Message::Add { val } => self.state += val,
                    Message::Sub { val } => self.state -= val,
                }
                StateResult::TransitionTo(IDX_STATE2)
            }

            fn state2(&mut self, msg: &Message) -> StateResult {
                match msg {
                    Message::Add { val } => self.state += 2 * val,
                    Message::Sub { val } => self.state -= 2 * val,
                }
                StateResult::TransitionTo(IDX_STATE1)
            }
        }

        // Create a sme and validate it's in the expected state
        let mut sme = StateMachine::new();
        assert_eq!(std::mem::size_of_val(sme.get_sm()), 4);
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE2), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE2), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE2), 0);
        assert_eq!(sme.get_sm().state, 0);

        sme.dispatch(&Message::Add { val: 2 });
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE2), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE2), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE2), 0);
        assert_eq!(sme.get_sm().state, 2);

        sme.dispatch(&Message::Sub { val: 1 });
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE1), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE1), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_STATE2), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_STATE2), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_STATE2), 0);
        assert_eq!(sme.get_sm().state, 0);
    }

    // Test SM with twos state with one field
    // plus derive Default
    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test_sm_1h_2s_not_handled_no_enter_no_exit() {
        #[derive(Default)]
        pub struct StateMachine {
            state: i32,
        }

        // Create a Protocol
        pub enum Message {
            Add { val: i32 },
            Sub { val: i32 },
        }

        const MAX_STATES: usize = 2;
        const IDX_PARENT: usize = 0;
        const IDX_CHILD: usize = 1;

        impl StateMachine {
            pub fn new() -> StateMachineExecutor<Self, Message> {
                let sm = StateMachine::default();
                let mut sme = StateMachineExecutor::build(sm, MAX_STATES, IDX_CHILD);

                sme.add_state(StateInfo::new("parent", None, Self::parent, None, None))
                    .add_state(StateInfo::new(
                        "child",
                        None,
                        Self::child,
                        None,
                        Some(IDX_PARENT),
                    ))
                    .initialize();

                sme
            }

            fn parent(&mut self, msg: &Message) -> StateResult {
                match msg {
                    Message::Add { val } => self.state += val,
                    Message::Sub { val } => self.state -= val,
                }
                StateResult::Handled
            }

            fn child(&mut self, _msg: &Message) -> StateResult {
                StateResult::NotHandled
            }
        }

        // Create a sme and validate it's in the expected state
        let mut sme = StateMachine::new();
        assert_eq!(std::mem::size_of_val(sme.get_sm()), 4);
        assert_eq!(sme.get_state_enter_cnt(IDX_PARENT), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_PARENT), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_PARENT), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_CHILD), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_CHILD), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_CHILD), 0);
        assert_eq!(sme.get_sm().state, 0);

        sme.dispatch(&Message::Add { val: 2 });
        assert_eq!(sme.get_state_enter_cnt(IDX_PARENT), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_PARENT), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_PARENT), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_CHILD), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_CHILD), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_CHILD), 0);
        assert_eq!(sme.get_sm().state, 2);

        sme.dispatch(&Message::Sub { val: 1 });
        assert_eq!(sme.get_state_enter_cnt(IDX_PARENT), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_PARENT), 2);
        assert_eq!(sme.get_state_exit_cnt(IDX_PARENT), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_CHILD), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_CHILD), 2);
        assert_eq!(sme.get_state_exit_cnt(IDX_CHILD), 0);
        assert_eq!(sme.get_sm().state, 1);
    }

    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test_leaf_transitions_in_a_tree() {
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
        const IDX_BASE: usize = 0;
        const IDX_INITIAL: usize = 1;
        const IDX_OTHER: usize = 2;

        impl StateMachine {
            pub fn new() -> StateMachineExecutor<Self, NoMessages> {
                let sm = StateMachine::default();
                let mut sme = StateMachineExecutor::build(sm, MAX_STATES, IDX_INITIAL);

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
                    Some(IDX_BASE),
                ))
                .add_state(StateInfo::new(
                    "other",
                    Some(Self::other_enter),
                    Self::other,
                    Some(Self::other_exit),
                    Some(IDX_BASE),
                ))
                .initialize();

                sme
            }

            fn base_enter(&mut self, _msg: &NoMessages) {}

            // This state has idx 0
            fn base(&mut self, _msg: &NoMessages) -> StateResult {
                StateResult::Handled
            }

            fn base_exit(&mut self, _msg: &NoMessages) {}

            fn initial_enter(&mut self, _msg: &NoMessages) {}

            // This state has idx 0
            fn initial(&mut self, _msg: &NoMessages) -> StateResult {
                StateResult::TransitionTo(IDX_OTHER)
            }

            fn initial_exit(&mut self, _msg: &NoMessages) {}

            fn other_enter(&mut self, _msg: &NoMessages) {}

            // This state has idx 0
            fn other(&mut self, _msg: &NoMessages) -> StateResult {
                StateResult::TransitionTo(IDX_INITIAL)
            }

            fn other_exit(&mut self, _msg: &NoMessages) {}
        }

        // Create a sme and validate it's in the expected state
        let mut sme = StateMachine::new();
        assert_eq!(std::mem::size_of_val(sme.get_sm()), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_INITIAL), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_INITIAL), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_INITIAL), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_OTHER), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_OTHER), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_OTHER), 0);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(IDX_BASE), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_INITIAL), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_INITIAL), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_INITIAL), 1);
        assert_eq!(sme.get_state_enter_cnt(IDX_OTHER), 0);
        assert_eq!(sme.get_state_process_cnt(IDX_OTHER), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_OTHER), 0);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(IDX_BASE), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_INITIAL), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_INITIAL), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_INITIAL), 1);
        assert_eq!(sme.get_state_enter_cnt(IDX_OTHER), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_OTHER), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_OTHER), 1);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(IDX_BASE), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_INITIAL), 2);
        assert_eq!(sme.get_state_process_cnt(IDX_INITIAL), 2);
        assert_eq!(sme.get_state_exit_cnt(IDX_INITIAL), 2);
        assert_eq!(sme.get_state_enter_cnt(IDX_OTHER), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_OTHER), 1);
        assert_eq!(sme.get_state_exit_cnt(IDX_OTHER), 1);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(IDX_BASE), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_INITIAL), 2);
        assert_eq!(sme.get_state_process_cnt(IDX_INITIAL), 2);
        assert_eq!(sme.get_state_exit_cnt(IDX_INITIAL), 2);
        assert_eq!(sme.get_state_enter_cnt(IDX_OTHER), 2);
        assert_eq!(sme.get_state_process_cnt(IDX_OTHER), 2);
        assert_eq!(sme.get_state_exit_cnt(IDX_OTHER), 2);

        sme.dispatch(&NoMessages);
        assert_eq!(sme.get_state_enter_cnt(IDX_BASE), 1);
        assert_eq!(sme.get_state_process_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_exit_cnt(IDX_BASE), 0);
        assert_eq!(sme.get_state_enter_cnt(IDX_INITIAL), 3);
        assert_eq!(sme.get_state_process_cnt(IDX_INITIAL), 3);
        assert_eq!(sme.get_state_exit_cnt(IDX_INITIAL), 3);
        assert_eq!(sme.get_state_enter_cnt(IDX_OTHER), 2);
        assert_eq!(sme.get_state_process_cnt(IDX_OTHER), 2);
        assert_eq!(sme.get_state_exit_cnt(IDX_OTHER), 2);
    }
}
