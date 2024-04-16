.PHONY: main clean build_win build_linux

main:
	cross build --release --target x86_64-unknown-linux-gnu
	upx target/x86_64-unknown-linux-gnu/release/bmapconv
	cross build --release --target x86_64-pc-windows-gnu
	upx target/x86_64-pc-windows-gnu/release/bmapconv.exe

clean:
	rm -rf target/x86*

build_linux:
	cross build --release --target x86_64-unknown-linux-gnu
	upx target/x86_64-unknown-linux-gnu/release/bmapconv

build_win:
	cross build --release --target x86_64-pc-windows-gnu
	upx target/x86_64-pc-windows-gnu/release/bmapconv.exe

