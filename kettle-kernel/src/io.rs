use core::fmt::Write;

use crate::console;

#[doc(hidden)]
pub fn _print(arguments: core::fmt::Arguments) {
	console::console().write_fmt(arguments).unwrap();
}

/// Prints to the console output.
///
/// Copy of <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
	($($arg:tt)*) => {{
		$crate::io::_print(format_args!($($arg)*));
	}};
}

/// Prints to the console output with a newline.
///
/// Copy of <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
	() => {
		$crate::print!("\n")
	};
	($($arg:tt)*) => {{
		$crate::io::_print(format_args_nl!($($arg)*));
	}};
}
