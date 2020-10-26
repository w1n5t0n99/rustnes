use super::mappers::Mapper;
use super::palette::*;

#[inline]
const fn ppu_output(color_index: u8, ppu_mask: u8) -> u16 {
    let pixel = (color_index & 0b00111111) as u16;
    // extract emphasis bits and append to pixel
    let emp_bits = ((ppu_mask & 0b11100000) as u16) << 1;

    emp_bits | pixel
}

#[derive(Debug)]
struct Tile {
    left_plane: [u8; 8],
    right_plane: [u8; 8],
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            left_plane: [0; 8],
            right_plane: [0; 8],
        }
    }
}

#[derive(Debug)]
pub struct PpuViewer {
    ppu_output_buffer: Vec<u16>,
    palette: Vec<u32>,
}

impl PpuViewer {
    pub fn new() -> PpuViewer {
        PpuViewer {
            ppu_output_buffer: Vec::new(),
            palette: generate_palette(DEFAULT_SATURATION, DEFAULT_HUE, DEFAULT_CONTRAST, DEFAULT_BRIGHTNESS, DEFAULT_GAMMA),
        }
    }

    fn extract_tiles_from_rom(&mut self, mapper: &mut dyn Mapper) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::new();
    
        // get tile data with whatever bank switching currently in effect
        for i in (0..0x2000).step_by(16) {
            let mut tile: Tile = Default::default();
            tile.left_plane[0] = mapper.peek_pattern_table(i as u16);
            tile.left_plane[1] = mapper.peek_pattern_table((i+1) as u16);
            tile.left_plane[2] = mapper.peek_pattern_table((i+2) as u16);
            tile.left_plane[3] = mapper.peek_pattern_table((i+3) as u16);
            tile.left_plane[4] = mapper.peek_pattern_table((i+4) as u16);
            tile.left_plane[5] = mapper.peek_pattern_table((i+5) as u16);
            tile.left_plane[6] = mapper.peek_pattern_table((i+6) as u16);
            tile.left_plane[7] = mapper.peek_pattern_table((i+7) as u16);
    
            tile.right_plane[0] = mapper.peek_pattern_table((i+8) as u16);
            tile.right_plane[1] = mapper.peek_pattern_table((i+9) as u16);
            tile.right_plane[2] = mapper.peek_pattern_table((i+10) as u16);
            tile.right_plane[3] = mapper.peek_pattern_table((i+11) as u16);
            tile.right_plane[4] = mapper.peek_pattern_table((i+12) as u16);
            tile.right_plane[5] = mapper.peek_pattern_table((i+13) as u16);
            tile.right_plane[6] = mapper.peek_pattern_table((i+14) as u16);
            tile.right_plane[7] = mapper.peek_pattern_table((i+15) as u16);
    
            tiles.push(tile);
        }
    
        tiles
    }

    pub fn gen_chr_data(&mut self, mapper: &mut dyn Mapper) {
        self.ppu_output_buffer.clear();

        let tiles = self.extract_tiles_from_rom(mapper);
        
        let mut tile_number: usize = 0;
        let mut tile_offset: usize = 0;
        let mut line_number: usize = 0;
    
        for _i in 0..(tiles.len() * 8) {        
            let left = tiles[tile_number + tile_offset].left_plane[line_number];
            let right = tiles[tile_number + tile_offset].right_plane[line_number];
            // 0
            
            let mut palette_index = ((left &  0x80) >> 7) | ((right & 0x80) >> 6);
            let mut color_index = mapper.peek_palette(palette_index as u16);      
            self.ppu_output_buffer.push(ppu_output(color_index, 0));
    
            // 1
            palette_index = ((left &  0x40) >> 6) | ((right & 0x40) >> 5);
            color_index = mapper.peek_palette(palette_index as u16);      
            self.ppu_output_buffer.push(ppu_output(color_index, 0));
    
            // 2
            palette_index = ((left &  0x20) >> 5) | ((right & 0x20) >> 4);
            color_index = mapper.peek_palette(palette_index as u16);      
            self.ppu_output_buffer.push(ppu_output(color_index, 0));
    
            // 3
            palette_index = ((left &  0x10) >> 4) | ((right & 0x10) >> 3);
            color_index = mapper.peek_palette(palette_index as u16);      
            self.ppu_output_buffer.push(ppu_output(color_index, 0));
    
            // 4
            palette_index = ((left &  0x08) >> 3) | ((right & 0x08) >> 2);
            color_index = mapper.peek_palette(palette_index as u16);      
            self.ppu_output_buffer.push(ppu_output(color_index, 0));
    
            // 5
            palette_index = ((left &  0x04) >> 2) | ((right & 0x04) >> 1);
            color_index = mapper.peek_palette(palette_index as u16);      
            self.ppu_output_buffer.push(ppu_output(color_index, 0));
    
            // 6
            palette_index = ((left &  0x02) >> 1) | (right & 0x02);
            color_index = mapper.peek_palette(palette_index as u16);
            self.ppu_output_buffer.push(ppu_output(color_index, 0));
    
            // 7
            palette_index = (left &  0x01) | ((right & 0x01) << 1);
            color_index = mapper.peek_palette(palette_index as u16);      
            self.ppu_output_buffer.push(ppu_output(color_index, 0));
    
            tile_number += 1;
            if tile_number > 31 {
                tile_number = 0;
                line_number += 1;
            }
    
            if line_number > 7 {
                line_number = 0;
                tile_offset += 32;
            }
        }
    }

    pub fn chr_buffer(&mut self) -> Vec<u32> {
        let scanlines = self.ppu_output_buffer.len() / 256;
        let mut ntsc_pixel_buffer: Vec<u32> = vec![0; 256*scanlines];
        
        
        for (i, p) in self.ppu_output_buffer.iter().enumerate() {
            ntsc_pixel_buffer[i] = self.palette[*p as usize];
        }
        
        
        ntsc_pixel_buffer
    }
}  


