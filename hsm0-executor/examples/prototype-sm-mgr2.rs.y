use std::sync::mpsc::{Sender, Receiver, RecvError, SendError, TryRecvError};

use custom_logger::env_logger_init;

use hsm0_executor::{DynError, Executor, StateInfo, StateResult, Handled};

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
        self.defer_rx[self.other_defer()].try_recv()
    }

    fn defer_send(&self,m: M) -> Result<(), SendError<M>> {
        self.defer_tx[self.current_defer()].send(m)
    }

    fn next_defer(&mut self) {
        self.current_defer_idx += 1 % self.defer_tx.len();
    }

    fn current_defer(&self) -> usize {
        self.current_defer_idx
    }

    fn other_defer(&self) -> usize {
        (self.current_defer_idx + 1) % self.defer_tx.len()
    }

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

#[derive(Debug)]
struct SendMsgToSelfSm<'a> {
    mm: &'a MsgMgr<Messages>,
    val: i32
}

const MAX_STATES: usize = 3;
const IDX_DEFERRING: usize = 0;
const IDX_DO_DEFERED_WORK: usize = 1;
const IDX_DONE: usize = 2;

impl<'a> SendMsgToSelfSm<'a> {
    pub fn new(mm: &'a MsgMgr<Messages>) -> Result<Executor<Self, Messages>, DynError> {
        let sm = SendMsgToSelfSm { mm, val: 0 };
        let mut sme = Executor::new(sm, MAX_STATES);

        sme.state(StateInfo::new(
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
                log::info!("deferring Messages::DeferedValue:+ val={}", val);
                self.mm.defer_send(msg.clone()).expect("defer_send failed unexpectedly");
                (Handled::Yes, None)
            }
            Messages::Done { val } => {
                log::info!("deferring: defer Messages::Done val={}", val);
                self.mm.defer_send(msg.clone()).expect("defer_send failed unexpectedly");
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
                self.mm.defer_send(msg.clone()).expect("defer_send failed unexpectedly");

                (Handled::Yes, Some(IDX_DONE))
            }
        }

    }

    fn done(&mut self, _msg: &Messages) -> StateResult {
        // Responsed with Done for any messages
        self.mm.send(Messages::Done { val: self.val }).ok();

        log::info!("done: self.val={}", self.val);
        (Handled::Yes, None)
    }
}

//fn dispatcher(sme: &mut Executor<SendMsgToSelfSm, Messages>, mm: &mut MsgMgr<Messages>, msg: &Messages) -> bool {
//    let transitioned = sme.dispatch(msg);
//
//    if transitioned {
//        mm.next_defer();
//
//
//        // Dispatch defered messages
//        loop {
//            match mm.defer_try_recv() {
//                Ok(m) => {
//                    // Transitions while handling previously deferred messages
//                    // will be "ignored" because if we did a "next_defer" here
//                    // the expected order would change as any "newly" deferred
//                    // message would be processed before "older" deferred messages!
//                    let transitioned = sme.dispatch(&m);
//                    println!("work: defer_try_recv dispatch ret: {transitioned} received m: {:?}", m);
//                }
//                Err(TryRecvError::Empty) |
//                Err(TryRecvError::Disconnected) => break,
//            }
//        }
//    }
//
//    true
//}

fn work() {
    log::info!("work:+");

    //let (tx, rx) = std::sync::mpsc::channel::<Messages>();
    let mm = MsgMgr::<Messages>::new();
    let mut sme = SendMsgToSelfSm::new(&mm).unwrap();

    // Dispatch DeferredValue messages
    for _ in 0..10 {
        let msg = Messages::DeferredValue { val: 1 };
        let transitioned = sme.dispatch(&msg);
        //let transitioned = dispatcher(&mut sme, &mut mm, &msg);
        println!("work: Sent {msg:?} dispatch ret: {transitioned}");
    }

    let msg = Messages::Done { val: 0 };
    let transitioned = sme.dispatch(&msg);
    println!("work: Sent {msg:?} dispatch ret: {transitioned}");

    assert!(transitioned);
    //mm.next_defer();

    // Dispatch defered messages
    loop {
        match mm.defer_try_recv() {
            Ok(m) => {
                // Transitions while handling previously deferred messages
                // will be "ignored" because if we did a "next_defer" here
                // the expected order would change as any "newly" deferred
                // message would be processed before "older" deferred messages!
                let transitioned = sme.dispatch(&m);
                println!("work: defer_try_recv dispatch ret: {transitioned} received m: {:?}", m);
            }
            Err(TryRecvError::Empty) |
            Err(TryRecvError::Disconnected) => break,
        }
    }

    // We should now recive one Messages::Done
    match mm.recv() {
        Ok(m) => {
            match m {
                Messages::DeferredValue { val: _ } => {
                    panic!("work: mm.recv() msg: {m:?}");
                }
                Messages::Done { val: _ } => {
                    println!("work: mm.recv() msg: {m:?}");
                }
            }
        }
        Err(e) => panic!("work: mm.recv() error: {e:?}"),
    }

    match mm.try_recv() {
        Ok(m) => {
            panic!("work: mm.try_recv() Unexpected msg {m:?}");
        }
        Err(e) => {
            match e {
                TryRecvError::Empty => println!("work: mm.try_recv() got the expected TryRecvError::Empty"),
                TryRecvError::Disconnected => panic!("work: mm.try_recv() Uexpected TryRecvError::Disconnected"),
            }
        }
    }

    log::info!("work:-");
}


fn main() {
    env_logger_init("info");
    log::info!("main:+");

    work();

    log::info!("main:-");
}
