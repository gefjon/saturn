BOARD ?= raspi3

TARGET ?= aarch64-unknown-none-softfloat
KERNEL ?= kernel8
KERNEL_IMAGE ?= $(KERNEL).img

OBJCOPY ?= objcopy
OBJCOPY_PARAMS ?= --strip-all -O binary

OBJDUMP ?= objdump
OBJDUMP_PARAMS ?= -disassemble -print-imm-hex

QEMU ?= qemu-system-aarch64
QEMU_PARAMS ?= -machine $(BOARD) -cpu cortex-a53 -m 1G

RELEASE_BIN = target/$(TARGET)/release/$(KERNEL)
DEBUG_BIN = target/$(TARGET)/debug/$(KERNEL)

RELEASE_IMG = target/$(TARGET)/release/$(KERNEL_IMAGE)
DEBUG_IMG = target/$(TARGET)/debug/$(KERNEL_IMAGE)

BOARD_LINK_VARS = board_link_vars.ld

LINKER_SCRIPT = link.ld

BUILD_DEPENDS = $(wildcard **/*.rs) Cargo.toml $(LINKER_SCRIPT) $(BOARD_LINK_VARS)

RUSTFLAGS = -C link-arg=-T$(LINKER_SCRIPT)
RUSTC_ARGS = --target=$(TARGET) --features=$(BOARD)

SD_DEV ?= /dev/mmcblk1p1

%.img: %
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< $@

.PHONY: build release emu emu_debug clean debug gdb clippy doc expand sd
build: release

$(BOARD_LINK_VARS): src/board/$(BOARD)/link.ld
	cp $< $@

clean:
	cargo clean
	rm -f $(RELEASE_BIN) $(RELEASE_IMG) $(DEBUG_BIN) $(DEBUG_IMG) $(BOARD_LINK_VARS)

release: $(RELEASE_IMG)
debug: $(DEBUG_IMG)

$(DEBUG_BIN): $(BUILD_DEPENDS)
	RUSTFLAGS="$(RUSTFLAGS)" cargo rustc $(RUSTC_ARGS)

$(RELEASE_BIN): $(BUILD_DEPENDS)
	RUSTFLAGS="$(RUSTFLAGS)" cargo rustc $(RUSTC_ARGS) --release

emu: $(RELEASE_IMG)
	$(QEMU) $(QEMU_PARAMS) -kernel $< -display none -serial stdio

emu_debug: $(DEBUG_IMG)
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

sd: $(RELEASE_BIN) $(RELEASE_IMG)
	sudo mkdir -p /mnt/saturn
	sudo mount $(SD_DEV) /mnt/saturn
	sudo cp $^ /mnt/saturn
	sudo umount /mnt/saturn
