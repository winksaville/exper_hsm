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
## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

