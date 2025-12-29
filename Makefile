.PHONY: all build run test clean install

all: build

build:
	cargo build --release

run: build
	qemu-system-x86_64 \
		-kernel target/x86_64-nateos/release/nateos \
		-serial stdio \
		-no-reboot \
		-no-shutdown

test:
	cargo test

clean:
	cargo clean

install: build
	@echo "Installation not yet implemented"

docs:
	cargo doc --no-deps

lint:
	cargo clippy -- -D warnings

format:
	cargo fmt

