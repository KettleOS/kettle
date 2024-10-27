#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/boot.rs"]
mod arch_boot;

#[cfg(target_arch = "aarch64")]
#[allow(unused_imports, unused_braces)]
pub use arch_boot::{_init_rt};

const UNEXPECTED_KERNEL_MAIN_RETURN: &'static str = "Returned from `kernel_main` unexpectedly. This should never happen!";
