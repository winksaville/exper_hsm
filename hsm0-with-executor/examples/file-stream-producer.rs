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
    file: Option<File>,

    // Need "zero-copy" buffering, i.e. we need
    // to move buffers when using send!
    // For the moment we'll clone them and keep track
    // what what is empty in a stack, (empty_buffers)
    buffers: Vec<Box<Vec<u8>>>,
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
            Messages::Empty { buf } => {
                println!("buf: {:p} *buf: ", buf);
                println!("*buf: {:p}", *buf);
                let x = buf.clone();
                println!("x: {x:p}");
                println!("x.as_ptr: {:p}", x.as_ptr());
                self.buffers.push(x);
                println!(
                    "Messages::Empty: {} {:p}",
                    self.buffers.len() + 1, &self.buffers.last(),
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
                for _ in 0..*buf_count {
                    let mut buf = Box::new(Vec::<u8>::with_capacity(*buf_capacity));
                    for i in 0..*buf_capacity {
                        buf.push((i % 256) as u8);
                        println!("buf[{i}={} &buf[{i}]={:p}", buf[i], &buf[i])
                    }
                    println!("open: &buf: {:p} buf.as_ref(): {:p} buf.as_ptr(): {:p}", &buf, buf.as_ref(), buf.as_ptr(), );
                    self.buffers.push(buf);
                    println!("open: empty_buffers.push({}) {:p} {:p}", self.buffers.len()-1, &self.buffers[self.buffers.len()-1], &self.buffers.last().as_ref());
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
