use super::instruction::{
	ArithmeticTarget, Instruction, JumpTest, LoadType, Reg8Source, Reg8Target, Reg16Source,
	Reg16Target, StackTarget,
};
use crate::cpu::{MemoryBus, Registers};

pub(crate) struct CPU {
	registers: Registers,
	bus: MemoryBus,
	pc: u16,
	sp: u16,
}
impl CPU {
	pub fn new() -> CPU {
		CPU {
			registers: Registers::new(),
			bus: MemoryBus::new(),
			pc: 0,
			sp: 0,
		}
	}
	fn step(&mut self) {
		let mut opcode = self.bus.read_byte(self.pc);
		let prefixed = opcode == 0xCB;
		if prefixed {
			// move past prefix
			self.pc = self.pc.wrapping_add(1);
			opcode = self.bus.read_byte(self.pc);
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
	fn push(&mut self, value: u16) {
		self.sp = self.sp.wrapping_sub(1);
		self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

		self.sp = self.sp.wrapping_sub(1);
		self.bus.write_byte(self.sp, (value & 0xFF) as u8);
	}
	fn pop(&mut self) -> u16 {
		let least = self.bus.read_byte(self.sp) as u16;
		self.sp = self.sp.wrapping_add(1);

		let most = self.bus.read_byte(self.sp) as u16;
		self.sp = self.sp.wrapping_add(1);

		(most << 8) | least
	}
	fn call(&mut self, should_jump: bool) -> u16 {
		let next_pc = self.pc.wrapping_add(3);
		if should_jump {
			self.push(next_pc);
			// TODO:
			// self.read_next_word()
			0 // tmp
		} else {
			next_pc
		}
	}
	fn return_(&mut self, should_jump: bool) -> u16 {
		if should_jump {
			self.pop()
		} else {
			self.pc.wrapping_add(1)
		}
	}
	fn execute(&mut self, instruction: Instruction) -> u16 {
		let mut next_pc = self.pc.wrapping_add(1);
		match instruction {
			Instruction::NOP => next_pc,
			Instruction::STOP => {
				// TODO: Implement low power mode / speed switch
				// for now treat it as 2 byte NOP
				next_pc = next_pc.wrapping_add(1);
				next_pc
			}
			Instruction::HALT => {
				// TODO: Implement CPU low power mode until interrupt occurs
				// depends on IME flag
				// for now treat as NOP
				next_pc
			}
			Instruction::PUSH(target) => {
				let v = match target {
					StackTarget::BC => self.registers.get_bc(),
					StackTarget::DE => self.registers.get_de(),
					StackTarget::HL => self.registers.get_hl(),
					_ => panic!("Invalid target for PUSH"),
				};
				self.push(v);
				next_pc
			}
			Instruction::POP(target) => {
				let v = self.pop();
				match target {
					StackTarget::BC => self.registers.set_bc(v),
					_ => panic!("Invalid target for POP"),
				};
				next_pc
			}
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
			Instruction::JR(test) => {
				let should_jump = match test {
					JumpTest::NotZero => !self.registers.f.zero,
					JumpTest::NotCarry => !self.registers.f.carry,
					JumpTest::Zero => self.registers.f.zero,
					JumpTest::Carry => self.registers.f.carry,
					JumpTest::Always => true,
				};
				// offset byte bump
				next_pc = next_pc.wrapping_add(1);
				if should_jump {
					let offset = self.bus.read_byte(self.pc + 1) as i8;
					next_pc.wrapping_add(offset as u16)
				} else {
					next_pc
				}
			}
			Instruction::LD(load_type) => match load_type {
				LoadType::Byte(target, source) => {
					let source_v = match source {
						Reg8Source::A => self.registers.a,
						Reg8Source::B => self.registers.b,
						Reg8Source::C => self.registers.c,
						Reg8Source::D => self.registers.d,
						Reg8Source::E => self.registers.e,
						Reg8Source::H => self.registers.h,
						Reg8Source::L => self.registers.l,
						Reg8Source::BCI => self.bus.read_byte(self.registers.get_bc()),
						Reg8Source::DEI => self.bus.read_byte(self.registers.get_de()),
						Reg8Source::HLI => self.bus.read_byte(self.registers.get_hl()),
						Reg8Source::D8 => {
							next_pc = next_pc.wrapping_add(1); // consumed an extra byte, bump next_pc
							self.bus.read_byte(self.pc + 1)
						}
						Reg8Source::HLINC => {
							let v = self.bus.read_byte(self.registers.get_hl());
							self.registers
								.set_hl(self.registers.get_hl().wrapping_add(1));
							v
						}
						Reg8Source::HLDEC => {
							let v = self.bus.read_byte(self.registers.get_hl());
							self.registers
								.set_hl(self.registers.get_hl().wrapping_sub(1));
							v
						}
					};
					match target {
						Reg8Target::A => self.registers.a = source_v,
						Reg8Target::B => self.registers.b = source_v,
						Reg8Target::C => self.registers.c = source_v,
						Reg8Target::D => self.registers.d = source_v,
						Reg8Target::E => self.registers.e = source_v,
						Reg8Target::H => self.registers.h = source_v,
						Reg8Target::L => self.registers.l = source_v,
						Reg8Target::BCI => self.bus.write_byte(self.registers.get_bc(), source_v),
						Reg8Target::DEI => self.bus.write_byte(self.registers.get_de(), source_v),
						Reg8Target::HLI => self.bus.write_byte(self.registers.get_hl(), source_v),
						Reg8Target::HLINC => {
							self.bus.write_byte(self.registers.get_hl(), source_v);
							self.registers
								.set_hl(self.registers.get_hl().wrapping_add(1));
						}
						Reg8Target::HLDEC => {
							self.bus.write_byte(self.registers.get_hl(), source_v);
							self.registers
								.set_hl(self.registers.get_hl().wrapping_sub(1));
						}
					}
					next_pc
				}
				LoadType::Word(target, source) => {
					let source_v = match source {
						Reg16Source::D16 => {
							let least = self.bus.read_byte(self.pc + 1) as u16;
							let most = self.bus.read_byte(self.pc + 2) as u16;
							(most << 8) | least
						}
					};
					self.registers.set_16(&target, source_v, &mut self.sp);
					self.pc.wrapping_add(3)
				}
				LoadType::IndirectFromSP => {
					let least = self.bus.read_byte(self.pc + 1) as u16;
					let most = self.bus.read_byte(self.pc + 2) as u16;
					let addr = (most << 8) | least;
					self.bus.write_byte(addr, (self.sp & 0xFF) as u8);
					self.bus.write_byte(addr + 1, (self.sp >> 8) as u8);

					self.pc.wrapping_add(3)
				}
			},
			// ARITHMETIC
			Instruction::ADD(target) => {
				let v = match target {
					ArithmeticTarget::D8 => {
						next_pc = next_pc.wrapping_add(1);
						self.bus.read_byte(self.pc + 1)
					}
					_ => self.registers.value(&target, &self.bus),
				};
				self.registers.a = self.add(v);
				next_pc
			}
			Instruction::ADD16(target) => {
				let v = self.registers.value_16(&target, self.sp);
				self.add_16(v); // always add to hl
				next_pc
			}
			Instruction::ADDC(target) => {
				let v = match target {
					ArithmeticTarget::D8 => {
						next_pc = next_pc.wrapping_add(1);
						self.bus.read_byte(self.pc + 1)
					}
					_ => self.registers.value(&target, &self.bus),
				};
				self.registers.a = self.add_carry(v);
				next_pc
			}
			Instruction::SUB(target) => {
				let v = match target {
					ArithmeticTarget::D8 => {
						next_pc = next_pc.wrapping_add(1);
						self.bus.read_byte(self.pc + 1)
					}
					_ => self.registers.value(&target, &self.bus),
				};
				self.registers.a = self.subtract(v);
				next_pc
			}
			Instruction::SUBC(target) => {
				let v = match target {
					ArithmeticTarget::D8 => {
						next_pc = next_pc.wrapping_add(1);
						self.bus.read_byte(self.pc + 1)
					}
					_ => self.registers.value(&target, &self.bus),
				};
				self.registers.a = self.subtract_carry(v);
				next_pc
			}
			Instruction::AND(target) => {
				let v = match target {
					ArithmeticTarget::D8 => {
						next_pc = next_pc.wrapping_add(1);
						self.bus.read_byte(self.pc + 1)
					}
					_ => self.registers.value(&target, &self.bus),
				};
				self.and(v);
				next_pc
			}
			Instruction::OR(target) => {
				let v = match target {
					ArithmeticTarget::D8 => {
						next_pc = next_pc.wrapping_add(1);
						self.bus.read_byte(self.pc + 1)
					}
					_ => self.registers.value(&target, &self.bus),
				};
				self.or(v);
				next_pc
			}
			Instruction::XOR(target) => {
				let v = match target {
					ArithmeticTarget::D8 => {
						next_pc = next_pc.wrapping_add(1);
						self.bus.read_byte(self.pc + 1)
					}
					_ => self.registers.value(&target, &self.bus),
				};
				self.xor(v);
				next_pc
			}
			Instruction::CMP(target) => {
				let v = match target {
					ArithmeticTarget::D8 => {
						next_pc = next_pc.wrapping_add(1);
						self.bus.read_byte(self.pc + 1)
					}
					_ => self.registers.value(&target, &self.bus),
				};
				self.cmp(v);
				next_pc
			}
			Instruction::INC(target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.inc(v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::INC16(target) => {
				let v = self.registers.value_16(&target, self.sp);
				let new_v = v.wrapping_add(1);
				self.registers.set_16(&target, new_v, &mut self.sp);
				next_pc
			}
			Instruction::DEC(target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.dec(v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::DEC16(target) => {
				let v = self.registers.value_16(&target, self.sp);
				let new_v = v.wrapping_sub(1);
				self.registers.set_16(&target, new_v, &mut self.sp);
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
				let v = self.registers.value(&target, &self.bus);
				self.bit(idx, v);
				next_pc
			}
			Instruction::RESET(idx, target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.reset(idx, v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::SET(idx, target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.set(idx, v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::SRL(target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.srl(v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::RR(target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.rr(v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::RL(target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.rl(v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::RRC(target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.rrc(v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::RLC(target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.rlc(v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::SRA(target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.sra(v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::SLA(target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.sla(v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::SWAP(target) => {
				let v = self.registers.value(&target, &self.bus);
				let new_v = self.swap(v);
				self.registers.set(&target, new_v, &mut self.bus);
				next_pc
			}
			Instruction::DAA => {
				let mut adjustment = 0;
				if self.registers.f.subtract {
					if self.registers.f.half_carry {
						adjustment += 0x06;
					}
					if self.registers.f.carry {
						adjustment += 0x60;
					}
					self.registers.a = self.registers.a.wrapping_sub(adjustment);
				} else {
					if self.registers.f.half_carry || (self.registers.a & 0x0F) > 0x09 {
						adjustment += 0x06;
					}
					if self.registers.f.carry || self.registers.a > 0x99 {
						adjustment += 0x60;
						self.registers.f.carry = true;
					}
					self.registers.a = self.registers.a.wrapping_add(adjustment);
				}
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.half_carry = false;
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
	fn add_16(&mut self, v: u16) {
		let hl = self.registers.get_hl();

		let (res, overflow) = hl.overflowing_add(v);
		self.registers.f.subtract = false;
		self.registers.f.carry = overflow;
		self.registers.f.half_carry = (hl & 0x0FFF) + (v & 0x0FFF) > 0x0FFF;
		self.registers.set_hl(res);
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
	fn xor(&mut self, value: u8) {
		self.registers.a ^= value;
		self.registers.f.zero = self.registers.a == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = false;
		self.registers.f.half_carry = false;
	}
	fn cmp(&mut self, value: u8) {
		// just set flags
		self.subtract(value);
	}
	fn inc(&mut self, value: u8) -> u8 {
		let new_v = value.wrapping_add(1);
		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = false;
		self.registers.f.half_carry = (value & 0xF) == 0xF;
		new_v
	}
	fn dec(&mut self, value: u8) -> u8 {
		let new_v = value.wrapping_sub(1);
		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = true;
		self.registers.f.half_carry = (value & 0xF) == 0x0;
		new_v
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
	fn reset(&mut self, idx: u8, value: u8) -> u8 {
		let mask = 1 << idx;
		value & !mask
	}
	fn set(&mut self, idx: u8, value: u8) -> u8 {
		let mask = 1 << idx;
		value | mask
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
	fn rrc(&mut self, value: u8) -> u8 {
		let bit0 = value & 0x1;
		let new_v = (value >> 1) | (bit0 << 7);

		self.registers.f.zero = new_v == 0;
		self.registers.f.carry = bit0 != 0;
		self.registers.f.half_carry = false;
		self.registers.f.subtract = false;
		new_v
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
	fn sla(&mut self, value: u8) -> u8 {
		let new_carry = value & 0x80;
		let new_v = value << 1;

		self.registers.f.zero = new_v == 0;
		self.registers.f.subtract = false;
		self.registers.f.carry = new_carry != 0;
		self.registers.f.half_carry = false;
		new_v
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
	fn ld16(&mut self, value: u16) {}
}
