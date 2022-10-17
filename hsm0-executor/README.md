# Attempt to use an executor model

An Executor Model where there is a seperation between
the Users StateMachine implementation and the code that
executes the StateMachine. Not sure this will work but
we'll see.

First step, I created StateMachineExecutor which was
previously StateMachine and has-a `smi: StateMachineInfo`
and along with that moved StateMachineInfo above StateMachineExecutor.
I added `sm: StateMachine` as a member of `StateMachineInfo`. And
then modified the calls to `enter`, `process` and `exit` in `dispatch_hdl`
to pass `sm` as the first parameter.

The next step will be to refactor `StateMachineInfo` into
`StateMachineExecutor`.

The subsequent step will be to make `StateMachineExecutor` generic
over `SM` (i.e. StateMachine) and then to make `StateMachineExecutor`
a module.

If this works we will have a generic executor that invokes "StateFns"
(`enter`, `process` and `exit`) which can mutate the `SM` and a user
can access an immutable instance of `SM`. This tightly enforces that
only methods inside `SM` can mutate the instance fields!

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

