# An HSM with 2 hierarchies of two states

Simple HSM with two hierarchies that uses hsm0-executor
and simply transitions back and forth between initial and other.

```
 other_base=2          initial_base=0
      ^                     ^
      |                     |
    other=3   <======>   initial=1
```

# Run

Debug:
```
$ cargo run
   Compiling hsm-2h-2s v0.2.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm-2h-2s)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/hsm-2h-2s`
main
[2022-10-18T20:29:11.433848601Z INFO  hsm_2h_2s  197  1] main:+
[2022-10-18T20:29:11.433887665Z INFO  hsm_2h_2s  201  1] main:-
```

Release:
```
$ cargo run --release
   Compiling hsm-2h-2s v0.2.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm-2h-2s)
    Finished release [optimized] target(s) in 0.28s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/release/hsm-2h-2s`
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

