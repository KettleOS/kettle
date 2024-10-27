/* Starting address of RAM address space in QEMU's `virt` mode. */
/* This is chosen because we are using an ELF file, which is not the Linux boot protocol. */
/* Thus, QEMU loads the program at RAM start. */
__virt_ram_start = 0x40000000;

/* This is where the binary will be loaded. */
__virt_binary_start = 0x40080000;

ENTRY(__virt_binary_start)

PHDRS {
	segment_stack	PT_LOAD	FLAGS(6);
	segment_code	PT_LOAD FLAGS(5);
	segment_data	PT_LOAD FLAGS(6);
}

SECTIONS {
	. = __virt_ram_start;

	.stack (NOLOAD) : {
		. = __virt_binary_start;
		__stack_end_exclusive = .;
	} :segment_stack

	/* Code */
	.text : {
		KEEP(*(.text._start))
		*(.text.static) /* Static constants as defined in Rust. */
		*(.text*) /* All other code. */
	} :segment_code

	.rodata : ALIGN(8) { *(.rodata*) } :segment_code

	/* Data */
	.data : { *(.data*) } :segment_data

	.bss (NOLOAD) : ALIGN(16) {
		__bss_start = .;
		*(.bss*);
		. = ALIGN(16);
		__bss_end_exclusive = .;
	} :segment_data

	/DISCARD/ : {
		*(.comment*)
	}
}
