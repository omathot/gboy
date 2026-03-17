use crate::cpu::{ArithmeticTarget, MemoryBus, instruction::Reg16Target};

const ZERO_FLAG_BYTE_POS: u8 = 7;
const SUBTRACT_FLAG_BYTE_POS: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POS: u8 = 5;
const CARRY_FLAG_BYTE_POS: u8 = 4;

#[derive(Debug, Clone, Copy)]
pub(crate) struct FlagsRegister {
	pub(crate) zero: bool,
	pub(crate) subtract: bool,
	pub(crate) half_carry: bool,
	pub(crate) carry: bool,
}
impl FlagsRegister {
	pub(crate) fn new() -> FlagsRegister {
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

pub(crate) struct Registers {
	pub(crate) a: u8,
	pub(crate) b: u8,
	pub(crate) c: u8,
	pub(crate) d: u8,
	pub(crate) e: u8,
	pub(crate) f: FlagsRegister,
	pub(crate) h: u8,
	pub(crate) l: u8,
}
impl Registers {
	pub(crate) fn new() -> Registers {
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
	pub(crate) fn value(&self, target: &ArithmeticTarget, bus: &MemoryBus) -> u8 {
		let v = match target {
			ArithmeticTarget::A => self.a,
			ArithmeticTarget::B => self.b,
			ArithmeticTarget::C => self.c,
			ArithmeticTarget::D => self.d,
			ArithmeticTarget::E => self.e,
			ArithmeticTarget::H => self.h,
			ArithmeticTarget::L => self.l,
			ArithmeticTarget::HL => bus.read_byte(self.get_hl()),
		};
		v
	}
	pub(crate) fn value_16(&self, target: &Reg16Target, sp: u16) -> u16 {
		let v = match target {
			Reg16Target::BC => self.get_bc(),
			Reg16Target::DE => self.get_de(),
			Reg16Target::HL => self.get_hl(),
			Reg16Target::SP => sp,
		};
		v
	}
	pub(crate) fn set(&mut self, target: &ArithmeticTarget, value: u8, bus: &mut MemoryBus) {
		match target {
			ArithmeticTarget::A => self.a = value,
			ArithmeticTarget::B => self.b = value,
			ArithmeticTarget::C => self.c = value,
			ArithmeticTarget::D => self.d = value,
			ArithmeticTarget::E => self.e = value,
			ArithmeticTarget::H => self.h = value,
			ArithmeticTarget::L => self.l = value,
			ArithmeticTarget::HL => bus.write_byte(self.get_hl(), value),
		}
	}
	pub(crate) fn set_16(&mut self, target: &Reg16Target, v: u16, sp: &mut u16) {
		match target {
			Reg16Target::BC => self.set_bc(v),
			Reg16Target::DE => self.set_de(v),
			Reg16Target::HL => self.set_hl(v),
			Reg16Target::SP => *sp = v,
		}
	}
	pub(crate) fn get_af(&self) -> u16 {
		(self.a as u16) << 8 | u8::from(self.f) as u16
	}
	pub(crate) fn get_bc(&self) -> u16 {
		(self.b as u16) << 8 | self.c as u16
	}
	pub(crate) fn get_de(&self) -> u16 {
		(self.d as u16) << 8 | self.e as u16
	}
	pub(crate) fn get_hl(&self) -> u16 {
		(self.h as u16) << 8 | self.l as u16
	}
	pub(crate) fn set_af(&mut self, value: u16) {
		self.a = ((value & 0xFF00) >> 8) as u8;
		self.f = FlagsRegister::from((value & 0xFF) as u8);
	}
	pub(crate) fn set_bc(&mut self, value: u16) {
		self.b = ((value & 0xFF00) >> 8) as u8;
		self.c = (value & 0xFF) as u8;
	}
	pub(crate) fn set_de(&mut self, value: u16) {
		self.d = ((value & 0xFF00) >> 8) as u8;
		self.e = (value & 0xFF) as u8;
	}
	pub(crate) fn set_hl(&mut self, value: u16) {
		self.h = ((value & 0xFF00) >> 8) as u8;
		self.l = (value & 0xFF) as u8;
	}
}
