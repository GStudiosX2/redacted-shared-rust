ENTRY(main)

SECTIONS {
    . = 0x1000;
    .text : { *(.text) }
    .bss : { *(.bss) }
    .rodata : { *(.rodata) }
    .data : { *(.data) }

    /DISCARD/ : {
        *(.comment .note .eh_frame)
    }
}