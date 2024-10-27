use core::arch::asm;

/// # Safety
/// You must ensure that your usecase is thread-safe.
#[inline(always)]
pub unsafe fn wfe() {
	unsafe { asm!("wfe") };
}

#[inline(always)]
pub fn wait_forever() -> ! {
	loop {
		// SAFETY:
		// After waiting for events, the function loops, waiting indefinitely.
		unsafe { wfe() };
	}
}
