.SUFFIXES:

EXTENSION_ROOT = ./extension

CARGO_WORKSPACE_ROOT = ./host
CARGO_WORKSPACE_TOML = $(addprefix $(CARGO_WORKSPACE_ROOT), /Cargo.toml)

MANIFEST_PATH = --manifest-path=$(CARGO_WORKSPACE_TOML)

.PHONY: all
all: build

.PHONY: build clean test
build clean test:
	cargo $@ $(MANIFEST_PATH)
	npm --prefix $(EXTENSION_ROOT) run $@

.PHONY: lint
lint:
	cargo clippy $(MANIFEST_PATH)
	npm --prefix $(EXTENSION_ROOT) run $@

.PHONY: install_host_manifest
install_host_manifest: build
	npm --prefix $(EXTENSION_ROOT) run create-host-manifest
