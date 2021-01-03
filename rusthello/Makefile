CARGO = cargo

CARGO_OPTS =

all:
	$(MAKE) clean test-full build

build:
	$(CARGO) $(CARGO_OPTS) build --release

clean:
	$(CARGO) $(CARGO_OPTS) clean

test-fast:
	$(CARGO) $(CARGO_OPTS) test

test-full:
	$(CARGO) $(CARGO_OPTS) test --features alphabetavsminimax

.PHONY: all build clean test-fast test-full
