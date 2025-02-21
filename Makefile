.PHONY: build
build:
	cargo build --release
	wasm-pack build --target web
	cp -rv pkg/* web
	cd web && npm install

.PHONY: prod
prod: build

.PHONY: web
web:
	npx http-server public -p 8001

.PHONY: npm-publish
npm-publish:
	cd web && \
		npm publish --access public
