![code-coverage](coverage/html/badges/flat.svg)

# Separate StateMachine from the code that executes it

In this model there is an `Executor` and the `StateMachine`. The user
creates the state machine and builds by giving it to `Executor::new()`
then add all of the states one at a time using `with_state()` and
finally `build()` passing the initial state.

## Run

Debug:
```
wink@3900x 22-12-21T01:41:29.690Z:~/prgs/rust/myrepos/exper_hsm/hsm0_with_executor (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/hsm0-with-executor`
main:+
main:  &sme=0x7ffdb9f73db8
state1:+ &self=0x7ffdb9f73e20
state1:-
state1:+ &self=0x7ffdb9f73e20
state2:-
state1:+ &self=0x7ffdb9f73e20
state1:-
state1:+ &self=0x7ffdb9f73e20
state2:-
main:-
```

Release:
```
wink@3900x 22-12-21T01:42:27.250Z:~/prgs/rust/myrepos/exper_hsm/hsm0_with_executor (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.02s
     Running `/home/wink/prgs/rust/myrepos/exper_hsm/target/release/hsm0-with-executor`
main:+
main:  &sme=0x7ffdacf7cef0
state1:+ &self=0x7ffdacf7cf58
state1:-
state1:+ &self=0x7ffdacf7cf58
state2:-
state1:+ &self=0x7ffdacf7cf58
state1:-
state1:+ &self=0x7ffdacf7cf58
state2:-
main:-
```

## Test

```
wink@3900x 22-12-21T01:42:28.296Z:~/prgs/rust/myrepos/exper_hsm/hsm0_with_executor (main)
$ cargo test
   Compiling hsm0-with-executor v0.8.0 (/home/wink/prgs/rust/myrepos/exper_hsm/hsm0_with_executor)
    Finished test [unoptimized + debuginfo] target(s) in 0.84s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/deps/hsm0_with_executor-ec5892669e0bc122)

running 17 tests
test test::test_2s_cycle ... ok
test test::test_2s_one_self_cycle ... ok
test test::test_1s_cycle ... ok
test test::test_3s_one_cycle ... ok
test test::test_5s_long_cycle ... ok
test test::test_leaf_transitions_between_trees ... ok
test test::test_leaf_transitions_in_a_tree ... ok
test test::test_sm_1h_2s_not_handled_no_enter_no_exit ... ok
test test::test_sm_1s_enter_no_exit ... ok
test test::test_sm_1s_get_names ... ok
test test::test_sm_1s_no_enter_no_exit ... ok
test test::test_sm_2s_get_names ... ok
test test::test_sm_2s_no_enter_no_exit ... ok
test test::test_sm_invalid_initial_state - should panic ... ok
test test::test_sm_2s_invalid_transition - should panic ... ok
test test::test_sm_out_of_bounds_initial_transition - should panic ... ok
test test::test_sm_out_of_bounds_invalid_transition - should panic ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/deps/hsm0_with_executor-4ec20b7a7edc8446)

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

