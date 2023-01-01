use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::instructions::port::Port;

// offset so we don't collide with exception interrupts
pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {
	ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET)
});

pub const PIC1_MASK: u8 = 0b11111110;
pub const PIC2_MASK: u8 = 0b11111111;

pub(super) fn init_pic() {
	unsafe {
		let mut pics = PICS.lock();
		pics.initialize();
		// mask/unmask irqs
		pics.write_masks(PIC1_MASK, PIC2_MASK);
	}
}

pub(super) fn init_pit() {
	unsafe {
		// configure PIT
		// mode/command
		let mut command: Port<u8> = Port::new(0x43);
		// channel 0
		let mut channel0: Port<u16> = Port::new(0x40);
		// configure
		command.write(0b00111100);
		// set reload value
		channel0.write(0xFFFF);
	}
}
