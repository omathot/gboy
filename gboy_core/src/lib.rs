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
	ADD(ArithmeticTarget),
	ADDC(ArithmeticTarget),
	ADDHL,
	SUB(ArithmeticTarget),
	SUBC(ArithmeticTarget),
	SUBHL,
	AND(ArithmeticTarget),
	ANDHL,
	OR(ArithmeticTarget),
	ORHL,
	XOR(ArithmeticTarget),
	XORHL,
	CMP(ArithmeticTarget),
	CMPHL,
	INC(ArithmeticTarget),
	INCHL,
	DEC(ArithmeticTarget),
	DECHL,
	CCF,
	SCF,
	RRA,
	RLA,
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
	memory: [u8; 0xFFFF + 1],
}
impl MemoryBus {
	pub fn new() -> MemoryBus {
		MemoryBus {
			memory: [0; 0xFFFF + 1],
		}
	}
	fn read_byte(&self, addr: u16) -> u8 {
		self.memory[addr as usize]
	}
	fn write_byte(&mut self, addr: u16, value: u8) {
		match addr {
			0x0000..=0x7FFF => { /* ROM */ }
			0x8000..=0x9FFF => { /* vram */ }
			0xC000..=0xDFFF => { /* wram */ }
			0xE000..=0xFDFF => { /* echo ram */ }
			0xFE00..=0xFE9F => { /* OAM sprite table */ }
			0xFF00..=0xFF7F => { /* io_registers */ }
			0xFF80..=0xFFFE => { /* High ram */ }
			0xFFFF => { /* interrupt */ }
			_ => {}
		}
		self.memory[addr as usize] = value;
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
			Instruction::ADD(target) => match target {
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
			Instruction::ADDC(target) => match target {
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
			Instruction::ADDHL => self.add_hl(),
			Instruction::SUB(target) => match target {
				ArithmeticTarget::A => {
					let value = self.registers.a;
					let new_v = self.subtract(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::B => {
					let value = self.registers.b;
					let new_v = self.subtract(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::C => {
					let value = self.registers.c;
					let new_v = self.subtract(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::D => {
					let value = self.registers.d;
					let new_v = self.subtract(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::E => {
					let value = self.registers.e;
					let new_v = self.subtract(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::H => {
					let value = self.registers.h;
					let new_v = self.subtract(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::L => {
					let value = self.registers.l;
					let new_v = self.subtract(value);
					self.registers.a = new_v;
				}
			},
			Instruction::SUBC(target) => match target {
				ArithmeticTarget::A => {
					let value = self.registers.a;
					let new_v = self.subtract_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::B => {
					let value = self.registers.b;
					let new_v = self.subtract_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::C => {
					let value = self.registers.c;
					let new_v = self.subtract_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::D => {
					let value = self.registers.d;
					let new_v = self.subtract_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::E => {
					let value = self.registers.e;
					let new_v = self.subtract_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::H => {
					let value = self.registers.h;
					let new_v = self.subtract_carry(value);
					self.registers.a = new_v;
				}
				ArithmeticTarget::L => {
					let value = self.registers.l;
					let new_v = self.subtract_carry(value);
					self.registers.a = new_v;
				}
			},
			Instruction::SUBHL => self.subtract_hl(),
			Instruction::AND(target) => match target {
				ArithmeticTarget::A => self.and(self.registers.a),
				ArithmeticTarget::B => self.and(self.registers.b),
				ArithmeticTarget::C => self.and(self.registers.c),
				ArithmeticTarget::D => self.and(self.registers.d),
				ArithmeticTarget::E => self.and(self.registers.e),
				ArithmeticTarget::H => self.and(self.registers.h),
				ArithmeticTarget::L => self.and(self.registers.l),
			},
			Instruction::ANDHL => self.and_hl(),
			Instruction::OR(target) => match target {
				ArithmeticTarget::A => self.or(self.registers.a),
				ArithmeticTarget::B => self.or(self.registers.b),
				ArithmeticTarget::C => self.or(self.registers.c),
				ArithmeticTarget::D => self.or(self.registers.d),
				ArithmeticTarget::E => self.or(self.registers.e),
				ArithmeticTarget::H => self.or(self.registers.h),
				ArithmeticTarget::L => self.or(self.registers.l),
			},
			Instruction::ORHL => self.or_hl(),
			Instruction::XOR(target) => match target {
				ArithmeticTarget::A => self.xor(self.registers.a),
				ArithmeticTarget::B => self.xor(self.registers.b),
				ArithmeticTarget::C => self.xor(self.registers.c),
				ArithmeticTarget::D => self.xor(self.registers.d),
				ArithmeticTarget::E => self.xor(self.registers.e),
				ArithmeticTarget::H => self.xor(self.registers.h),
				ArithmeticTarget::L => self.xor(self.registers.l),
			},
			Instruction::XORHL => self.xor_hl(),
			Instruction::CMP(target) => match target {
				ArithmeticTarget::A => self.cmp(self.registers.a),
				ArithmeticTarget::B => self.cmp(self.registers.b),
				ArithmeticTarget::C => self.cmp(self.registers.c),
				ArithmeticTarget::D => self.cmp(self.registers.d),
				ArithmeticTarget::E => self.cmp(self.registers.e),
				ArithmeticTarget::H => self.cmp(self.registers.h),
				ArithmeticTarget::L => self.cmp(self.registers.l),
			},
			Instruction::CMPHL => self.cmp_hl(),
			Instruction::INC(target) => match target {
				ArithmeticTarget::A => {
					let new_v = self.inc(self.registers.a);
					self.registers.a = new_v;
				}
				ArithmeticTarget::B => {
					let new_v = self.inc(self.registers.b);
					self.registers.b = new_v;
				}
				ArithmeticTarget::C => {
					let new_v = self.inc(self.registers.c);
					self.registers.c = new_v;
				}
				ArithmeticTarget::D => {
					let new_v = self.inc(self.registers.d);
					self.registers.d = new_v;
				}
				ArithmeticTarget::E => {
					let new_v = self.inc(self.registers.e);
					self.registers.e = new_v;
				}
				ArithmeticTarget::H => {
					let new_v = self.inc(self.registers.h);
					self.registers.h = new_v;
				}
				ArithmeticTarget::L => {
					let new_v = self.inc(self.registers.l);
					self.registers.l = new_v;
				}
			},
			Instruction::INCHL => self.inc_hl(),
			Instruction::DEC(target) => match target {
				ArithmeticTarget::A => {
					let new_v = self.dec(self.registers.a);
					self.registers.a = new_v;
				}
				ArithmeticTarget::B => {
					let new_v = self.dec(self.registers.b);
					self.registers.b = new_v;
				}
				ArithmeticTarget::C => {
					let new_v = self.dec(self.registers.c);
					self.registers.c = new_v;
				}
				ArithmeticTarget::D => {
					let new_v = self.dec(self.registers.d);
					self.registers.d = new_v;
				}
				ArithmeticTarget::E => {
					let new_v = self.dec(self.registers.e);
					self.registers.e = new_v;
				}
				ArithmeticTarget::H => {
					let new_v = self.dec(self.registers.h);
					self.registers.h = new_v;
				}
				ArithmeticTarget::L => {
					let new_v = self.dec(self.registers.l);
					self.registers.l = new_v;
				}
			},
			Instruction::DECHL => self.dec_hl(),
			Instruction::CCF => self.registers.f.carry = !self.registers.f.carry,
			Instruction::SCF => self.registers.f.carry = true,
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
	fn subtract_carry(&mut self, value: u8) -> u8 {
		let carry = self.registers.f.carry as u8;

		let (v1, overflow1) = self.registers.a.overflowing_sub(value);
		let (v2, overflow2) = v1.overflowing_sub(carry);
		self.registers.f.zero = v2 == 0;
		self.registers.f.subtract = true;
		self.registers.f.carry = overflow1 | overflow2;
		self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
		v2
	}
	fn subtract_hl(&mut self) {
		let addr: u16 = (self.registers.h as u16) << 8 | self.registers.l as u16;
		let value = self.bus.read_byte(addr);
		let new_v = self.subtract(value);
		self.registers.a = new_v;
	}
	fn and(&mut self, value: u8) {
		self.registers.a &= value;
		self.registers.f.zero = self.registers.a == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = false;
		self.registers.f.half_carry = true; // always true for &. just what the spec states, no explanation
	}
	fn or(&mut self, value: u8) {
		self.registers.a |= value;
		self.registers.f.zero = self.registers.a == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = false;
		self.registers.f.half_carry = false;
	}
	fn and_hl(&mut self) {
		let addr: u16 = (self.registers.h as u16) << 8 | self.registers.l as u16;
		let value = self.bus.read_byte(addr);
		self.and(value);
	}
	fn or_hl(&mut self) {
		let addr: u16 = (self.registers.h as u16) << 8 | self.registers.l as u16;
		let value = self.bus.read_byte(addr);
		self.or(value);
	}
	fn xor(&mut self, value: u8) {
		self.registers.a ^= value;
		self.registers.f.zero = self.registers.a == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = false;
		self.registers.f.half_carry = false;
	}
	fn xor_hl(&mut self) {
		let addr: u16 = (self.registers.h as u16) << 8 | self.registers.l as u16;
		let value = self.bus.read_byte(addr);
		self.xor(value);
	}
	fn cmp(&mut self, value: u8) {
		self.subtract(value);
	}
	fn cmp_hl(&mut self) {
		let addr: u16 = (self.registers.h as u16) << 8 | self.registers.l as u16;
		let value = self.bus.read_byte(addr);
		self.subtract(value);
	}
	fn inc(&mut self, value: u8) -> u8 {
		let new_v = value.wrapping_add(1);
		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = false;
		self.registers.f.half_carry = (value & 0xF) == 0xF;
		new_v
	}
	fn inc_hl(&mut self) {
		let addr: u16 = (self.registers.h as u16) << 8 | self.registers.l as u16;
		let value = self.bus.read_byte(addr);
		let new_v = self.inc(value);
		self.bus.write_byte(addr, new_v);
	}
	fn dec(&mut self, value: u8) -> u8 {
		let new_v = value.wrapping_sub(1);
		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = true;
		self.registers.f.half_carry = (value & 0xF) == 0x0;
		new_v
	}
	fn dec_hl(&mut self) {
		let addr: u16 = (self.registers.h as u16) << 8 | self.registers.l as u16;
		let value = self.bus.read_byte(addr);
		let new_v = self.dec(value);
		self.bus.write_byte(addr, new_v);
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
