// Loads an address of a symbol into a register as PC-relative.
// https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register,	\symbol
	add	\register,	\register,	#:lo12:\symbol
.endm

.section .text._start

// fn _start()
_start:
	// Park if not on the boot core.
	mrs	x0,	MPIDR_EL1
	and	x0,	x0,	#0xFFFF // Mask Aff0/Aff1 fields
	ldr	x1,	BOOT_CORE_ID // BOOT_CORE_ID is platform-defined
	cmp	x0,	x1
	b.ne	.park

	// Initialize stack
	ADR_REL	x0,	__bss_start
	ADR_REL	x1,	__bss_end_exclusive

// Initialize .bss
.bss_init_loop:
	cmp	x0,	x1 // Ensure .bss is empty.
	b.eq	.prepare_rt
	stp	xzr,	xzr,	[x0],	#16 // Move __bss_start to __bss_end_exclusive.
	b	.bss_init_loop

// Prepare the Rust runtime.
.prepare_rt:
	// Set the stack pointer
	ADR_REL	x0,	__stack_end_exclusive
	mov	sp,	x0

	// Initialize the kernel runtime
	b	_init_rt

// Park the core.
.park:
	// wait for events and loop
	wfe
	b	.park

.size	_start,	. - _start
.type	_start,	function
.global	_start
