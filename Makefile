.PHONY: all clean

all: clean
	cargo test
	cargo build --release
	# upx --force-macos target/release/bmapconv

clean:
	rm -rf target/
