#!/bin/sh

set -e

mkdir -p /app/target /home/appuser/.cargo
chown -R appuser:appuser /app/target /home/appuser/.cargo

export HOME=/home/appuser
export CARGO_HOME=/home/appuser/.cargo

exec gosu appuser:appuser \
	sh -c 'cd /app/apps/ldapi && exec cargo watch -x "run -p ldapi --bin ldapi" -w src -w config'
