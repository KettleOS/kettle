//! Kernel's driver subsystem.

pub mod error;

use error::{DriverError, DriverInitError};
use uuid::Uuid;

use crate::sync::{KernelInitLock, RwExLock};

pub static DRIVER_MANAGER: DriverManager<16> = DriverManager::new();

pub type DriverInitResult<T> = Result<T, DriverInitError>;
pub type DriverResult<T> = Result<T, DriverError>;
pub type PostDriverInitCallback = unsafe fn() -> DriverInitResult<()>;

struct InnerDriverManager<const DRIVER_COUNT: usize> {
	drivers: [Option<KernelDriverDescriptor<'static>>; DRIVER_COUNT],
	drivers_size: usize,
}

/// A struct for managing driver runtimes.
pub struct DriverManager<const DRIVER_COUNT: usize> {
	inner: KernelInitLock<InnerDriverManager<DRIVER_COUNT>>,
}

impl<const DRIVER_COUNT: usize> DriverManager<DRIVER_COUNT> {
	pub const fn new() -> Self {
		Self {
			inner: KernelInitLock::new(
				InnerDriverManager {
					drivers: [const { None }; DRIVER_COUNT],
					drivers_size: 0,
				}
			),
		}
	}

	pub fn register(&self, driver: KernelDriverDescriptor<'static>) -> DriverResult<()> {
		self.inner.read(|inner| {
			// Check for existing driver.
			for existing_driver in inner.drivers.iter() {
				if let Some(existing_driver) = existing_driver {
					if existing_driver.get().uuid() == driver.driver.uuid() {
						return Err(DriverError::Duplicate { uuid: existing_driver.get().uuid(), name: existing_driver.get().name() })
					}
				}
			}

			Ok(())
		})?;
		self.inner.write(|inner| {
			inner.drivers.as_mut_slice()[inner.drivers_size] = Some(driver);
			inner.drivers_size += 1;
		});

		Ok(())
	}

	pub fn init(&'static mut self) -> DriverResult<()> {
		self.inner.write(|inner: &mut InnerDriverManager<DRIVER_COUNT>| {
			for driver in inner.drivers.iter_mut() {
				if let Some(driver) = driver {
					unsafe { driver.get_mut().init()? };
				}
			}

			Ok(())
		})
	}
}

/// A container for a [KernelDriver] with metadata.
pub struct KernelDriverDescriptor<'a> {
	driver: &'a mut (dyn KernelDriver + Sync),
	post_init_callback: PostDriverInitCallback,
}

impl<'a> KernelDriverDescriptor<'a> {
	#[inline]
	pub fn get(&'a self) -> &'a (dyn KernelDriver + Sync) {
		self.driver
	}

	#[inline]
	pub fn get_mut(&'a mut self) -> &'a mut (dyn KernelDriver + Sync) {
		self.driver
	}

	#[inline]
	pub fn post_init_callback(&self) -> PostDriverInitCallback {
		self.post_init_callback
	}
}

/// A kernel-mode driver.
///
/// These drivers are only available in kernel-mode and are statically linked.
/// Dynamically-linked usermode drivers are available, but they are unavailable in a kernel environment.
pub trait KernelDriver {
	/// A UUID representing this kernel driver.
	fn uuid(&self) -> Uuid;

	/// This driver's human-readable name.
	fn name(&self) -> &'static str;

	/// # Safety
	/// Each [KernelDriver::init] function has unique safety requirements that potentially encompass the entire system. Refer to their documentation for more information.
	unsafe fn init(&mut self) -> DriverInitResult<()>;
}
