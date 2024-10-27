pub mod boot;

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;

#[allow(unused_imports, unused_braces)]
pub use arch_cpu::{wait_forever};
