default: test

build:
		cargo build --verbose
test: build
		cargo test --verbose
run:
		cargo run
