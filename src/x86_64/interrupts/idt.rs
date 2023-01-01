use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use super::exceptions;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		exceptions::set_exception_handlers(&mut idt);
		super::set_interrupt_handlers(&mut idt);
		idt
	};
}

pub(super) fn init_idt() {
	IDT.load();
}
