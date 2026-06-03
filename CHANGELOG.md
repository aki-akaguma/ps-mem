# Changelog: ps-mem
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.3] - 2026-06-01

### Added
- Document comprehensive code review findings in `docs/reviews/2026-06-01_code_review.3.md`

### Changed
- Reorganize and rename previous code review files into `docs/reviews/` directory for better traceability
- Refactor `do_proc_in` in `src/run.rs` to use safe error handling instead of `unwrap()`, improving robustness against process state changes
- Improve error message for PID type conversion in `do_proc_invoke` for better clarity
- Refactor configuration helper methods in `src/conf/mod.rs` to use idiomatic iterator-based approaches for better readability
- Clean up commented-out code in `src/run.rs` and `src/conf/mod.rs`
- Enable tests in `src/util/opt_uc_x_param.rs`
- Include explicit `flush()` with error handling to `LineWriter` usage in `src/run.rs` to ensure all output is written

### Fixed
- Resolve `clippy::field_reassign_with_default` warning in `src/conf/parse.rs`
- Correct integration tests in `src/util/opt_uc_x_param.rs`

## [0.3.2] - 2026-05-26

### Changed
- Update dependencies: `linux-procfs` (0.4.2), `signal-hook` (0.4.4)

### Fixed
- Set default base directory to `/` in `CmdOptConf::base_dir()`

## [0.3.1] - 2026-05-20

### Changed
- Simplify sorting logic in `src/run.rs` using tuple comparison for better readability
- Refactor process data gathering in `do_proc_in` using functional iterators (`filter_map`, `collect`)
- Improve error handling in `do_proc_invoke` to return `anyhow::Result` instead of panicking on command start failure
- Use safe PID type conversion from `u32` to `i32` in `do_proc_invoke`
- Clarify `sioe` (Standard Input/Output/Error streams) definition in `src/lib.rs` documentation
- Clarify intent of default 10ms sleep interval in `src/conf/parse.rs`
- Update dependencies: `flood-tide` (0.2.14), `flood-tide-gen` (0.2.2), `runnel` (0.4.2), `regex` (1.12)
- Update `rustc` version to 1.71.0 in `.github/workflows/test-windows.yml`
- Set minimum supported Rust version to 1.68.0

### Fixed
- Resolve `clippy::needless_borrow` warning
- Resolve `clippy::derivable_impls` warning
- Resolve `clippy::unnecessary_sort_by` warning

### Removed
- `memx-cdy` dependency

## [0.3.0] - 2025-09-15

### Added
- Project specifications in `specs` directory

### Changed
- Implement `IntoIterator` compatibility for arguments in `execute()`
- Update dependencies: `runnel` (0.4.0), `rust-version-info-file` (0.2)

### Fixed
- Correct minimum supported version in documentation

### Removed
- `execute_env()` function

## [0.2.15] - 2024-06-19

### Changed
- Support Rust 1.65.0 in GitHub workflows
- Set `rust-version = "1.60.0"` in `Cargo.toml`

## [0.2.14] - 2024-06-19

### Added
- GitHub Actions workflows: `test-ubuntu.yml`, `test-macos.yml`, and `test-windows.yml`
- Include test status badges in `README.tpl`
- Include `test-helper` in the workspace
- Miri support for tests
- Tarpaulin support in `Makefile`

### Changed
- Rename `config` to `config.toml`
- Refactor `Makefile`
- Update dependencies: `anyhow` (1.0.86), `flood-tide` (0.2.11), `flood-tide-gen` (0.1.22), `linux-procfs` (0.3.16), `memx-cdy` (0.1.13), `runnel` (0.3.19), `assert-text` (0.2.10), `exec-target` (0.2.9), `rust-version-info-file` (0.1.10), `indoc` (2.0.5)

### Removed
- `COPYING` file

