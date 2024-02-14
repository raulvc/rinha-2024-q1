lint-fix:
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged

build:
	cargo build --release

db-build:
	cd db && docker build . -t raulvc/rinha-libsql-server

db-publish:
	docker push raulvc/rinha-libsql-server