pub(crate) const VRAM_BEGIN: usize = 0x8000;
pub(crate) const VRAM_END: usize = 0x97FF;
pub(crate) const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;
pub(crate) const MAX_TILE_NUMBER: usize = 384;

#[derive(Copy, Clone, Debug)]
enum TilePixelValue {
	Zero,  // white
	One,   // dark gray
	Two,   // light gray
	Three, // black
}
type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile {
	[[TilePixelValue::Zero; 8]; 8]
}

pub(crate) struct GPU {
	vram: [u8; VRAM_SIZE],
	tile_set: [Tile; MAX_TILE_NUMBER],
}
impl GPU {
	pub(crate) fn new() -> GPU {
		GPU {
			vram: [0; VRAM_SIZE],
			tile_set: [empty_tile(); MAX_TILE_NUMBER],
		}
	}
	pub(crate) fn read_vram(&self, addr: usize) -> u8 {
		self.vram[addr as usize]
	}
	pub(crate) fn write_vram(&mut self, addr: u16, v: u8) {}
}
