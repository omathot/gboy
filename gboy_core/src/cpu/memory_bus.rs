use super::gpu::{GPU, VRAM_BEGIN, VRAM_END};

pub(crate) struct MemoryBus {
	memory: [u8; 0xFFFF + 1],
	gpu: GPU,
}
impl MemoryBus {
	pub(crate) fn new() -> MemoryBus {
		MemoryBus {
			memory: [0; 0xFFFF + 1],
			gpu: GPU::new(),
		}
	}
	pub(crate) fn read_byte(&self, addr: u16) -> u8 {
		let addr = addr as usize;
		match addr {
			VRAM_BEGIN..VRAM_END => self.gpu.read_vram(addr - VRAM_BEGIN),
			_ => unimplemented!("Support more memory areas"),
		}
		// self.memory[addr as usize]
	}
	pub(crate) fn write_byte(&mut self, addr: u16, value: u8) {
		match addr {
			0x0000..=0x00FF => { /* Boot ROM */ }
			0x0000..=0x3FFF => { /* Game ROM Bank 0 */ }
			0x4000..=0x7FFF => { /* Game ROM Bank N */ }
			0x8000..=0x97FF => { /* Tile RAM */ }
			0x9800..=0x9FFF => { /* Background Map */ }
			0xA000..=0xBFFF => { /* Extra Cartridge RAM */ }
			0xC000..=0xDFFF => { /* Working RAM */ }
			0xE000..=0xFDFF => { /* Echo Ram: Nintendo strongly discouraged against using this, can pretend it doesn't exist */
			}
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
