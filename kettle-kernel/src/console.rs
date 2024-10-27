#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/console.rs"]
mod arch_console;

pub trait Console: core::fmt::Write {}

/// Returns the global console that printing macros use.
#[inline(always)]
pub fn console() -> impl Console {
	arch_console::console()
}
