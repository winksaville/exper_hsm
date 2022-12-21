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

There is [`hsm1/src/main.rs`](hsm1/src/main.rs) which implements a trivial Finite
State Machine (FSM) that can be run. Also, see [`hsm1/tests`](hsm1/tests)
for some other examples. Eventually there will be other sub-packages as
additional examples.

I find it simplest to run and test from the sub-package, so initiall I `cd hsm1`:
```
wink@3900x 22-07-26T19:17:05.968Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm1 (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/hsm1`
Fsm::initial: Add 15 data=15
main: fsm initial_counter=1 data=15
```

## Tests
```
wink@3900x 22-07-26T19:18:49.568Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm1 (main)
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm1-9d926193173f76cc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm1-d12cec030b367ee9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/low-level-tests.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/low_level_tests-702b20a4269af6d5)

running 12 tests
test test_dispatch ... ok
test test_initial_then_tree_of_parent_and_do_work_then_done ... ok
test test_initial_and_do_work_and_done_all_with_enter_exit ... ok
test test_initial_and_done_both_with_exit ... ok
test test_initialization ... ok
test test_initial_and_done_both_with_enter ... ok
test test_initial_and_done_no_enter_exit ... ok
test test_one_tree_plus_done_as_separate ... ok
test test_parent_with_child_initial ... ok
test test_parent_with_enter_exit_and_one_child_initial ... ok
test test_transition_to ... ok
test test_tree_parent_with_children_initial_do_work_done_all_with_enter_exit ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/multiple-state-machines.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/multiple_state_machines-983102007e6b3ae6)

running 1 test
test multiple_state_machines ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/mut-msg-tests.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/mut_msg_tests-06ee428151409df1)

running 1 test
test mut_msg_tests ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests hsm1

running 1 test
test src/lib.rs - hsm1 (line 284) ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

# Benchmarks

Running `cargo criterion` runs all the benchmarks as specified in
the `[[bench]]` sections in Cargo.toml, i.e. `bench-dispatch` and `bench-dispatch-iai`:
> Note: Execute `cargo install cargo-criterion` to install the criterion command.
> You'll also need to install `valgrind`.
```
wink@3900x 22-07-26T22:53:56.139Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm1 (main)
$ time cargo criterion
    Finished bench [optimized] target(s) in 0.04s
Gnuplot not found, using plotters backend
bench_minimal_fsm_returning_handled                                                                              
                        time:   [3.1206 ns 3.1212 ns 3.1219 ns]

bench_minimal_fsm_returning_not_handled                                                                              
                        time:   [3.3446 ns 3.3470 ns 3.3498 ns]

bench_minimal_fsm_returning_transition_to_self                                                                              
                        time:   [6.7229 ns 6.7247 ns 6.7266 ns]

bench_minimal_fsm_returning_transition_to_self_with_enter                                                                              
                        time:   [7.3337 ns 7.3357 ns 7.3377 ns]

bench_minimal_fsm_returning_transition_to_self_with_exit                                                                              
                        time:   [7.1287 ns 7.1323 ns 7.1366 ns]

bench_minimal_fsm_returning_transition_to_self_with_ee                                                                              
                        time:   [7.9757 ns 7.9783 ns 7.9809 ns]

bench_fsm_setup
  Instructions:                 936 (No change)
  L1 Accesses:                 1363 (No change)
  L2 Accesses:                    5 (No change)
  RAM Accesses:                  24 (No change)
  Estimated Cycles:            2228 (No change)

bench_minimal_fsm_returning_handled
  Instructions:                 992 (No change)
  L1 Accesses:                 1441 (No change)
  L2 Accesses:                    6 (No change)
  RAM Accesses:                  30 (No change)
  Estimated Cycles:            2521 (No change)

bench_minimal_fsm_returning_not_handled
  Instructions:                 992 (No change)
  L1 Accesses:                 1438 (No change)
  L2 Accesses:                    8 (No change)
  RAM Accesses:                  32 (No change)
  Estimated Cycles:            2598 (No change)

bench_minimal_fsm_returning_transition_to_self
  Instructions:                1054 (No change)
  L1 Accesses:                 1521 (No change)
  L2 Accesses:                    6 (No change)
  RAM Accesses:                  37 (No change)
  Estimated Cycles:            2846 (No change)

bench_minimal_fsm_returning_transition_to_self_with_enter
  Instructions:                1076 (No change)
  L1 Accesses:                 1557 (No change)
  L2 Accesses:                    7 (No change)
  RAM Accesses:                  40 (No change)
  Estimated Cycles:            2992 (No change)

bench_minimal_fsm_returning_transition_to_self_with_exit
  Instructions:                1076 (No change)
  L1 Accesses:                 1558 (No change)
  L2 Accesses:                    7 (No change)
  RAM Accesses:                  39 (No change)
  Estimated Cycles:            2958 (No change)

bench_minimal_fsm_returning_transition_to_self_with_ee
  Instructions:                1087 (No change)
  L1 Accesses:                 1576 (No change)
  L2 Accesses:                    9 (No change)
  RAM Accesses:                  37 (No change)
  Estimated Cycles:            2916 (No change)


real	1m11.157s
user	6m57.022s
sys	0m1.508s
```

You can also run one at a time, and which `bench-dispatch-iai` seems more useful.
It also runs in about 1 second as compared to 1 minute:
```
wink@3900x 22-07-26T23:07:17.486Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm1 (main)
$ time cargo criterion bench-dispatch-iai
    Finished bench [optimized] target(s) in 0.04s
Gnuplot not found, using plotters backend
bench_fsm_setup
  Instructions:                 936 (No change)
  L1 Accesses:                 1363 (No change)
  L2 Accesses:                    5 (No change)
  RAM Accesses:                  24 (No change)
  Estimated Cycles:            2228 (No change)

bench_minimal_fsm_returning_handled
  Instructions:                 992 (No change)
  L1 Accesses:                 1441 (No change)
  L2 Accesses:                    6 (No change)
  RAM Accesses:                  30 (No change)
  Estimated Cycles:            2521 (No change)

bench_minimal_fsm_returning_not_handled
  Instructions:                 992 (No change)
  L1 Accesses:                 1438 (No change)
  L2 Accesses:                    8 (No change)
  RAM Accesses:                  32 (No change)
  Estimated Cycles:            2598 (No change)

bench_minimal_fsm_returning_transition_to_self
  Instructions:                1054 (No change)
  L1 Accesses:                 1521 (No change)
  L2 Accesses:                    6 (No change)
  RAM Accesses:                  37 (No change)
  Estimated Cycles:            2846 (No change)

bench_minimal_fsm_returning_transition_to_self_with_enter
  Instructions:                1076 (No change)
  L1 Accesses:                 1557 (No change)
  L2 Accesses:                    7 (No change)
  RAM Accesses:                  40 (No change)
  Estimated Cycles:            2992 (No change)

bench_minimal_fsm_returning_transition_to_self_with_exit
  Instructions:                1076 (No change)
  L1 Accesses:                 1558 (No change)
  L2 Accesses:                    7 (No change)
  RAM Accesses:                  39 (No change)
  Estimated Cycles:            2958 (No change)

bench_minimal_fsm_returning_transition_to_self_with_ee
  Instructions:                1087 (No change)
  L1 Accesses:                 1576 (No change)
  L2 Accesses:                    9 (No change)
  RAM Accesses:                  37 (No change)
  Estimated Cycles:            2916 (No change)


real	0m1.405s
user	0m1.573s
sys	0m1.019s
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

