n ?= 50

build:
	cargo build --release

clean:
	cargo clean

server: build
	./target/release/server -n $(n)

client: build
	./target/release/client