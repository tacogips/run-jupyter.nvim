TEST_FILTER:=

build:
	cargo build --release
	cp ./target/release/librun_jupyter.so lua/librun-jupyter.so

build-dev:
	cargo build
	cp ./target/debug/librun_jupyter.so lua/librun-jupyter.so

gitsubmodule:
	git submodule update --recursive

.PHONY: test
test:
	make test-jupyter-up
	make cargo-test

.PHONY: test-jupyter-up
test-jupyter-up:
	docker compose down
	docker compose up --detach --quiet-pull --wait

.PHONY: cargo-test
cargo-test:
	  cargo nextest run ${TEST_FILTER}
