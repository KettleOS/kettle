use core::panic::PanicInfo;

use crate::cpu;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	cpu::wait_forever();
}
