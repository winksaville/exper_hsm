use std::sync::mpsc::{Sender, Receiver, RecvError, SendError, TryRecvError};

use custom_logger::env_logger_init;

use hsm0_executor::{DynError, Executor, StateInfo, StateResult, Handled};

#[derive(Debug)]
#[allow(unused)]
pub struct MsgMgr<M> {
    primary_tx: Sender<M>,
    primary_rx: Receiver<M>,
    defer_tx: [Sender<M>; 2],
    defer_rx: [Receiver<M>; 2],
    current_defer_idx: usize,
}

#[allow(unused)]
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

    fn send(&self, m: M) -> Result<(), SendError<M>>  {
        self.primary_tx.send(m)
    }

    fn clone_sender(&self) -> Sender<M> {
        self.primary_tx.clone()
    }

    fn defer_try_recv(&self) -> Result<M, TryRecvError> {
        self.defer_rx[self.current_defer_idx].try_recv()
    }

    fn defer_send(&self,m: M) -> Result<(), SendError<M>> {
        self.defer_tx[self.current_defer_idx].send(m)
    }

    fn current_defer(&self) -> (&Sender<M>, &Receiver<M>) {
        (&self.defer_tx[self.current_defer_idx], &self.defer_rx[self.current_defer_idx])
    }

    fn other_defer(&self) -> (&Sender<M>, &Receiver<M>) {
        (&self.defer_tx[(self.current_defer_idx + 1) % self.defer_tx.len()],
        &self.defer_rx[(self.current_defer_idx + 1) % self.defer_rx.len()])
    }

}

#[derive(Debug, Clone)]
enum Messages {
    Value {
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
const IDX_BASE: usize = 0;
const IDX_DONE: usize = 1;
const IDX_PREPARE_TO_COMPLETE: usize = 2;

impl<'a> SendMsgToSelfSm<'a> {
    pub fn new(mm: &'a MsgMgr<Messages>) -> Result<Executor<Self, Messages>, DynError> {
        let sm = SendMsgToSelfSm { mm, val: 0 };
        let mut sme = Executor::new(sm, MAX_STATES);

        sme.state(StateInfo::new(
            "base",
            None,
            Self::base,
            None,
            None,
        ))
        .state(StateInfo::new(
            "prepare_to_done",
            None,
            Self::prepare_to_complete,
            None,
            None))
        .state(StateInfo::new(
            "done",
            None,
            Self::done,
            None,
            None))
        .initialize(IDX_BASE)
        .expect("Unexpected error initializing");

        log::info!(
            "new: inital state={} idxs_enter_fns={:?}",
            sme.get_current_state_name(),
            sme.idxs_enter_fns
        );

        Ok(sme)
    }

    fn base(&mut self, msg: &Messages) -> StateResult {

        match msg {
            Messages::Value { val } => {
                log::info!("base Messages::Value:+ val={}", val);
                if self.val < 10 {
                    // Doing work, 
                    self.val += val;
                    if self.mm.send(msg.clone()).is_ok() {
                        log::info!("base Messages::Value:- self.val={}", self.val);
                        (Handled::Yes, None)
                    } else {
                        log::info!("base Messages::Value:- ERR so DONE self.val={}", self.val);
                        (Handled::Yes, Some(IDX_DONE))
                    }

                } else {
                    // We're done
                    self.send_done();

                    log::info!("base Messages::Value:- Done self.val={}", self.val);
                    (Handled::Yes, Some(IDX_PREPARE_TO_COMPLETE))
                }
            }
            Messages::Done { val: _ } => {
                self.send_done();
                (Handled::Yes, Some(IDX_DONE))
            }
        }
    }

    fn prepare_to_complete(&mut self, _msg: &Messages) -> StateResult {
        // Responsed with Done for any messages
        //self.send_done();
        log::info!("prepare_to_complete:+- self.val={}", self.val);
        (Handled::Yes, None)
    }

    fn done(&mut self, _msg: &Messages) -> StateResult {
        // Responsed with Done for any messages
        self.send_done();
        log::info!("done: self.val={}", self.val);
        (Handled::Yes, None)
    }

    fn send_done(&mut self) {
        self.mm.send(Messages::Done { val: self.val }).ok();
    }
}

fn main() {
    env_logger_init("info");
    log::info!("main:+");

    //let (tx, rx) = std::sync::mpsc::channel::<Messages>();
    let mm = MsgMgr::<Messages>::new();
    let mut sme = SendMsgToSelfSm::new(&mm).unwrap();

    // Dispatch the first message
    let msg = Messages::Value{ val: 1 };
    sme.dispatch(&msg);

    // Receive messages until SendMsgToSelfSm reports Done or rx is closed
    while let Ok(m) = mm.recv() {
        match m {
            Messages::Value { val: _ } => {
                // Dispatch the message received
                sme.dispatch(&m);
            }
            Messages::Done { val } => {
                println!("main: Done val={val}");
                break;
            }
        }
    }

    log::info!("main:-");
}
