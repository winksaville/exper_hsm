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
wink@3900x 22-10-19T20:20:33.993Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor
$ cargo tarpaulin --lib
Oct 19 13:20:36.022  INFO cargo_tarpaulin::config: Creating config
Oct 19 13:20:36.063  INFO cargo_tarpaulin: Running Tarpaulin
Oct 19 13:20:36.063  INFO cargo_tarpaulin: Building project
Oct 19 13:20:36.063  INFO cargo_tarpaulin::cargo: Cleaning project
   Compiling memchr v2.5.0
   Compiling libc v0.2.126
   Compiling log v0.4.17
   Compiling regex-syntax v0.6.27
   Compiling cfg-if v1.0.0
   Compiling termcolor v1.1.3
   Compiling humantime v2.1.0
   Compiling aho-corasick v0.7.18
   Compiling atty v0.2.14
   Compiling regex v1.6.0
   Compiling env_logger v0.9.1
   Compiling custom-logger v0.1.0 (https://github.com/winksaville/custom-logger#4d828a35)
   Compiling hsm0-executor v0.2.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor)
    Finished test [unoptimized + debuginfo] target(s) in 2.21s
Oct 19 13:20:38.398  INFO cargo_tarpaulin::process_handling::linux: Launching test
Oct 19 13:20:38.398  INFO cargo_tarpaulin::process_handling: running /home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_executor-7f4fd4adf08c77fc

running 5 tests
test test::test_leaf_transitions_in_a_tree ... ok
test test::test_sm_1h_2s_not_handled_no_enter_no_exit ... ok
test test::test_sm_1s_enter_no_exit ... ok
test test::test_sm_1s_no_enter_no_exit ... ok
test test::test_sm_2s_no_enter_no_exit ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Oct 19 13:20:38.617  INFO cargo_tarpaulin::report: Coverage Results:
|| Tested/Total Lines:
|| src/lib.rs: 57/73 +0.00%
||
78.08% coverage, 57/73 lines covered, +0% change in coverage
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

