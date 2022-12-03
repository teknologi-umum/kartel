all:

build:
	cargo build --release
build.mold:
	mold -run cargo build --release
build.run:
	./target/release/kartel

build-dev:
	cargo build
build-dev.mold:
	mold -run cargo build
build-dev.run:
	./target/debug/kartel

test:
	cargo test
test.mold:
	mold -run cargo test

clean:
	cargo clean
clean.mold:
	mold -run cargo clean