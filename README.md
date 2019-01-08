# intel-tex
========

[![intel-tex on travis-ci.com](https://travis-ci.com/gwihlidal/intel-tex-rs.svg?branch=master)](https://travis-ci.com/gwihlidal/intel-tex-rs)
[![Latest version](https://img.shields.io/crates/v/intel-tex.svg)](https://crates.io/crates/intel-tex)
[![Documentation](https://docs.rs/intel-tex/badge.svg)](https://docs.rs/intel-tex)
[![](https://tokei.rs/b1/github/gwihlidal/intel-tex-rs)](https://github.com/gwihlidal/intel-tex-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![APACHE2](https://img.shields.io/badge/license-APACHE2-blue.svg)

Rust bindings for Intel's ISPC texture compression

* https://github.com/GameTechDev/ISPCTextureCompressor

State of the art texture compression for BC6H, BC7, ETC1, ASTC and BC1/BC3.

Requires the ISPC compiler to be installed: https://ispc.github.io/

## Supported compression formats:

* BC6H (FP16 HDR input)
* BC7
* ASTC (LDR, block sizes up to 8x8)
* ETC1
* BC1, BC3 (aka DXT1, DXT5)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
intel-tex = "0.1.0"
```

and add this to your crate root:

```rust
extern crate intel_tex;
```

## Example

```shell
cargo run --release --example main
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

Contributions are always welcome; please look at the [issue tracker](https://github.com/gwihlidal/intel-tex-rs/issues) to see what
known improvements are documented.

## Code of Conduct

Contribution to the intel-tex crate is organized under the terms of the
Contributor Covenant, the maintainer of intel-tex, @gwihlidal, promises to
intervene to uphold that code of conduct.