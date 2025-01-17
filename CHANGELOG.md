# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.8.0] - 2023-06-18
### :sparkles: New Features
- [`301ceea`](https://github.com/JakeStanger/corn/commit/301ceea0c4dafffd38f4688fcf1df6d7bfdb448b) - **parser**: add hex and underscore separator support to integers. *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`53afac7`](https://github.com/JakeStanger/corn/commit/53afac74dce229f57873e1af3edd6e86cb793ce7) - **parser**: add escape char, interpolation support to strings *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :bug: Bug Fixes
- [`d51bf5a`](https://github.com/JakeStanger/corn/commit/d51bf5abd4a6ad2b86cb8b1e8a5fd3a65ac0ea30) - **parser**: crash when spreading invalid type *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :recycle: Refactors
- [`5d2b7c8`](https://github.com/JakeStanger/corn/commit/5d2b7c85ecb6431a1f2cebf39366024224e389c1) - remove no longer required `TomlValue` struct *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`dcffab6`](https://github.com/JakeStanger/corn/commit/dcffab62803d3b4c8ccb91da219781cf673765fe) - **cli**: reduce duplicate code *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`047d1d3`](https://github.com/JakeStanger/corn/commit/047d1d3f9cc037ccf3827ccc264b50e5e0536662) - **parser**: minor env var performance improvement *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`42fa830`](https://github.com/JakeStanger/corn/commit/42fa830d0bd2f9fefc86b8e841c3abbbd0fe68f2) - fix clippy warning *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :white_check_mark: Tests
- [`c7253db`](https://github.com/JakeStanger/corn/commit/c7253dbef9782c8d85cd1b285112532da653207e) - fix invalid spread test input *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`0ec37d8`](https://github.com/JakeStanger/corn/commit/0ec37d871742ab46c5c3b3ad3732c0444413f839) - add benchmarking *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`693c91e`](https://github.com/JakeStanger/corn/commit/693c91ec50aca1004b79b74b30313cd798d3cfac) - add coverage for float exponent syntax *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`de57c71`](https://github.com/JakeStanger/corn/commit/de57c71b9713307ce0dbed47c4d7572bf71eb116) - **invalid spread**: add case for array spread *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :memo: Documentation Changes
- [`e760b8c`](https://github.com/JakeStanger/corn/commit/e760b8ceaf2428c691ff6a8abfc6e92cd610c02e) - update CHANGELOG.md for v0.7.0 [skip ci] *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`c4cd6a8`](https://github.com/JakeStanger/corn/commit/c4cd6a8218c09480c22f151c01e4f8b888c6fa7b) - **readme**: add nvim section *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`6f5f066`](https://github.com/JakeStanger/corn/commit/6f5f06683a448c765bb64cc59a9fc08b16e20762) - **readme**: update to cover new features *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`749fe37`](https://github.com/JakeStanger/corn/commit/749fe379adc4c02865f756ef8d29641a0e8ba185) - add landing page readmes for crates *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v0.7.0] - 2023-05-24
### :sparkles: New Features
- [`48304d4`](https://github.com/JakeStanger/corn/commit/48304d4d809c1bcb3fdedfdffe3377952ca2a767) - spread operator *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :recycle: Refactors
- [`ab1af29`](https://github.com/JakeStanger/corn/commit/ab1af29219dc82ce86e32eb466a9d24ee6f195b6) - improve error handling and code quality *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :memo: Documentation Changes
- [`2be933d`](https://github.com/JakeStanger/corn/commit/2be933dc949b4357b9938643f3abe3ff22f33e39) - update CHANGELOG.md for v0.6.1 [skip ci] *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`17a99d2`](https://github.com/JakeStanger/corn/commit/17a99d2d0939b4fb0e197b2ad061fe13dfdb5bb2) - **readme**: correct a few bits, add more detail *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v0.6.1] - 2022-12-12
### :bug: Bug Fixes
- [`b6e93b2`](https://github.com/JakeStanger/corn/commit/b6e93b202d961f51ce6c92c58a9ed30111a820af) - **lib**: deserializer not handling invalid inputs *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :recycle: Refactors
- [`21e1ee0`](https://github.com/JakeStanger/corn/commit/21e1ee03cb3e81ea1e8dd97fd300fbb12fcb8341) - tidy error handling *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :memo: Documentation Changes
- [`9dbb9d6`](https://github.com/JakeStanger/corn/commit/9dbb9d6dc3dc018f47f38b270d31cadc2406d8be) - update CHANGELOG.md for v0.6.0 [skip ci] *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v0.6.0] - 2022-11-28
### :sparkles: New Features
- [`7a2f7b5`](https://github.com/JakeStanger/corn/commit/7a2f7b5a961689413ccc8f9b1fb75f998ceebac8) - **de**: `from_slice` func *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :bug: Bug Fixes
- [`e6c8e90`](https://github.com/JakeStanger/corn/commit/e6c8e901ac87d01137cd06e4317cf009e7325e59) - **de**: from_str panicking instead of returning result *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`7ea024d`](https://github.com/JakeStanger/corn/commit/7ea024d047862b89c57b78cb8480009514221d24) - **parser**: panic when input references another input *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v0.5.0] - 2022-11-27
### :sparkles: New Features
- [`9fbf1b0`](https://github.com/JakeStanger/corn/commit/9fbf1b0c9ca53c14f787a997bbb067d918142b24) - serde deserialization support *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :white_check_mark: Tests
- [`d035fa2`](https://github.com/JakeStanger/corn/commit/d035fa2fd92a5e62081b7d51a56d63222bb6e73e) - update test assets *(commit by [@JakeStanger](https://github.com/JakeStanger))*


[v0.5.0]: https://github.com/JakeStanger/corn/compare/v0.4.0...v0.5.0
[v0.6.0]: https://github.com/JakeStanger/corn/compare/v0.5.0...v0.6.0
[v0.6.1]: https://github.com/JakeStanger/corn/compare/v0.6.0...v0.6.1
[v0.7.0]: https://github.com/JakeStanger/corn/compare/v0.6.1...v0.7.0
[v0.8.0]: https://github.com/JakeStanger/corn/compare/v0.7.0...v0.8.0