use core::fmt::{Arguments, Write};
use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use conquer_once::spin::OnceCell;
use font8x8::{BASIC_FONTS, UnicodeFonts};
use spin::{Mutex, MutexGuard};

/// Width and height of each character
pub static CHAR_SIZE: usize = 8;
/// Gap between each line
pub static LINE_SPACE: usize = 1;
/// Default foreground color
pub static FOREGROUND: (u8, u8, u8) = (255, 255, 255);
/// Default background color
pub static BACKGROUND: (u8, u8, u8) = (0, 0, 0);
/// Error color
pub static ERR_COLOR: (u8, u8, u8) = (235, 77, 75);
/// Thread-safe logger
pub static mut LOGGER: OnceCell<Mutex<Logger>> = OnceCell::uninit();

/// Initialize the logger
/// **Note:** This function must be called before printing to the log. Using it before initialized
/// will cause a panic!
pub fn init_logger(
	framebuffer: &'static mut FrameBuffer,
	foreground: (u8, u8, u8),
	background: (u8, u8, u8),
	err_color: (u8, u8, u8)
) {
	unsafe {
		LOGGER = OnceCell::new(Mutex::new(Logger::new(
			framebuffer,
			foreground,
			background,
			err_color
		)));
	}
}

pub fn logger<'a>() -> MutexGuard<'a, Logger> {
	unsafe {
		LOGGER.get().expect("Logger not initialized!").lock()
	}
}

pub fn set_foreground(foreground: (u8, u8, u8)) {
	logger().foreground = foreground;
}

pub fn foreground() -> (u8, u8, u8) {
	logger().foreground
}

pub fn background() -> (u8, u8, u8) {
	logger().background
}

pub fn err_color() -> (u8, u8, u8) {
	logger().err_color
}

pub struct Logger {
	framebuffer: &'static mut [u8],
	info: FrameBufferInfo,
	x_pos: usize,
	y_pos: usize,
	foreground: (u8, u8, u8),
	background: (u8, u8, u8),
	err_color: (u8, u8, u8),
}

impl Logger {
	pub fn new(
		framebuffer: &'static mut FrameBuffer,
		foreground: (u8, u8, u8),
		background: (u8, u8, u8),
		err_color: (u8, u8, u8)
	) -> Self {
		let info = framebuffer.info();
		let framebuffer = framebuffer.buffer_mut();
		let mut logger = Self {
			framebuffer,
			info,
			x_pos: 0,
			y_pos: 0,
			foreground,
			background,
			err_color,
		};
		logger.clear();
		logger
	}
	
	pub fn clear(&mut self) {
		Logger::fill_with_slice(&mut self.framebuffer, &self.background);
	}
	
	fn rgb_to_slice(rgb: &(u8, u8, u8)) -> [u8; 3] {
		[rgb.0, rgb.1, rgb.2]
	}
	
	fn fill_with_slice(bytes: &mut [u8], rgb: &(u8, u8, u8)) {
		let background = [rgb.0, rgb.1, rgb.2];
		for (i, byte) in bytes.iter_mut().enumerate() {
			*byte = background[i % 3];
		}
	}
	
	fn newline(&mut self) {
		self.y_pos += CHAR_SIZE + LINE_SPACE;
		self.carriage_return();
	}
	
	fn carriage_return(&mut self) {
		self.x_pos = 0;
	}
	
	fn write_char(&mut self, c: char) {
		match c {
			'\n' => self.newline(),
			'\r' => self.carriage_return(),
			_ => {
				if self.x_pos >= self.info.horizontal_resolution {
					self.newline();
				}
				
				if self.y_pos >= self.info.vertical_resolution {
					self.scroll();
				}
				
				// print each pixel
				// TODO: custom char size
				if let Some(glyph) = BASIC_FONTS.get(c) {
					for (y, byte) in glyph.iter().enumerate() {
						for (x, bit) in (0..8).enumerate() {
							match *byte & 1 << bit {
								0 => {},
								_ => self.write_pixel(
									self.x_pos + x,
									self.y_pos + y,
									self.foreground
								),
							}
						}
					}
				}
				
				self.x_pos += CHAR_SIZE;
			}
		}
	}
	
	fn clear_line(&mut self, line: usize) {
		let line_start = line * self.info.stride * CHAR_SIZE;
		let byte_start = line_start * self.info.bytes_per_pixel;
		Logger::fill_with_slice(&mut self.framebuffer[byte_start..(byte_start + self.info.horizontal_resolution * self.info.bytes_per_pixel * (CHAR_SIZE + LINE_SPACE))], &self.background);
	}
	
	fn scroll(&mut self) {
		// clear first line
		self.clear_line(0);
		// move everything up by one line
		self.framebuffer
			.rotate_left(self.info.horizontal_resolution * self.info.bytes_per_pixel * (CHAR_SIZE + LINE_SPACE));
	}
	
	fn write_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
		let pixel_offset = y * self.info.stride + x;
		let byte_offset = pixel_offset * self.info.bytes_per_pixel;
		let color = match self.info.pixel_format {
			PixelFormat::RGB => [rgb.0, rgb.1, rgb.2, 0],
			PixelFormat::BGR => [rgb.2, rgb.1, rgb.0, 0],
			PixelFormat::U8 => [rgb.0, 0, 0, 0],
			_ => [0, 0, 0, 0],
		};
		self.framebuffer[byte_offset..(byte_offset + self.info.bytes_per_pixel)]
			.copy_from_slice(&color[..self.info.bytes_per_pixel]);
	}
}

impl Write for Logger {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for c in s.chars() {
			self.write_char(c);
		}
		Ok(())
	}
	
	fn write_char(&mut self, c: char) -> core::fmt::Result {
		self.write_char(c);
		Ok(())
	}
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => {
		$crate::logger::_print(format_args!($($arg)*));
	};
}

pub fn _print(args: Arguments) {
	logger().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => {
		$crate::print!("{}\n", format_args!($($arg)*));
	}
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
	    let foreground = $crate::logger::foreground();
	    $crate::logger::set_foreground($crate::logger::err_color());
	    $crate::print!("{}\n", format_args!($($arg)*));
	    $crate::logger::set_foreground(foreground);
    };
}
