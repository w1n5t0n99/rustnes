/*
    The NES doesn't output an RGB signal; it directly outputs analog video signal, hence
    there is a multitude of ways of interpreting the colors it generates.

    Implementation based on https://wiki.nesdev.com/w/index.php/NTSC_video
*/

pub const DEFAULT_SATURATION: f32 = 1.0;
pub const DEFAULT_HUE: f32 = 0.0;
pub const DEFAULT_CONTRAST: f32 = 1.0;
pub const DEFAULT_BRIGHTNESS: f32 = 1.0;
pub const DEFAULT_GAMMA: f32 = 1.4;

#[derive(PartialEq, Debug, Clone, Copy)]
struct RgbColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl RgbColor {
    pub fn merge(&self) -> u32 {
        (self.r << 24) | (self.g << 16) | self.b
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct EmphasisColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

fn gamma_fix(f: f32, gamma: f32) -> f32 {
    if f < 0.0 { 0.0 } else { f.powf(2.2 / gamma) }
}

fn bound<T: std::cmp::Ord>(lower: T, value: T, upper: T) -> T {
    std::cmp::max(lower, std::cmp::min(value, upper))
}

const fn wave(p: u32, color: u32) -> bool {
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
        
        // Normalize
        let mut v = (spot - BLACK) / (WHITE - BLACK);

        // Ideal TV NTSC demodulator
        // Apply contrast/brightness
        v = (v - 0.5) * contrast + 0.5;
        v *= brightness / 12.0;

        y += v;
        i += v * (std::f32::consts::PI / 6.0).cos() * (p as f32 + hue);
        q += v * (std::f32::consts::PI / 6.0).sin() * (p as f32 + hue);
    }

    i *= saturation;
    q *= saturation;

    // Convert YIQ into RGB according to FCC-sanctioned conversion matrix.
    let mut rgb = RgbColor { r: 0, g: 0, b: 0};
    rgb.r = bound(0x00, (255.0 * gamma_fix(y + 0.946882 * i + 0.623557 * q, gamma)) as i32, 0xFF) as u32;
    rgb.g = bound(0x00, (255.0 * gamma_fix(y + -0.274788 * i + -0.635691 * q, gamma)) as i32, 0xff) as u32;
    rgb.b = bound(0x00, (255.0 * gamma_fix(y + -1.108545 * i + 1.709007 * q, gamma)) as i32, 0xff) as u32;

    rgb
}

pub fn generate_palette(saturation: f32, hue: f32, contrast: f32, brightness: f32, gamma: f32) -> Vec<u32> {
    (0..0x200).map(|pixel | calc_rgb_color(pixel, saturation, hue, contrast, brightness, gamma).merge()).collect()
}



