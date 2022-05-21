default: test

test:
		cargo build --verbose
		cargo test --verbose

run:
		cargo run
