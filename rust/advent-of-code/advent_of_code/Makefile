.PHONY: check
check: ## Run `cargo check`
	cargo check

.PHONY: clippy
clippy: ## Run `cargo clippy`, the Rust linter.
	cargo clippy

.PHONY: fmt
fmt: ## Run `cargo fmt` (convenience so that you can write `make fmt check test clippy` in one command)
	cargo fmt

.PHONY: test
test: ## Run tests [TEST=test_name (optional)]
	cargo test $$TEST

.PHONY: watch-local
watch-local: ## Cargo watch
	cargo watch -x help -x check

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-z.A-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
