use crate::cpu::{ArithmeticTarget, Instruction, Registers};

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
			0x0000..=0x00FF => { /* Boot ROM */ }
			0x0000..=0x3FFF => { /* Game ROM Bank 0 */ }
			0x4000..=0x7FFF => { /* Game ROM Bank N */ }
			0x8000..=0x97FF => { /* Tile RAM */ }
			0x9800..=0x9FFF => { /* Background Map */ }
			0xA000..=0xBFFF => { /* Cartridge RAM */ }
			0xC000..=0xDFFF => { /* Working RAM */ }
			0xE000..=0xFDFF => { /* echo ram */ }
			0xFE00..=0xFE9F => { /* OAM sprite table */ }
			0xFEA0..=0xFEFF => { /* Unused */ }
			0xFF00..=0xFF7F => { /* io_registers */ }
			0xFF80..=0xFFFE => { /* High ram */ }
			0xFFFF => { /* interrupt */ }
		}
		// TODO: write byte in relevant addr section
		self.memory[addr as usize] = value;
	}
}

pub(crate) struct CPU {
	registers: Registers,
	bus: MemoryBus,
	pc: u16,
}
impl CPU {
	pub fn new() -> CPU {
		CPU {
			registers: Registers::new(),
			bus: MemoryBus::new(),
			pc: 0,
		}
	}
	fn execute(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::ADD(target) => {
				let v = self.registers.value(&target);
				self.registers.a = self.add(v);
			}
			Instruction::ADDC(target) => {
				let v = self.registers.value(&target);
				self.registers.a = self.add_carry(v);
			}
			Instruction::ADDHL => self.add_hl(),
			Instruction::SUB(target) => {
				let v = self.registers.value(&target);
				self.registers.a = self.subtract(v);
			}
			Instruction::SUBC(target) => {
				let v = self.registers.value(&target);
				self.registers.a = self.subtract_carry(v);
			}
			Instruction::SUBHL => self.subtract_hl(),
			Instruction::AND(target) => {
				let v = self.registers.value(&target);
				self.and(v);
			}
			Instruction::ANDHL => self.and_hl(),
			Instruction::OR(target) => {
				let v = self.registers.value(&target);
				self.or(v);
			}
			Instruction::ORHL => self.or_hl(),
			Instruction::XOR(target) => {
				let v = self.registers.value(&target);
				self.xor(v);
			}
			Instruction::XORHL => self.xor_hl(),
			Instruction::CMP(target) => {
				let v = self.registers.value(&target);
				self.cmp(v);
			}
			Instruction::CMPHL => self.cmp_hl(),
			Instruction::INC(target) => {
				let v = self.registers.value(&target);
				let new_v = self.inc(v);
				self.registers.set(&target, new_v);
			}
			Instruction::INCHL => self.inc_hl(),
			Instruction::DEC(target) => {
				let v = self.registers.value(&target);
				let new_v = self.dec(v);
				self.registers.set(&target, new_v);
			}
			Instruction::DECHL => self.dec_hl(),
			Instruction::CCF => {
				self.registers.f.carry = !self.registers.f.carry;
				self.registers.f.subtract = false;
				self.registers.f.half_carry = false;
			}
			Instruction::SCF => {
				self.registers.f.carry = true;
				self.registers.f.subtract = false;
				self.registers.f.half_carry = false;
			}
			Instruction::RRA => self.rra(),
			Instruction::RLA => self.rla(),
			Instruction::RRCA => self.rrca(),
			Instruction::RLCA => self.rlca(),
			Instruction::CPL => self.cpl(),
			Instruction::BIT(idx, target) => {
				let v = self.registers.value(&target);
				self.bit(idx, v);
			}
			Instruction::BITHL(idx) => self.bit_hl(idx),
			Instruction::RESET(idx, target) => {
				let v = self.registers.value(&target);
				let new_v = self.reset(idx, v);
				self.registers.set(&target, new_v);
			}
			Instruction::RESETHL(idx) => self.reset_hl(idx),
			Instruction::SET(idx, target) => {
				let v = self.registers.value(&target);
				let new_v = self.set(idx, v);
				self.registers.set(&target, new_v);
			}
			Instruction::SETHL(idx) => self.set_hl(idx),
			Instruction::SRL(target) => {
				let v = self.registers.value(&target);
				let new_v = self.srl(v);
				self.registers.set(&target, new_v);
			}
			Instruction::SRLHL => self.srl_hl(),
			Instruction::RR(target) => {
				let v = self.registers.value(&target);
				let new_v = self.rr(v);
				self.registers.set(&target, new_v);
			}
			Instruction::RRHL => self.rr_hl(),
			Instruction::RL(target) => {
				let v = self.registers.value(&target);
				let new_v = self.rl(v);
				self.registers.set(&target, new_v);
			}
			Instruction::RLHL => self.rl_hl(),
			Instruction::RRC(target) => {
				let v = self.registers.value(&target);
				let new_v = self.rrc(v);
				self.registers.set(&target, new_v);
			}
			Instruction::RRCHL => self.rrc_hl(),
			Instruction::RLC(target) => {
				let v = self.registers.value(&target);
				let new_v = self.rlc(v);
				self.registers.set(&target, new_v);
			}
			Instruction::RLCHL => self.rlc_hl(),
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
		let addr = self.registers.get_hl();
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
		let addr = self.registers.get_hl();
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
		let addr = self.registers.get_hl();
		let value = self.bus.read_byte(addr);
		self.and(value);
	}
	fn or_hl(&mut self) {
		let addr = self.registers.get_hl();
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
		let addr = self.registers.get_hl();
		let value = self.bus.read_byte(addr);
		self.xor(value);
	}
	fn cmp(&mut self, value: u8) {
		self.subtract(value);
	}
	fn cmp_hl(&mut self) {
		let addr = self.registers.get_hl();
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
		let addr = self.registers.get_hl();
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
		let addr = self.registers.get_hl();
		let value = self.bus.read_byte(addr);
		let new_v = self.dec(value);
		self.bus.write_byte(addr, new_v);
	}
	fn rra(&mut self) {
		let old_carry = self.registers.f.carry as u8;
		let new_carry = self.registers.a & 0x1; // bit 0 falls

		self.registers.a = (self.registers.a >> 1) | (old_carry << 7);
		self.registers.f.zero = false;
		self.registers.f.subtract = false;
		self.registers.f.carry = new_carry != 0;
		self.registers.f.half_carry = false;
	}
	fn rla(&mut self) {
		let old_carry = self.registers.f.carry as u8;
		let new_carry = self.registers.a & 0x80; // bit 7 falls

		self.registers.a = (self.registers.a << 1) | old_carry;
		self.registers.f.zero = false;
		self.registers.f.subtract = false;
		self.registers.f.carry = new_carry != 0;
		self.registers.f.half_carry = false;
	}
	fn rrca(&mut self) {
		let bit0 = self.registers.a & 0x1;
		self.registers.a = (self.registers.a >> 1) | (bit0 << 7);
		self.registers.f.zero = false;
		self.registers.f.subtract = false;
		self.registers.f.carry = bit0 != 0;
		self.registers.f.half_carry = false;
	}
	fn rlca(&mut self) {
		let bit7 = self.registers.a & 0x80;
		self.registers.a = (self.registers.a << 1) | (bit7 >> 7);
		self.registers.f.zero = false;
		self.registers.f.subtract = false;
		self.registers.f.carry = bit7 != 0;
		self.registers.f.half_carry = false;
	}
	fn cpl(&mut self) {
		self.registers.a = !self.registers.a;
		self.registers.f.half_carry = true;
		self.registers.f.subtract = true;
	}
	fn bit(&mut self, idx: u8, value: u8) {
		let mask = 1 << idx;
		self.registers.f.zero = (value & mask) == 0;
		self.registers.f.subtract = false;
		self.registers.f.half_carry = true;
	}
	fn bit_hl(&mut self, idx: u8) {
		let addr = self.registers.get_hl();
		let value = self.bus.read_byte(addr);
		let mask = 1 << idx;

		self.registers.f.zero = (value & mask) == 0;
		self.registers.f.subtract = false;
		self.registers.f.half_carry = true;
	}
	fn reset(&mut self, idx: u8, value: u8) -> u8 {
		let mask = 1 << idx;
		value & !mask
	}
	fn reset_hl(&mut self, idx: u8) {
		let addr = self.registers.get_hl();
		let value = self.bus.read_byte(addr);
		let mask = 1 << idx;
		self.bus.write_byte(addr, value & !mask);
	}
	fn set(&mut self, idx: u8, value: u8) -> u8 {
		let mask = 1 << idx;
		value | mask
	}
	fn set_hl(&mut self, idx: u8) {
		let addr = self.registers.get_hl();
		let value = self.bus.read_byte(addr);
		let mask = 1 << idx;
		self.bus.write_byte(addr, value | mask);
	}
	fn srl(&mut self, value: u8) -> u8 {
		let new_carry = value & 0x1;
		let new_v = value >> 1;

		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = new_carry != 0;
		self.registers.f.half_carry = false;
		new_v
	}
	fn srl_hl(&mut self) {
		let addr = self.registers.get_hl();
		let v = self.bus.read_byte(addr);
		let new_v = self.srl(v);
		self.bus.write_byte(addr, new_v);
	}
	fn rr(&mut self, value: u8) -> u8 {
		let old_carry = self.registers.f.carry as u8;
		let new_carry = value & 0x1;
		let new_v = (value >> 1) | (old_carry << 7);

		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = false;
		self.registers.f.half_carry = false;
		self.registers.f.carry = new_carry != 0;
		new_v
	}
	fn rr_hl(&mut self) {
		let addr = self.registers.get_hl();
		let v = self.bus.read_byte(addr);
		let new_v = self.rr(v);
		self.bus.write_byte(addr, new_v);
	}
	fn rl(&mut self, value: u8) -> u8 {
		let old_carry = self.registers.f.carry as u8;
		let new_carry = value & 0x80;
		let new_v = (value << 1) | old_carry;

		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = new_carry != 0;
		self.registers.f.half_carry = false;
		new_v
	}
	fn rl_hl(&mut self) {
		let addr = self.registers.get_hl();
		let v = self.bus.read_byte(addr);
		let new_v = self.rl(v);
		self.bus.write_byte(addr, new_v);
	}
	fn rrc(&mut self, value: u8) -> u8 {
		let bit0 = value & 0x1;
		let new_v = (value >> 1) | (bit0 << 7);

		self.registers.f.zero = new_v == 0;
		self.registers.f.carry = bit0 != 0;
		self.registers.f.half_carry = false;
		self.registers.f.subtract = false;
		new_v
	}
	fn rrc_hl(&mut self) {
		let addr = self.registers.get_hl();
		let v = self.bus.read_byte(addr);
		let new_v = self.rrc(v);
		self.bus.write_byte(addr, new_v);
	}
	fn rlc(&mut self, value: u8) -> u8 {
		let bit7 = value & 0x80;
		let new_v = (value << 1) | (bit7 >> 7);

		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = bit7 != 0;
		self.registers.f.half_carry = false;
		new_v
	}
	fn rlc_hl(&mut self) {
		let addr = self.registers.get_hl();
		let v = self.bus.read_byte(addr);
		let new_v = self.rlc(v);
		self.bus.write_byte(addr, new_v);
	}
}
