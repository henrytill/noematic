.SUFFIXES:

CARGO_WORKSPACE_ROOT = ./host
CARGO_WORKSPACE_TOML = $(addprefix $(CARGO_WORKSPACE_ROOT), /Cargo.toml)

MANIFEST_PATH = --manifest-path=$(CARGO_WORKSPACE_TOML)

.PHONY: all
all: build

.PHONY: build clean clippy test
build clean clippy test:
	cargo $@ $(MANIFEST_PATH)

