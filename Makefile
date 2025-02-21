.PHONY: build
build:
	cargo build --release
	wasm-pack build --target web
	cp -rv pkg/* web
	cd web && npm install

.PHONY: run
run: build
	npx http-server web 8000

.PHONY: npm-publish
npm-publish:
	cd web && \
		npm publish --access public
