OUTPUT_FORMAT(elf64-littleaarch64)
ENTRY(_el2_entry)

INCLUDE board_link_vars.ld

SECTIONS
{
    . = __text_start;
    .text :
    {
      KEEP(*(.text.boot.el2_entry))
      KEEP(*(.text.boot))
      *(.text .text.*)
    }

    .rodata ALIGN(8):
    {
        *(.rodata .rodata.*)
    }

    .data ALIGN(8):
    {
        __data_start = .;
        *(.data .data.*)
        __data_end = .;
    }

    __data_loadaddr = LOADADDR(.data);

    .bss ALIGN(8):
    {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(8);
        __bss_end = .;
    }

    . = ALIGN(4096);
    __kernel_end = .;

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}

