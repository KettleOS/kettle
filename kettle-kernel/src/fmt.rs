#[allow(unused)]
pub trait Write {
	fn write_str(&self, s: &str) -> core::fmt::Result;
	fn write_fmt(&self, args: core::fmt::Arguments<'_>) -> core::fmt::Result;
}
