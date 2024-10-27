pub struct QEMUConsole;

impl super::Console for QEMUConsole {}

impl core::fmt::Write for QEMUConsole {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for char in s.bytes() {
			unsafe {
				core::ptr::write_volatile(crate::platform::aarch64::UART0 as *mut u8, char);
			}
		}

		Ok(())
	}
}

pub fn console() -> impl super::Console {
	QEMUConsole
}
