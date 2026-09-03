.PHONY: fmt clippy test doc coverage check

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-features --all-targets -- -D warnings

test:
	cargo test --all-features --verbose

doc:
	RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --all-features

coverage:
	cargo tarpaulin --all-features --engine llvm --skip-clean

check: fmt clippy test doc
