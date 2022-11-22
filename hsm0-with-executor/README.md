![code-coverage](coverage/html/badges/flat.svg)

# Separate StateMachine from the code that executes it

In this model there is an `Executor` and the `StateMachine`. The user
creates the state machine and builds by giving it to `Executor::new()`
then add all of the states one at a time using `with_state()` and
finally `build()` passing the initial state.

## Run

Debug:
```
$ cargo run
   Compiling libc v0.2.126
   Compiling atty v0.2.14
   Compiling env_logger v0.9.3
   Compiling custom-logger v0.1.0 (https://github.com/winksaville/custom-logger#4d828a35)
   Compiling hsm0-with-executor v0.7.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor)
    Finished dev [unoptimized + debuginfo] target(s) in 1.27s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/hsm0-with-executor`
main:+
main:  &sme=0x7fff45dbe968
state1:+ &self=0x7fff45dbe970
state1:-
state1:+ &self=0x7fff45dbe970
state2:-
state1:+ &self=0x7fff45dbe970
state1:-
state1:+ &self=0x7fff45dbe970
state2:-
main:-
```

Release:
```
$ cargo run --release
    Finished release [optimized] target(s) in 0.02s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/release/hsm0-executor`
main
```

## Test

```
$ cargo test
   Compiling hsm0-executor v0.6.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor)
    Finished test [unoptimized + debuginfo] target(s) in 0.46s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_executor-66fb8aaa1aba2c1e)

running 17 tests
test test::test_1s_cycle ... ok
test test::test_2s_one_self_cycle ... ok
test test::test_2s_cycle ... ok
test test::test_3s_one_cycle ... ok
test test::test_5s_long_cycle ... ok
test test::test_leaf_transitions_between_trees ... ok
test test::test_sm_1h_2s_not_handled_no_enter_no_exit ... ok
test test::test_sm_1s_enter_no_exit ... ok
test test::test_leaf_transitions_in_a_tree ... ok
test test::test_sm_1s_get_names ... ok
test test::test_sm_1s_no_enter_no_exit ... ok
test test::test_sm_2s_get_names ... ok
test test::test_sm_2s_no_enter_no_exit ... ok
test test::test_sm_2s_invalid_transition - should panic ... ok
test test::test_sm_invalid_initial_state - should panic ... ok
test test::test_sm_out_of_bounds_initial_transition - should panic ... ok
test test::test_sm_out_of_bounds_invalid_transition - should panic ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_executor-9723d24c8ebea6da)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests hsm0-executor

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Code coverage

<!--
Using html preview.github.io. Or how display/render a URL/link from a github readme.md file.

The `Html Results` link has to be "absolute" and thus will always render the "main" coverage information
not the "current branch". You can manually edit the URL change "main" to the branch name and you'll then
the what you'd probably want to see.

To see it on your clone, prior to committing and merging to main,
execute `google-chrome coverage/html/index.html` at the command line:

   wink@3900x 22-10-23T20:01:27.503Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor (rework-3-coverage-100%)
   $ google-chrome coverage/html/index.html &
   [4] 97726
   wink@3900x 22-10-23T20:01:29.432Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor (rework-3-coverage-100%)
-->

[Html Results](https://htmlpreview.github.io/?https://github.com/winksaville/proc-macro-hsm1/blob/main/hsm0-executor/coverage/html/index.html)


```
$ cargo xt gen-cov
   Compiling xtask v0.1.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/xtask)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/xtask gen-cov`
Run cargo clean []
Create profraw data at /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor/coverage
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
   Compiling hsm0-executor v0.6.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor)
    Finished test [unoptimized + debuginfo] target(s) in 3.64s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_executor-4748ba702caea268)

running 17 tests
test test::test_2s_cycle ... ok
test test::test_2s_one_self_cycle ... ok
test test::test_5s_long_cycle ... ok
test test::test_leaf_transitions_between_trees ... ok
test test::test_sm_1s_no_enter_no_exit ... ok
test test::test_leaf_transitions_in_a_tree ... ok
test test::test_3s_one_cycle ... ok
test test::test_sm_1s_get_names ... ok
test test::test_sm_invalid_initial_state - should panic ... ok
test test::test_1s_cycle ... ok
test test::test_sm_1h_2s_not_handled_no_enter_no_exit ... ok
test test::test_sm_2s_no_enter_no_exit ... ok
test test::test_sm_out_of_bounds_invalid_transition - should panic ... ok
test test::test_sm_2s_invalid_transition - should panic ... ok
test test::test_sm_1s_enter_no_exit ... ok
test test::test_sm_2s_get_names ... ok
test test::test_sm_out_of_bounds_initial_transition - should panic ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Create /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor/coverage/html
Create /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor/coverage/tests.lcov
Create /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor/coverage/tests.covdir.json
```

# Examples

```
$ cargo run --example hsm-1h-3s
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/examples/hsm-1h-3s`
main
[2022-10-23T20:53:01.030475295Z INFO  hsm_1h_3s  163  1] main:+
[2022-10-23T20:53:01.030506083Z INFO  hsm_1h_3s  167  1] main:-
wink@3900x 22-10-23T20:53:01.032Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor (make-hsm-1h-3s-and-hsm-2h-2s-hsm0-executor-examples)
$ cargo run --example hsm-2h-2s
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/examples/hsm-2h-2s`
main
[2022-10-23T20:53:08.205121365Z INFO  hsm_2h_2s  197  1] main:+
[2022-10-23T20:53:08.205155509Z INFO  hsm_2h_2s  201  1] main:-
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

