pub mod driver;

/// The core on which boot code is run.
#[no_mangle]
#[link_section = ".text.static"]
pub static BOOT_CORE_ID: u64 = 0;

#[link_section = ".text.static"]
pub static UART0: usize = 0x0900_0000;
