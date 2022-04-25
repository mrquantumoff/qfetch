main:
	make clean
	make bin
	echo "Please run \"sudo make install\" to install the program."
clean:
	cargo clean
bin:
	cargo build --release
install:
	install -Dm755 -t /usr/bin/ target/release/qfetch