.PHONY: all clean

all: clean
	cargo test
	cargo build --release
	upx target/release/bmapconv

clean:
	rm -rf target/
