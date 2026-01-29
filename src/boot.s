	.section .text._start, "ax"
	.global _start
	.type _start, @function
_start:
	// Initialize stack
	adr x7, {}; // Move address of STACK slice into x7
	mov x8, {}; // Move value of STACK_SIZE into x8
	add x7, x7, x8; // Find stack starting pointer
	mov sp, x7; // Set stack pointer to start

	# Call main
	bl main
