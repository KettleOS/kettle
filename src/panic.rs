use core::panic::PanicInfo;
// use crate::error;

// called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// error!("{}", _info);
	loop {}
}
