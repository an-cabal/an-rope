<a name="0.0.3"></a>
### 0.0.3 (2017-01-05)


#### Features

* **RopeSlice:**  make RopeSlice fmt::Debug make sense ([ad1f2fa8](https://github.com/hawkw/an-editor/commit/ad1f2fa8c0c1bbb1d52ae95d07d1fe56b3d90e9b))
* **rope:**
  *  split strings when creating a rope ([97221909](https://github.com/hawkw/an-editor/commit/97221909108f7faa6ccd69daf273b6ac9d6d4595))
  *  line indexing WIP ([8973aa8b](https://github.com/hawkw/an-editor/commit/8973aa8be8331e43a2330d1e08c249b959f47d29))
  *  More work on line indexing ([f47579a6](https://github.com/hawkw/an-editor/commit/f47579a67e8752f9c4f914a6bdb2f46a13c24f0c))
  *  some WIP for line counting/indexing ([d5d900f3](https://github.com/hawkw/an-editor/commit/d5d900f367800097c5e0adade3fb933cebf9e8ea))
  *  grapheme-boundary indexing quasi-working ([9cedc86b](https://github.com/hawkw/an-editor/commit/9cedc86bd735b6db51df344643f7298cbe6accf7))
  *  grapheme-boundary indexing compiles ([def3c473](https://github.com/hawkw/an-editor/commit/def3c473d0aa420a1f42ccf8db4b5bddaf37cdf8))
  *  implemented measuring for all BranchNode ([949dac01](https://github.com/hawkw/an-editor/commit/949dac01d2f544b1711a3cb08b0ba19f437b548c))
  *  start on grapheme metric for ropes ([906e5185](https://github.com/hawkw/an-editor/commit/906e5185e6245d8b1c5744106204203aaee0a87e))
  *  first pass on Rope metrics ([6c8aa1ef](https://github.com/hawkw/an-editor/commit/6c8aa1ef4cfa1c3f85c9561a2d6856fb1cfa91f7))
  *  start on grapheme metric for ropes ([60efa302](https://github.com/hawkw/an-editor/commit/60efa30299772d3ba092d9eade3ffc9f748b9d8d))
  *  first pass on Rope metrics ([f3c88551](https://github.com/hawkw/an-editor/commit/f3c88551c17f8e73e05a3b1093de4806763a5d15))
  *  start on helper trait for unicode indexing ([0de17568](https://github.com/hawkw/an-editor/commit/0de17568fc599c567a16b16467636ceaa3ad34dc))
  *  add unicode iterators to Rope ([f75674f2](https://github.com/hawkw/an-editor/commit/f75674f2583c595ca8d8739d279145ea91fe0bf2))
  *  add unimplemented grapheme_indices() iterator ([d166f583](https://github.com/hawkw/an-editor/commit/d166f583bf8982abe08c6fb16648a25b180b3497))
* **rope metrics:**
  *  final pass on metric API ([d0775049](https://github.com/hawkw/an-editor/commit/d0775049f2ebbe052060b5b333800afdbd54f5c6))
  *  replace semigroup with ops::Add ([93a12951](https://github.com/hawkw/an-editor/commit/93a1295148f91d071a2828c91671d7b7a35c87e9))
  *  monoid uses `default()` for identity ([fd049bed](https://github.com/hawkw/an-editor/commit/fd049bed2121c4be92952fbfd129af5f86a319ca))
  *  final pass on metric API ([b1958d9f](https://github.com/hawkw/an-editor/commit/b1958d9f740e308f7c230143362c3b58ffa59b7b))
  *  replace semigroup with ops::Add ([4dc1c4a5](https://github.com/hawkw/an-editor/commit/4dc1c4a5d9a19fb8c898293b76ca0d76e7cabc7c))
  * change Metric API slightly ([44e6d399](https://github.com/hawkw/an-editor/commit/44e6d3996444b75b55755b614db2400256a99e6c))
  *  add Ordering to Metric ([c1452332](https://github.com/hawkw/an-editor/commit/c145233219c02905a448e0316784f585a8943b10))
* **unicode:**
  *  more work on grapheme indexing, still WIP ([a95cf380](https://github.com/hawkw/an-editor/commit/a95cf380866f0c2a69b4e2a4d23c4d019a816f0e))
  *  ropes indexed using grapheme indices ([3a45e749](https://github.com/hawkw/an-editor/commit/3a45e749dde027a5d4022f22dd147b3281e966a1))
  *  rewrote conversion of grapheme indices ([da5b6e62](https://github.com/hawkw/an-editor/commit/da5b6e62bbd4320ead772b0b93e2217d37c3b80e))
  *  depend on unicode-segmentation crate ([8d988d3c](https://github.com/hawkw/an-editor/commit/8d988d3cafb6245ee69cfebb79d7a588cca532eb))
  *  more work on grapheme indexing, still WIP ([18a1c84f](https://github.com/hawkw/an-editor/commit/18a1c84f28f506e017cb56f223597e26595e152a))
  *  ropes indexed using grapheme indices ([7dcf591e](https://github.com/hawkw/an-editor/commit/7dcf591e2ad1df0a4fa68c816c5411466e6a7e48))
  *  rewrote conversion of grapheme indices ([21f1bf55](https://github.com/hawkw/an-editor/commit/21f1bf55c57cd128c0345f3500de1c2eb0365158))
  *  depend on unicode-segmentation crate ([f162ac39](https://github.com/hawkw/an-editor/commit/f162ac39b2664fc50310448c01698e5612dd72f0))
  *  start on helper trait for unicode indexing ([625748eb](https://github.com/hawkw/an-editor/commit/625748ebc33bee9e860935c556b062c220422b98))
  *  add unicode iterators to Rope ([6c409478](https://github.com/hawkw/an-editor/commit/6c4094788f4ad4c8dd00355470980a1223d3fe7e))
  *  add unimplemented grapheme_indices() iterator ([7bb8a6f9](https://github.com/hawkw/an-editor/commit/7bb8a6f905137d35434c2df46a45503f3f102b4a))

#### Bug Fixes

* **RopeSlice:**  rope slices can now equal things ([e0f6c64a](https://github.com/hawkw/an-editor/commit/e0f6c64a579971d566d34c2a7031ba6eeb23f7df))
* **rope:**
  *  fix Totally Incorrect grapheme byte index calculation ([f35e4284](https://github.com/hawkw/an-editor/commit/f35e4284bf7f9defc9786cdf66fabda735d432d7))
  *  fix some issues in Rope.delete() on nightly ([72ee73bd](https://github.com/hawkw/an-editor/commit/72ee73bd6b4daa70e9bbb6bb2ec313fd25d27e8b))
  *  fix issue where tests failed due to changed panic message ([d607cc9f](https://github.com/hawkw/an-editor/commit/d607cc9f176824d72df4a19bb5bbdd75395f608e))
  *  fix issue where delete didn't work on nightly ([282d8d12](https://github.com/hawkw/an-editor/commit/282d8d1246fa9b290514c01f8ece44e836c351f5))
  *  fix Rope.lines() iterator not containing last line ([a707a63a](https://github.com/hawkw/an-editor/commit/a707a63aea270c23d53ef7a49a067e982642ebc9))
  *  first pass on new Lines iterator for Rope ([6966f65f](https://github.com/hawkw/an-editor/commit/6966f65f9d6e6f8e04a409675d6056a5879b7eb9), closes [#31](https://github.com/hawkw/an-editor/issues/31))
  *  fix indexing off by one error ([f18e9b0a](https://github.com/hawkw/an-editor/commit/f18e9b0a6720b12767c7faf5ee33796aea6592ed))
  *  fix stable rust support for new unicode methods ([892bbc1f](https://github.com/hawkw/an-editor/commit/892bbc1fc830975ffb1efc04619fb31ebeacee20))
  *  fix stable rust support for new unicode methods ([ac1cbe80](https://github.com/hawkw/an-editor/commit/ac1cbe8063ae5f0311e0a9bad0b77a334e429cbe))
* **unicode:**  make grapheme indexing actually work ([febdf16f](https://github.com/hawkw/an-editor/commit/febdf16fb0b6721cef0d59f3fe3d1ca3c94b4f2d))
