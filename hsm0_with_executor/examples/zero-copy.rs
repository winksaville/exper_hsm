/// Shows passing msg as an owned parameter in process_msg2 allows it to be
/// passed through the channel with the Vec<u8> being NOT moving. But when you
/// use process_msg and pass msg as a borrow the cloning does create a copy
/// of the data and we see it in a now location, i.e. it's address changes.
///
/// $ cargo run --example zero-copy
///     Finished dev [unoptimized + debuginfo] target(s) in 0.03s
///      Running `target/debug/examples/zero-copy`
/// main:         msg=0x5613f68b3310 [1, 2, 3]
/// partner: recv_msg=0x5613f68b3310 [1, 2, 3]
/// process_msg: 6
/// partner: sum=6
/// main:    recv_msg=0x7f2b84000e50 [1, 2, 3]
/// partner: recv_msg=0x7f2b84000e50 [1, 2, 3]
/// process_msg: 6
/// partner: sum=6
/// main:    recv_msg=0x5613f68b3310 [1, 2, 3]
/// partner: recv_msg=0x5613f68b3310 [1, 2, 3]
/// process_msg2: 6
/// partner: sum=6
/// main:    recv_msg=0x5613f68b3310 [1, 2, 3]
/// partner: recv_msg=0x5613f68b3310 [1, 2, 3]
/// process_msg2: 6
/// partner: sum=6
/// main:    recv_msg=0x5613f68b3310 [1, 2, 3]
/// partner: recv_msg=0x5613f68b3310 [1, 2, 3]
/// process_msg: 6
/// partner: sum=6
/// main:    recv_msg=0x7f2b84000e50 [1, 2, 3]
/// partner: recv_msg=0x7f2b84000e50 [1, 2, 3]
/// process_msg2: 6
/// partner: sum=6
/// main:    recv_msg=0x7f2b84000e50 [1, 2, 3]
/// partner: recv_msg=0x7f2b84000e50 [1, 2, 3]
/// process_msg: 6
/// partner: sum=6
/// main:    recv_msg=0x5613f68b3310 [1, 2, 3]
/// partner: recv_msg=0x5613f68b3310 [1, 2, 3]
/// process_msg: 6
/// partner: sum=6
/// main:    recv_msg=0x7f2b84000e50 [1, 2, 3]
/// partner: recv_msg=0x7f2b84000e50 [1, 2, 3]
/// process_msg2: 6
/// partner: sum=6
/// main:    recv_msg=0x7f2b84000e50 [1, 2, 3]
/// partner: recv_msg=0x7f2b84000e50 [1, 2, 3]
/// process_msg2: 6
/// partner: sum=6
/// main:    recv_msg=0x7f2b84000e50 [1, 2, 3]
/// partner: error receiving recv_msg why='receiving on a closed channel'
use std::{
    fmt::Display,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use rand::random;

#[derive(Clone)]
struct Message {
    v: Vec<u8>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p} {:0X?}", &self.v[0], self.v)
    }
}

#[allow(unused)]
fn process_msg(tx: &Sender<Message>, msg: &Message) -> i32 {
    let mut sum = 0i32;
    for v in msg.v.iter() {
        sum += *v as i32;
    }
    tx.send(msg.clone()).expect("darn");

    println!("process_msg: {}", sum);
    sum
}

#[allow(unused)]
fn process_msg2(tx: &Sender<Message>, msg: Message) -> i32 {
    #[allow(unused)]
    let mut sum = 0i32;
    for v in msg.v.iter() {
        sum += *v as i32;
    }
    tx.send(msg).expect("darn");

    println!("process_msg2: {}", sum);
    sum
}

fn partner(tx: Sender<Message>, rx: Receiver<Message>) {
    thread::spawn(move || loop {
        let recv_msg = match rx.recv() {
            Ok(m) => m,
            Err(why) => {
                println!("partner: error receiving recv_msg why='{}'", why);
                break;
            }
        };
        println!("partner: recv_msg={}", recv_msg);
        let sum = if random::<bool>() {
            process_msg(&tx, &recv_msg)
        } else {
            process_msg2(&tx, recv_msg)
        };
        println!("partner: sum={}", sum);
    });
}

fn main() {
    let (tx, partner_rx) = channel::<Message>();
    let (partner_tx, rx) = channel::<Message>();

    partner(partner_tx, partner_rx);

    let mut msg = Message { v: vec![1, 2, 3] };
    println!("main:         msg={}", msg);
    for _ in 0..10 {
        tx.send(msg).expect("main: error sending msg");
        let recv_msg = rx.recv().expect("main: error receiving recv_msg");
        println!("main:    recv_msg={}", &recv_msg);
        msg = recv_msg;
    }
}
