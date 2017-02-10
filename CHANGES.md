<a name="0.3.1"></a>
### 0.3.1 (2017-02-10)


#### Features

* **Rope:**  add fmt::Display to Rope ([63200aa9](https://github.com/hawkw/an-editor/commit/63200aa97e6bca8d348194bd28484df84dacc8e6))



<a name="0.3.0"></a>
## 0.3.0 (2017-01-14)


#### Breaking Changes

* **rope internals:**
  *  factor out metrics & add caching ([2b471c16](https://github.com/hawkw/an-editor/commit/2b471c168a8cfe9806b1f38a147d06c5170b7d16), closes [#57](https://github.com/hawkw/an-editor/issues/57), [#71](https://github.com/hawkw/an-editor/issues/71)

#### Performance

* **rope internals:**
  *  remove `format!` in `split` that was slow ([17890191](https://github.com/hawkw/an-editor/commit/17890191b12910f094b523c254542250153df3d2))
  *  factor out metrics & add caching ([2b471c16](https://github.com/hawkw/an-editor/commit/2b471c168a8cfe9806b1f38a147d06c5170b7d16), closes [#57](https://github.com/hawkw/an-editor/issues/57), [#71](https://github.com/hawkw/an-editor/issues/71), breaks [#](https://github.com/hawkw/an-editor/issues/))
* **rope metrics:**
  *  avoid constructing unneeded iterator in Line ([ddf76be0](https://github.com/hawkw/an-editor/commit/ddf76be02b1e730ac29f7a1ca9f333d802e1a356))
  
#### Features

* **node:**
  *  lazy fields can be gotten optionally ([6d59c224](https://github.com/hawkw/an-editor/commit/6d59c22450cbf1f9a29ce0e7200f41cf87134bd2))
  *  nicer fmt::Debug implementation for lazy fields ([dd0d8486](https://github.com/hawkw/an-editor/commit/dd0d8486aa28981859dfdb0b14f308a75d1a3a54))
  
#### Bug Fixes

* **node:**
  *  fix grammar in node metrics formatting ([9ea3f2ab](https://github.com/hawkw/an-editor/commit/9ea3f2ab49c08e3b332bc946e29795637590972c))
* **rope internals:**
  *  re-enable missing to_byte_index fn ([771b61de](https://github.com/hawkw/an-editor/commit/771b61de4ed693617589475824c189c4690add7d))
  *  make node debug formatting prettier _again_ ([d54281c5](https://github.com/hawkw/an-editor/commit/d54281c58dd18eaa1730a0bcb79e1080bfca421b))
  *  make node fmt::Debug implementations less wordy ([0ba69bfc](https://github.com/hawkw/an-editor/commit/0ba69bfc7fa7f9768837a0f8cfa837de3b620d78))
* **rope metrics:**
  *  fix usize overflow in Line metric ([2750f679](https://github.com/hawkw/an-editor/commit/2750f67996832252891e54c97a6273c10cc4aa89))
* **tendril:**  quick fix for Tendril metrics ([d9c0e14d](https://github.com/hawkw/an-editor/commit/d9c0e14dc39c003b0e26415b4343c71bffc1906c))



<a name="0.2.0"></a>
## 0.2.0 (2017-01-14)


#### Breaking Changes

* **rope:**  begin rewriting Rope to be persistent ([23055fc8](https://github.com/hawkw/an-editor/commit/23055fc82019567cc1727c8a62a2fa1d19fea476), breaks [#](https://github.com/hawkw/an-editor/issues/), [#](https://github.com/hawkw/an-editor/issues/))

#### Bug Fixes

* **atomic rope:**
  *  continued removal of Rc<AtomicTendril> ([c21794ec](https://github.com/hawkw/an-editor/commit/c21794ecd10285611b129e6174483866c4f694e0))
  *  fix atomic tendril ropes not using Arc ([b7f2c9b2](https://github.com/hawkw/an-editor/commit/b7f2c9b2798b6ad6e2be773026c90b178bb674b6))
* **rope:**
  *  disable move iters for now ([0cf80c69](https://github.com/hawkw/an-editor/commit/0cf80c697c29422a51c02801a7b587d228067ab6))
  *  fix Rope.delete() on unstable ([2a199e14](https://github.com/hawkw/an-editor/commit/2a199e143f28cca6d8dc4108043b68bcfbbc5f19))
  *  forcibly escort mutability from rope ([893c3b9f](https://github.com/hawkw/an-editor/commit/893c3b9fefbb7ab6f28bc34981dd517fe646b98a))
* **rope internals:**
  *  fix NodeLink ctor panicking on empty strings ([ec46f94a](https://github.com/hawkw/an-editor/commit/ec46f94a2f426d6346de9728caa628439aedf199))
  *  construct new branches from different types ([023aa85c](https://github.com/hawkw/an-editor/commit/023aa85cecb8f5c1d763f9bc883800f10267577d))
  *  fix incorrect feature flag syntax ([edeb6c7e](https://github.com/hawkw/an-editor/commit/edeb6c7eeb820e5e1dd7d24da5bbb45adfe4e7eb))
* **tendril:**
  *  fix issues in convert::From impls with tendrils ([3ab7e137](https://github.com/hawkw/an-editor/commit/3ab7e13740627b16deb0ce73b7bb7511ba8ef019))
  *  tendril support builds again ([3e585d70](https://github.com/hawkw/an-editor/commit/3e585d7011f754c1e023346404c48efe5af9a253), closes [#63](https://github.com/hawkw/an-editor/issues/63))

#### Performance

* **rope metrics:**  evaluate grapheme length lazily ([7750a396](https://github.com/hawkw/an-editor/commit/7750a3966da6c891419b810773983ec8619c1e56))

#### Features

* **rope:**
  * begin rewriting Rope to be persistent ([23055fc8](https://github.com/hawkw/an-editor/commit/23055fc82019567cc1727c8a62a2fa1d19fea476), breaks [#](https://github.com/hawkw/an-editor/issues/), [#](https://github.com/hawkw/an-editor/issues/))
  *  RopeSlices are back now (sort of) ([b92848bc](https://github.com/hawkw/an-editor/commit/b92848bce4dae457e6ea3e7bb0180f17070d174b))
  *  add feature flag for atomic ropes ([eec255e4](https://github.com/hawkw/an-editor/commit/eec255e4523aa5d1d6fa6b86d217018e0785144c))



<a name="0.1.2"></a>
### 0.1.2 (2017-01-09)


#### Bug Fixes

* **rope:**  depend on a static Clippy version to fix crates.io release ([78e3ced9](https://github.com/hawkw/an-editor/commit/78e3ced9201cdb8c989c8f447fc52eece15b6df5))





<a name="0.1.1"></a>
### 0.1.1 (2017-01-09)


#### Bug Fixes

* **rope:**  use only crates.io dependencies so we can publish ([9d9174b7](https://github.com/hawkw/an-editor/commit/9d9174b72207d37f12121e13b2a0f2e8a4fddda6))




<a name="0.1.0"></a>
## 0.1.0 (2017-01-09)


#### Bug Fixes

* **rope internals:**  hacky fix for Node::index ([0a66f825](https://github.com/hawkw/an-editor/commit/0a66f8251ec417743c00a839e54ab3845ff649c5))
* **rope metrics:**
  *  fix incorrect grapheme to byte index calculation ([104c5d4f](https://github.com/hawkw/an-editor/commit/104c5d4f430c3d79ef8195c8dbc930595f8fc5e4))
  *  fix nightly compatibility for parameteized metrics ([ac50697e](https://github.com/hawkw/an-editor/commit/ac50697e51355322325680517faa1582496e21b4))

#### Features

* **Rope:**
  *  add char metric ([f2f368e2](https://github.com/hawkw/an-editor/commit/f2f368e2d0f0f2ba6a8eeff43b9debe4e6b4ff33))
  *  add Rope.is_empty() function ([15ca3dc4](https://github.com/hawkw/an-editor/commit/15ca3dc420774ec8e3568c3a39bef1757485f3bd))
  *  derive Default implementation ([ea2e319b](https://github.com/hawkw/an-editor/commit/ea2e319b144f3e187b622e298d62ed4265d9e137))
* **RopeSlice:**  add RopeSlice.is_empty() / RopeSliceMut.is_empty() ([3f20c196](https://github.com/hawkw/an-editor/commit/3f20c1969e6727cdc6fcecff6d64ba1770105fcc))
* **rope metrics:**
  *  nicer fmt::Debug implementations for Metrics ([1c594c79](https://github.com/hawkw/an-editor/commit/1c594c79dbacc97fbfe659458a40ceff6aebc400))
  *  all rope methods can be parameterized w/ metrics ([1267c4e1](https://github.com/hawkw/an-editor/commit/1267c4e16ebd2bf6069d54059b148ad580ca3d6b))
  *  make metrics fmt::Debug ([3bffe54a](https://github.com/hawkw/an-editor/commit/3bffe54ad79b710863281b87e050b3d51cf386be))
  *  make Ropes measured ([795d7b75](https://github.com/hawkw/an-editor/commit/795d7b75ef2e561fee71ed64046d595d8959e019))
  *  parameterize existing methods with metrics! ([2e684259](https://github.com/hawkw/an-editor/commit/2e684259047ef2b6a9dbfdc96ace3bcadad26142))
  *  make metrics user-selectable ([3b3dcd8d](https://github.com/hawkw/an-editor/commit/3b3dcd8d12634da92b20f2e24acbba7421aa24e4))


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
