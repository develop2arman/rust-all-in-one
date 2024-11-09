.PHONY: help clean build lint fmt bump  

# Help command to display available targets and their descriptions  
help: ## Show this help message  
	@echo "Available commands:"  
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'  

clean: ## Clean the project using cargo  
	cargo clean  

build: ## Build the project using cargo  
	cargo build  

lint: ## Lint the project using cargo-clippy  
	@if ! rustup component list --installed | grep -q clippy; then \
		echo "Installing clippy component..."; \
		rustup component add clippy; \
	fi  
	cargo clippy  

fmt: ## Format the project using rustfmt  
	@if ! rustup component list --installed | grep -q rustfmt; then \
		echo "Installing rustfmt component..."; \
		rustup component add rustfmt; \
	fi  
	cargo fmt  

bump: ## Bump the version number  
	@current_version=$$(grep 'version =' Cargo.toml | sed -E 's/.*= "([0-9]+\.[0-9]+\.[0-9]+)".*/\1/'); \
	echo "Current version is $$current_version"; \
	read -p "Enter new version number (Leave empty to cancel): " version; \
	if [ -z "$$version" ]; then \
		echo "Version bump cancelled."; \
		exit 1; \
	fi; \
	sed -i.bak -E "s/version = \"[0-9]+\.[0-9]+\.[0-9]+\"/version = \"$$version\"/" Cargo.toml; \
	rm Cargo.toml.bak; \
	echo "New version is $$version"  

.PHONY : setup
setup :
	rustup component add rustfmt
	rustup component add clippy


.PHONY : check-clippy
check-clippy :
	cargo clippy --workspace --all-targets --all-features -- \
			-D warnings \
			-A clippy::upper-case-acronyms

.PHONY : build
build :
	cargo build

.PHONY : setup
setup :
	rustup component add rustfmt
	rustup component add clippy



.PHONY : install-mdbook
check-fmt :
	cargo install mdbook


.PHONY : test
test :
	@cargo test --workspace --lib
	@cargo test --workspace --doc
	@cargo test --test codegen task_codegen
	@cargo test --no-run --test codegen app_codegen
	@cargo test --no-run --test codegen beat_codegen

.PHONY : broker-tests
broker-tests :
	@cargo test --test integrations brokers::amqp
	@cargo test --test integrations brokers::redis

.PHONY : run-all-tests
run-all-tests :
	@cargo test --workspace --lib
	@cargo test --workspace --doc
	@cargo test --test codegen task_codegen
	@cargo test --no-run --test codegen app_codegen
	@cargo test --no-run --test codegen beat_codegen	
	@cargo test --test integrations brokers::amqp
	@cargo test --test integrations brokers::redis	

.PHONY : test-docs
build-docs :
	cargo test --doc  --workspace

.PHONY : build-docs
build-docs :
	mdbook build
	cargo doc --workspace --message-format short --no-deps --color always
