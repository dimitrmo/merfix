.PHONY: build
build:
	cargo build --release
	wasm-pack build --target web
	cp -rv pkg/* web

.PHONY: run
run: build
	npx http-server web 8000