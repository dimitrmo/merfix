.PHONY: build
build:
	cargo build --release
	wasm-pack build --target web
	cd pkg && npm install

.PHONY: prod
prod: build

.PHONY: web
web:
	npx http-server public -p 8001

.PHONY: npm-publish
npm-publish:
	cd pkg && \
		npm publish  --provenance --access public
