#[cfg(feature = "platform_virt")]
mod _virt;

#[cfg(feature = "platform_virt")]
use _virt as internal_platform;

#[cfg(feature = "platform_virt")]
#[allow(unused_imports, unused_braces)]
pub mod virt {
	pub use super::_virt::driver;
}

/// Per-architecutre platform-specific exports.
#[cfg(target_arch = "aarch64")]
#[allow(unused_imports, unused_braces)]
pub mod aarch64 {
	pub use super::internal_platform::{BOOT_CORE_ID, UART0};

	pub mod driver {
		#[cfg(feature = "platform_virt")]
		pub use super::super::virt::driver::uart::{UartDriver, UART_DRIVER};
	}
}

/// Per-architecutre platform-specific exports.
#[cfg(target_arch = "x86_64")]
#[allow(unused_imports, unused_braces)]
pub mod x86_64 {}
