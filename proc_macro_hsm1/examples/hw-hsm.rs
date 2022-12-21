use proc_macro_hsm1::{handled, hsm1, hsm1_initial_state, hsm1_state, not_handled, StateResult};

struct NoMessages;

hsm1!(
    struct Hsm {
        a_field: i32,
    }

    #[hsm1_state]
    fn parent(&mut self, msg: &NoMessages) -> StateResult!() {
        println!("Hello World, Hsm::parent:  a_field={}", self.a_field);

        handled!()
    }

    #[hsm1_initial_state(parent)]
    fn initial(&mut self, msg: &NoMessages) -> StateResult!() {
        println!("Hello World, Hsm::initial: a_field={}", self.a_field);
        self.a_field += 1;

        not_handled!()
    }
);

fn main() {
    let mut hsm = Hsm::new();
    let msg = NoMessages;
    hsm.dispatch(&msg);
}
