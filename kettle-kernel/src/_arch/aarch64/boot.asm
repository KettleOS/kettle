.section .text._start

// fn _start()
_start:
.loop:
	// wait for events and loop
	wfe
	b	.loop

.size	_start,	. - _start
.type	_start,	function
.global	_start
