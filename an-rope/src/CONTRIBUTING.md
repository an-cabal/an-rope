Contributing to An Editor
=========================

Merging Pull Requests
---------------------

In order to be accepted and merged, a pull request must meet the following conditions.

#### Pull requests MUST

+ Build successfully on [Travis](https://travis-ci.org/hawkw/an-editor)
+ Include RustDoc comments for any public-facing API functions or types
+ Include tests for any added features
+ Reference any closed issues with the text "Closes #XX" or "Fixes #XX" in the pull requet description

#### Pull requests MUST NOT

+ Include any failing tests
+ Decrease overall project test coverage
+ Have any outstanding changes requested by a reviewer.
