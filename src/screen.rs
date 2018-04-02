const WIDTH: usize = 60;
const HEIGHT: usize = 25;
const PIXELS: usize = WIDTH * HEIGHT;

use card::Database;

pub struct Screen {
	art: [char; PIXELS],
}

impl Screen {
	pub fn new(c: char) -> Screen {
		Screen { art: [c; PIXELS] }
	}

	pub fn clear(&mut self, c: char) {
		for i in 0..PIXELS {
			self.art[i] = c;
		}
	}

	pub fn put(&mut self, c: char, coord: (usize, usize)) {
		self.art[Self::index(coord)] = c
	}

	pub fn print(&self) {
		let mut index = 0;
		for j in 0..HEIGHT {
			for i in 0..WIDTH {
				print!("{}", self.art[index]);
				index += 1;
			}
			print!("\n");
		}
	}

	#[inline]
	pub fn index(coord: (usize, usize)) -> usize {
		coord.1 * WIDTH + coord.0
	}

	#[inline]
	pub fn coord(index: usize) -> (usize, usize) {
		(index % WIDTH, index / WIDTH) 
	}
}