all: dist/imrefs

dist/imrefs: target/release/imrefs
	mkdir -p dist
	cp target/release/imrefs dist/imrefs

target/release/imrefs: src/main.rs
	cargo build --release

clean:
	cargo clean