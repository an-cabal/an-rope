# an rope
[![Build Status](https://travis-ci.org/an-cabal/an-rope.svg?branch=master)](https://travis-ci.org/an-cabal/an-rope)
[![codecov](https://codecov.io/gh/an-cabal/an-rope/branch/master/graph/badge.svg)](https://codecov.io/gh/an-cabal/an-rope)
[![Dependency Status](https://dependencyci.com/github/an-cabal/an-rope/badge)](https://dependencyci.com/github/an-cabal/an-rope)
[![Clippy Linting Result](https://img.shields.io/badge/clippy-linted-green.svg)](https://clippy.bashy.io/github/an-cabal/an-rope/master/log)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/an-cabal/an-rope/blob/master/LICENSE) [![crates.io](https://img.shields.io/crates/v/an-rope.svg)](https://crates.io/crates/an-rope)
[![RustDoc documentation](https://docs.rs/an-rope/badge.svg)](https://docs.rs/an-rope)
[![Master RustDoc](https://img.shields.io/badge/docs-master-blue.svg)](https://an-cabal.github.io/an-rope)

An immutable Rope data structure for storing large text documents. This implementation is a component of the [`an-editor` project](https://github.com/an-cabal/an-editor).

A rope is an efficient data structure for large strings. It's
essentially a binary tree whose leaves are strings.

For more information, see the following resources:

+ http://scienceblogs.com/goodmath/2009/01/26/ropes-twining-together-strings/
+ https://www.ibm.com/developerworks/library/j-ropes/
+ http://citeseer.ist.psu.edu/viewdoc/download?doi=10.1.1.14.9450&rep=rep1&type=pdf

### compatibility

`an-rope` is [built against](https://travis-ci.org/an-cabal/an-rope) the latest stable, beta, and nightly Rust releases, on macOS and Ubuntu. Some features rely on nightly Rust, and may not be available on other release channels.

### cargo feature flags

+ `tendril`: use the [`tendril`](https://docs.rs/crate/tendril/0.2.3) library to optimise performance for small strings.
+ `rebalance`: enable Rope rebalancing.
+ `atomic`: ensure Ropes are thread-safe (use `Arc` or atomic `tendril`s)
+ `unstable`: enable nightly Rust features. pass this flag if building on nightly Rust.
