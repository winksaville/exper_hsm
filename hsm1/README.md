# Hierarchical State Machine (HSM) proc macro

Define a `proc_macro` to make it easier to create HSM's.

# Examples

Two examples; MyFsm is the simplest FSM with just one state.
MyHsm is the simplest HSM with two states, initial with base
as its parent.

```ignore // Ignore because clippy warnings of neeless main
use proc_macro_hsm1::{handled, hsm1, hsm1_state, not_handled};

// These two use's needed as hsm1 is dependent upon them.
// How can hsm1 proc_macro signify the dependency?
use std::collections::VecDeque;
use state_result::*;

hsm1!(
    struct MyFsm {
        initial_counter: u64,
    }

    #[hsm1_state]
    fn initial(&mut self) -> StateResult {
        // Mutate the state
        self.initial_counter += 1;

        // Let the parent state handle all invocations
        handled!()
    }
);

hsm1!(
    struct MyHsm {
        base_counter: u64,
        initial_counter: u64,
    }

    #[hsm1_state]
    fn base(&mut self) -> StateResult {
        // Mutate the state
        self.base_counter += 1;

        // Return the desired StateResult
        handled!()
    }

    #[hsm1_state(base)]
    fn initial(&mut self) -> StateResult {
        // Mutate the state
        self.initial_counter += 1;

        // Let the parent state handle all invocations
        not_handled!()
    }
);

fn main() {
    let mut fsm = MyFsm::new();

    fsm.dispatch();
    println!( "fsm: fsm intial_counter={}", fsm.initial_counter);
    assert_eq!(fsm.initial_counter, 1);

    let mut hsm = MyHsm::new();

    hsm.dispatch();
    println!(
        "hsm: hsm base_counter={} intial_counter={}",
        hsm.base_counter, hsm.initial_counter
    );
    assert_eq!(hsm.base_counter, 1);
    assert_eq!(hsm.initial_counter, 1);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

