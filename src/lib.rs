/*!
This crate implements generators for Rust. Generators are a feature common across many
programming language. They let you yield a sequence of values from a function. A few
common use cases are:

- Easily building iterators.
- Avoiding allocating a list for a function which returns multiple values.

Rust has this feature too, but it is currently unstable (and thus nightly-only). But
with this crate, you can use them on stable Rust!

# Example

Here is how it works in a nutshell:

```rust
use genawaiter::rc::{Co, Gen};

async fn odd_numbers_less_than_ten(co: Co<i32>) {
    let mut n = 1;
    while n < 10 {
        co.yield_(n).await;
        n += 2;
    }
}

# let mut xs = Vec::new();
for n in Gen::new(odd_numbers_less_than_ten) {
    # xs.push(n);
    println!("{}", n);
}
# assert_eq!(xs, [1, 3, 5, 7, 9]);
```

Result:

```text
1
3
5
7
9
```

# Backported stdlib types

This crate supplies [`Generator`] and [`GeneratorState`]. They are copy/pasted from the
stdlib (with stability attributes removed) so they can be used on stable Rust. If/when
real generators are stabilized, hopefully they would be drop-in replacements.

A Javscript developer might recognize this as a _polyfill_.

# Choose your guarantees

This crate currently supplies two concrete implementations of the [`Generator`] trait:

1. [`genawaiter::rc`] – This uses 100% safe code, but requires allocation.
2. [`genawaiter::stack`] – This works without allocating memory, but has a number of
   downsides:

   - It uses a macro.
   - It uses unsafe code under the hood.
   - It is possible to violate memory safety (but only if you do silly things with the
     `co` object).

[`genawaiter::rc`]: rc
[`genawaiter::stack`]: stack
*/

#![cfg_attr(feature = "nightly", feature(async_await, async_closure))]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(missing_docs, clippy::pedantic)]
// #![warn(clippy::cargo)]
#![cfg_attr(feature = "strict", deny(warnings))]

pub use ops::{Generator, GeneratorState};

mod ops;
pub mod rc;
pub mod stack;
mod waker;
