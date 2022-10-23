![code-coverage](coverage/html/badges/flat.svg)

# Attempt to use an executor model

An Executor Model where there is a seperation between
the Users `StateMachine` and the code that executes the
state machine, `StateMachineExecutor`.

## Run

Debug:
```
$ cargo run
   Compiling memchr v2.5.0
   Compiling libc v0.2.126
   ..
   Compiling hsm0-executor v0.3.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor)
    Finished dev [unoptimized + debuginfo] target(s) in 2.31s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/hsm0-executor`
main
[2022-10-23T18:08:11.763590830Z INFO  hsm0_executor  163  1] main:+
[2022-10-23T18:08:11.763635314Z INFO  hsm0_executor  167  1] main:-
```

Release:
```
$ cargo run --release
   Compiling memchr v2.5.0
   Compiling libc v0.2.126
   ...
   Compiling hsm0-executor v0.3.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor)
    Finished release [optimized] target(s) in 2.92s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/release/hsm0-executor`
main
```

## Test

```
$ cargo test
   Compiling hsm0-executor v0.3.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor)
    Finished test [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_executor-8f4c28fe171892bf)

running 8 tests
test test::test_leaf_transitions_in_a_tree ... ok
test test::test_leaf_transitions_between_trees ... ok
test test::test_sm_1h_2s_not_handled_no_enter_no_exit ... ok
test test::test_sm_1s_enter_no_exit ... ok
test test::test_sm_1s_get_names ... ok
test test::test_sm_1s_no_enter_no_exit ... ok
test test::test_sm_2s_get_names ... ok
test test::test_sm_2s_no_enter_no_exit ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_executor-4025c8c27d8102c6)

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
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/xtask gen-cov`
Create profraw data at /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor/coverage
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_executor-8a71702aeb1d740f)

running 8 tests
test test::test_leaf_transitions_in_a_tree ... ok
test test::test_leaf_transitions_between_trees ... ok
test test::test_sm_1h_2s_not_handled_no_enter_no_exit ... ok
test test::test_sm_1s_get_names ... ok
test test::test_sm_1s_enter_no_exit ... ok
test test::test_sm_1s_no_enter_no_exit ... ok
test test::test_sm_2s_get_names ... ok
test test::test_sm_2s_no_enter_no_exit ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Create /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor/coverage/html
Create /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor/coverage/tests.lcov
Create /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor/coverage/tests.covdir.json
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

