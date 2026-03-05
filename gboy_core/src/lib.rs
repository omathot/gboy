/*
	-cpu: Sharp LR35902
*/

// 20x18 tiles
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

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
	Add(ArithmeticTarget),
	AddCarry(ArithmeticTarget),
	AddHL,
	Subtract(ArithmeticTarget),
	SubtractCarry(ArithmeticTarget),
	Compare(ArithmeticTarget),
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

struct MemoryBus {
	memory: [u8; 0xFFFF],
}

impl MemoryBus {
	pub fn new() -> MemoryBus {
		MemoryBus {
			memory: [0; 0xFFFF],
		}
	}
	fn read_byte(&self, address: u16) -> u8 {
		self.memory[address as usize]
	}
}

pub struct CPU {
	registers: Registers,
	bus: MemoryBus,
}
impl CPU {
	pub fn new() -> CPU {
		CPU {
			registers: Registers::new(),
			bus: MemoryBus::new(),
		}
	}
	fn execute(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::Add(target) => match target {
				ArithmeticTarget::A => {
					let value = self.registers.a;
					let new_v = self.add(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::B => {
					let value = self.registers.b;
					let new_v = self.add(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::C => {
					let value = self.registers.c;
					let new_v = self.add(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::D => {
					let value = self.registers.d;
					let new_v = self.add(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::E => {
					let value = self.registers.e;
					let new_v = self.add(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::H => {
					let value = self.registers.h;
					let new_v = self.add(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::L => {
					let value = self.registers.l;
					let new_v = self.add(value);
					self.registers.a = new_v;
				}
			},
			Instruction::AddCarry(target) => match target {
				ArithmeticTarget::A => {
					let value = self.registers.a;
					let new_v = self.add_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::B => {
					let value = self.registers.b;
					let new_v = self.add_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::C => {
					let value = self.registers.c;
					let new_v = self.add_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::D => {
					let value = self.registers.d;
					let new_v = self.add_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::E => {
					let value = self.registers.e;
					let new_v = self.add_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::H => {
					let value = self.registers.h;
					let new_v = self.add_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::L => {
					let value = self.registers.l;
					let new_v = self.add_carry(value);
					self.registers.a = new_v;
				}
			},
			Instruction::AddHL => self.add_hl(),
			Instruction::Subtract(target) => match target {
				ArithmeticTarget::A => {
					let value = self.registers.a;
					let new_v = self.subtract(value);
					self.registers.a = new_v;
				}
				_ => {}
			},
			// TODO: support more insturctions
			_ => {}
		}
	}
	fn add(&mut self, value: u8) -> u8 {
		let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
		self.registers.f.zero = new_value == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = did_overflow;
		self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
		new_value
	}
	fn add_carry(&mut self, value: u8) -> u8 {
		let carry = self.registers.f.carry as u8;
		let (v1, overflow1) = self.registers.a.overflowing_add(value);
		let (v2, overflow2) = v1.overflowing_add(carry);

		self.registers.f.zero = v2 == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = overflow1 | overflow2;
		self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + carry > 0xF;
		v2
	}
	fn add_hl(&mut self) {
		let addr: u16 = (self.registers.h as u16) << 8 | self.registers.l as u16;
		let value = self.bus.read_byte(addr);
		let new_v = self.add(value);
		self.registers.a = new_v;
	}
	fn subtract(&mut self, value: u8) -> u8 {
		let (new_v, overflow) = self.registers.a.overflowing_sub(value);
		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = true;
		self.registers.f.carry = overflow;
		self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);

		new_v
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
