use crate::{driver::{DriverResult, KernelDriver}, platform, sync::{KernelInitLock, RwExLock}};

struct Data {
	address: usize,
}

pub static UART_DRIVER: UartDriver = platform::virt::driver::uart::UartDriver::new();

pub struct UartDriver {
	data: KernelInitLock<Option<Data>>,
}

impl UartDriver {
	pub const fn new() -> Self {
		Self {
			data: KernelInitLock::new(None),
		}
	}

	fn data(&self) -> DriverResult<&Data> {
		self.data
			.read(|data| data)
			.as_ref()
			.ok_or(self.uninitialized())
	}

	pub fn address(&self) -> DriverResult<usize> {
		Ok(self.data()?.address)
	}
}

impl KernelDriver for UartDriver {
	fn uuid(&self) -> uuid::Uuid {
		uuid::uuid!("0192da18-290c-7911-8eae-41e5e4fdf33d")
	}

	fn name(&self) -> &'static str {
		"UART"
	}

	unsafe fn init(&self) -> crate::driver::DriverInitResult<()> {
		let data = Data {
			address: platform::aarch64::UART0,
		};
		self.data.write(|self_data| {
			self_data.replace(data)
		});

		Ok(())
	}
}
