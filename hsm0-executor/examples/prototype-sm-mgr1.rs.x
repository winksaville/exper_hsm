use std::sync::mpsc::{Sender, Receiver, RecvError, SendError, TryRecvError};

use custom_logger::env_logger_init;

use hsm0_executor::{DynError, Executor, StateInfo, StateResult, Handled};

pub struct SmMgr<SM, M> {
    e: Executor<SM, M>,
    mm: MsgMgr<M>,
}

impl<SM, P> SmMgr<SM, P> {
    fn new(sm: SM, max_states: usize) -> Self {
        let e = Executor::new(sm, max_states);
        let mm = MsgMgr::<P>::new();

        Self {
            e,
            mm,
        }
    }

    // Add a state to the the executor
    pub fn state(&mut self, state_info: StateInfo<SM, P>) -> &mut Self {
        self.e.state(state_info);

        self
    }

    pub fn initialize(&mut self, idx_initial_state: usize) -> Result<(), DynError> {
        self.e.initialize(idx_initial_state)
    }

    pub fn dispatch(&mut self, msg: &P) -> bool {
        self.e.dispatch(msg)
    }

    pub fn get_state_name(&self, idx: usize) -> &str {
        &self.e.states[idx].name
    }

    pub fn get_current_state_name(&self) -> &str {
        self.e.get_state_name(self.e.idx_current_state)
    }

    pub fn get_sm(&mut self) -> &SM {
        &self.e.sm
    }

    pub fn get_state_enter_cnt(&self, idx: usize) -> usize {
        self.e.states[idx].enter_cnt
    }
    pub fn get_state_process_cnt(&self, idx: usize) -> usize {
        self.e.states[idx].process_cnt
    }

    pub fn get_state_exit_cnt(&self, idx: usize) -> usize {
        self.e.states[idx].exit_cnt
    }

    // MsgMgr
    fn recv(&self) -> Result<P, RecvError> {
        self.mm.primary_rx.recv()
    }

    fn try_recv(&self) -> Result<P, TryRecvError> {
        self.mm.primary_rx.try_recv()
    }

    fn send(&self, m: P) -> Result<(), SendError<P>>  {
        self.mm.primary_tx.send(m)
    }

    fn defer_try_recv(&self) -> Result<P, TryRecvError> {
        self.mm.defer_rx[self.mm.current_defer_idx].try_recv()
    }

    fn defer_send(&self,m: P) -> Result<(), SendError<P>> {
        self.mm.defer_tx[self.mm.current_defer_idx].send(m)
    }
}


#[derive(Debug)]
pub struct MsgMgr<M> {
    primary_tx: Sender<M>,
    primary_rx: Receiver<M>,
    defer_tx: [Sender<M>; 2],
    defer_rx: [Receiver<M>; 2],
    current_defer_idx: usize,
}

impl<M> MsgMgr<M> {
    fn new() -> MsgMgr<M> {
        let (primary_tx, primary_rx) = std::sync::mpsc::channel::<M>();
        let (defer0_tx, defer0_rx) = std::sync::mpsc::channel::<M>();
        let (defer1_tx, defer1_rx) = std::sync::mpsc::channel::<M>();

        MsgMgr {
            primary_tx,
            primary_rx,
            defer_tx: [defer0_tx, defer1_tx],
            defer_rx: [defer0_rx, defer1_rx],
            current_defer_idx: 0,
        }
    }

    fn recv(&self) -> Result<M, RecvError> {
        self.primary_rx.recv()
    }

    fn try_recv(&self) -> Result<M, TryRecvError> {
        self.primary_rx.try_recv()
    }

    fn send(&self, m: M) -> Result<(), SendError<M>>  {
        self.primary_tx.send(m)
    }

    //fn clone_sender(&self) -> Sender<M> {
    //    self.primary_tx.clone()
    //}

    fn defer_try_recv(&self) -> Result<M, TryRecvError> {
        self.defer_rx[self.current_defer_idx].try_recv()
    }

    fn defer_send(&self,m: M) -> Result<(), SendError<M>> {
        self.defer_tx[self.current_defer_idx].send(m)
    }

    //fn current_defer(&self) -> (&Sender<M>, &Receiver<M>) {
    //    (&self.defer_tx[self.current_defer_idx], &self.defer_rx[self.current_defer_idx])
    //}

    //fn other_defer(&self) -> (&Sender<M>, &Receiver<M>) {
    //    (&self.defer_tx[(self.current_defer_idx + 1) % self.defer_tx.len()],
    //    &self.defer_rx[(self.current_defer_idx + 1) % self.defer_rx.len()])
    //}

}

