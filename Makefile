TEST_FILTER:=

build:
	cargo build --release
	cp ./target/release/librun_jupyter.so lua/librun_jupyter.so

build-dev:
	cargo build
	cp ./target/debug/librun_jupyter.so lua/librun_jupyter.so

gitsubmodule:
	git submodule update --recursive

.PHONY: test
test:
	make cargo-test

.PHONY: cargo-test
cargo-test:
	  cargo nextest run ${TEST_FILTER}
