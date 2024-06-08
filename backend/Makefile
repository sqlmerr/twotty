run:
	docker compose up

run-build:
	docker compose up --build

run-test:
	docker compose -f docker-compose-test.yaml up -d

run-test-build:
	docker compose -f docker-compose-test.yaml up --build -d

test:
	cargo test -- --test-threads=1

stop-test:
	docker compose -f docker-compose-test.yaml down

format:
	cargo fmt

lint/check:
	cargo clippy

lint/fix:
	cargo clippy --fix --allow-dirty