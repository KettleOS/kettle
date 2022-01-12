use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::error;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		idt.double_fault.set_handler_fn(doublefault_handler);
		idt.page_fault.set_handler_fn(pagefault_handler);
		idt
	};
}

pub fn init_idt() {
	IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
	error!("BREAKPOINT:\n{:#?}", frame);
}

extern "x86-interrupt" fn doublefault_handler(frame: InterruptStackFrame, error_code: u64) -> ! {
	error!("DOUBLEFAULT:\n{:#?}\nError Code: {:#X}", frame, error_code);
	
	loop {}
}

extern "x86-interrupt" fn pagefault_handler(frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
	error!("PAGEFAULT:\n{:#?}\nError Code: {:?}", frame, error_code);
	
	loop {} // TODO: check where pagefault occurred. if kernel, panic; else, quit process
}
