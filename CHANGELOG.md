TBD
===
Unreleased changes. Release notes have not yet been written.

0.2.9 (2022-05-22)
=====

* update depends: runnel(0.3.10), memx(0.1.20)
* update depends: anyhow(1.0.57), libc(0.2.126), regex(1.5.6)
* update depends: signal-hook(0.3.14)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

0.2.8 (2021-11-15)
=====

* minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)
* add more documents
* update depends: flood-tide(0.2.4), linux-procfs(0.3.10), cfg-iif(0.2.2), memx(0.1.18), memx-cdy(0.1.7), naive_opt(0.1.16), runnel(0.3.9)
* update depends: assert-text(0.2.59), exec-target(0.2.4), flood-tide-gen(0.1.15)
* update depends: anyhow(1.0.45), libc(0.2.107)

0.2.7 (2021-09-11)
=====

* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* update depends: signal-hook(0.3.10), linux-procfs(0.3.9)

0.2.6 (2021-07-21)
=====

* fix bug: conf.opt_sleep: default sleep msec is more zero
* add depends: indoc(1.0.3)
* update depends: linux-procfs(0.3.7), memx-cdy(0.1.4)
* update depends: memx(0.1.14), naive_opt(0.1.13)
* update depends: anyhow(1.0.42)

0.2.5 (2021-07-03)
=====

* to github.com

0.2.4 (2021-07-03)
=====

* update depends: linux-procfs(0.3.6)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: assert-text(0.2.4), exec-target(0.2.3)
* minimum support rustc 1.46.0 (04488afe3 2020-08-24)
* remove util::my_matches!()
* remove depends: rustc_version

0.2.3 (2021-06-26)
=====

* update depends: cfg-iif(0.2.0)
* add `memx_cdy::memx_init(); // fast mem operation.`
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_ps-mem")`
* bug fix: `#[cfg(feature = "debian_build")]`

0.2.2 (2021-06-06)
=====

* update depends: flood-tide-gen(0.1.13), flood-tide(0.2.2)
* update depends: rust-version-info-file(0.1.2)
* add support features = \["debian_build"\]
* bug fix command option: -X rust-version-info

0.2.1 (2021-04-23)
=====

* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* update depends: bug fix: regex(1.4.6)

0.2.0 (2021-04-06)
=====

* change from optpa-util-1 to flood-tide
* add runnel and anyhow
* update depends: rustc_version(0.3)

0.1.9 (2021-04-05)
=====

* fix clippy warning
* update depends: signal-hook(0.3)

0.1.8 (2020-12-29)
=====

* update crates
* remove optpaerr-1

0.1.7 (2020-11-17)
=====

* fix old version: rustc_version(=0.2.3), v0.3.0 is not compile new semver on deb10-buster
* change optpa_util to optpa_util_1
* add README.md, COPYING, LICENSE-APACHE, LICENSE-MIT
* update crates

0.1.6 (2020-10-19)
=====

* modify invoke one process output format
* update crates

0.1.5 (2020-10-09)
=====

* add command invoking
* add flag: --pid
* update crates

0.1.4 (2020-10-08)
=====

* add flag: --all
* update crates

0.1.3 (2020-10-06)
=====

* chg: pid +1 columns, kB to Ki
* update crates

0.1.2 (2020-08-09)
=====

* update crates
* a lot of things

0.1.0 (2019-11-02)
=====
first commit
