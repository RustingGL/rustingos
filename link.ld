ENTRY(_start)

SECTIONS {
    . = 0x80000;

    /* Boot block: first 512 bytes */
    .text.boot : {
        KEEP(obj/boot.o(.text.boot*))
    }

    /* Rest of kernel text */
    .text : {
        *(.text*)
    }

    .rodata : { *(.rodata*) }
    .data   : { *(.data*) }
    .bss    : {
        *(.bss*)
        *(COMMON)
    }
}
