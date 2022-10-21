# Attempt to use an executor model

An Executor Model where there is a seperation between
the Users `StateMachine` and the code that executes the
state machine, `StateMachineExecutor`.

## Run

Debug:
```
$ cargo run
   Compiling hsm0-executor v0.2.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/hsm0-executor`
main
[2022-10-18T19:36:19.395050902Z INFO  hsm0_executor  163  1] main:+
[2022-10-18T19:36:19.395088333Z INFO  hsm0_executor  167  1] main:-
```

Release:
```
$ cargo run --release
   Compiling hsm0-executor v0.2.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor)
    Finished release [optimized] target(s) in 0.33s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/release/hsm0-executor`
main
```

## Test

```
wink@3900x 22-10-19T20:19:12.936Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_executor-7f4fd4adf08c77fc)

running 5 tests
test test::test_sm_1s_enter_no_exit ... ok
test test::test_leaf_transitions_in_a_tree ... ok
test test::test_sm_1h_2s_not_handled_no_enter_no_exit ... ok
test test::test_sm_1s_no_enter_no_exit ... ok
test test::test_sm_2s_no_enter_no_exit ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_executor-266a9523d785e8d5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests hsm0-executor

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Code coverage

```

running 7 tests
test test::test_leaf_transitions_in_a_tree ... ok
test test::test_sm_1h_2s_not_handled_no_enter_no_exit ... ok
test test::test_sm_1s_enter_no_exit ... ok
test test::test_sm_1s_get_names ... ok
test test::test_sm_1s_no_enter_no_exit ... ok
test test::test_sm_2s_get_names ... ok
test test::test_sm_2s_no_enter_no_exit ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Oct 20 09:39:32.128  INFO cargo_tarpaulin::report: Coverage Results:
|| Uncovered Lines:
|| src/lib.rs: 80, 93, 95, 127, 132, 163, 167, 169, 194, 202
|| Tested/Total Lines:
|| src/lib.rs: 61/71 +2.35%
|| 
85.92% coverage, 61/71 lines covered, +2.3538491221300433% change in coverage```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

