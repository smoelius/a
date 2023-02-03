# a

```
cargo +nightly install --git https://github.com/AFLplusplus/cargo-libafl --force
cargo libafl init
```

Edit `Cargo.toml` and `fuzz_target_1.rs`. Then:

```
cargo +nightly libafl run fuzz_target_1
```
