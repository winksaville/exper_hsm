# Task for automation

Based on https://github.com/winksaville/cargo-xtask/examples/wink, which
is based on https://github.com/matklad/cargo-xtask.

## Tasks

I've created two alias `xtask` and `xt` so these can be executed
with `cargo xtask xxx` or `cargo xt xxx`.

### pre-commit

Runs `cargo fmt` and `cargo test`

