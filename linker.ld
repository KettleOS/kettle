ENTRY(kernel_main)

PHDRS {
	null PT_NULL FLAGS(0);
	text PT_LOAD FLAGS((1 << 0) | (1 << 2));
	rodata PT_LOAD FLAGS((1 << 2));
	data PT_LOAD FLAGS((1 << 1) | (1 << 2));
}

SECTIONS {
	. = 0xffffffff80000000;

	.text : {
		*(.text .text.*)
	} :text

	. += CONSTANT(MAXPAGESIZE);

	.rodata : {
		*(.rodata .rodata.*)
	} :rodata

	. += CONSTANT(MAXPAGESIZE);

	.data : {
		*(.data .data.*)
	} :data

	.bss : {
		*(.bss .bss.*)
	} :data
}
