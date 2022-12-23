# Hierarchical State Machine (HSM) proc macro

Define a `proc_macro` to make it easier to create HSM's.

# Transition Rule from Source state to Destination state

* Create exit path from source to its top most parent
* Create entry path from destination to its top most parent
* Find the common parent, three positibilites:
  * The paths are the same which means source and destination are the same.
    * This means this is a transition to "self" and exit enter should be
    executed on this one state.
  * The paths differ then the common parent is lowest common parent.
    * First, exit should be executed starting at source and
    proceeding upwards to but not including the common parent.
    * Next, enter should be executed starting below the common parent
    down to the destination.


# Examples

Two examples; MyFsm is the simplest FSM with just one state.
MyHsm is the simplest HSM with two states, initial with base
as its parent.

```ignore // Ignore because clippy warnings of neeless main
use proc_macro_hsm1::{handled, hsm1, hsm1_state, not_handled, StateResult};

// These two use's needed as hsm1 is dependent upon them.
// How can hsm1 proc_macro signify the dependency?

hsm1!(
    struct MyFsm {
        initial_counter: u64,
    }

    #[hsm1_state]
    fn initial(&mut self) -> StateResult!() {
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
    fn base(&mut self) -> StateResult!() {
        // Mutate the state
        self.base_counter += 1;

        // Return the desired StateResult
        handled!()
    }

    #[hsm1_state(base)]
    fn initial(&mut self) -> StateResult!() {
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

## Build and run

There is [`proc_macro_hsm1/src/main.rs`](proc_macro_hsm1/src/main.rs) which implements a trivial Finite
State Machine (FSM) that can be run. Also, see [`proc_macro_hsm1/tests`](proc_macro_hsm1/tests)
for some other examples. Eventually there will be other sub-packages as
additional examples.

I find it simplest to run and test from the sub-package, so initiall I `cd proc_macro_hsm1`:
```
wink@3900x 22-12-23T23:06:06.308Z:~/prgs/rust/myrepos/exper_hsm/proc_macro_hsm1 (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/proc_macro_hsm1`
Fsm::initial: Add 15 data=15
main: fsm initial_counter=1 data=15
```

## Tests
```
wink@3900x 22-12-23T23:07:50.510Z:~/prgs/rust/myrepos/exper_hsm/proc_macro_hsm1 (main)
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/deps/proc_macro_hsm1-02d34de914c0123e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/deps/proc_macro_hsm1-efd5f5e752da32ed)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/low-level-tests.rs (/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/deps/low_level_tests-6f1d572a9fca2e39)

running 12 tests
test test_initial_and_do_work_and_done_all_with_enter_exit ... ok
test test_dispatch ... ok
test test_initial_and_done_both_with_exit ... ok
test test_initial_and_done_no_enter_exit ... ok
test test_initial_and_done_both_with_enter ... ok
test test_initialization ... ok
test test_initial_then_tree_of_parent_and_do_work_then_done ... ok
test test_parent_with_child_initial ... ok
test test_parent_with_enter_exit_and_one_child_initial ... ok
test test_one_tree_plus_done_as_separate ... ok
test test_transition_to ... ok
test test_tree_parent_with_children_initial_do_work_done_all_with_enter_exit ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/multiple-state-machines.rs (/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/deps/multiple_state_machines-a4cb46daf151531a)

running 1 test
test multiple_state_machines ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/mut-msg-tests.rs (/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/deps/mut_msg_tests-4a6e5cc15887446e)

running 1 test
test mut_msg_tests ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/transitions.rs (/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/deps/transitions-0e424ce703e9c398)

running 3 tests
test test_transitions_between_two_unrelated_states ... ok
test test_transitions_with_one_state ... ok
test test_transitions_between_leafs_of_trees ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests proc_macro_hsm1

running 1 test
test src/lib.rs - hsm1 (line 293) ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

# Benchmarks

Running `cargo criterion` runs all the benchmarks as specified in
the `[[bench]]` sections in Cargo.toml, i.e. `bench-dispatch` and `bench-dispatch-iai`:
> Note: Execute `cargo install cargo-criterion` to install the criterion command.
> You'll also need to install `valgrind` if you enable `bench-dispatch-iai`.
```
wink@3900x 22-12-23T23:38:50.087Z:~/prgs/rust/myrepos/exper_hsm/proc_macro_hsm1 (main)
$ cargo criterion 
    Finished bench [optimized] target(s) in 0.05s
bench_minimal_fsm_returning_handled                                                                              
                        time:   [3.6606 ns 3.6617 ns 3.6633 ns]
                        change: [-1.0130% -0.9252% -0.8459%] (p = 0.00 < 0.05)
                        Change within noise threshold.

bench_minimal_fsm_returning_not_handled                                                                              
                        time:   [3.1419 ns 3.1446 ns 3.1475 ns]
                        change: [-0.7926% -0.6844% -0.5775%] (p = 0.00 < 0.05)
                        Change within noise threshold.

bench_minimal_fsm_returning_transition_to_self                                                                              
                        time:   [6.9123 ns 6.9145 ns 6.9169 ns]
                        change: [-3.6846% -3.5623% -3.4467%] (p = 0.00 < 0.05)
                        Performance has improved.

bench_minimal_fsm_returning_transition_to_self_with_enter                                                                              
                        time:   [7.9939 ns 8.0042 ns 8.0153 ns]
                        change: [+1.0017% +1.1176% +1.2492%] (p = 0.00 < 0.05)
                        Performance has regressed.

bench_minimal_fsm_returning_transition_to_self_with_exit                                                                              
                        time:   [7.9073 ns 7.9140 ns 7.9224 ns]
                        change: [-0.1358% -0.0389% +0.0619%] (p = 0.45 > 0.05)
                        No change in performance detected.

bench_minimal_fsm_returning_transition_to_self_with_ee                                                                              
                        time:   [8.3066 ns 8.3126 ns 8.3193 ns]
                        change: [+1.1532% +1.2500% +1.3437%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

You can also run one benchmark at a time:
```
wink@3900x 22-12-23T23:42:52.833Z:~/prgs/rust/myrepos/exper_hsm/proc_macro_hsm1 (main)
$ cargo criterion bench_minimal_fsm_returning_not_handled
    Finished bench [optimized] target(s) in 0.05s
bench_minimal_fsm_returning_not_handled                                                                              
                        time:   [3.2545 ns 3.2567 ns 3.2591 ns]
                        change: [+1.8471% +1.9575% +2.0623%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

