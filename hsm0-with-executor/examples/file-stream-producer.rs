use std::{
    cell::RefCell,
    fs::File,
    sync::mpsc::{channel, Receiver, Sender},
};

use custom_logger::env_logger_init;

use hsm0_with_executor::{DynError, Executor, Handled, StateInfo, StateResult};

#[derive(Debug)]
pub enum Messages {
    Open {
        // Name of file open
        file_name: String,
        buf_count: usize,
        buf_capacity: usize,
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
        buf_idx: usize,
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
    file: Option<File>,

    // Need "zero-copy" buffering, i.e. we need
    // to move buffers when using send!
    // For the moment we'll clone them and keep track
    // what what is empty in a stack, (empty_buffers)
    buffers: Vec<Vec<u8>>,
    empty_buffers: Vec<usize>,
}

const MAX_STATES: usize = 2;
const IDX_BASE: usize = 0;
const IDX_OPEN: usize = 1;

impl FileStreamProducer {
    fn new() -> Result<Executor<FileStreamProducer, Messages>, DynError> {
        let (tx, rx) = channel::<Messages>();

        let fsp = RefCell::new(Self {
            tx,
            rx,
            file: None,
            buffers: Vec::new(),
            empty_buffers: Vec::new(),
        });

        let sme = Executor::new(fsp, MAX_STATES)
            // 0: IDX_BASE
            .state(StateInfo::new("base", Self::base))
            .state(StateInfo::new("open", Self::open).parent_idx(IDX_BASE))
            .build(IDX_OPEN);

        sme
    }

    // This is the parent of all states and handles all
    // as best as it can for now :)
    fn base(&mut self, e: &Executor<Self, Messages>, msg: &Messages) -> StateResult {
        match msg {
            Messages::Open { .. } => println!(
                "Ignoring Messages::Open in state {}",
                e.get_state_name(IDX_BASE)
            ),
            Messages::Start => println!(
                "Messages::Start not supported in {}",
                e.get_state_name(IDX_BASE)
            ),
            Messages::Read => println!(
                "Messages::Read not supported in {}",
                e.get_state_name(IDX_BASE)
            ),
            Messages::Data { .. } => panic!(
                "Messages::Data not supported in {}",
                e.get_state_name(IDX_BASE)
            ),
            Messages::Empty { buf_idx } => {
                assert!(*buf_idx < self.buffers.len());
                self.empty_buffers.push(*buf_idx);
                println!(
                    "Messages::Empty: {} {:?}",
                    self.empty_buffers.len(),
                    self.empty_buffers
                );
            }
            Messages::Done { result: _ } => panic!(
                "Messages:Done not supported in {}",
                e.get_state_name(IDX_BASE)
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
            } => {
                self.file = match File::open(file_name) {
                    Ok(file) => {
                        println!("open: file_name={}", file_name);
                        Some(file)
                    }
                    Err(why) => {
                        //tx.send(Messages::Done { result: Err(Box::new(why))});
                        panic!("error: {why}")
                    }
                };

                self.buffers = Vec::with_capacity(*buf_count);
                println!("open: self.buffers.capacity {}", self.buffers.capacity());
                self.empty_buffers = Vec::with_capacity(*buf_count);
                println!(
                    "open: self.empty_buffers.capacity {}",
                    self.empty_buffers.capacity()
                );
                for buf_idx in 0..*buf_count {
                    println!("open: empty_buffers.push({})", buf_idx);
                    self.buffers.push(Vec::with_capacity(*buf_capacity));
                    self.empty_buffers.push(buf_idx);
                }

                println!(
                    "Handled Messages::Open in state {}",
                    e.get_state_name(IDX_BASE)
                );
                (Handled::Yes, None)
            }
            Messages::Empty { .. }
            | Messages::Start { .. }
            | Messages::Read { .. }
            | Messages::Data { .. }
            | Messages::Done { .. } => (Handled::No, None),
        }
    }
}

fn main() {
    env_logger_init("info");
    log::info!("main:+");

    let mut efsp = FileStreamProducer::new().expect("Error Fsp::new");
    println!("new: fsp={:?}", efsp.get_sm());

    efsp.dispatcher(&Messages::Open {
        file_name: "hello.txt".to_owned(),
        buf_count: 2,
        buf_capacity: 3,
    });

    log::info!("main:-");
}
