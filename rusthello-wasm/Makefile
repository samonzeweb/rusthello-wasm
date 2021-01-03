CARGO = cargo
CARGO_OPTS =

WASM_PACK = wasm-pack

all:
	$(MAKE) clean build-bundler

build-bundler: clean-pkg
	$(WASM_PACK) build --target bundler --release
	$(WASM_PACK) pack

build-web: clean-pkg
	$(WASM_PACK) build --target web --release

build-node: clean-pkg
	$(WASM_PACK) build --target nodejs --release

clean-pkg:
	rm -Rf pkg

clean-cargo:
	$(CARGO) $(CARGO_OPTS) clean

clean: clean-pkg clean-cargo


.PHONY: all build-bundler build-web build-node clean-pkg clean-cargo clean