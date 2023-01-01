use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic::PIC1_OFFSET;
use crate::kernel_print;

mod exceptions;
mod pic;
mod idt;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum InterruptIndex {
	Timer = PIC1_OFFSET,
	IRQ7 = PIC1_OFFSET + 8,
}

impl InterruptIndex {
	pub fn as_u8(self) -> u8 {
		self as u8
	}
	
	pub fn as_usize(self) -> usize {
		self as usize
	}
}

pub(crate) fn init_interrupts() {
	idt::init_idt();
	pic::init_pic();
	pic::init_pit();
	x86_64::instructions::interrupts::enable();
}

pub(self) fn set_interrupt_handlers(idt: &mut InterruptDescriptorTable) {
	idt[InterruptIndex::IRQ7.as_usize()]
		.set_handler_fn(irq7_handler);
	idt[InterruptIndex::Timer.as_usize()]
		.set_handler_fn(timer_handler);
}

extern "x86-interrupt" fn timer_handler(_: InterruptStackFrame) {
	kernel_print!(".");
	
	unsafe {
		pic::PICS.lock()
			.notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
	}
}

// IRQ7 workaround
extern "x86-interrupt" fn irq7_handler(_: InterruptStackFrame) {
	kernel_print!("?");
	
	unsafe {
		let mut command: Port<u8> = Port::new(0x20);
		command.write(0x0B);
		let irr = command.read();
		
		if irr & 0x80 == 0 {
			return;
		} else {
			pic::PICS.lock()
				.notify_end_of_interrupt(InterruptIndex::IRQ7.as_u8());
		}
	}
}
