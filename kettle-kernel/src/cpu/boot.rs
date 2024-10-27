#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/boot.rs"]
mod arch_boot;

#[cfg(target_arch = "aarch64")]
#[allow(unused_imports, unused_braces)]
pub use arch_boot::{_init_rt};
