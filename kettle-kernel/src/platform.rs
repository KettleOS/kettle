#[cfg(feature = "platform_virt")]
mod virt;

#[cfg(feature = "platform_virt")]
use virt as internal_platform;

#[allow(unused_imports, unused_braces)]
pub use internal_platform::{BOOT_CORE_ID};