### Fixed
- Correct `LICENSE-APACHE` and `LICENSE-MIT` files
- Address clippy warnings: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`, `uninlined_format_args`, `unused_imports`
- Set `rust-version` to "1.58.0" (from "1.56.0")

## [0.2.13] - 2023-01-11

### Added
- Badges in `README.tpl`
- `rust-version = "1.56.0"` in `Cargo.toml`

### Changed
- Reformat `CHANGELOG.md`
- Update dependencies: `anyhow` (1.0.68), `flood-tide` (0.2.8), `flood-tide-gen` (0.1.19), `memx-cdy` (0.1.10), `runnel` (0.3.15), `regex` (1.7.1), `linux-procfs` (0.3.13), `num-format` (0.4.4), `signal-hook` (0.3.14)

### Fixed
- Address clippy warnings: `needless_borrow`, `uninlined_format_args`

## [0.2.12] - 2022-09-04

### Added
- Total sum output printing

### Changed
- Update dependencies: `flood-tide-gen` (0.1.17), `anyhow` (1.0.63), `libc` (0.2.132), `regex` (1.6.0), `semver` (1.0.13)

### Fixed
- Address clippy warning: derived `PartialEq` should also implement `Eq`

## [0.2.11] - 2022-06-18

### Changed
- Update project to Edition 2021
- Update dependencies: `cfg-iif` (0.2.3), `flood-tide` (0.2.5), `linux-procfs` (0.3.11), `memx` (0.1.21), `memx-cdy` (0.1.8), `naive_opt` (0.1.18), `runnel` (0.3.11), `assert-text` (0.2.6), `exec-target` (0.2.6), `flood-tide-gen` (0.1.16), `rust-version-info-file` (0.1.6), `semver` (1.0.10)

## [0.2.10] - 2022-05-31

### Added
- Command line view option: `-l, --cmdline`

## [0.2.9] - 2022-05-22

### Changed
- Update dependencies: `runnel` (0.3.10), `memx` (0.1.20), `anyhow` (1.0.57), `libc` (0.2.126), `regex` (1.5.6), `signal-hook` (0.3.14), `exec-target` (0.2.5), `rust-version-info-file` (0.1.5)

## [0.2.8] - 2021-11-15

### Added
- Comprehensive documentation

### Changed
- Set minimum supported Rust version to 1.47.0
- Update dependencies: `flood-tide` (0.2.4), `linux-procfs` (0.3.10), `cfg-iif` (0.2.2), `memx` (0.1.18), `memx-cdy` (0.1.7), `naive_opt` (0.1.16), `runnel` (0.3.9), `assert-text` (0.2.59), `exec-target` (0.2.4), `flood-tide-gen` (0.1.15), `anyhow` (1.0.45), `libc` (0.2.107)

## [0.2.7] - 2021-09-11

### Changed
- Pass `cargo clippy` checks
- Update dependencies: `anyhow` (1.0.43), `flood-tide-gen` (0.1.14), `flood-tide` (0.2.3), `memx-cdy` (0.1.6), `runnel` (0.3.8), `signal-hook` (0.3.10), `linux-procfs` (0.3.9)

## [0.2.6] - 2021-07-21

### Added
- Dependency: `indoc` (1.0.3)

### Changed
- Update dependencies: `linux-procfs` (0.3.7), `memx-cdy` (0.1.4), `memx` (0.1.14), `naive_opt` (0.1.13), `anyhow` (1.0.42)

### Fixed
- Resolve issue in `conf.opt_sleep`: default sleep msec should be greater than zero

## [0.2.5] - 2021-07-03

### Changed
- Migrate project to GitHub

## [0.2.4] - 2021-07-03

### Changed
- Update `linux-procfs` (0.3.6)
- Rewrite `TARGET_EXE_PATH` using `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
- Update dependencies: `assert-text` (0.2.4), `exec-target` (0.2.3)
- Set minimum supported Rust version to 1.46.0

### Removed
- `util::my_matches!()` macro
- `rustc_version` dependency

## [0.2.3] - 2021-06-26

### Added
- `memx_cdy::memx_init()` for fast memory operations

### Changed
- Update `cfg-iif` (0.2.0)
- Rewrite `TARGET_EXE_PATH` using `env!("CARGO_BIN_EXE_ps-mem")`

### Fixed
- Support for `#[cfg(feature = "debian_build")]`

## [0.2.2] - 2021-06-06

### Added
- Support for `features = ["debian_build"]`

### Changed
- Update dependencies: `flood-tide-gen` (0.1.13), `flood-tide` (0.2.2), `rust-version-info-file` (0.1.2)

### Fixed
- Support for command option: `-X rust-version-info`

## [0.2.1] - 2021-04-23

### Changed
- Update dependencies: `flood-tide-gen` (0.1.12), `flood-tide` (0.2.1)
- Resolve bug in `regex` (1.4.6)

## [0.2.0] - 2021-04-06

### Added
- `runnel` and `anyhow` dependencies

### Changed
- Switch from `optpa-util-1` to `flood-tide`
- Update `rustc_version` (0.3)

## [0.1.9] - 2021-04-05

### Changed
- Update `signal-hook` (0.3)

### Fixed
- Address clippy warnings

## [0.1.8] - 2020-12-29

### Changed
- Update internal crates

### Removed
- `optpaerr-1` dependency

## [0.1.7] - 2020-11-17

### Added
- `README.md`, `COPYING`, `LICENSE-APACHE`, and `LICENSE-MIT` files

### Changed
- Switch from `optpa_util` to `optpa_util_1`
- Update internal crates

### Fixed
- Pin `rustc_version` to 0.2.3 to avoid compilation issues with new `semver` on Debian 10 Buster

## [0.1.6] - 2020-10-19

### Changed
- Adjust output format for single process invocation
- Update internal crates

## [0.1.5] - 2020-10-09

### Added
- Command invocation support
- Command option: `--pid`

### Changed
- Update internal crates

## [0.1.4] - 2020-10-08

### Added
- Command option: `--all`

### Changed
- Update internal crates

## [0.1.3] - 2020-10-06

### Changed
- Expand PID column by 1 and change unit from KB to Ki
- Update internal crates

## [0.1.2] - 2020-08-09

### Changed
- Update internal crates
- Miscellaneous improvements

## [0.1.0] - 2019-11-02

### Added
- Initial release

[Unreleased]: https://github.com/aki-akaguma/ps-mem/compare/v0.3.3..HEAD
[0.3.3]: https://github.com/aki-akaguma/ps-mem/compare/v0.3.2..v0.3.3
[0.3.2]: https://github.com/aki-akaguma/ps-mem/compare/v0.3.1..v0.3.2
[0.3.1]: https://github.com/aki-akaguma/ps-mem/compare/v0.3.0..v0.3.1
[0.3.0]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.15..v0.3.0
[0.2.15]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.14..v0.2.15
[0.2.14]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.13..v0.2.14
[0.2.13]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.12..v0.2.13
[0.2.12]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.11..v0.2.12
[0.2.11]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.10..v0.2.11
[0.2.10]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.9..v0.2.10
[0.2.9]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.8..v0.2.9
[0.2.8]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.7..v0.2.8
[0.2.7]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.6..v0.2.7
[0.2.6]: https://github.com/aki-akaguma/ps-mem/compare/v0.2.5..v0.2.6
[0.2.5]: https://github.com/aki-akaguma/ps-mem/releases/tag/v0.2.5
