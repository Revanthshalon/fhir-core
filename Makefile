.PHONY: fmt clippy test doc feature-matrix coverage check

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-features --all-targets -- -D warnings

test:
	cargo test --all-features --verbose

doc:
	RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --all-features

feature-matrix:
	cargo clippy --no-default-features --features r4 --all-targets -- -D warnings
	cargo clippy --no-default-features --features r5 --all-targets -- -D warnings

coverage:
	cargo tarpaulin --all-features --engine llvm --skip-clean --out Lcov

check: fmt clippy test doc feature-matrix
