![code-coverage](coverage/html/badges/flat.svg)

# Separate StateMachine from the code that executes it

In this model there is an `Executor` and the `StateMachine`. The user
creates the state machine and builds by giving it to `Executor::new()`
then add all of the states one at a time using `with_state()` and
finally `build()` passing the initial state.

## Run

Debug:
```
wink@3900x 22-11-23T00:10:56.777Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor (main)
$ cargo run
   Compiling hsm0-with-executor v0.8.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.55s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/hsm0-with-executor`
main:+
main:  &sme=0x7ffc1c43ad88
state1:+ &self=0x7ffc1c43ad90
state1:-
state1:+ &self=0x7ffc1c43ad90
state2:-
state1:+ &self=0x7ffc1c43ad90
state1:-
state1:+ &self=0x7ffc1c43ad90
state2:-
main:-
```

Release:
```
wink@3900x 22-11-23T00:11:30.981Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.02s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/release/hsm0-with-executor`
main:+
main:  &sme=0x7ffde19de210
state1:+ &self=0x7ffde19de218
state1:-
state1:+ &self=0x7ffde19de218
state2:-
state1:+ &self=0x7ffde19de218
state1:-
state1:+ &self=0x7ffde19de218
state2:-
main:-
```

## Test

```
wink@3900x 22-11-23T00:11:31.812Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor (main)
$ cargo test
   Compiling hsm0-with-executor v0.8.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor)
    Finished test [unoptimized + debuginfo] target(s) in 0.56s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_with_executor-e73f722435e06fda)

running 17 tests
test test::test_2s_cycle ... ok
test test::test_1s_cycle ... ok
test test::test_2s_one_self_cycle ... ok
test test::test_5s_long_cycle ... ok
test test::test_3s_one_cycle ... ok
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
test test::test_sm_out_of_bounds_invalid_transition - should panic ... ok
test test::test_sm_out_of_bounds_initial_transition - should panic ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_with_executor-dd8bd98141e9e6ba)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests hsm0-with-executor

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

[Html Results](https://htmlpreview.github.io/?https://github.com/winksaville/proc-macro-hsm1/blob/main/hsm0-with-executor/coverage/html/index.html)


```
wink@3900x 22-11-23T00:37:37.179Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor (main)
$ cargo xt gen-cov
    Blocking waiting for file lock on build directory
   Compiling xtask v0.1.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/xtask)
    Finished dev [unoptimized + debuginfo] target(s) in 3.10s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/xtask gen-cov`
Run cargo clean []
Create profraw data at /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor/coverage
   Compiling memchr v2.5.0
   Compiling libc v0.2.126
   Compiling log v0.4.17
   Compiling regex-syntax v0.6.27
   Compiling cfg-if v1.0.0
   Compiling humantime v2.1.0
   Compiling termcolor v1.1.3
   Compiling aho-corasick v0.7.18
   Compiling atty v0.2.14
   Compiling regex v1.6.0
   Compiling env_logger v0.9.3
   Compiling custom-logger v0.1.0 (https://github.com/winksaville/custom-logger#4d828a35)
   Compiling hsm0-with-executor v0.8.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor)
    Finished test [unoptimized + debuginfo] target(s) in 3.96s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/deps/hsm0_with_executor-bac466462cf1501b)

running 17 tests
test test::test_2s_one_self_cycle ... ok
test test::test_2s_cycle ... ok
test test::test_1s_cycle ... ok
test test::test_sm_1h_2s_not_handled_no_enter_no_exit ... ok
test test::test_3s_one_cycle ... ok
test test::test_5s_long_cycle ... ok
test test::test_leaf_transitions_between_trees ... ok
test test::test_leaf_transitions_in_a_tree ... ok
test test::test_sm_1s_get_names ... ok
test test::test_sm_1s_no_enter_no_exit ... ok
test test::test_sm_1s_enter_no_exit ... ok
test test::test_sm_2s_get_names ... ok
test test::test_sm_2s_invalid_transition - should panic ... ok
test test::test_sm_out_of_bounds_initial_transition - should panic ... ok
test test::test_sm_out_of_bounds_invalid_transition - should panic ... ok
test test::test_sm_invalid_initial_state - should panic ... ok
test test::test_sm_2s_no_enter_no_exit ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Create /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor/coverage/html
Create /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor/coverage/tests.lcov
Create /home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor/coverage/tests.covdir.json
```

# Examples

