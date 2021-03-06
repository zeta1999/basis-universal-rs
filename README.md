# basis-universal-rs

Bindings for Binomial LLC's [`basis-universal`](https://github.com/BinomialLLC/basis_universal) Supercompressed GPU 
Texture Codec

`basis-universal` functionality can be grouped into two categories:
 * Encoding: Compresses and encode textures (optionally combining multiple images and mipmap layers in a single 
   file/binary blob)
 * Transcoding: Unpacks the texture into GPU-friendly compression formats. The final format can be chosen based on what
   the available GPU hardware can support.

Encoding can be done ahead of time using a command line tool in the [upstream repository](https://github.com/BinomialLLC/basis_universal).

The encoded data can either be stored as a file or a binary blob. This data can include multiple images, and each image 
can store multiple levels. This is commonly used for storing cube textures and textures with precomputed mipmaps. This
library also supports generating mipmaps for you.

Please refer to https://github.com/BinomialLLC/basis_universal for more details.

## Status

These bindings should be pretty close to production ready. However, I want to take a few weeks to road test it with my
rendering framework before releasing 0.1.0. This crate will not follow semver until 0.1.0 has been published.

## Performance

The C++ code that is wrapped by these bindings is built at the same optimization level used for building the 
`basis-universal-sys` crate. Most users of these bindings will benefit from building this crate with optimizations
enabled, even during development. To do that, place this in your Cargo.toml file.

```
[profile.dev.package."basis-universal-sys"]
opt-level = 3
```

Enable optimizations for ALL upstream crates works too:

```
[profile.dev.package."*"]
opt-level = 3
```

## License

The bindings are licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Upstream Dependencies

Some dependencies may be licensed under other terms. These licenses include "ISC", "CC0-1.0", "BSD-2-Clause",
"BSD-3-Clause", and "Zlib". This is validated on a best-effort basis in every CI run using cargo-deny.

Binomial LLC's `basis-universal` Supercompressed GPU Texture Codec is licensed under Apache License 2.0:

> Copyright (C) 2019-2020 Binomial LLC. All Rights Reserved.
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
> http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
