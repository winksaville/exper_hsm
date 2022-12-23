/// Currently this is "working" but the biggest know problem
/// is that when passing buffers around I've got to clone them
/// when I'd like to achieve zero-copy!
use std::{
    cell::RefCell,
    fs::File,
    io::Read,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use custom_logger::env_logger_init;

use hsm0_with_executor::{DynError, Executor, Handled, StateInfo, StateResult};

#[derive(Debug, Clone)]
pub enum Messages {
    Open {
        // Name of file open
        file_name: String,
        buf_count: usize,
        buf_capacity: usize,
        partner_tx: Sender<Messages>,
    },
    Start,
    Read,

    // Currently I'm copying data around
    // but need to implement zero-copy
    Data {
        buf: Vec<u8>,
    },

    Empty {
        // Return an empty buffer
        buf: Box<Vec<u8>>,
    },

    Done {
        // true successful, false failed
        // TODO: Return Error
        result: bool,
    },

    StopThread,
}

#[allow(unused)]
#[derive(Debug)]
pub struct FileStreamProducer {
    // Public field
    pub tx: Sender<Messages>,

    // Private fields
    rx: Receiver<Messages>,
    partner_tx: Option<Sender<Messages>>,
    file: Option<File>,

    // Need "zero-copy" buffering, i.e. we need
    // to move buffers when using send!
    // For the moment we'll clone them and keep track
    // what what is empty in a stack, (empty_buffers)
    buffers: Vec<Box<Vec<u8>>>,
}

const MAX_STATES: usize = 5;
const IDX_BASE: usize = 0;
const IDX_OPEN: usize = 1;
const IDX_WAIT_FOR_START: usize = 2;
const IDX_READ: usize = 3;
const IDX_WAIT_FOR_EMPTY: usize = 4;

impl FileStreamProducer {
    fn new() -> Result<Executor<FileStreamProducer, Messages>, DynError> {
        let (tx, rx) = channel::<Messages>();

        let fsp = RefCell::new(Self {
            tx,
            rx,
            partner_tx: None,
            file: None,
            buffers: Vec::new(),
        });

        let sme = Executor::new(fsp, MAX_STATES)
            // IDX_BASE
            .state(StateInfo::new("base", Self::base))
            // IDX_OPEN
            .state(StateInfo::new("open", Self::open).parent_idx(IDX_BASE))
            // IDX_WAIT_FOR_START
            .state(StateInfo::new("wait_for_start", Self::wait_for_start).parent_idx(IDX_BASE))
            // IDX_READ
            .state(StateInfo::new("read", Self::read).parent_idx(IDX_BASE))
            // IDX_WAIT_FOR_EMPTY
            .state(StateInfo::new("wait_for_empty", Self::wait_for_empty).parent_idx(IDX_BASE))
            .build(IDX_OPEN);

        sme
    }

    // This is the parent of all states and handles all
    // as best as it can for now :)
    fn base(&mut self, e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
        match msg {
            Messages::Open { .. } => log::info!(
                "base: Ignoring Messages::Open in state {}",
                e.get_current_state_name()
            ),
            Messages::Start => log::info!(
                "base: Messages::Start not supported in {}",
                e.get_current_state_name()
            ),
            Messages::Read => log::info!(
                "base: Messages::Read not supported in {}",
                e.get_current_state_name()
            ),
            Messages::Data { .. } => panic!(
                "base: Messages::Data not supported in {}",
                e.get_current_state_name()
            ),
            // Maybe the msg parameters should be "msg: Messages" and we'd consume it
            // or "msg: &mut Messages" then we could "take" it??
            Messages::Empty { buf } => {
                log::info!("base: Messages::Empty: &buf[0]: {:p} {:0X?}", &buf[0], *buf);
                let x = buf.clone();
                log::info!("base: Messages::Empty:   &x[0]: {:p} {:0X?}", &x[0], x);
                self.buffers.push(x);
            }
            Messages::Done { result: _ } => panic!(
                "base: Messages:Done not supported in {}",
                e.get_current_state_name()
            ),

            Messages::StopThread => log::info!(
                "base: Messages::StopThread IGNORING {}",
                e.get_current_state_name()
            ),
        }

        (Handled::Yes, None)
    }

    fn open(&mut self, e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
        match msg {
            Messages::Open {
                file_name,
                buf_count,
                buf_capacity,
                partner_tx,
            } => {
                // TODO: I don't want to clone this, I'd like to take ownership
                // of this partner. Another reason to have the `msg: &Messages`
                // parameter be `msg: Messages`.
                self.partner_tx = Some(partner_tx.clone());
                self.file = match File::open(file_name) {
                    Ok(file) => {
                        log::info!("open: file_name={}", file_name);
                        Some(file)
                    }
                    Err(why) => {
                        if let Some(partner_tx) = &self.partner_tx {
                            if let Err(why_send) = partner_tx.send(Messages::Done { result: false })
                            {
                                log::info!("open: couldn't send err: '{why}' to partner_tx because of err: '{why_send}'");
                            }
                        } else {
                            log::info!(
                                "open: couldn't send err: '{why}' because partner_tx is None"
                            );
                        }
                        return (Handled::Yes, None);
                    }
                };

                self.buffers = Vec::with_capacity(*buf_count);
                log::info!(
                    "open: buf_count={} buf_capacity={}",
                    buf_count,
                    buf_capacity
                );
                for buf_idx in 0..*buf_count {
                    let mut buf = Box::new(Vec::<u8>::with_capacity(*buf_capacity));
                    let first_value = buf_idx * *buf_capacity;
                    for i in 0..*buf_capacity {
                        buf.push(((first_value + i) % 256) as u8);
                        //log::info!("open: buf[{i}={} &buf[{i}]={:p}", buf[i], &buf[i])
                    }
                    log::info!("open: &buf[0]: {:p} {:0X?}", &buf[0], buf);
                    self.buffers.push(buf);
                }

                log::info!(
                    "open: Handled Messages::Open transition to '{}'",
                    e.get_state_name(IDX_WAIT_FOR_START)
                );
                (Handled::Yes, Some(IDX_WAIT_FOR_START))
            }
            _ => (Handled::No, None),
        }
    }

    fn wait_for_start(&mut self, e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
        match msg {
            Messages::Start => {
                e.send(Messages::Read).expect("SNH");
                log::info!(
                    "wait_for_start: Got Start, tranistion to '{}'",
                    e.get_state_name(IDX_READ)
                );
                (Handled::Yes, Some(IDX_READ))
            }
            _ => (Handled::No, None),
        }
    }

    fn read(&mut self, e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
        match msg {
            Messages::Read => {
                if let Some(buf) = self.buffers.pop() {
                    let mut buf = *buf;
                    log::info!(
                        "read: before read len={} &buf[0]: {:p} {:0X?}",
                        buf.len(),
                        &buf[0],
                        buf
                    );
                    buf.truncate(buf.capacity());
                    if let Some(f) = &mut self.file {
                        let count = f.read(&mut buf).expect("ATM SNH");
                        // Truncate to count otherwise the len will be capacity!
                        buf.truncate(count);
                        log::info!(
                            "read:  after read len={} &buf[0]: {:p} {:0X?}",
                            buf.len(),
                            &buf[0],
                            buf
                        );
                        if count < buf.capacity() {
                            log::info!("read: EOF");
                            if let Some(partner_tx) = &self.partner_tx {
                                log::info!("read: EOF Send {} bytes to partner", buf.len());
                                partner_tx
                                    .send(Messages::Done { result: true })
                                    .expect("SNH");
                            } else {
                                panic!(
                                    "read: EOF SNH sending Messages::Done self.partner_tx is None"
                                );
                            }

                            // Read all data back to open
                            log::info!(
                                "read: EOF transitition to '{}'",
                                e.get_state_name(IDX_OPEN)
                            );
                            (Handled::Yes, Some(IDX_OPEN))
                        } else {
                            if let Some(partner_tx) = &self.partner_tx {
                                log::info!("read: Send Data {} to partner", buf.len());
                                partner_tx.send(Messages::Data { buf }).expect("SNH");
                            } else {
                                panic!("read: SNH self.partner_tx is None");
                            }

                            // Send message to ourselves so we continue processing
                            e.send(Messages::Read).expect("SNH");
                            (Handled::Yes, None)
                        }
                    } else {
                        // No file so we're done, back to IDX_OPEN
                        log::info!(
                            "read: SNH, self.file is NONE, transition to '{}'",
                            e.get_state_name(IDX_OPEN)
                        );
                        (Handled::Yes, Some(IDX_OPEN))
                    }
                } else {
                    // There are no buffers, wait for an empty one
                    log::info!(
                        "read: no buffers, transition to '{}'",
                        e.get_state_name(IDX_WAIT_FOR_EMPTY)
                    );
                    (Handled::Yes, Some(IDX_WAIT_FOR_EMPTY))
                }
            }
            _ => {
                log::info!("read: unhandled {:0X?}", msg);
                (Handled::No, None)
            }
        }
    }

    fn wait_for_empty(&mut self, e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
        match msg {
            Messages::Empty { .. } => {
                // Would be "faster" if we handled Empty here but DRY so let base do it.
                e.send(Messages::Read).expect("SNH");
                (Handled::No, Some(IDX_READ))
            }
            //Messages::Read => {
            //    // SNH ???
            //    log::info!("wait_for_empty: Read received, defer");
            //    e.defer_send(msg.clone()).expect("SNH");
            //    (Handled::Yes, None)
            //}
            _ => (Handled::No, None),
        }
    }
}

fn main() {
    env_logger_init("info");
    log::info!("main:+");

    let (tx, rx) = channel::<Messages>();

    let mut efsp = FileStreamProducer::new().expect("Error Fsp::new");
    log::info!("new: fsp={:?}", efsp.get_sm());

    // get tx for efsp
    let efsp_tx = efsp.clone_sender();

    // Spawn efsp in another thread
    let efsp_thread = thread::spawn(move || {
        log::info!("efsp thread:+");
        while let Ok(msg) = efsp.recv() {
            log::info!("efsp thread:  recv msg={:0X?}", msg);
            efsp.dispatcher(&msg);
            match msg {
                Messages::StopThread => {
                    log::info!("efsp thread: Stopping");
                    break;
                }
                _ => (),
            }
        }
        log::info!("efsp thread:-");
    });

    efsp_tx
        .send(Messages::Open {
            file_name: "hello.txt".to_owned(),
            buf_count: 2,
            buf_capacity: 3,
            partner_tx: tx,
        })
        .unwrap();

    efsp_tx.send(Messages::Start).unwrap();

    while let Ok(r) = rx.recv() {
        match r {
            Messages::Data { buf } => {
                log::info!("main: Data {} {:p} {:0X?}", buf.len(), &buf[0], buf);
                efsp_tx
                    .send(Messages::Empty { buf: Box::new(buf) })
                    .unwrap();
            }
            Messages::Done { result } => {
                log::info!("main: Done result={result}");
                break;
            }
            _ => log::info!("main: unexpected msg: {:?}", r),
        }
    }

    efsp_tx.send(Messages::StopThread).unwrap();
    efsp_thread.join().expect("Error efsp_thread");

    log::info!("main:-");
}
