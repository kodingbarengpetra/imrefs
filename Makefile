all: dist/imrefs

dist/imrefs: main.rs
	rustc -o dist/imrefs main.rs

clean:
	rm -f main