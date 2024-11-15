n ?= 50
test ?= false

build:
	cargo build --release

clean:
	cargo clean

server: build
	./target/release/server -n $(n)

client: build
	@if [ "$(test)" = "true" ]; then \
		./target/release/client --test; \
	else \
		./target/release/client; \
	fi