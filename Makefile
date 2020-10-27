TARGET ?= aarch64-unknown-none-softfloat
KERNEL ?= kernel8
KERNEL_IMAGE ?= $(KERNEL).img

OBJCOPY ?= llvm-objcopy
OBJCOPY_PARAMS ?= --strip-all -O binary

OBJDUMP ?= llvm-objdump
OBJDUMP_PARAMS ?= -disassemble -print-imm-hex

QEMU ?= qemu-system-aarch64
QEMU_PARAMS ?= -M raspi3

RELEASE_BIN = target/$(TARGET)/release/$(KERNEL)
DEBUG_BIN = target/$(TARGET)/debug/$(KERNEL)

RELEASE_IMG = target/$(TARGET)/release/$(KERNEL_IMAGE)
DEBUG_IMG = target/$(TARGET)/debug/$(KERNEL_IMAGE)

LINKER_SCRIPT = link.ld

BUILD_DEPENDS = $(wildcard **/*.rs) Cargo.toml $(LINKER_SCRIPT)

RUSTFLAGS = -C link-arg=-T$(LINKER_SCRIPT)

%.img: %
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< $@

.PHONY: build release emu emu_debug clean debug gdb clippy doc
build: release

clean:
	cargo clean
	rm -f $(RELEASE_BIN) $(RELEASE_IMG) $(DEBUG_BIN) $(DEBUG_IMG)

release: $(RELEASE_IMG)
debug: $(DEBUG_IMG)

$(DEBUG_BIN): $(BUILD_DEPENDS)
	RUSTFLAGS="$(RUSTFLAGS)" cargo rustc --target=$(TARGET)

$(RELEASE_BIN): $(BUILD_DEPENDS)
	RUSTFLAGS="$(RUSTFLAGS)" cargo rustc --target=$(TARGET) --release

emu: $(RELEASE_IMG)
	$(QEMU) $(QEMU_PARAMS) -kernel $< -display none -serial stdio

emu_debug: $(DEBUG_IMG)
	$(QEMU) $(QEMU_PARAMS) -kernel $< -display none -serial stdio

gdb: $(DEBUG_IMG)
	gdb \
	-ex "target remote | $(QEMU) $(QEMU_PARAMS) -kernel $< -S -gdb stdio -display none" \
	-ex "add-symbol-file $(DEBUG_BIN)"

doc: $(BUILD_DEPENDS)
	cargo doc --target=$(TARGET) --document-private-items

clippy: $(BUILD_DEPENDS)
	cargo xclippy --target=$(TARGET)
