# rust-gravatar

[![Build Status](https://img.shields.io/travis/chowdhurya/rust-gravatar/master.svg?style=flat-square)](https://travis-ci.org/chowdhurya/rust-gravatar)
[![Cargo version](https://img.shields.io/crates/v/gravatar.svg?style=flat-square)](https://crates.io/crates/gravatar)
[![Libraries.io for GitHub](https://img.shields.io/librariesio/github/chowdhurya/rust-gravatar.svg?style=flat-square)](https://libraries.io/cargo/gravatar/)
[![License](https://img.shields.io/crates/l/gravatar.svg?style=flat-square)](https://github.com/chowdhurya/rust-gravatar/blob/master/LICENSE)

[Documentation](https://docs.rs/gravatar/)

`rust-gravatar` is a small Rust library that generates Gravatar image URLs based
on the
[official Gravatar specification](http://en.gravatar.com/site/implement/images/).

## Example

```rust
extern crate gravatar;
use gravatar::{Gravatar, Rating};

let url = Gravatar::new("email@example.com")
    .set_size(Some(150))
    .set_rating(Some(Rating::Pg))
    .image_url();
assert_eq!(
    url.as_str(),
    "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e?s=150&r=pg"
);
```
