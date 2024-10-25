/* Starting address of RAM address space in QEMU's `virt` mode. */
/* This is chosen because we are using an ELF file, which is not the Linux boot protocol. */
/* Thus, QEMU loads the program at RAM start. */
__virt_ram_start = 0x40000000;

ENTRY(__virt_ram_start)

PHDRS {
	text PT_LOAD FLAGS(5);
}

SECTIONS {
	. = __virt_ram_start;

	.text : {
		KEEP(*(.text._start))
	} :text
}
