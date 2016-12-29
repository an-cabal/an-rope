# an rope [![Build Status](https://travis-ci.org/hawkw/an-editor.svg?branch=master)](https://travis-ci.org/hawkw/an-editor) [![codecov](https://codecov.io/gh/hawkw/an-editor/branch/master/graph/badge.svg)](https://codecov.io/gh/hawkw/an-editor) [![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/hawkw/an-editor/blob/master/LICENSE) [![crates.io](https://img.shields.io/crates/v/an-rope.svg)](https://crates.io/crates/an-rope) [![RustDoc documentation](https://docs.rs/an-rope/badge.svg)](https://docs.rs/an-rope)

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
non-destructive, persistant API.
