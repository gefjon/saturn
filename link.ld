SECTIONS
{
    . = 0x80000;
    __text_start = .;
    .text :
    {
      KEEP(*(.text.boot))
      KEEP(*(.text.boot.*))
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

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
