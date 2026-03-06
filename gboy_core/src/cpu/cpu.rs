use crate::cpu::{Instruction, JumpTest, MemoryBus, Registers};

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
	fn step(&mut self) {
		let mut opcode = self.bus.read_byte(self.pc);
		let prefixed = opcode == 0xCB;
		if prefixed {
			opcode = self.bus.read_byte(self.pc + 1);
		}

		let next_pc = if let Some(instruction) = Instruction::from_byte(opcode, prefixed) {
			self.execute(instruction)
		} else {
			panic!("Unknown instruction found: 0x{:x}", opcode);
		};
		self.pc = next_pc;
	}
	fn jump(&self, should_jump: bool) -> u16 {
		if should_jump {
			// little endian
			let least = self.bus.read_byte(self.pc + 1) as u16;
			let most = self.bus.read_byte(self.pc + 2) as u16;
			(most << 8) | least
		} else {
			// 1 byte for tag + 2 for jp addr
			self.pc.wrapping_add(3)
		}
	}
	fn execute(&mut self, instruction: Instruction) -> u16 {
		let next_pc = self.pc.wrapping_add(1);
		match instruction {
			Instruction::JP(test) => {
				let jump_condition = match test {
					JumpTest::NotZero => !self.registers.f.zero,
					JumpTest::NotCarry => !self.registers.f.carry,
					JumpTest::Zero => self.registers.f.zero,
					JumpTest::Carry => self.registers.f.carry,
					JumpTest::Always => true,
				};
				self.jump(jump_condition)
			}
			Instruction::ADD(target) => {
				let v = self.registers.value(&target);
				self.registers.a = self.add(v);
				next_pc
			}
			Instruction::ADDC(target) => {
				let v = self.registers.value(&target);
				self.registers.a = self.add_carry(v);
				next_pc
			}
			Instruction::ADDHL => {
				self.add_hl();
				next_pc
			}
			Instruction::SUB(target) => {
				let v = self.registers.value(&target);
				self.registers.a = self.subtract(v);
				next_pc
			}
			Instruction::SUBC(target) => {
				let v = self.registers.value(&target);
				self.registers.a = self.subtract_carry(v);
				next_pc
			}
			Instruction::SUBHL => {
				self.subtract_hl();
				next_pc
			}
			Instruction::AND(target) => {
				let v = self.registers.value(&target);
				self.and(v);
				next_pc
			}
			Instruction::ANDHL => {
				self.and_hl();
				next_pc
			}
			Instruction::OR(target) => {
				let v = self.registers.value(&target);
				self.or(v);
				next_pc
			}
			Instruction::ORHL => {
				self.or_hl();
				next_pc
			}
			Instruction::XOR(target) => {
				let v = self.registers.value(&target);
				self.xor(v);
				next_pc
			}
			Instruction::XORHL => {
				self.xor_hl();
				next_pc
			}
			Instruction::CMP(target) => {
				let v = self.registers.value(&target);
				self.cmp(v);
				next_pc
			}
			Instruction::CMPHL => {
				self.cmp_hl();
				next_pc
			}
			Instruction::INC(target) => {
				let v = self.registers.value(&target);
				let new_v = self.inc(v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::INCHL => {
				self.inc_hl();
				next_pc
			}
			Instruction::DEC(target) => {
				let v = self.registers.value(&target);
				let new_v = self.dec(v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::DECHL => {
				self.dec_hl();
				next_pc
			}
			Instruction::CCF => {
				self.registers.f.carry = !self.registers.f.carry;
				self.registers.f.subtract = false;
				self.registers.f.half_carry = false;
				next_pc
			}
			Instruction::SCF => {
				self.registers.f.carry = true;
				self.registers.f.subtract = false;
				self.registers.f.half_carry = false;
				next_pc
			}
			Instruction::RRA => {
				self.rra();
				next_pc
			}
			Instruction::RLA => {
				self.rla();
				next_pc
			}
			Instruction::RRCA => {
				self.rrca();
				next_pc
			}
			Instruction::RLCA => {
				self.rlca();
				next_pc
			}
			Instruction::CPL => {
				self.cpl();
				next_pc
			}
			Instruction::BIT(idx, target) => {
				let v = self.registers.value(&target);
				self.bit(idx, v);
				next_pc
			}
			Instruction::BITHL(idx) => {
				self.bit_hl(idx);
				next_pc
			}
			Instruction::RESET(idx, target) => {
				let v = self.registers.value(&target);
				let new_v = self.reset(idx, v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::RESETHL(idx) => {
				self.reset_hl(idx);
				next_pc
			}
			Instruction::SET(idx, target) => {
				let v = self.registers.value(&target);
				let new_v = self.set(idx, v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::SETHL(idx) => {
				self.set_hl(idx);
				next_pc
			}
			Instruction::SRL(target) => {
				let v = self.registers.value(&target);
				let new_v = self.srl(v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::SRLHL => {
				self.srl_hl();
				next_pc
			}
			Instruction::RR(target) => {
				let v = self.registers.value(&target);
				let new_v = self.rr(v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::RRHL => {
				self.rr_hl();
				next_pc
			}
			Instruction::RL(target) => {
				let v = self.registers.value(&target);
				let new_v = self.rl(v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::RLHL => {
				self.rl_hl();
				next_pc
			}
			Instruction::RRC(target) => {
				let v = self.registers.value(&target);
				let new_v = self.rrc(v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::RRCHL => {
				self.rrc_hl();
				next_pc
			}
			Instruction::RLC(target) => {
				let v = self.registers.value(&target);
				let new_v = self.rlc(v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::RLCHL => {
				self.rlc_hl();
				next_pc
			}
			Instruction::SRA(target) => {
				let v = self.registers.value(&target);
				let new_v = self.sra(v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::SRAHL => {
				self.sra_hl();
				next_pc
			}
			Instruction::SLA(target) => {
				let v = self.registers.value(&target);
				let new_v = self.sla(v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::SLAHL => {
				self.sla_hl();
				next_pc
			}
			Instruction::SWAP(target) => {
				let v = self.registers.value(&target);
				let new_v = self.swap(v);
				self.registers.set(&target, new_v);
				next_pc
			}
			Instruction::SWAPHL => {
				self.swap_hl();
				next_pc
			}
			// TODO: support more insturctions
			_ => next_pc,
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
	fn sra(&mut self, value: u8) -> u8 {
		let bit7 = value & 0x80;
		let new_carry = value & 0x1;
		let new_v = (value >> 1) | bit7;

		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = new_carry != 0;
		self.registers.f.half_carry = false;
		new_v
	}
	fn sra_hl(&mut self) {
		let addr = self.registers.get_hl();
		let v = self.bus.read_byte(addr);
		let new_v = self.sra(v);
		self.bus.write_byte(addr, new_v);
	}
	fn sla(&mut self, value: u8) -> u8 {
		let new_carry = value & 0x80;
		let new_v = value << 1;

		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = new_carry != 0;
		self.registers.f.half_carry = false;
		new_v
	}
	fn sla_hl(&mut self) {
		let addr = self.registers.get_hl();
		let v = self.bus.read_byte(addr);
		let new_v = self.sla(v);
		self.bus.write_byte(addr, new_v);
	}
	fn swap(&mut self, value: u8) -> u8 {
		let high = value >> 4;
		let low = value & 0x0F;
		let new_v = (low << 4) | high;

		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = false;
		self.registers.f.half_carry = false;
		new_v
	}
	fn swap_hl(&mut self) {
		let addr = self.registers.get_hl();
		let v = self.bus.read_byte(addr);
		let new_v = self.swap(v);
		self.bus.write_byte(addr, new_v);
	}
}
