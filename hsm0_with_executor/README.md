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

### file-stream-producer

```
wink@3900x 22-12-21T21:52:46.652Z:~/prgs/rust/myrepos/exper_hsm (main)
$ cargo run --example file-stream-producer
   Compiling hsm0-with-executor v0.8.0 (/home/wink/prgs/rust/myrepos/exper_hsm/hsm0_with_executor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/examples/file-stream-producer`
[2022-12-21T21:53:13.272500135Z INFO  file_stream_producer  307  1] main:+
[2022-12-21T21:53:13.272547944Z INFO  file_stream_producer  312  1] new: fsp=RefCell { value: FileStreamProducer { tx: Sender { .. }, rx: Receiver { .. }, partner_tx: None, file: None, buffers: [] } }
[2022-12-21T21:53:13.272661757Z INFO  file_stream_producer  319  2] efsp thread:+
[2022-12-21T21:53:13.272701532Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Open { file_name: "hello.txt", buf_count: 2, buf_capacity: 3, partner_tx: Sender { .. } }
[2022-12-21T21:53:13.272730326Z INFO  file_stream_producer  157  2] open: file_name=hello.txt
[2022-12-21T21:53:13.272741507Z INFO  file_stream_producer  174  2] open: buf_count=2 buf_capacity=3
[2022-12-21T21:53:13.272753399Z INFO  file_stream_producer  185  2] open: &buf[0]: 0x7f2ec4000e40 [0, 1, 2]
[2022-12-21T21:53:13.272766964Z INFO  file_stream_producer  185  2] open: &buf[0]: 0x7f2ec4000e80 [3, 4, 5]
[2022-12-21T21:53:13.272777785Z INFO  file_stream_producer  189  2] open: Handled Messages::Open transition to 'wait_for_start'
[2022-12-21T21:53:13.272791550Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Start
[2022-12-21T21:53:13.272802371Z INFO  file_stream_producer  203  2] wait_for_start: Got Start, tranistion to 'read'
[2022-12-21T21:53:13.272811939Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Read
[2022-12-21T21:53:13.272820935Z INFO  file_stream_producer  218  2] read: before read len=3 &buf[0]: 0x7f2ec4000e80 [3, 4, 5]
[2022-12-21T21:53:13.277947080Z INFO  file_stream_producer  229  2] read:  after read len=3 &buf[0]: 0x7f2ec4000e80 [48, 65, 6C]
[2022-12-21T21:53:13.277961036Z INFO  file_stream_producer  253  2] read: Send Data 3 to partner
[2022-12-21T21:53:13.277983789Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Read
[2022-12-21T21:53:13.277990591Z INFO  file_stream_producer  218  2] read: before read len=3 &buf[0]: 0x7f2ec4000e40 [0, 1, 2]
[2022-12-21T21:53:13.277987646Z INFO  file_stream_producer  348  1] main: Data 3 0x7f2ec4000e80 [48, 65, 6C]
[2022-12-21T21:53:13.277999488Z INFO  file_stream_producer  229  2] read:  after read len=3 &buf[0]: 0x7f2ec4000e40 [6C, 6F, 20]
[2022-12-21T21:53:13.278007112Z INFO  file_stream_producer  253  2] read: Send Data 3 to partner
[2022-12-21T21:53:13.278016009Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Empty { buf: [48, 65, 6C] }
[2022-12-21T21:53:13.278017842Z INFO  file_stream_producer  348  1] main: Data 3 0x7f2ec4000e40 [6C, 6F, 20]
[2022-12-21T21:53:13.278024094Z INFO  file_stream_producer  281  2] read: unhandled Empty { buf: [48, 65, 6C] }
[2022-12-21T21:53:13.278032390Z INFO  file_stream_producer  124  2] base: Messages::Empty: &buf[0]: 0x7f2ec4000e80 [48, 65, 6C]
[2022-12-21T21:53:13.278041958Z INFO  file_stream_producer  126  2] base: Messages::Empty:   &x[0]: 0x7f2ec4000ea0 [48, 65, 6C]
[2022-12-21T21:53:13.278049812Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Read
[2022-12-21T21:53:13.278055483Z INFO  file_stream_producer  218  2] read: before read len=3 &buf[0]: 0x7f2ec4000ea0 [48, 65, 6C]
[2022-12-21T21:53:13.278063298Z INFO  file_stream_producer  229  2] read:  after read len=3 &buf[0]: 0x7f2ec4000ea0 [57, 6F, 72]
[2022-12-21T21:53:13.278070411Z INFO  file_stream_producer  253  2] read: Send Data 3 to partner
[2022-12-21T21:53:13.278078766Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Empty { buf: [6C, 6F, 20] }
[2022-12-21T21:53:13.278081141Z INFO  file_stream_producer  348  1] main: Data 3 0x7f2ec4000ea0 [57, 6F, 72]
[2022-12-21T21:53:13.278086271Z INFO  file_stream_producer  281  2] read: unhandled Empty { buf: [6C, 6F, 20] }
[2022-12-21T21:53:13.278094045Z INFO  file_stream_producer  124  2] base: Messages::Empty: &buf[0]: 0x7f2ec4000e40 [6C, 6F, 20]
[2022-12-21T21:53:13.278101269Z INFO  file_stream_producer  126  2] base: Messages::Empty:   &x[0]: 0x559baa5a0b50 [6C, 6F, 20]
[2022-12-21T21:53:13.278108823Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Read
[2022-12-21T21:53:13.278114513Z INFO  file_stream_producer  218  2] read: before read len=3 &buf[0]: 0x559baa5a0b50 [6C, 6F, 20]
[2022-12-21T21:53:13.278122418Z INFO  file_stream_producer  229  2] read:  after read len=3 &buf[0]: 0x559baa5a0b50 [6C, 64, 21]
[2022-12-21T21:53:13.278129421Z INFO  file_stream_producer  253  2] read: Send Data 3 to partner
[2022-12-21T21:53:13.278137837Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Empty { buf: [57, 6F, 72] }
[2022-12-21T21:53:13.278139971Z INFO  file_stream_producer  348  1] main: Data 3 0x559baa5a0b50 [6C, 64, 21]
[2022-12-21T21:53:13.278145371Z INFO  file_stream_producer  281  2] read: unhandled Empty { buf: [57, 6F, 72] }
[2022-12-21T21:53:13.278152825Z INFO  file_stream_producer  124  2] base: Messages::Empty: &buf[0]: 0x7f2ec4000ea0 [57, 6F, 72]
[2022-12-21T21:53:13.278160019Z INFO  file_stream_producer  126  2] base: Messages::Empty:   &x[0]: 0x559baa5a0b70 [57, 6F, 72]
[2022-12-21T21:53:13.278167643Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Read
[2022-12-21T21:53:13.278173504Z INFO  file_stream_producer  218  2] read: before read len=3 &buf[0]: 0x559baa5a0b70 [57, 6F, 72]
[2022-12-21T21:53:13.278181248Z INFO  file_stream_producer  229  2] read:  after read len=1 &buf[0]: 0x559baa5a0b70 [A]
[2022-12-21T21:53:13.278187781Z INFO  file_stream_producer  236  2] read: EOF
[2022-12-21T21:53:13.278193080Z INFO  file_stream_producer  238  2] read: EOF Send 1 bytes to partner
[2022-12-21T21:53:13.278201256Z INFO  file_stream_producer  249  2] read: EOF transitition to 'open'
[2022-12-21T21:53:13.278204021Z INFO  file_stream_producer  354  1] main: Done result=true
[2022-12-21T21:53:13.278208970Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=Empty { buf: [6C, 64, 21] }
[2022-12-21T21:53:13.278217145Z INFO  file_stream_producer  124  2] base: Messages::Empty: &buf[0]: 0x559baa5a0b50 [6C, 64, 21]
[2022-12-21T21:53:13.278224068Z INFO  file_stream_producer  126  2] base: Messages::Empty:   &x[0]: 0x559baa5a0b70 [6C, 64, 21]
[2022-12-21T21:53:13.278231182Z INFO  file_stream_producer  321  2] efsp thread:  recv msg=StopThread
[2022-12-21T21:53:13.278236632Z INFO  file_stream_producer  134  2] base: Messages::StopThread IGNORING open
[2022-12-21T21:53:13.278241832Z INFO  file_stream_producer  325  2] efsp thread: Stopping
[2022-12-21T21:53:13.278247162Z INFO  file_stream_producer  331  2] efsp thread:-
[2022-12-21T21:53:13.278285163Z INFO  file_stream_producer  364  1] main:-
```

### defer-msgs

```
wink@3900x 22-12-21T21:50:23.580Z:~/prgs/rust/myrepos/exper_hsm (main)
$ cargo run --example defer-msgs
   Compiling hsm0-with-executor v0.8.0 (/home/wink/prgs/rust/myrepos/exper_hsm/hsm0_with_executor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.51s
     Running `target/debug/examples/defer-msgs`
[2022-12-21T21:52:46.646380636Z INFO  defer_msgs   91  1] main:+
[2022-12-21T21:52:46.646420881Z INFO  defer_msgs   35  1] new: inital state=starting idxs_enter_fns=[0]
[2022-12-21T21:52:46.646429618Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
[2022-12-21T21:52:46.646437462Z INFO  defer_msgs   99  1] main: Sent DeferredValue { val: 1 }
[2022-12-21T21:52:46.646443443Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
[2022-12-21T21:52:46.646448463Z INFO  defer_msgs   99  1] main: Sent DeferredValue { val: 1 }
[2022-12-21T21:52:46.646454013Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
[2022-12-21T21:52:46.646459243Z INFO  defer_msgs   99  1] main: Sent DeferredValue { val: 1 }
[2022-12-21T21:52:46.646464513Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
[2022-12-21T21:52:46.646469402Z INFO  defer_msgs   99  1] main: Sent DeferredValue { val: 1 }
[2022-12-21T21:52:46.646474632Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
[2022-12-21T21:52:46.646479491Z INFO  defer_msgs   99  1] main: Sent DeferredValue { val: 1 }
[2022-12-21T21:52:46.646484821Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
[2022-12-21T21:52:46.646489680Z INFO  defer_msgs   99  1] main: Sent DeferredValue { val: 1 }
[2022-12-21T21:52:46.646494900Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
[2022-12-21T21:52:46.646499799Z INFO  defer_msgs   99  1] main: Sent DeferredValue { val: 1 }
[2022-12-21T21:52:46.646505069Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
[2022-12-21T21:52:46.646510078Z INFO  defer_msgs   99  1] main: Sent DeferredValue { val: 1 }
[2022-12-21T21:52:46.646515288Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
[2022-12-21T21:52:46.646520267Z INFO  defer_msgs   99  1] main: Sent DeferredValue { val: 1 }
[2022-12-21T21:52:46.646525457Z INFO  defer_msgs   47  1] deferring: Messages::DeferredValue:+ val=1
[2022-12-21T21:52:46.646530326Z INFO  defer_msgs   99  1] main: Sent DeferredValue { val: 1 }
[2022-12-21T21:52:46.646536036Z INFO  defer_msgs   52  1] deferring: Messages::Complete, transition to do_deferred_work
[2022-12-21T21:52:46.646542839Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=1
[2022-12-21T21:52:46.646548079Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=2
[2022-12-21T21:52:46.646553289Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=3
[2022-12-21T21:52:46.646558608Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=4
[2022-12-21T21:52:46.646563748Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=5
[2022-12-21T21:52:46.646568898Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=6
[2022-12-21T21:52:46.646574047Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=7
[2022-12-21T21:52:46.646579317Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=8
[2022-12-21T21:52:46.646584447Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=9
[2022-12-21T21:52:46.646589606Z INFO  defer_msgs   67  1] do_deferred_work: Messages::DeferredValue:+ val=1 self.val=10
[2022-12-21T21:52:46.646594946Z INFO  defer_msgs   77  1] do_deferred_work: Messages::Complete, sending Done { val: 10 }, transition to deferring
[2022-12-21T21:52:46.646603733Z INFO  defer_msgs  105  1] main: Sent Complete { tx: Sender { .. } }
[2022-12-21T21:52:46.646609373Z INFO  defer_msgs  111  1] main: Got Expected reponse=Done { val: 10 }
[2022-12-21T21:52:46.646614653Z INFO  defer_msgs  127  1] main: rx.try_recv() got the expected TryRecvError::Empty, e=Empty
[2022-12-21T21:52:46.646619552Z INFO  defer_msgs  136  1] main:- result_value=10
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

