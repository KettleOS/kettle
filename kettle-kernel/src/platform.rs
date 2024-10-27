#[cfg(feature = "platform_virt")]
mod _virt;

#[cfg(feature = "platform_virt")]
use _virt as internal_platform;

#[cfg(feature = "platform_virt")]
pub mod virt {}

/// Per-architecutre platform-specific exports.
#[cfg(target_arch = "aarch64")]
#[allow(unused_imports, unused_braces)]
pub mod aarch64 {
	pub use super::internal_platform::{BOOT_CORE_ID};
}

/// Per-architecutre platform-specific exports.
#[cfg(target_arch = "x86_64")]
#[allow(unused_imports, unused_braces)]
pub mod x86_64 {}
