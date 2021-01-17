TARGET = clocksw

RUST_TARGET_DIR = ./target/arm-unknown-linux-gnueabihf
OUTPUT_DIR = ./build

.PHONY: debug release fonts

debug: $(OUTPUT_DIR)/debug .toolchain.lock fonts
	cargo build
	cp $(RUST_TARGET_DIR)/debug/$(TARGET) $(OUTPUT_DIR)/debug/

release: $(OUTPUT_DIR)/release .toolchain.lock fonts
	cargo build --release
	cp $(RUST_TARGET_DIR)/release/$(TARGET) $(OUTPUT_DIR)/release/
	llvm-strip $(OUTPUT_DIR)/release/$(TARGET)

fonts:
	$(MAKE) -C data

$(OUTPUT_DIR)/debug:
	mkdir -p $(OUTPUT_DIR)/debug

$(OUTPUT_DIR)/release:
	mkdir -p $(OUTPUT_DIR)/release

.toolchain.lock:
	rustup target add arm-unknown-linux-gnueabihf
	touch .toolchain.lock
