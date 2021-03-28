# Chande Log

## 0.3.1 *2021-3-28*
* switching from GPL 2 to MIT
## 0.3.0
This is a breaking change, but no one but me is using this according to crates.io, so I'm not bumping the major version number.
* created the `Lerror` enum for better error handling
* change the result of `get` and `offset` to return `Result`s with this error class
* changed the success result type of `get` to be `&str` instead of `String` so there's less allocation
## 0.2.0
* added `Larry::offset`
## 0.1.1
* fixed documentation tests
