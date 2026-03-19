mod cpu;
use cpu::CPU;

// 20x18 tiles
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

const NUM_KEYS: usize = 10;

pub struct GBoy {
	keys: [u8; NUM_KEYS],
	cpu: CPU,
}
impl GBoy {
	pub fn new() -> Self {
		GBoy {
			keys: [0; NUM_KEYS],
			cpu: CPU::new(),
		}
	}
	pub fn reset(&mut self) {
		self.keys = [0; NUM_KEYS];
		self.cpu = CPU::new();
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {}
}
