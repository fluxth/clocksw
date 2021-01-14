TARGET_DIR = ./target/arm-unknown-linux-guneabihf

debug: build/debug .toolchain.lock fonts
	cargo build
	cp $(TARGET_DIR)/debug/clocksw build/debug/

release: build/release .toolchain.lock fonts
	cargo build --release
	cp $(TARGET_DIR)/release/clocksw build/release/
	llvm-strip build/release/clocksw

fonts:
	$(MAKE) -C data

build/debug: build
	mkdir -p build/debug

build/release:
	mkdir -p build/release

.toolchain.lock:
	rustup target add arm-unknown-linux-gnueabihf
	touch .toolchain.lock
