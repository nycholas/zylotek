.PHONY: setup prek prek-all prek-install build  \
        clean help

# =============================================================================
# Setup
# =============================================================================

setup:
	cargo install --locked prek sccache cargo-llvm-cov cargo-fuzz cargo-bench \
	cargo-audit cargo-outdated

# =============================================================================
# Pre-commit hooks
# =============================================================================

# Run prek on staged files
prek:
	prek run

# Run prek on all files
prek-all:
	prek run --all-files

# Install prek git hooks
prek-install:
	prek install

# =============================================================================
# Helpers
# =============================================================================

migrate:
	cargo run -p zylotek-cli -- db migrate

# =============================================================================
# Build targets
# =============================================================================

# Development build (fast compile, debug symbols)
build:
	cargo build -p zylotek-cli

# =============================================================================
# Help
# =============================================================================

help:
	@echo "Zylotek System"
	@echo ""
	@echo "Setup:"
	@echo "  make setup             - Initial setup"
	@echo ""
	@echo "Build targets:"
	@echo "  make build             - Development build (fast)"
	@echo ""
