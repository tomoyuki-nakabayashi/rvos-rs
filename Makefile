TARGET := rvos-rs
RUST_TARGET := librvos_rs.a
RISCV-LD := riscv32-unknown-elf-ld

BASE_DIR := $(PWD)

# Determine target directory of cargo's module build
CARGO_BUILD_DIR := $(BASE_DIR)/target/riscv32imac-unknown-none-elf/debug

$(TARGET): $(CARGO_BUILD_DIR)/$(RUST_TARGET)
	$(RISCV-LD) -T linker.ld $(CARGO_BUILD_DIR)/$(RUST_TARGET) -o $(TARGET)

$(CARGO_BUILD_DIR)/$(RUST_TARGET):
	cargo xbuild --target riscv32imac-unknown-none-elf.json
