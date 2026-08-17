ifneq (,$(wildcard ./.env))
    include .env
    export
endif

RUST_VERSION := $(shell grep 'channel' rust-toolchain.toml | head -1 | sed 's/.*"//;s/"//')

export RUST_VERSION
export UID := $(shell id -u)
export GID := $(shell id -g)

.DEFAULT_GOAL := run
.PHONY: run clean fmt lint migration machete clean-db db setup down seed

setup:
	docker compose build

run:
	docker compose up

down:
	docker compose down

seed:
	cargo run --bin seeder

clean:
	docker compose down -v

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

migration:
	sqlx migrate add --source ./config/migrations "$(name)"

machete:
	cargo machete
