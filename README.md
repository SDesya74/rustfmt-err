# A bug in fustrmt

```rust
fn main() {
    let vec = Vec::<(usize,)>::new();
    let mut v: usize = 0;

    for (mut i,) in vec.iter() { // clippy: variable does not need to be mutable
        foo(&mut v, i)
    }
}

fn foo<T>(_: &mut T, _: T) {}

```

## Output

```
➜  rustfmt-err git:(master) ✗ cargo clippy --fix --allow-dirty
    Checking rustfmt-err v0.1.0 (/Users/me/dev/tests/rustfmt-err)
warning: failed to automatically apply fixes suggested by rustc to crate `rustfmt_err`

after fixes were automatically applied the compiler reported errors within these files:

  * src/main.rs

This likely indicates a bug in either rustc or cargo itself,
and we would appreciate a bug report! You're likely to see
a number of compiler warnings after this message which cargo
attempted to fix but failed. If you could open an issue at
https://github.com/rust-lang/rust/issues
quoting the full output of this command we'd be very appreciative!
Note that you may be able to make some more progress in the near-term
fixing code with the `--broken-code` flag

The following errors were reported:
error[E0308]: mismatched types
  --> src/main.rs:6:21
   |
6  |         foo(&mut v, i)
   |         ---         ^ expected `usize`, found `&usize`
   |         |
   |         arguments to this function are incorrect
   |
note: function defined here
  --> src/main.rs:10:4
   |
10 | fn foo<T>(_: &mut T, _: T) {}
   |    ^^^               ----
help: consider dereferencing the borrow
   |
6  |         foo(&mut v, *i)
   |                     +

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Original diagnostics will follow.

warning: variable does not need to be mutable
 --> src/main.rs:5:10
  |
5 |     for (mut i,) in vec.iter() {
  |          ----^
  |          |
  |          help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

warning: `rustfmt-err` (bin "rustfmt-err") generated 1 warning (run `cargo clippy --fix --bin "rustfmt-err"` to apply 1 suggestion)
warning: `rustfmt-err` (bin "rustfmt-err" test) generated 1 warning (1 duplicate)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
```
