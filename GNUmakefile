.SUFFIXES:

CARGO_WORKSPACE_ROOT = ./host
CARGO_WORKSPACE_TOML = $(addprefix $(CARGO_WORKSPACE_ROOT), /Cargo.toml)
CARGO_WORKSPACE_TARGET = $(addprefix $(CARGO_WORKSPACE_ROOT), /target)
WASM_DEBUG_BUILD_DIR = $(addprefix $(CARGO_WORKSPACE_TARGET), /wasm32-unknown-unknown/debug)
WASM_DEBUG_BUILD = $(addprefix $(WASM_DEBUG_BUILD_DIR), /noematic_web.wasm)
WASM_OUT_DIR = ./extension/src/generated

MANIFEST_PATH = --manifest-path=$(CARGO_WORKSPACE_TOML)

.PHONY: all
all: build

.PHONY: build
build:
	cargo build $(MANIFEST_PATH) --target wasm32-unknown-unknown -p noematic-web
	wasm-bindgen --target web --out-dir $(WASM_OUT_DIR)  $(WASM_DEBUG_BUILD)

.PHONY: check test
check test:
	cargo $@ $(MANIFEST_PATH)

.PHONY: clean
clean:
	rm -rf $(WASM_OUT_DIR)
