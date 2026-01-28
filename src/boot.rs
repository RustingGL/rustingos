use core::arch::global_asm;

global_asm!(r#"
    .section ".text.boot"
	.global _start
_start:
	bl rusting_k_start;
	.balign 512, 0;
"#);