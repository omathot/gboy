pub(crate) enum Instruction {
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
	RRCA,
	RLCA,
	CPL,
	BIT(u8, ArithmeticTarget),
	BITHL(u8),
	RESET(u8, ArithmeticTarget),
	RESETHL(u8),
	SET(u8, ArithmeticTarget),
	SETHL(u8),
	SRL(ArithmeticTarget),
	SRLHL,
	RR(ArithmeticTarget),
	RRHL,
	RL(ArithmeticTarget),
	RLHL,
	RRC(ArithmeticTarget),
	RRCHL,
	RLC(ArithmeticTarget),
	RLCHL,
	SRA(ArithmeticTarget),
	SRAHL,
	SLA(ArithmeticTarget),
	SLAHL,
	SWAP(ArithmeticTarget),
	SWAPHL,
	JP(JumpTest),
}
impl Instruction {
	pub(crate) fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
		if prefixed {
			Instruction::from_byte_prefixed(byte)
		} else {
			Instruction::from_byte_not_prefixed(byte)
		}
	}
	fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
		match byte {
			0x00 => Some(Instruction::RLC(ArithmeticTarget::B)),
			/* TODO: implement more */
			_ => {
				unimplemented!("Unimplemented opcode: {:x}", byte);
			}
		}
	}
	fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
		match byte {
			0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
			0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
			_ => {
				unimplemented!("Unimplemented opcode: {}", byte)
			}
		}
	}
}
pub(crate) enum ArithmeticTarget {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
}
pub(crate) enum JumpTest {
	NotZero,
	Zero,
	NotCarry,
	Carry,
	Always,
}
