SRC := $(wildcard src/**/*.rs)

build: $(SRC)
	@cargo lambda build --release

.PHONY: srv-run
srv-run:
	@cargo lambda watch
