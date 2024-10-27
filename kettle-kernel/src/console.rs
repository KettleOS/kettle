#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/console.rs"]
mod arch_console;

#[cfg(target_arch = "aarch64")]
#[allow(unused_imports, unused_braces)]
pub mod aarch64 {
	pub use super::arch_console::{QEMUConsole};
}

pub trait Console: core::fmt::Write {}

/// Returns the global console that printing macros use.
#[inline(always)]
pub fn console() -> impl Console {
	arch_console::console()
}
