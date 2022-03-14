# Build

```
wasm-pack build --target web
```

# Compile Error

```
error[E0080]: erroneous constant used
  --> /Users/claravanstaden/.cargo/registry/src/github.com-1ecc6299db9ec823/ssz_rs-0.7.0/src/ser.rs:33:22
   |
33 |     if total_size >= MAXIMUM_LENGTH {
   |                      ^^^^^^^^^^^^^^ referenced constant has errors

error[E0080]: evaluation of constant value failed
  --> /Users/claravanstaden/.cargo/registry/src/github.com-1ecc6299db9ec823/ssz_rs-0.7.0/src/ser.rs:11:13
   |
11 |     #[error("the encoded length is {0} which exceeds the maximum length {MAXIMUM_LENGTH}")]
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors

For more information about this error, try `rustc --explain E0080`.
error: could not compile `ssz_rs` due to 2 previous errors
Error: Compiling your crate to WebAssembly failed
Caused by: failed to execute `cargo build`: exited with exit status: 101
```