```
wink@3900x 22-11-23T00:41:47.686Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor (main)
$ cargo run --example defer-msgs
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/examples/defer-msgs`
[2022-11-23T00:41:56.378917590Z INFO  defer_msgs   91  1] main:+
[2022-11-23T00:41:56.378956463Z INFO  defer_msgs   35  1] new: inital state=starting idxs_enter_fns=[0]
[2022-11-23T00:41:56.378965219Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-11-23T00:41:56.378975769Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-11-23T00:41:56.378981179Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-11-23T00:41:56.378986549Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-11-23T00:41:56.378991729Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-11-23T00:41:56.378996949Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-11-23T00:41:56.379002169Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-11-23T00:41:56.379007248Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-11-23T00:41:56.379012278Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-11-23T00:41:56.379018349Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-11-23T00:41:56.379024010Z INFO  defer_msgs   52  1] deferring: Messages::Complete, transition to do_deferred_work
[2022-11-23T00:41:56.379030031Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=1
[2022-11-23T00:41:56.379034700Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=2
[2022-11-23T00:41:56.379039178Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=3
[2022-11-23T00:41:56.379043907Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=4
[2022-11-23T00:41:56.379048556Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=5
[2022-11-23T00:41:56.379053144Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=6
[2022-11-23T00:41:56.379057723Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=7
[2022-11-23T00:41:56.379062542Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=8
[2022-11-23T00:41:56.379067010Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=9
[2022-11-23T00:41:56.379071469Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=10
[2022-11-23T00:41:56.379076097Z INFO  defer_msgs   77  1] do_deferred_work: Messages::Complete, sending Done { val: 10 }, transition to deferring
main: Sent Complete { tx: Sender { .. } }
main: Got Expected reponse=Done { val: 10 }
main: rx.try_recv() got the expected TryRecvError::Empty, e=Empty
[2022-11-23T00:41:56.379088831Z INFO  defer_msgs  136  1] main:- result_value=10
wink@3900x 22-11-23T00:41:56.380Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor (main)
$ cargo run --example hsm-1h-3s
   Compiling hsm0-with-executor v0.8.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/examples/hsm-1h-3s`
main
[2022-11-23T00:42:25.577596927Z INFO  hsm_1h_3s  161  1] main:+
[2022-11-23T00:42:25.577652190Z INFO  hsm_1h_3s  165  1] main:-
wink@3900x 22-11-23T00:42:25.578Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor (main)
$ cargo run --example hsm-2h-2s
   Compiling hsm0-with-executor v0.8.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/examples/hsm-2h-2s`
main
[2022-11-23T00:42:30.750509072Z INFO  hsm_2h_2s  192  1] main:+
[2022-11-23T00:42:30.750570687Z INFO  hsm_2h_2s  196  1] main:-
wink@3900x 22-11-23T00:42:30.752Z:~/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor (main)
$ cargo run --example send-msg-to-self
   Compiling hsm0-with-executor v0.8.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-with-executor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/examples/send-msg-to-self`
[2022-11-23T00:42:40.956293614Z INFO  send_msg_to_self   87  1] main:+
[2022-11-23T00:42:40.956336484Z INFO  send_msg_to_self   35  1] new: inital state=base idxs_enter_fns=[0]
[2022-11-23T00:42:40.956345491Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956353316Z INFO  send_msg_to_self   52  1] base Messages::Value:- self.val=1
[2022-11-23T00:42:40.956359508Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956364607Z INFO  send_msg_to_self   52  1] base Messages::Value:- self.val=2
[2022-11-23T00:42:40.956369536Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956374626Z INFO  send_msg_to_self   52  1] base Messages::Value:- self.val=3
[2022-11-23T00:42:40.956379675Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956384625Z INFO  send_msg_to_self   52  1] base Messages::Value:- self.val=4
[2022-11-23T00:42:40.956389754Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956394694Z INFO  send_msg_to_self   52  1] base Messages::Value:- self.val=5
[2022-11-23T00:42:40.956399583Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956404332Z INFO  send_msg_to_self   52  1] base Messages::Value:- self.val=6
[2022-11-23T00:42:40.956409892Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956414821Z INFO  send_msg_to_self   52  1] base Messages::Value:- self.val=7
[2022-11-23T00:42:40.956419680Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956424439Z INFO  send_msg_to_self   52  1] base Messages::Value:- self.val=8
[2022-11-23T00:42:40.956429429Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956434238Z INFO  send_msg_to_self   52  1] base Messages::Value:- self.val=9
[2022-11-23T00:42:40.956439327Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956444176Z INFO  send_msg_to_self   52  1] base Messages::Value:- self.val=10
[2022-11-23T00:42:40.956449166Z INFO  send_msg_to_self   47  1] base Messages::Value:+ val=1
[2022-11-23T00:42:40.956454155Z INFO  send_msg_to_self   62  1] base Messages::Value:- Done self.val=10
main: Done val=10
[2022-11-23T00:42:40.956462501Z INFO  send_msg_to_self  110  1] main:-
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