[Html Results](https://htmlpreview.github.io/?https://github.com/winksaville/exper_hsm/blob/main/hsm0_with_executor/coverage/html/index.html)


```
wink@3900x 22-12-21T01:42:50.436Z:~/prgs/rust/myrepos/exper_hsm/hsm0_with_executor (main)
$ cargo xt gen-cov
   Compiling xtask v0.1.0 (/home/wink/prgs/rust/myrepos/exper_hsm/xtask)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/xtask gen-cov`
Run cargo clean []
Create profraw data at /home/wink/prgs/rust/myrepos/exper_hsm/hsm0_with_executor/coverage
   Compiling libc v0.2.138
   Compiling cfg-if v1.0.0
   Compiling memchr v2.5.0
   Compiling log v0.4.17
   Compiling regex-syntax v0.6.28
   Compiling humantime v2.1.0
   Compiling ppv-lite86 v0.2.17
   Compiling termcolor v1.1.3
   Compiling aho-corasick v0.7.20
   Compiling getrandom v0.2.8
   Compiling atty v0.2.14
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling regex v1.7.0
   Compiling rand v0.8.5
   Compiling env_logger v0.9.3
   Compiling custom-logger v0.1.0 (https://github.com/winksaville/custom-logger#4d828a35)
   Compiling hsm0-with-executor v0.8.0 (/home/wink/prgs/rust/myrepos/exper_hsm/hsm0_with_executor)
    Finished test [unoptimized + debuginfo] target(s) in 4.52s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/deps/hsm0_with_executor-f2534b38e0f4b2c0)

running 17 tests
test test::test_2s_cycle ... ok
test test::test_1s_cycle ... ok
test test::test_2s_one_self_cycle ... ok
test test::test_3s_one_cycle ... ok
test test::test_5s_long_cycle ... ok
test test::test_leaf_transitions_in_a_tree ... ok
test test::test_sm_1h_2s_not_handled_no_enter_no_exit ... ok
test test::test_leaf_transitions_between_trees ... ok
test test::test_sm_1s_enter_no_exit ... ok
test test::test_sm_1s_no_enter_no_exit ... ok
test test::test_sm_1s_get_names ... ok
test test::test_sm_2s_get_names ... ok
test test::test_sm_2s_no_enter_no_exit ... ok
test test::test_sm_2s_invalid_transition - should panic ... ok
test test::test_sm_invalid_initial_state - should panic ... ok
test test::test_sm_out_of_bounds_initial_transition - should panic ... ok
test test::test_sm_out_of_bounds_invalid_transition - should panic ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Create /home/wink/prgs/rust/myrepos/exper_hsm/hsm0_with_executor/coverage/html
Create /home/wink/prgs/rust/myrepos/exper_hsm/hsm0_with_executor/coverage/tests.lcov
Create /home/wink/prgs/rust/myrepos/exper_hsm/hsm0_with_executor/coverage/tests.covdir.json
```

# Examples

To see the list of examples:
```
wink@3900x 22-12-21T01:47:47.214Z:~/prgs/rust/myrepos/exper_hsm/hsm0_with_executor (main)
$ cargo run --example
error: "--example" takes one argument.
Available examples:
    defer-msgs
    file-stream-producer
    hsm-1h-3s
    hsm-2h-2s
    send-msg-to-self
    zero-copy
```

## file-stream-producer

```
wink@3900x 22-12-21T06:09:02.645Z:~/prgs/rust/myrepos/exper_hsm (main)
$ cargo run --example file-stream-producer
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/file-stream-producer`
[2022-12-21T06:09:09.331051640Z INFO  file_stream_producer  307  1] main:+
new: fsp=RefCell { value: FileStreamProducer { tx: Sender { .. }, rx: Receiver { .. }, partner_tx: None, file: None, buffers: [] } }
efsp thread:+
efsp thread:  recv msg=Open { file_name: "hello.txt", buf_count: 2, buf_capacity: 3, partner_tx: Sender { .. } }
open: file_name=hello.txt
open: buf_count=2 buf_capacity=3
open: &buf[0]: 0x7fe2e0000cc0 [0, 1, 2]
open: &buf[0]: 0x7fe2e0000d00 [3, 4, 5]
open: Handled Messages::Open transition to 'wait_for_start'
efsp thread:  recv msg=Start
wait_for_start: Got Start, tranistion to 'read'
efsp thread:  recv msg=Read
read: before read len=3 &buf[0]: 0x7fe2e0000d00 [3, 4, 5]
read:  after read len=3 &buf[0]: 0x7fe2e0000d00 [48, 65, 6C]
read: Send Data 3 to partner
efsp thread:  recv msg=Read
read: before read len=3 &buf[0]: 0x7fe2e0000cc0 [0, 1, 2]
read:  after read len=3 &buf[0]: 0x7fe2e0000cc0 [6C, 6F, 20]
read: Send Data 3 to partner
main: Data 3 0x7fe2e0000d00 [48, 65, 6C]
main: Data 3 0x7fe2e0000cc0 [6C, 6F, 20]
efsp thread:  recv msg=Read
read: no buffers, transition to 'wait_for_empty'
efsp thread:  recv msg=Empty { buf: [48, 65, 6C] }
base: Messages::Empty: &buf[0]: 0x7fe2e0000d00 [48, 65, 6C]
base: Messages::Empty:   &x[0]: 0x7fe2e0000d20 [48, 65, 6C]
efsp thread:  recv msg=Empty { buf: [6C, 6F, 20] }
read: unhandled Empty { buf: [6C, 6F, 20] }
base: Messages::Empty: &buf[0]: 0x7fe2e0000cc0 [6C, 6F, 20]
base: Messages::Empty:   &x[0]: 0x7fe2e0000d00 [6C, 6F, 20]
efsp thread:  recv msg=Read
read: before read len=3 &buf[0]: 0x7fe2e0000d00 [6C, 6F, 20]
read:  after read len=3 &buf[0]: 0x7fe2e0000d00 [57, 6F, 72]
read: Send Data 3 to partner
efsp thread:  recv msg=Read
read: before read len=3 &buf[0]: 0x7fe2e0000d20 [48, 65, 6C]
main: Data 3 0x7fe2e0000d00 [57, 6F, 72]
read:  after read len=3 &buf[0]: 0x7fe2e0000d20 [6C, 64, 21]
read: Send Data 3 to partner
efsp thread:  recv msg=Empty { buf: [57, 6F, 72] }
main: Data 3 0x7fe2e0000d20 [6C, 64, 21]
read: unhandled Empty { buf: [57, 6F, 72] }
base: Messages::Empty: &buf[0]: 0x7fe2e0000d00 [57, 6F, 72]
base: Messages::Empty:   &x[0]: 0x5629cffdda50 [57, 6F, 72]
efsp thread:  recv msg=Read
read: before read len=3 &buf[0]: 0x5629cffdda50 [57, 6F, 72]
read:  after read len=1 &buf[0]: 0x5629cffdda50 [A]
read: EOF
read: EOF Send 1 bytes to partner
read: EOF transitition to 'open'
main: Done result=true
efsp thread:  recv msg=Empty { buf: [6C, 64, 21] }
base: Messages::Empty: &buf[0]: 0x7fe2e0000d20 [6C, 64, 21]
base: Messages::Empty:   &x[0]: 0x5629cffdda50 [6C, 64, 21]
efsp thread:  recv msg=StopThread
base: Messages::StopThread IGNORING open
efsp thread: Stopping
efsp thread:-
[2022-12-21T06:09:09.331416701Z INFO  file_stream_producer  364  1] main:-
```

## defer-msgs

```
wink@3900x 22-12-21T01:45:18.046Z:~/prgs/rust/myrepos/exper_hsm/hsm0_with_executor (main)
$ cargo run --example defer-msgs
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `/home/wink/prgs/rust/myrepos/exper_hsm/target/debug/examples/defer-msgs`
[2022-12-21T01:45:19.928969623Z INFO  defer_msgs   91  1] main:+
[2022-12-21T01:45:19.929015078Z INFO  defer_msgs   35  1] new: inital state=starting idxs_enter_fns=[0]
[2022-12-21T01:45:19.929024285Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-12-21T01:45:19.929035667Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-12-21T01:45:19.929042309Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-12-21T01:45:19.929048410Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-12-21T01:45:19.929054823Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-12-21T01:45:19.929063008Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-12-21T01:45:19.929070742Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-12-21T01:45:19.929076303Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-12-21T01:45:19.929082194Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-12-21T01:45:19.929087875Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
main: Sent DeferredValue { val: 1 }
[2022-12-21T01:45:19.929093946Z INFO  defer_msgs   52  1] deferring: Messages::Complete, transition to do_deferred_work
[2022-12-21T01:45:19.929103033Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=1
[2022-12-21T01:45:19.929110818Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=2
[2022-12-21T01:45:19.929117610Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=3
[2022-12-21T01:45:19.929124413Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=4
[2022-12-21T01:45:19.929131637Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=5
[2022-12-21T01:45:19.929139171Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=6
[2022-12-21T01:45:19.929145503Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=7
[2022-12-21T01:45:19.929152345Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=8
[2022-12-21T01:45:19.929157936Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=9
[2022-12-21T01:45:19.929164288Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=10
[2022-12-21T01:45:19.929170449Z INFO  defer_msgs   77  1] do_deferred_work: Messages::Complete, sending Done { val: 10 }, transition to deferring
main: Sent Complete { tx: Sender { .. } }
main: Got Expected reponse=Done { val: 10 }
main: rx.try_recv() got the expected TryRecvError::Empty, e=Empty
[2022-12-21T01:45:19.929188083Z INFO  defer_msgs  136  1] main:- result_value=10
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

