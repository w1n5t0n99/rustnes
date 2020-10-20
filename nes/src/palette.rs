/*
    The NES doesn't output an RGB signal; it directly outputs analog video signal, hence
    there is a multitude of ways of interpreting the colors it generates.

    Implementation based on https://wiki.nesdev.com/w/index.php/NTSC_video
*/

const DEFAULT_SATURATION: f32 = 1.0;
const DEFAULT_HUE: f32 = 0.0;
const DEFAULT_CONTRAST: f32 = 1.0;
const DEFAULT_BRIGHTNESS: f32 = 1.0;
const DEFAULT_GAMMA: f32 = 1.4;

#[derive(PartialEq, Debug, Clone, Copy)]
struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct EmphasisColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn wave(p: u32, color: u32) -> bool {
    ((color + p + 8) % 12) < 6
}
 
fn calc_rgb_color(pixel: u16, saturation: f32, hue: f32, contrast: f32, brightness: f32, gamma: f32) -> RgbColor {
    // The input value is a NES color index (with de-emphasis bits).
	// We need RGB values. Convert the index into RGB.
	// For most part, this process is described at:
    // http://wiki.nesdev.com/w/index.php/NTSC_video
    
    // decode the nes color
    let color = (pixel & 0x0F) as u8;
    let level = ((pixel >> 4) & 0x03) as u8;
    let emphasis = (pixel >> 6) as u8;

    // voltage levels relative to sync voltage
    const BLACK: f32 = 0.518;
    const WHITE: f32 = 1.962;
    const ATTENUATION: f32 = 0.746;

    const LEVELS: [f32; 8] = [
        0.350, 0.518, 0.962, 1.550, // Signal low
		1.094, 1.506, 1.962, 1.962  // Signal high
     ];

     let lo_and_hi: [f32; 2] = [ 
         LEVELS[(level + 4 * (color == 0) as u8) as usize],
         LEVELS[(level + 4 * (color < 0x0D) as u8) as usize],
        ];
    
    // Calculate the luma and chroma by emulating the relevant circuits
    let mut y = 0.0_f32;
    let mut i = 0.0_f32;
    let mut q = 0.0_f32;
    
    // 12 clock cycles (samples) per pixel
    for p in 0..12 {
        // NES NTSC modulator (square wave between two voltage levels)
        let mut spot = lo_and_hi[wave(p, color as u32) as usize];

        // De-emphasis bits attenuate a part of the signal
        if ((emphasis & 0x01) > 0 && wave(p, 12)) || ((emphasis & 0x02) > 0 && wave(p, 4)) || ((emphasis & 0x04) > 0 && wave(p, 8)) {
			spot *= ATTENUATION;
		}

    }

     RgbColor { r: 0, g: 0, b: 0}
}



