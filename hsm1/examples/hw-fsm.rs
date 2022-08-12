use hsm1::{handled, hsm1, hsm1_initial_state, StateResult};

struct NoMessages;

hsm1!(
    struct Fsm {
        a_field: i32,
    }

    #[hsm1_initial_state]
    fn initial(&mut self, msg: &NoMessages) -> StateResult!() {
        println!("Hello World, Fsm::initial: a_field={}", self.a_field);

        handled!()
    }
);

fn main() {
    let mut fsm = Fsm::new();
    let msg = NoMessages;
    fsm.dispatch(&msg);
}
