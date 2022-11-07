use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::{Receiver, RecvError, SendError, Sender, TryRecvError},
};

use custom_logger::env_logger_init;

use hsm0_executor::{DynError, Executor, Handled, StateInfo, StateResult};

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

    fn send(&self, m: M) -> Result<(), SendError<M>> {
        self.primary_tx.send(m)
    }

    //fn clone_sender(&self) -> Sender<M> {
    //    self.primary_tx.clone()
    //}

    pub fn defer_send(&self, m: M) -> Result<(), SendError<M>> {
        self.defer_tx[self.current_defer()].send(m)
    }

    fn defer_try_recv(&self) -> Result<M, TryRecvError> {
        self.defer_rx[self.other_defer()].try_recv()
    }

    fn next_defer(&mut self) {
        self.current_defer_idx = (self.current_defer_idx + 1) % self.defer_tx.len();
    }

    fn current_defer(&self) -> usize {
        self.current_defer_idx
    }

    fn other_defer(&self) -> usize {
        (self.current_defer_idx + 1) % self.defer_tx.len()
    }
}

type MsgMgrRcRefCell<M> = Rc<RefCell<MsgMgr<M>>>;

#[derive(Debug, Clone)]
enum Messages {
    DeferredValue { val: i32 },
    Done { val: i32 },
}

struct SendMsgToSelfSm<'a> {
    mm: &'a MsgMgrRcRefCell<Messages>,
    val: i32,
}

const MAX_STATES: usize = 3;
const IDX_DEFERRING: usize = 0;
const IDX_DO_DEFERRED_WORK: usize = 1;
const IDX_DONE: usize = 2;

impl<'a> SendMsgToSelfSm<'a> {
    pub fn new(mm: &'a MsgMgrRcRefCell<Messages>) -> Result<Executor<Self, Messages>, DynError> {
        let sm = SendMsgToSelfSm { mm, val: 0 };
        let mut sme = Executor::new(sm, MAX_STATES);

        sme.state(StateInfo::new(
            "starting",
            None,
            Self::deferring,
            None,
            None,
        ))
        .state(StateInfo::new(
            "deferring",
            None,
            Self::do_deferred_work,
            None,
            None,
        ))
        .state(StateInfo::new("done", None, Self::done, None, None))
        .initialize(IDX_DEFERRING)
        .expect("Unexpected error initializing");

        log::info!(
            "new: inital state={} idxs_enter_fns={:?}",
            sme.get_current_state_name(),
            sme.idxs_enter_fns
        );

        Ok(sme)
    }

    fn deferring(&mut self, msg: &Messages) -> StateResult {
        match msg {
            Messages::DeferredValue { val } => {
                log::info!("deferring: Messages::DeferredValue:+ val={}", val);
                self.mm
                    .borrow()
                    .defer_send(msg.clone())
                    .expect("defer_send failed unexpectedly");
                (Handled::Yes, None)
            }
            Messages::Done { val } => {
                log::info!("deferring: defer Messages::Done val={}", val);
                self.mm
                    .borrow()
                    .defer_send(msg.clone())
                    .expect("defer_send failed unexpectedly");
                (Handled::Yes, Some(IDX_DO_DEFERRED_WORK))
            }
        }
    }

    fn do_deferred_work(&mut self, msg: &Messages) -> StateResult {
        match msg {
            Messages::DeferredValue { val } => {
                self.val += val;
                log::info!(
                    "do_deferred_work: Messages::DeferredValue:+ val={} self.val={}",
                    val,
                    self.val
                );

                (Handled::Yes, None)
            }
            Messages::Done { val } => {
                log::info!("do_deferred_work: defer Messages::Done val={}", val);
                self.mm
                    .borrow()
                    .defer_send(msg.clone())
                    .expect("defer_send failed unexpectedly");

                (Handled::Yes, Some(IDX_DONE))
            }
        }
    }

    fn done(&mut self, _msg: &Messages) -> StateResult {
        // Responsed with Done for any messages
        self.mm.borrow().send(Messages::Done { val: self.val }).ok();

        log::info!("done: self.val={}", self.val);
        (Handled::Yes, None)
    }
}

fn main() {
    env_logger_init("info");
    log::info!("main:+");

    // We need two references to mm one in here, in main, and
    // another in SendMsgToSelfSm. Both need to be mutable so
    // we use an Rc for multiple references and that will have
    // an T of RefCell which whill have T of MsgMgr<Messages>.
    let mm: Rc<RefCell<MsgMgr<Messages>>> = Rc::new(RefCell::new(MsgMgr::<Messages>::new()));
    let mut sme = SendMsgToSelfSm::new(&mm).unwrap();

    // Dispatch DeferredValue messages
    for _ in 0..10 {
        let msg = Messages::DeferredValue { val: 1 };
        let transitioned = sme.dispatch(&msg);
        assert!(!transitioned);
        println!("main: Sent {msg:?} dispatch ret: {transitioned}");
    }

    let msg = Messages::Done { val: 0 };
    let transitioned = sme.dispatch(&msg);
    println!("main: Sent {msg:?} dispatch ret: {transitioned}");

    assert!(transitioned);

    // Process deferred messages until there are
    // no transitions and the other deferred channel
    // is empty.
    //   DANGER, this could be an endless loop if the
    //   states processed while deferring transition
    //   at least once and defers one of the messages!
    loop {
        let mut transitioned = false;

        mm.borrow_mut().next_defer();
        println!(
            "process deferred:+ TOLO other_defer={}",
            mm.borrow().other_defer()
        );

        // Dispatch deferred messages
        loop {
            println!(
                "process deferred:  TOLI other_defer={}",
                mm.borrow().other_defer()
            );

            let r = mm.borrow().defer_try_recv();
            match r {
                Ok(m) => {
                    transitioned |= sme.dispatch(&m);
                    if transitioned {
                        println!("process deferred:  defer_try_recv TRANSITIONED dispatch ret: {transitioned} received m: {:?}", m);
                    } else {
                        println!("process deferred:  defer_try_recv dispatch ret: {transitioned} received m: {:?}", m);
                    }
                }
                Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => {
                    println!("process deferred:  BOLI done, Empty/Disconnected");
                    break;
                }
            }
            println!("process deferred:  BOLI continuing");
        }

        println!("process deferred:- BOLO transitioned={transitioned}");
        if !transitioned {
            break;
        }
    }

    // We should now recive one Messages::Done
    match mm.borrow().recv() {
        Ok(m) => match m {
            Messages::DeferredValue { val: _ } => {
                panic!("main: mm.recv() msg: {m:?}");
            }
            Messages::Done { val: _ } => {
                println!("main: mm.recv() msg: {m:?}");
            }
        },
        Err(e) => panic!("main: mm.recv() error: {e:?}"),
    }

    match mm.borrow().try_recv() {
        Ok(m) => {
            panic!("main: mm.try_recv() Unexpected msg {m:?}");
        }
        Err(e) => match e {
            TryRecvError::Empty => {
                println!("main: mm.try_recv() got the expected TryRecvError::Empty")
            }
            TryRecvError::Disconnected => {
                panic!("main: mm.try_recv() Uexpected TryRecvError::Disconnected")
            }
        },
    }

    log::info!("main:-");
}
