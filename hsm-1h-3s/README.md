# An HSM with 1 hierarchy of three states

Simple HSM that uses hsm0-executor and simply transitions back and forth
between initial and other.

```
                base=0
        --------^  ^-------
       /                   \
      /                     \
    other=2   <======>   initial=1
```

# Run

Debug:
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/debug/hsm-1h-3s`
main
[2022-10-18T19:58:59.073059285Z INFO  hsm_1h_3s  163  1] main:+
[2022-10-18T19:58:59.073095934Z INFO  hsm_1h_3s  167  1] main:-
```

Release:
```
$ cargo run --release
   Compiling hsm0-executor v0.2.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm0-executor)
   Compiling hsm-1h-3s v0.2.0 (/home/wink/prgs/rust/myrepos/proc-macro-hsm1/hsm-1h-3s)
    Finished release [optimized] target(s) in 0.34s
     Running `/home/wink/prgs/rust/myrepos/proc-macro-hsm1/target/release/hsm-1h-3s`
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

