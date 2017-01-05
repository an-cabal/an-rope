Contributing to An Editor
=========================

**Looking for a first issue?** You might want to start out by looking at [issues tagged "easy"](https://github.com/hawkw/an-editor/issues?q=is%3Aissue+is%3Aopen+label%3Aeasy). These are issues that, while important, will probably require less knowledge of Rust, less programming experience, and/or less knowledge of the `an-editor` codebase and might make good jumping-off points for potential contibutors.

Git Conventions
===============

Pull requests
-------------

In order to be accepted and merged, a pull request must meet the following conditions.

#### Pull requests MUST

+ Build successfully on [Travis](https://travis-ci.org/hawkw/an-editor)
+ Include RustDoc comments for any public-facing API functions or types
+ Include tests for any added features
+ Reference any closed issues with the text "Closes #XX" or "Fixes #XX" in the pull request description

#### Pull requests MUST NOT

+ Include any failing tests
+ Decrease overall project test coverage
+ Have any outstanding changes requested by a reviewer.

Commit messages
---------------

Commit messages should follow the [Angular.js Commit Message Conventions](https://github.com/conventional-changelog/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md). We use [`clog`](https://github.com/clog-tool/clog-cli) for automatically generating changelogs, and commit messages must be in a format that `clog` can parse.

It is recommended that contributors read the linked documentation for the Angular commit message convention in full –– it's not that long. For the impatient, here are some of the most important guidelines:

#### Commit messages MUST

+ Be in present tense
+ Follow the form `<type>(<scope>): <subject>`
    + where `<type>` is one of:
        * **feat**: A new feature
        * **fix**: A bug fix
        * **docs**: Documentation only changes
        * **style**: Changes that do not affect the meaning of the code (white-space, formatting, missing
        semi-colons, etc)
        * **refactor**: A code change that neither fixes a bug or adds a feature
        * **perf**: A code change that improves performance
        * **test**: Adding missing tests
        * **chore**: Changes to the build process or auxiliary tools and libraries such as documentation
        generation
    + and `<scope>` (optionally) specifies the specific element or component of the project that was changed.

#### Commit messages MUST NOT

+ Include lines exceeding 100 characters

#### Commit messages MAY

+ Include the text `[skip ci]` if changing non-Rustdoc documentation.
    + This will cause Travis CI to skip building that commit.
    + Commits which change RustDoc documentation in `.rs` source code files should still be built on CI -- `[skip ci]` should only be used for commits which change external documentation files such as `README.md`
    + Commits which change configuration files for tools not used by Travis may also skip the CI build, at the disgression of the committer.


Code Style
==========

Code committed to `an-editor` should conform to the [Rust style guidelines](https://doc.rust-lang.org/1.12.0/style/README.html) and to the ["Effective Rust" section](https://doc.rust-lang.org/book/effective-rust.html) of the Rust Book, whenever possible.

In particular, it should:
 + be indented with 4 spaces
 + not end files with trailing whitespace
 + follow the naming conventions in the Rust style guidelines

The following deviations from the style guidelines are _permitted_, but not required:

+ [Comma-first style](https://gist.github.com/isaacs/357981) _may_ be used for all comma-delimited constructs. For example:

    ```rust
    let a_list = [ a
                 , b
                 , c
                 ];
    ```

    and

    ```rust
    let a_list = [ a, b, c, d
                 , e, f, g, h
                 ];
    ```

    are considered good style.
+ When wrapping `where` clauses, the `where` clause _may_ be placed at the same indentation level as the corresponding `fn` or `impl` statement. For example:

    ```rust
    // Considered good style
    fn bar<A, B>(a: A) -> B
    where A: Something
        , B: Something + SomethingElse {
        ...
    }
    ```

    is considered good style.

## Tools to Assist With Coding Style

### EditorConfig

An [`.editorconfig` file](https://github.com/hawkw/an-editor/blob/master/.editorconfig) is available for [compatible text editors](http://editorconfig.org/#download). If the EditorConfig plugin is installed in your text editor, it will use this file to automatically configure certain formatting settings for the `an-editor` repository.

### rustfmt

[`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) is a tool for automatically formatting Rust source code according to style guidelines. This repository provides a `rustfmt.toml` file for automatically configuring `rustfmt` to use our style guidelines.

`rustfmt` may be installed by running

```bash
$ cargo install rustfmt
```

and invoked on a crate by running

```bash
$ cargo fmt
```

Additionally, there are `rustfmt` plugins [available](https://github.com/rust-lang-nursery/rustfmt#running-rustfmt-from-your-editor) for many popular editors and IDEs.

`rustfmt` may also be added as a [git pre-commit hook](https://git-scm.com/book/uz/v2/Customizing-Git-Git-Hooks) to ensure that all commits conform to the style guidelines.
