# an rope [![Build Status](https://travis-ci.org/an-cabal/an-rope.svg?branch=master)](https://travis-ci.org/an-cabal/an-rope) [![codecov](https://codecov.io/gh/an-cabal/an-rope/branch/master/graph/badge.svg)](https://codecov.io/gh/an-cabal/an-rope) [![Dependency Status](https://dependencyci.com/github/an-cabal/an-editor/badge)](https://dependencyci.com/github/an-cabal/an-rope) [![Clippy Linting Result](https://clippy.bashy.io/github/an-cabal/an-rope/master/badge.svg)](https://clippy.bashy.io/github/an-cabal/an-rope/master/log) [![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/an-cabal/an-rope/blob/master/LICENSE) [![crates.io](https://img.shields.io/crates/v/an-rope.svg)](https://crates.io/crates/an-rope) [![RustDoc documentation](https://docs.rs/an-rope/badge.svg)](https://docs.rs/an-rope)

An implementation of the Rope data structure for storing large text documents. This implementation is a component of the [`an-editor` project](https://github.com/an-cabal/an-editor).

A rope is an efficient data structure for large mutable strings. It's
essentially a binary tree whose leaves are strings.

For more information, see the following resources:

+ http://scienceblogs.com/goodmath/2009/01/26/ropes-twining-together-strings/
+ https://www.ibm.com/developerworks/library/j-ropes/
+ http://citeseer.ist.psu.edu/viewdoc/download?doi=10.1.1.14.9450&rep=rep1&type=pdf

Our `Rope` implementation aims to eventually function as a superset of
Rust's [`String`](https://doc.rust-lang.org/1.3.0/std/string/struct.String.html),
providing the same API plus additional methods. Therefore, code which uses
`String` can easily be ported to use `Rope`.

`Rope` provides two APIs for editing a `Rope`: a destructive,
edit-in-place API whose methods match those of `String`, and a
non-destructive, persistent API.

### compatibility

`an-rope` is [built against](https://travis-ci.org/an-cabal/an-rope) the latest stable, beta, and nightly Rust releases, on macOS and Ubuntu. Some features rely on nightly Rust, and may not be available on other release channels.

### cargo feature flags

+ `tendril`: use the [`tendril`](https://docs.rs/crate/tendril/0.2.3) library to optimise performance for small strings.
+ `rebalance`: enable Rope rebalancing.
+ `unstable`: enable nightly Rust features. pass this flag if building on nightly Rust.
