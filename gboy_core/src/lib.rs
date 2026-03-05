/*
	-Sharp LR35902
*/

const SCREEN_WIDTH: usize = 240;
const SCREEN_HEIGHT: usize = 160;

const TILEMAP_LAYERS: usize = 4;
const MAX_SPRITES: usize = 128;
const NUM_KEYS: usize = 10;
const NUM_SOUND_CHANNELS: usize = 6;

const ZERO_FLAG_BYTE_POS: u8 = 7;
const SUBTRACT_FLAG_BYTE_POS: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POS: u8 = 5;
const CARRY_FLAG_BYTE_POS: u8 = 4;

#[derive(Debug, Clone, Copy)]
pub struct FlagsRegister {
	zero: bool,
	subtract: bool,
	half_carry: bool,
	carry: bool,
}
impl std::convert::From<FlagsRegister> for u8 {
	fn from(flag: FlagsRegister) -> Self {
		(if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POS
			| (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POS
			| (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POS
			| (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POS
	}
}
impl std::convert::From<u8> for FlagsRegister {
	fn from(byte: u8) -> Self {
		let zero = ((byte >> ZERO_FLAG_BYTE_POS) & 0b1) != 0;
		let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POS) & 0b1) != 0;
		let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POS) & 0b1) != 0;
		let carry = ((byte >> CARRY_FLAG_BYTE_POS) & 0b1) != 0;

		FlagsRegister {
			zero,
			subtract,
			half_carry,
			carry,
		}
	}
}
pub struct Registers {
	a: u8,
	b: u8,
	c: u8,
	d: u8,
	e: u8,
	f: FlagsRegister,
	h: u8,
	l: u8,
}
impl Registers {
	pub fn get_af(&self) -> u16 {
		(self.a as u16) << 8 | u8::from(self.f) as u16
	}
	pub fn get_bc(&self) -> u16 {
		(self.b as u16) << 8 | self.c as u16
	}
	pub fn get_de(&self) -> u16 {
		(self.d as u16) << 8 | self.e as u16
	}
	pub fn set_af(&mut self, value: u16) {
		self.a = ((value & 0xFF00) >> 8) as u8;
		self.f = FlagsRegister::from((value & 0xFF) as u8);
	}
	pub fn set_bc(&mut self, value: u16) {
		self.b = ((value & 0xFF00) >> 8) as u8;
		self.c = (value & 0xFF) as u8;
	}
	pub fn set_de(&mut self, value: u16) {
		self.d = ((value & 0xFF00) >> 8) as u8;
		self.e = (value & 0xFF) as u8;
	}
}

enum Instruction {
	ADD(ArithmeticTarget),
}
enum ArithmeticTarget {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
}

pub struct GBoy {
	pc: u16,
	screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
	keys: [u8; NUM_KEYS],
}
impl GBoy {
	pub fn new() -> Self {
		GBoy {
			pc: 0,
			screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
			keys: [0; NUM_KEYS],
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
