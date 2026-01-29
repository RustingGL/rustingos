ENTRY(_start)

SECTIONS {
	. = 0x0;

	/* get rid of unwinding / exception metadata */
	/DISCARD/ : {
		*(.eh_frame)
		*(.eh_frame_hdr)
	}

	.text : {
		KEEP(*(.start))   /* your _start section */
		*(.text*)
	}

	.rodata : { *(.rodata*) }
	.data   : { *(.data*) }
	.stack  : { KEEP(*(.stack*)) }
	.bss    : { *(.bss*) *(COMMON) }
}
