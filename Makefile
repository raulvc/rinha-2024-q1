.PHONY: load-test

lint-fix:
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged

build:
	cargo build --release
	docker build . -t raulvc/rinha-app

publish:
	docker push raulvc/rinha-app

db-build:
	cd db && docker build . -t raulvc/rinha-libsql-server

db-publish:
	docker push raulvc/rinha-libsql-server

gatling-build:
	cd load-test && docker build . -t raulvc/rinha-gatling

gatling-publish:
	docker push raulvc/rinha-gatling

load-test:
	@./load-test/run_tests.sh
