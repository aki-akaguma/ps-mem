
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test
	cargo test -p test-helper

clean:
	cargo clean

clippy:
	cargo clippy --tests
