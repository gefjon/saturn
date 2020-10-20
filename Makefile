TARGET ?= aarch64-unknown-none
KERNEL ?= kernel8
KERNEL_IMAGE ?= $(KERNEL).img

OBJCOPY ?= llvm-objcopy
OBJCOPY_PARAMS ?= --strip-all -O binary

OBJDUMP ?= llvm-objdump
OBJDUMP_PARAMS ?= -disassemble -print-imm-hex

QEMU ?= qemu-system-aarch64
QEMU_PARAMS ?= -M raspi3

BUILD_DEPENDS=$(wildcard **/*.rs) Cargo.toml link.ld

.PHONY: build emu clean debug objdump clippy doc
build: $(KERNEL_IMAGE)

clean:
	cargo clean
	rm -f kernel8 kernel8.img

debug: target/$(TARGET)/debug/$(KERNEL)

target/$(TARGET)/debug/$(KERNEL): $(BUILD_DEPENDS)
	cargo xbuild --target=$(TARGET)

target/$(TARGET)/release/$(KERNEL): $(BUILD_DEPENDS)
	cargo xbuild --target=$(TARGET) --release

$(KERNEL): target/$(TARGET)/release/$(KERNEL)
	cp $< $@

$(KERNEL_IMAGE): $(KERNEL)
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< $@

emu: $(KERNEL_IMAGE)
	$(QEMU) $(QEMU_PARAMS) -kernel $< -display none -serial stdio

asm: $(BUILD_DEPENDS)
	cargo xrustc --target=$(TARGET) --release -- --emit=asm

doc: $(wildcard **/*.rs) Cargo.toml
	cargo doc --target=$(TARGET) --document-private-items

objdump: $(KERNEL)
	$(OBJDUMP) $(OBJDUMP_PARAMS) $<

clippy: $(wildcard src/**.rs)
	cargo xclippy --target=$(TARGET)
