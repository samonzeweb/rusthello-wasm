NPM = npm
WASM_PACK = wasm-pack

all: build

build: setup
	$(NPM) run build

serve-dev:
	$(info ⚠ Run 'make setup' before if needed ⚠)
	$(NPM) run serve

clean:
	rm -Rf node_modules
	rm -Rf dist
	
setup: clean
	$(NPM) install || $(NPM) install

.PHONY: all build serce-dev clean setup
