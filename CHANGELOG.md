# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].


## 0.4.0 (2025-06-10)


### ⭐ Added

- `ConvertAngle` and `FromOther` traits, to facilitate easier conversions


### 🔧 Changed

- Use faster, simpler math for many conversions


### 🔥 Removed

- Generic implemention of `From` for creating `Angle` from numerics (use explicit constructors or implicit conversions instead)


### 🐛 Fixed

- Certain types of generic conversion could not be compiled



## 0.3.0 (2025-06-10)


### ⭐ Added

- `Angle` now implements `Default`
- Manually replicated core documentation in README



## 0.2.0 (2025-06-09)


### ⭐ Added

- This changelog
- Full [no_std](https://docs.rust-embedded.org/book/intro/no-std.html) support
- Minor improvements to documentation and README file



## 0.1.0 (2025-06-07)


- Initial release, based on code from my larger coordinate library project



[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
