rust-gravatar
=============

[![Build Status](https://img.shields.io/travis/chowdhurya/rust-gravatar/master.svg)](https://travis-ci.org/chowdhurya/rust-gravatar)
[![Cargo version](https://img.shields.io/crates/v/gravatar.svg)](https://crates.io/crates/gravatar)
[![License](https://img.shields.io/crates/l/gravatar.svg)](https://github.com/chowdhurya/rust-gravatar/blob/master/LICENSE)

[Documentation](https://chowdhurya.github.io/rust-gravatar/gravatar/)

`rust-gravatar` is a small Rust library that generates Gravatar image URLs based
on the
[official Gravatar specification](http://en.gravatar.com/site/implement/images/).

Example
--------
```rust
extern crate gravatar;
use gravatar::{Gravatar, Rating};

let mut g = Gravatar::new("email@example.com");
g.size = Some(150);
g.rating = Some(Rating::Pg);
assert_eq!(
    g.image_url(),
    "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e\
     ?s=150&r=pg"
);
```
