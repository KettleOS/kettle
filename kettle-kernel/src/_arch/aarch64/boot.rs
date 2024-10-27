use core::arch::global_asm;

use crate::kernel_main;

global_asm!(
	include_str!("boot.s")
);

#[no_mangle]
#[doc = include_str!("../_init_rt.md")]
pub unsafe extern "C" fn _init_rt() -> ! {
	unsafe {
		kernel_main();
	}

	panic!();
}
