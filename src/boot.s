	.section .text._start, "ax"
	.global _start
	.type _start, @function
_start:
	// Initialize stack
	adrp x7, {0}; // Load page address of STACK
	add x7, x7, :lo12:{0}; // Add low 12 bits
	mov x8, {1}; // Move value of STACK_SIZE into x8
	add x7, x7, x8; // Find stack starting pointer
	mov sp, x7; // Set stack pointer to start

	# Call main
	bl main
