.PHONY: build
build:
	cargo build --release
	mkdir -p ./lua/deps/
	rm -f ./lua/norgberg.so
	cp ./target/release/libnorgberg.so ./lua/norgberg.so
    # if your Rust project has dependencies,
    # you'll need to do this as well
	cp ./target/release/deps/*.rlib ./lua/deps/

