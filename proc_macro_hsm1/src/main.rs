use proc_macro_hsm1::{handled, hsm1, hsm1_initial_state, StateResult};

#[derive(Debug)]
pub enum MessagesType {
    Add { value: i32 },
}

// FSM
hsm1!(
    struct Fsm {
        initial_counter: u64,
        data: i32,
    }

    #[hsm1_initial_state]
    fn initial(&mut self, msg: &MessagesType) -> StateResult!() {
        self.initial_counter += 1;

        match msg {
            MessagesType::Add { value } => {
                self.data += value;
                println!("Fsm::initial: Add {} data={}", value, self.data);

                handled!()
            }
        }
    }
);

fn main() {
    let mut fsm = Fsm::new();
    assert_eq!(fsm.initial_counter, 0);

    let msg = MessagesType::Add { value: 15 };
    fsm.dispatch(&msg);
    assert_eq!(fsm.data, 15);
    assert_eq!(fsm.initial_counter, 1);

    println!(
        "main: fsm initial_counter={} data={}",
        fsm.initial_counter, fsm.data
    );
}
