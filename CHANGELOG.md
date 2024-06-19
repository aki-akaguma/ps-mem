# Changelog: ps-mem

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`
* test-helper into `workspace`
* miri supports on tests
* `tarpaulin` supports into `Makefile`

### Changed
* rename: `config` to `config.toml`
* refactored `Makefile`
* update depends: anyhow(1.0.68)
* update depends: flood-tide(0.2.10), flood-tide-gen(0.1.20)
* update depends: linux-procfs(0.3.15)
* update depends: memx-cdy(0.1.12), runnel(0.3.17)
* update depends: assert-text(0.2.9), exec-taget(0.2.9), rust-version-info-file(0.1.9)
* update depends: indoc(2.0.0)

### Removed
* `COPYING`

### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`
* clippy: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`
* clippy: `uninlined_format_args`, `unused_imports`
* rust-version: "1.56.0" to "1.58.0"


## [0.2.13] (2023-01-11)
### Added
* badges into `README.tpl`
* rust-version = "1.56.0" into Cargo.toml

### Changed
* reformat `CHANGELOG.md`
* update depends: anyhow(1.0.68)
* update depends: flood-tide(0.2.8), flood-tide-gen(0.1.19)
* update depends: memx-cdy(0.1.10), runnel(0.3.15)
* update depends: regex(1.7.1)
* update depends: linux-procfs(0.3.13)
* update depends: num-format(0.4.4), signal-hook(0.3.14)

### Fixed
* clippy: needless_borrow
* clippy: uninlined_format_args

## [0.2.12] (2022-09-04)
### Added
* total sum print

### Changed
* update depends: flood-tide-gen(0.1.17)
* update depends: anyhow(1.0.63), libc(0.2.132), regex(1.6.0)
* update depends: semver(1.0.13)

### Fixed
* clippy: you are deriving `PartialEq` and can implement `Eq`

## [0.2.11] (2022-06-18)
### Changed
* changes to edition 2021
* update depends: cfg-iif(0.2.3), flood-tide(0.2.5), linux-procfs(0.3.11)
* update depends: memx(0.1.21), memx-cdy(0.1.8), naive_opt(0.1.18), runnel(0.3.11)
* update depends: assert-text(0.2.6), exec-target(v0.2.6), flood-tide-gen(0.1.16)
* update depends: rust-version-info-file(v0.1.6)
* update depends: semver(1.0.10)

## [0.2.10] (2022-05-31)
### Added
* command option: `-l, --cmdline` view command line

## [0.2.9] (2022-05-22)
### Changed
* update depends: runnel(0.3.10), memx(0.1.20)
* update depends: anyhow(1.0.57), libc(0.2.126), regex(1.5.6)
* update depends: signal-hook(0.3.14)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

## [0.2.8] (2021-11-15)
### Added
* more documents

### Changed
* minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)
* update depends: flood-tide(0.2.4), linux-procfs(0.3.10), cfg-iif(0.2.2), memx(0.1.18), memx-cdy(0.1.7), naive_opt(0.1.16), runnel(0.3.9)
* update depends: assert-text(0.2.59), exec-target(0.2.4), flood-tide-gen(0.1.15)
* update depends: anyhow(1.0.45), libc(0.2.107)

## [0.2.7] (2021-09-11)
### Changed
* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* update depends: signal-hook(0.3.10), linux-procfs(0.3.9)

## [0.2.6] (2021-07-21)
### Added
* depends: indoc(1.0.3)

### Changed
* update depends: linux-procfs(0.3.7), memx-cdy(0.1.4)
* update depends: memx(0.1.14), naive_opt(0.1.13)
* update depends: anyhow(1.0.42)

### Fixed
* bug: conf.opt_sleep: default sleep msec is more zero

## [0.2.5] (2021-07-03)
### Changed
* to github.com

## 0.2.4 (2021-07-03)
### Changed
* update depends: linux-procfs(0.3.6)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: assert-text(0.2.4), exec-target(0.2.3)
* minimum support rustc 1.46.0 (04488afe3 2020-08-24)

### Removed
* remove util::my_matches!()
* remove depends: rustc_version

## 0.2.3 (2021-06-26)
### Added
* `memx_cdy::memx_init(); // fast mem operation.`

### Changed
* update depends: cfg-iif(0.2.0)
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_ps-mem")`

### Fixed
* bug: `#[cfg(feature = "debian_build")]`

## 0.2.2 (2021-06-06)
### Added
* support `features = \["debian_build"\]`

### Changed
* update depends: flood-tide-gen(0.1.13), flood-tide(0.2.2)
* update depends: rust-version-info-file(0.1.2)

### Fixed
* bug: command option: -X rust-version-info

## 0.2.1 (2021-04-23)
### Changed
* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* update depends: bug fix: regex(1.4.6)

## 0.2.0 (2021-04-06)
### Added
* `runnel` and `anyhow`

### Changed
* change from optpa-util-1 to flood-tide
* update depends: rustc_version(0.3)

## 0.1.9 (2021-04-05)
### Changed
* update depends: signal-hook(0.3)

### Fixed
* fix clippy warning

## 0.1.8 (2020-12-29)
### Changed
* update crates

### Removed
* remove optpaerr-1

## 0.1.7 (2020-11-17)
### Added
* `README.md`, `COPYING`, `LICENSE-APACHE`, `LICENSE-MIT`

### Changed
* change optpa_util to optpa_util_1
* update crates

### Fixed
* fix old version: rustc_version(=0.2.3), v0.3.0 is not compile new semver on deb10-buster

## 0.1.6 (2020-10-19)
### Changed
* modify invoke one process output format
* update crates

## 0.1.5 (2020-10-09)
### Added
* command invoking
* command option: `--pid`

### Changed
* update crates

## 0.1.4 (2020-10-08)
### Added
* command option: `--all`

### Changed
* update crates

## 0.1.3 (2020-10-06)
### Changed
* chg: pid +1 columns, kB to Ki
* update crates

## 0.1.2 (2020-08-09)
### Changed
* update crates
* a lot of things

## 0.1.0 (2019-11-02)
* first commit

[Unreleased]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.13..HEAD
[0.2.13]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.12..v0.2.13
[0.2.12]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.11..v0.2.12
[0.2.11]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.10..v0.2.11
[0.2.10]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.9..v0.2.10
[0.2.9]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.8..v0.2.9
[0.2.8]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.7..v0.2.8
[0.2.7]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.6..v0.2.7
[0.2.6]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.5..v0.2.6
[0.2.5]: https://github.com/aki-akaguma/ps-mem/releases/tag/v0.2.5
