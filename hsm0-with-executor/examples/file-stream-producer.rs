use std::{
    cell::RefCell,
    fs::File,
    io::Read,
    sync::mpsc::{channel, Receiver, Sender},
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
            Messages::Open { .. } => println!(
                "base: Ignoring Messages::Open in state {}",
                e.get_current_state_name()
            ),
            Messages::Start => println!(
                "base: Messages::Start not supported in {}",
                e.get_current_state_name()
            ),
            Messages::Read => println!(
                "base: Messages::Read not supported in {}",
                e.get_current_state_name()
            ),
            Messages::Data { .. } => panic!(
                "base: Messages::Data not supported in {}",
                e.get_current_state_name()
            ),
            Messages::Empty { buf } => {
                println!("base: Messages::Empty: buf: {:p} *buf: ", buf);
                println!("base: Messages::Empty: *buf: {:p}", *buf);
                let x = buf.clone();
                println!("base: Messages::Empty: x: {x:p}");
                println!("base: Messages::Empty: x.as_ptr: {:p}", x.as_ptr());
                self.buffers.push(x);
                println!(
                    "base: Messages::Empty: {} {:p}",
                    self.buffers.len() + 1,
                    &self.buffers.last(),
                );
            }
            Messages::Done { result: _ } => panic!(
                "base: Messages:Done not supported in {}",
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
                        println!("open: file_name={}", file_name);
                        Some(file)
                    }
                    Err(why) => {
                        if let Some(partner_tx) = &self.partner_tx {
                            if let Err(why_send) = partner_tx.send(Messages::Done { result: false })
                            {
                                println!("open: couldn't send err: '{why}' to partner_tx because of err: '{why_send}'");
                            }
                        } else {
                            println!("open: couldn't send err: '{why}' because partner_tx is None");
                        }
                        return (Handled::Yes, None);
                    }
                };

                self.buffers = Vec::with_capacity(*buf_count);
                println!("open: self.buffers.capacity {}", self.buffers.capacity());
                for _ in 0..*buf_count {
                    let mut buf = Box::new(Vec::<u8>::with_capacity(*buf_capacity));
                    for i in 0..*buf_capacity {
                        buf.push((i % 256) as u8);
                        println!("open: buf[{i}={} &buf[{i}]={:p}", buf[i], &buf[i])
                    }
                    println!(
                        "open: &buf: {:p} buf.as_ref(): {:p} buf.as_ptr(): {:p}",
                        &buf,
                        buf.as_ref(),
                        buf.as_ptr(),
                    );
                    self.buffers.push(buf);
                    println!(
                        "open: empty_buffers.push({}) {:p} {:p}",
                        self.buffers.len() - 1,
                        &self.buffers[self.buffers.len() - 1],
                        &self.buffers.last().as_ref()
                    );
                }

                println!(
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
                println!(
                    "wait_for_start: Got Start, tranistion to '{}'",
                    e.get_state_name(IDX_READ)
                );
                e.defer_send(Messages::Read).expect("SNH");
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
                    if let Some(f) = &mut self.file {
                        let count = f.read(&mut buf).expect("ATM SNH");
                        println!("read: count={}", count);
                        if count < buf.capacity() {
                            println!("read: EOF");
                            if let Some(partner_tx) = &self.partner_tx {
                                println!("read: EOF Send Data {} to partner", buf.len());
                                partner_tx
                                    .send(Messages::Done { result: true })
                                    .expect("SNH");
                            } else {
                                panic!(
                                    "read: EOF SNH sending Messages::Done self.partner_tx is None"
                                );
                            }

                            // Read all data back to open
                            println!("read: EOF transitition to '{}'", e.get_state_name(IDX_OPEN));
                            (Handled::Yes, Some(IDX_OPEN))
                        } else {
                            if let Some(partner_tx) = &self.partner_tx {
                                println!("read: Send Data {} to partner", buf.len());
                                partner_tx.send(Messages::Data { buf }).expect("SNH");
                            } else {
                                panic!("read: SNH self.partner_tx is None");
                            }

                            // Send message to ourselves so process our own deferred Message::Read??
                            e.defer_send(Messages::Read).expect("SNH");
                            (Handled::Yes, Some(IDX_READ))
                        }
                    } else {
                        // No file so we're done, back to IDX_OPEN
                        println!(
                            "read: error reading, transition to '{}'",
                            e.get_state_name(IDX_OPEN)
                        );
                        (Handled::Yes, Some(IDX_OPEN))
                    }
                } else {
                    // Defer start
                    e.defer_send(msg.clone()).expect("SNH");
                    (Handled::Yes, Some(IDX_WAIT_FOR_EMPTY))
                }
            }
            _ => {
                println!("read: unhandled {:?}", msg);
                (Handled::No, None)
            }
        }
    }

    fn wait_for_empty(&mut self, e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
        match msg {
            // Maybe the msg parameters should be "msg: Messages" and we'd consume it
            // or "msg: &mut Messages" then we could "take" it??
            Messages::Empty { buf } => {
                println!(
                    "wait_for_empty: Empty received, transition to '{}'",
                    e.get_state_name(IDX_READ)
                );

                self.buffers.push(buf.clone()); // What is this cloning

                (Handled::Yes, Some(IDX_READ))
            }
            Messages::Read => {
                println!("wait_for_empty: Read received, defer");
                e.defer_send(msg.clone()).expect("SNH");
                (Handled::Yes, None)
            }
            _ => (Handled::No, None),
        }
    }
}

fn main() {
    env_logger_init("info");
    log::info!("main:+");

    let (tx, rx) = channel::<Messages>();

    let mut efsp = FileStreamProducer::new().expect("Error Fsp::new");
    println!("new: fsp={:?}", efsp.get_sm());

    efsp.dispatcher(&Messages::Open {
        file_name: "hello.txt".to_owned(),
        buf_count: 2,
        buf_capacity: 3,
        partner_tx: tx,
    });

    efsp.dispatcher(&Messages::Start);

    while let Ok(r) = rx.recv() {
        match r {
            Messages::Data { buf } => {
                println!("main: Data {} {:0X?}", buf.len(), buf.as_ptr());
                efsp.dispatcher(&Messages::Empty { buf: Box::new(buf) });
            }
            Messages::Done { result } => {
                println!("main: Done result={result}");
                break;
            }
            _ => println!("main: unexpected msg: {:?}", r),
        }
    }
    log::info!("main:-");
}
