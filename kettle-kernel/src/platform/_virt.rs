/// The core on which boot code is run.
#[no_mangle]
#[link_section = ".text.static"]
pub static BOOT_CORE_ID: u64 = 0;
