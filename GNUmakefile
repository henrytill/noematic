.SUFFIXES:

EXTENSION_ROOT = ./extension

.PHONY: all
all: build

.PHONY: build
build:
	cargo $@ --all-targets
	npm --prefix $(EXTENSION_ROOT) run $@

.PHONY: build clean test
clean test:
	cargo $@
	npm --prefix $(EXTENSION_ROOT) run $@

.PHONY: lint
lint:
	cargo clippy --all-targets
	npm --prefix $(EXTENSION_ROOT) run $@

.PHONY: create_host_manifest
create_host_manifest: build
	npm --prefix $(EXTENSION_ROOT) run create-host-manifest
