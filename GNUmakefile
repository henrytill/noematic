.SUFFIXES:

EXTENSION_ROOT = ./extension

CARGO_WORKSPACE_ROOT = ./host
CARGO_WORKSPACE_TOML = $(addprefix $(CARGO_WORKSPACE_ROOT), /Cargo.toml)

MANIFEST_PATH = --manifest-path=$(CARGO_WORKSPACE_TOML)

.PHONY: all
all: build

.PHONY: build clean clippy fmt
build clean clippy fmt:
	cargo $@ $(MANIFEST_PATH)

.PHONY: test
test:
	cargo test $(MANIFEST_PATH)
	npm --prefix $(EXTENSION_ROOT) test

.PHONY: install_host_manifest
install_host_manifest: build
	npm --prefix $(EXTENSION_ROOT) run install-host-manifest
