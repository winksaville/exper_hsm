use std::sync::mpsc::Sender;

use custom_logger::env_logger_init;

use hsm0_executor::{DynError, Executor, StateInfo, StateResult, Handled};


#[derive(Debug, Clone)]
struct Message {
    val: i32
}

#[derive(Debug)]
#[allow(unused)]
struct SendMsgToSelfSm {
    self_tx: Sender<Message>,
    val: i32
}

const MAX_STATES: usize = 2;
const IDX_BASE: usize = 0;

impl SendMsgToSelfSm {
    pub fn new(sender: Sender<Message>) -> Result<Executor<Self, Message>, DynError> {
        let sm = SendMsgToSelfSm { self_tx: sender, val: 0 };
        let mut sme = Executor::new(sm, MAX_STATES);

        sme.state(StateInfo::new(
            "base",
            None,
            Self::base,
            None,
            None,
        ))
        .initialize(IDX_BASE)
        .expect("Unexpected error initializing");

        log::trace!(
            "new: inital state={} idxs_enter_fns={:?}",
            sme.get_current_state_name(),
            sme.idxs_enter_fns
        );

        Ok(sme)
    }

    fn base(&mut self, msg: &Message) -> StateResult {
        log::info!("base:+ msg.val={}", msg.val);

        self.val += msg.val;
        self.self_tx.send(msg.clone()).unwrap();

        log::info!("base:- self.val={}", self.val);
        (Handled::Yes, None)
    }
}

fn main() {
    println!("main");
    env_logger_init("info");
    log::info!("main:+");

    let (tx, rx) = std::sync::mpsc::channel::<Message>();
    let mut sme = SendMsgToSelfSm::new(tx).unwrap();

    // Dispatch the first message
    let msg = Message { val: 1 };
    sme.dispatch(&msg);

    // Receive 10 message from the SendMsgToSelfSm
    for _ in 0..10 {
        let m = rx.recv().unwrap();    
        sme.dispatch(&m);
    }

    log::info!("main:-");
}
