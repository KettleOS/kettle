use core::fmt::Write;

use crate::{platform::aarch64::driver::{UartDriver, UART_DRIVER}, sync::{KernelInitLock, RwExLock}};

struct UartConsoleInner {
	driver: &'static UartDriver,
}

impl core::fmt::Write for UartConsoleInner {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for char in s.bytes() {
			unsafe {
				core::ptr::write_volatile(self.driver.address().unwrap() as *mut u8, char);
			}
		}

		Ok(())
	}
}

pub struct UartConsole {
	inner: KernelInitLock<UartConsoleInner>,
}

static CONSOLE: UartConsole = UartConsole {
	inner: KernelInitLock::new(
		UartConsoleInner {
			driver: &UART_DRIVER,
		},
	),
};

impl super::Console for UartConsole {}

impl crate::fmt::Write for UartConsole {
	fn write_str(&self, s: &str) -> core::fmt::Result {
		self.inner.write(|inner| inner.write_str(s))
	}

	fn write_fmt(&self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
		self.inner.write(|inner| inner.write_fmt(args))
	}
}

pub fn console() -> &'static impl super::Console {
	&CONSOLE
}
