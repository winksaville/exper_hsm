# An HSM with 1 hierarchy of three states

Simple HSM manually coded which is a model for hsm1,
the hsm implemented using procedural macro.

The StateMachine here simply transitions back and forth
between `initial` and `other` one hierarchy with two
states:
```
                base=0
        --------^  ^-------
       /                   \
      /                     \
    other=2   <======>   initial=1
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

