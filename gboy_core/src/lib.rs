pub mod cpu;
use cpu::*;

/*
	-cpu: Sharp LR35902
*/

// 20x18 tiles
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

const NUM_KEYS: usize = 10;

pub struct GBoy {
	pc: u16,
	screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
	keys: [u8; NUM_KEYS],
	cpu: CPU,
}
impl GBoy {
	pub fn new() -> Self {
		GBoy {
			pc: 0,
			screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
			keys: [0; NUM_KEYS],
			cpu: CPU::new(),
		}
	}
	pub fn reset(&mut self) {
		self.pc = 0;
		self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
		self.keys = [0; NUM_KEYS];
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {}
}