#[derive(Debug, Clone)]
enum Messages {
    DeferredValue {
        val: i32,
    },
    Done {
        val: i32,
    },
}

struct SendMsgToSelfSm<'a> {
    smm: Option<&'a SmMgr<Self, Messages>>,
    val: i32,
}

const MAX_STATES: usize = 3;
const IDX_DEFERRING: usize = 0;
const IDX_DO_DEFERED_WORK: usize = 1;
const IDX_DONE: usize = 2;

impl<'a> SendMsgToSelfSm<'a> {
    pub fn new() -> Result<SmMgr<Self, Messages>, DynError> {
        let mut sm = SendMsgToSelfSm { smm: None, val: 0 };
        let mut smm = SmMgr::<Self, Messages>::new(sm, MAX_STATES);
        sm.smm = Some(&smm);


        smm.state(StateInfo::new(
            "starting",
            None,
            Self::deferring,
            None,
            None))
        .state(StateInfo::new(
            "deferring",
            None,
            Self::do_defered_work,
            None,
            None))
        .state(StateInfo::new(
            "done",
            None,
            Self::done,
            None,
            None))
        .initialize(IDX_DEFERRING)
        .expect("Unexpected error initializing");

        log::info!("new: inital state={}", smm.get_current_state_name());

        Ok(smm)
    }

    fn deferring(&mut self, msg: &Messages) -> StateResult {
        match msg {
            Messages::DeferredValue { val } => {
                log::info!("deferring Messages::DeferedValue:+ val={}", val);
                self.smm.unwrap().defer_send(msg.clone()).expect("defer_send failed unexpectedly");
                (Handled::Yes, None)
            }
            Messages::Done { val } => {
                log::info!("deferring: defer Messages::Done val={}", val);
                self.smm.unwrap().defer_send(msg.clone()).expect("defer_send failed unexpectedly");
                (Handled::Yes, Some(IDX_DO_DEFERED_WORK))
            }
        }
    }

    fn do_defered_work(&mut self, msg: &Messages) -> StateResult {
        match msg {
            Messages::DeferredValue { val } => {
                self.val += val;
                log::info!("do_defered_work: Messages::DeferedValue:+ val={} self.val={}", val, self.val);

                (Handled::Yes, None)
            }
            Messages::Done { val } => {
                log::info!("do_defered_work: defer Messages::Done val={}", val);
                self.smm.unwrap().defer_send(msg.clone()).expect("defer_send failed unexpectedly");

                (Handled::Yes, Some(IDX_DONE))
            }
        }

    }

    fn done(&mut self, _msg: &Messages) -> StateResult {
        // Responsed with Done for any messages
        self.smm.unwrap().send(Messages::Done { val: self.val }).ok();

        log::info!("done: self.val={}", self.val);
        (Handled::Yes, None)
    }
}

fn main() {
    env_logger_init("info");
    log::info!("main:+");

    let mut smm = SendMsgToSelfSm::new().unwrap();

    // Dispatch DeferredValue messages
    for _ in 0..10 {
        let msg = Messages::DeferredValue { val: 1 };
        let transitioned = smm.dispatch(&msg);
        println!("main: Sent {msg:?} dispatch ret: {transitioned}");
    }

    let msg = Messages::Done { val: 0 };
    let transitioned = smm.dispatch(&msg);
    println!("main: Sent {msg:?} dispatch ret: {transitioned}");

    // Dispatch defered messages
    loop {
        match smm.defer_try_recv() {
            Ok(m) => {
                let transitioned = smm.dispatch(&m);
                println!("main: defer_try_recv dispatch ret: {transitioned} received m: {:?}", m);
            }
            Err(TryRecvError::Empty) |
            Err(TryRecvError::Disconnected) => break,
        }
    }

    // We should now recive one Messages::Done
    match smm.recv() {
        Ok(m) => {
            match m {
                Messages::DeferredValue { val: _ } => {
                    panic!("main: mm.recv() msg: {m:?}");
                }
                Messages::Done { val: _ } => {
                    println!("main: mm.recv() msg: {m:?}");
                }
            }
        }
        Err(e) => panic!("main: mm.recv() error: {e:?}"),
    }

    match smm.try_recv() {
        Ok(m) => {
            panic!("main: mm.try_recv() Unexpected msg {m:?}");
        }
        Err(e) => {
            match e {
                TryRecvError::Empty => println!("main: mm.try_recv() got the expected TryRecvError::Empty"),
                TryRecvError::Disconnected => panic!("main: mm.try_recv() Uexpected TryRecvError::Disconnected"),
            }
        }
    }

    log::info!("main:-");
}
