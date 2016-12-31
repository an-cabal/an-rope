# an editor [![Build Status](https://travis-ci.org/hawkw/an-editor.svg?branch=master)](https://travis-ci.org/hawkw/an-editor) [![codecov](https://codecov.io/gh/hawkw/an-editor/branch/master/graph/badge.svg)](https://codecov.io/gh/hawkw/an-editor) [![Clippy Linting Result](https://clippy.bashy.io/github/hawkw/an-editor/master/badge.svg)](https://clippy.bashy.io/github/hawkw/an-editor/master/log) [![Dependency Status](https://dependencyci.com/github/hawkw/an-editor/badge)](https://dependencyci.com/github/hawkw/an-editor) [![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/hawkw/an-editor/blob/master/LICENSE)

An text editor implemented in Rust.

# Modules

An editor is designed with modularity in mind. Thus, it consists of several modules which exist as independant crates:

## an-rope [![crates.io](https://img.shields.io/crates/v/an-rope.svg)](https://crates.io/crates/an-rope) [![RustDoc documentation](https://docs.rs/an-rope/badge.svg)](https://docs.rs/an-rope)

[`an-rope`](https://github.com/hawkw/an-editor/tree/master/an-rope) is an implementation of the Rope data structure for representing large text documents.
