readme:
	cargo readme > README.md

test:
	cargo test tests
	cargo test --release --features debug debug
	cargo test --features debug debug

publish:
	cargo release --dry-run -vv