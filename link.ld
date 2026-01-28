ENTRY(_start)

SECTIONS {
	. = 0x80000;

	.text : {
		KEEP(*(.text._start*))
		*(.text*)
	}
	.data : {*(.data*)}
	.rodata : {*(.rodata*)}
	.bss : {*(.bss*)}
}
