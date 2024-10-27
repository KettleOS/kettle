use core::arch::global_asm;

use crate::kernel_main;
use super::UNEXPECTED_KERNEL_MAIN_RETURN;

global_asm!(include_str!("boot.s"));

#[no_mangle]
#[doc = include_str!("../_init_rt.md")]
pub unsafe extern "C" fn _init_rt() -> ! {
	unsafe {
		kernel_main();
	}

	panic!("{UNEXPECTED_KERNEL_MAIN_RETURN}");
}
