BOARD ?= raspi3

TARGET ?= aarch64-unknown-none-softfloat
KERNEL ?= kernel8

OBJDUMP ?= objdump
OBJDUMP_PARAMS ?= -disassemble -print-imm-hex

QEMU ?= qemu-system-aarch64
QEMU_PARAMS ?= -machine $(BOARD) -cpu cortex-a53 -m 1G

RELEASE_BIN = target/$(TARGET)/release/$(KERNEL)
DEBUG_BIN = target/$(TARGET)/debug/$(KERNEL)

BOARD_LINK_VARS = board_link_vars.ld

LINKER_SCRIPT = link.ld

BUILD_DEPENDS = $(wildcard src/*.rs) $(wildcard src/*/*.rs) Cargo.toml $(LINKER_SCRIPT) $(BOARD_LINK_VARS)

RUSTFLAGS = -C link-arg=-T$(LINKER_SCRIPT)
RUSTC_ARGS = --target=$(TARGET) --features=$(BOARD)


.PHONY: build release emu emu_debug clean debug gdb clippy doc expand
build: release

$(BOARD_LINK_VARS): src/board/$(BOARD)/link.ld
	cp $< $@

clean:
	cargo clean
	rm -f $(RELEASE_BIN) $(RELEASE_BIN) $(DEBUG_BIN) $(DEBUG_BIN) $(BOARD_LINK_VARS)

release: $(RELEASE_BIN)
debug: $(DEBUG_BIN)

print_build_deps:
	echo $(BUILD_DEPENDS)

$(DEBUG_BIN): $(BUILD_DEPENDS)
	RUSTFLAGS="$(RUSTFLAGS)" cargo rustc $(RUSTC_ARGS)

$(RELEASE_BIN): $(BUILD_DEPENDS)
	RUSTFLAGS="$(RUSTFLAGS)" cargo rustc $(RUSTC_ARGS) --release

emu: $(RELEASE_BIN)
	$(QEMU) $(QEMU_PARAMS) -kernel $< -display none -serial stdio

emu_debug: $(DEBUG_BIN)
	$(QEMU) $(QEMU_PARAMS) -kernel $< -display none -serial stdio

gdb: $(DEBUG_BIN)
	gdb \
	-ex "target remote | $(QEMU) $(QEMU_PARAMS) -kernel $< -S -gdb stdio -display none -serial file:serial.out" \
	-ex "add-symbol-file $(DEBUG_BIN)"

doc: $(BUILD_DEPENDS)
	cargo doc --target=$(TARGET) --document-private-items

clippy: $(BUILD_DEPENDS)
	cargo xclippy --target=$(TARGET)

expand: $(BUILD_DEPENDS)
	RUSTFLAGS="$(RUSTFLAGS)" cargo expand
