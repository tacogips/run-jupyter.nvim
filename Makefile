build:
	cargo build --release
	cp ./target/release/librun_jupyter.so lua/librun-jupyter.so

build-dev:
	cargo build
	cp ./target/debug/librun_jupyter.so lua/librun-jupyter.so

gitsubmodule:
	git submodule update --recursive
