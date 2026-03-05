/*
	-Sharp LR35902
*/

const SCREEN_WIDTH: usize = 240;
const SCREEN_HEIGHT: usize = 160;

const NUM_KEYS: usize = 10;

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
impl FlagsRegister {
	pub fn new() -> FlagsRegister {
		FlagsRegister {
			zero: false,
			subtract: false,
			half_carry: false,
			carry: false,
		}
	}
}
impl std::convert::From<FlagsRegister> for u8 {
	fn from(flag: FlagsRegister) -> u8 {
		(if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POS
			| (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POS
			| (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POS
			| (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POS
	}
}
impl std::convert::From<u8> for FlagsRegister {
	fn from(byte: u8) -> FlagsRegister {
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
	pub fn new() -> Registers {
		Registers {
			a: 0,
			b: 0,
			c: 0,
			d: 0,
			e: 0,
			f: FlagsRegister::new(),
			h: 0,
			l: 0,
		}
	}
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

pub enum Instruction {
	ADD(ArithmeticTarget),
}
pub enum ArithmeticTarget {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
}

pub struct CPU {
	registers: Registers,
}
impl CPU {
	pub fn new() -> CPU {
		CPU {
			registers: Registers::new(),
		}
	}
}

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
