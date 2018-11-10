TARGET := rvos-rs
RUST_TARGET := librvos_rs.a
RISCV-LD := riscv32-unknown-linux-gnu-ld

BASE_DIR := $(PWD)

# Determine target directory of cargo's module build
CARGO_BUILD_DIR := $(BASE_DIR)/target/riscv32imac-unknown-none-elf/debug

$(TARGET): $(CARGO_BUILD_DIR)/$(RUST_TARGET) linker.ld Makefile
	$(RISCV-LD) -T linker.ld $(CARGO_BUILD_DIR)/$(RUST_TARGET) -o $(TARGET)

$(CARGO_BUILD_DIR)/$(RUST_TARGET): src/lib.rs src/io/uart.rs Makefile
	cargo xbuild --target riscv32imac-unknown-none-elf

run: $(TARGET)
	/opt/riscv-qemu/bin/qemu-system-riscv32 -nographic -machine sifive_u -kernel $(TARGET)
