# Hierarchical State Machine (HSM) proc macro

Define [`hsm1!`](hsm1/README.md) a `proc_macro` to make it easier to create HSM's.

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

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

