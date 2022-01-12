use core::panic::PanicInfo;
use crate::error;

// called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	error!("{}", info);
	loop {}
}
