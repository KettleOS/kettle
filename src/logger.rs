use core::fmt::Write;
use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use font8x8::{BASIC_FONTS, UnicodeFonts};

pub static CHAR_SIZE: usize = 8;
pub static LINE_SPACE: usize = 1;

pub struct Logger {
	framebuffer: &'static mut [u8],
	info: FrameBufferInfo,
	x_pos: usize,
	y_pos: usize,
	color: (u8, u8, u8),
}

impl Logger {
	pub fn new(framebuffer: &'static mut FrameBuffer, color: (u8, u8, u8)) -> Self {
		let mut info = framebuffer.info();
		let framebuffer = framebuffer.buffer_mut();
		let mut logger = Self {
			framebuffer,
			info,
			x_pos: 0,
			y_pos: 0,
			color,
		};
		logger.clear();
		logger
	}
	
	pub fn clear(&mut self) {
		self.framebuffer.fill(0);
	}
	
	fn newline(&mut self) {
		self.y_pos += 8 + LINE_SPACE;
		self.carriage_return();
	}
	
	fn carriage_return(&mut self) {
		self.x_pos = 0;
	}
	
	pub fn write_char(&mut self, c: char) {
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
				if let Some(glyph) = BASIC_FONTS.get(c) {
					for (y, byte) in glyph.iter().enumerate() {
						for (x, bit) in (0..8).enumerate() {
							match *byte & 1 << bit {
								0 => {},
								_ => self.write_pixel(self.x_pos + x, self.y_pos + y, self.color),
							}
						}
					}
				}
			}
		}
		
		self.x_pos += 8;
	}
	
	fn clear_line(&mut self, line: usize) {
		let line_start = line * self.info.stride * CHAR_SIZE;
		let byte_start = line_start * self.info.bytes_per_pixel;
		self.framebuffer[byte_start..(byte_start + self.info.horizontal_resolution * self.info.bytes_per_pixel * (CHAR_SIZE + LINE_SPACE))]
			.fill(0);
	}
	
	fn scroll(&mut self) {
		// clear first line
		self.clear_line(0);
		// move everything up by one line
		self.framebuffer
			.rotate_left(self.info.horizontal_resolution * self.info.bytes_per_pixel * (CHAR_SIZE + LINE_SPACE));
	}
	
	pub fn write_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
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
}
