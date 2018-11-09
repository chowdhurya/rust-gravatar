# Changelog

## 0.2.0 (2018-11-10)

- Rewrote the module to hide struct members and use setter functions instead
- Restricted size parameter to being between 1px and 2048px regardless of user
  input
- Changed MD5 hashing library to RustCrypto's `md-5` crate

## 0.1.5 (2016-05-14)

- Updated dependencies

## 0.1.4 (2016-03-02)

- Fixed type resolution failure

## 0.1.3 (2015-04-03)

- Removed use of unstable features to work with Rust 1.0.0-beta
- Updated dependencies

## 0.1.2 (2015-03-26)

- Replaced use of the deprecated `as_slice()` function with the newer `as_ref()`
  function to convert a `String` into a `&str`
- Updated dependencies

## 0.1.1 (2015-03-23)

- Updated dependencies

## 0.1.0 (2015-03-20)

- Initial release
