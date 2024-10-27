use core::{panic::PanicInfo, sync::atomic::AtomicBool};

use crate::{brand::*, cpu, println};

static PANIC_ENTERED: AtomicBool = AtomicBool::new(false);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	// Prevent panic re-entering.
	if PANIC_ENTERED.load(core::sync::atomic::Ordering::Relaxed) {
		cpu::wait_forever();
	}

	PANIC_ENTERED.store(true, core::sync::atomic::Ordering::Relaxed);

	let (file, line, column) = match info.location() {
		Some(loc) => (loc.file(), loc.line(), loc.column()),
		_ => ("???", 0, 0),
	};

	println!(
		"Kernel panic!\n\n\
		{file}:{line}:{column}: {}\n\n\
		If you are sure this is a {BRAND} issue, report this on our issue tracker: {ISSUE_TRACKER}",
		info.message()
	);

	cpu::wait_forever();
}
