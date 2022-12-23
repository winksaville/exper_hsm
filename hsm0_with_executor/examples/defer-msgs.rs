use std::{
    cell::RefCell,
    sync::mpsc::{Sender, TryRecvError},
};

use custom_logger::env_logger_init;

use hsm0_with_executor::{DynError, Executor, Handled, StateInfo, StateResult};

#[derive(Debug, Clone)]
enum Messages {
    DeferredValue { val: i32 },
    Complete { tx: Sender<Messages> },
    Done { val: i32 },
}

#[derive(Debug)]
struct DeferMsgsSm {
    val: i32,
}

const MAX_STATES: usize = 2;
const IDX_DEFERRING: usize = 0;
const IDX_DO_DEFERRED_WORK: usize = 1;

impl DeferMsgsSm {
    pub fn new() -> Result<Executor<Self, Messages>, DynError> {
        let sm = RefCell::new(DeferMsgsSm { val: 0 });
        let sme = Executor::new(sm, MAX_STATES)
            .state(StateInfo::new("starting", Self::deferring))
            .state(StateInfo::new("deferring", Self::do_deferred_work))
            .build(IDX_DEFERRING)
            .expect("Unexpected error initializing");

        log::info!(
            "new: inital state={} idxs_enter_fns={:?}",
            sme.get_current_state_name(),
            sme.idxs_enter_fns
        );

        Ok(sme)
    }

    fn deferring(&mut self, e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
        match msg {
            Messages::DeferredValue { val } => {
                log::info!("deferring: Messages::DeferredValue:+ val={}", val);
                e.defer_send(msg.clone()).unwrap();
                (Handled::Yes, None)
            }
            Messages::Complete { tx: _ } => {
                log::info!("deferring: Messages::Complete, transition to do_deferred_work");
                e.defer_send(msg.clone()).unwrap();
                (Handled::Yes, Some(IDX_DO_DEFERRED_WORK))
            }
            Messages::Done { val: _ } => {
                log::info!("deferring: Messages::Done, Unexpected Dropping");
                (Handled::Yes, None)
            }
        }
    }

    fn do_deferred_work(&mut self, _e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
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
            Messages::Complete { tx } => {
                let response = Messages::Done { val: self.val };
                log::info!("do_deferred_work: Messages::Complete, sending {response:?}, transition to deferring");
                tx.send(response).unwrap();
                (Handled::Yes, Some(IDX_DEFERRING))
            }
            Messages::Done { val: _ } => {
                log::info!("deferring: defer Messages::Done, Unexpected Dropping");
                (Handled::Yes, None)
            }
        }
    }
}

fn main() {
    env_logger_init("info");
    log::info!("main:+");

    let mut sme = DeferMsgsSm::new().unwrap();

    // Dispatch DeferredValue messages
    for _ in 0..10 {
        let msg = Messages::DeferredValue { val: 1 };
        sme.dispatcher(&msg);
        log::info!("main: Sent {msg:?}");
    }

    let (tx, rx) = std::sync::mpsc::channel::<Messages>();
    let msg = Messages::Complete { tx };
    sme.dispatcher(&msg);
    log::info!("main: Sent {msg:?}");

    // We should now recive one Messages::Done
    let result_value = match rx.recv() {
        Ok(response) => match response {
            Messages::Done { val } => {
                log::info!("main: Got Expected reponse={response:?}");
                val
            }
            _ => panic!("main: Unexpected response={response:?}"),
        },
        Err(e) => {
            panic!("main: No response message received: {e:?}");
        }
    };

    match rx.try_recv() {
        Ok(m) => {
            panic!("main: rx.try_recv() Unexpected msg {m:?}");
        }
        Err(e) => match e {
            TryRecvError::Empty => {
                log::info!("main: rx.try_recv() got the expected TryRecvError::Empty, e={e:?}");
            }
            TryRecvError::Disconnected => {
                panic!("main: rx.try_recv() Uexpected TryRecvError::Disconnected, e={e:?}");
            }
        },
    }

    assert_eq!(result_value, 10);
    log::info!("main:- result_value={result_value}");
}
