# Makefile for Rummage Documentation

DOCS_DIR := .
MARKDOWN_FILES := $(shell find $(DOCS_DIR) -name "*.md")
OUTPUT_DIR := ../target/docs
MDBOOK_BIN := mdbook
MARKDOWNLINT_BIN := markdownlint

.PHONY: all build lint toc check clean serve help

all: lint build ## Perform linting and build documentation

help: ## Display this help message
	@echo "Rummage Documentation Makefile"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the documentation using mdbook
	@echo "Building documentation with mdbook..."
	@mkdir -p $(OUTPUT_DIR)
	@$(MDBOOK_BIN) build -d $(OUTPUT_DIR)

lint: ## Lint markdown files
	@echo "Linting markdown files..."
	@$(MARKDOWNLINT_BIN) $(MARKDOWN_FILES) || echo "Linting issues found."

toc: ## Generate table of contents for each document
	@echo "Generating table of contents for markdown files..."
	@for file in $(MARKDOWN_FILES); do \
		echo "Processing $$file..."; \
		markdown-toc -i $$file || echo "TOC generation failed for $$file"; \
	done

check: ## Check for broken links in documentation
	@echo "Checking for broken links..."
	@$(MDBOOK_BIN) test

clean: ## Clean build artifacts
	@echo "Cleaning documentation build artifacts..."
	@rm -rf $(OUTPUT_DIR)

serve: ## Serve documentation locally
	@echo "Starting local documentation server..."
	@$(MDBOOK_BIN) serve -p 3000

# Install required tools
install-tools: ## Install required tools for documentation
	@echo "Installing documentation tools..."
	@cargo install mdbook
	@cargo install mdbook-linkcheck
	@npm install -g markdownlint-cli
	@npm install -g markdown-toc

# Validate documentation structure
validate: ## Validate documentation structure
	@echo "Validating documentation structure..."
	@python scripts/validate_docs.py

# Update last modified dates
update-dates: ## Update last modified dates in documentation
	@echo "Updating last modified dates..."
	@python scripts/update_dates.py 