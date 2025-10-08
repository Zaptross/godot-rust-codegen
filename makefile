.PHONY: example
example: clean-example build-example

.PHONY: build-example
build-example:
	cd example && cargo build

.PHONY: clean-example
clean-example:
	cd example && cargo clean