	.section .text._start, "ax"
	.global _start
	.type _start, @function
_start:
	// Set up stack pointer to STACK + STACK_SIZE
	ldr x0, ={0}
	ldr x1, ={1}
	add sp, x0, x1

	// Call main
	bl main

	// If main returns, hang forever
	b .
