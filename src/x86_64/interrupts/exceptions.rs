use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::kernel_error;
use crate::x86_64::gdt::IstIndex;

pub(super) fn set_exception_handlers(idt: &mut InterruptDescriptorTable) {
	unsafe {
		idt.double_fault.set_handler_fn(doublefault_handler)
			.set_stack_index(IstIndex::DOUBLEFAULT.as_u16());
	}
	
	idt.page_fault.set_handler_fn(pagefault_handler);
	idt.general_protection_fault.set_handler_fn(general_protection_handler);
	idt.breakpoint.set_handler_fn(breakpoint_handler);
}

extern "x86-interrupt" fn doublefault_handler(frame: InterruptStackFrame, error_code: u64) -> ! {
	kernel_error!("DOUBLEFAULT:\n{:#?}\nError Code: {:#X}", frame, error_code);
	
	loop {}
}

extern "x86-interrupt" fn pagefault_handler(frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
	kernel_error!("PAGEFAULT:\n{:#?}\nError Code: {:?}", frame, error_code);
	
	loop {} // TODO: check where pagefault occurred. if kernel, panic; else, quit process
}

extern "x86-interrupt" fn general_protection_handler(frame: InterruptStackFrame, error_code: u64) {
	kernel_error!("GENPROT:\n{:#?}\nError Code: {:?}", frame, error_code);
	
	loop {} // TODO: check where genprot occurred. if kernel, panic; else, quit process
}

extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
	kernel_error!("BREAKPOINT:\n{:#?}", frame);
}
