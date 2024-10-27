#[cfg(feature = "platform_virt")]
mod _virt;

#[cfg(feature = "platform_virt")]
use _virt as internal_platform;

#[allow(unused_imports, unused_braces)]
pub use internal_platform::{BOOT_CORE_ID};

pub mod virt {}
