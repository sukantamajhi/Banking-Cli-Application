.PHONY: dev build clean

dev:
	@cargo run -q

start:
	@cargo run --release -q


build:
	@cargo build -q

clean:
	@cargo clean -q
