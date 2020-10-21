/*
    The NES doesn't output an RGB signal; it directly outputs analog video signal, hence
    there is a multitude of ways of interpreting the colors it generates.

    Implementation based on https://wiki.nesdev.com/w/index.php/NTSC_video
*/

pub static DEFAULT_SATURATION: f32 = 1.0;
pub static DEFAULT_HUE: f32 = 0.0;
pub static DEFAULT_CONTRAST: f32 = 1.0;
pub static DEFAULT_BRIGHTNESS: f32 = 1.0;
pub static DEFAULT_GAMMA: f32 = 1.4;

static BLACK: f32 = 0.518;
static WHITE: f32 = 1.962;
static ATTENUATION: f32 = 0.746;

static LEVELS: [f32; 8] = [
    0.350, 0.518, 0.962,1.550,  // Signal low
    1.094,1.506,1.962,1.962  // Signal high
    ];

#[inline]
fn incolor_phase(color: u64, phase: u64) -> bool {
    color.wrapping_add(phase).wrapping_rem_euclid(12) < 6
}

#[inline]
fn gamma_fix(f: f32) -> f32 {
    let gamma = 2.0_f32; // Assumed display gamma
    if f <= 0.0_f32 {
        0.0_f32
    }
    else {
        f.powf(2.2_f32/gamma)
    }
}

#[inline]
fn clamp(v: u32) -> u32 {
    if v > 255 { 255 } else { v }
}

//========================================================================
// pixel: value returned from palette ram(6 bit) + emphasis bits(3 bits)
//========================================================================

fn ntsc_signal(pixel: u16, phase: u64) -> f32 {
    // output: Output color (9-bit) given as input. Bitmask format: "eeellcccc".
    // phase: Signal phase (0..11). It is a variable that increases by 8 each pixel.
    let color = (pixel & 0x0F) as u64;                                        // 0..15 "cccc"
    let level = if color > 13 { 1 } else { (pixel >> 4) & 0x03 };             // 0..3  "ll", For colors 14..15, level 1 is forced.
    let emphasis = pixel >> 6;                                                // 0..7  "eee"

    // The square wave for this color alternates between these two voltages:
    let low = if color == 0 { LEVELS[(4+level) as usize] } else { LEVELS[(0+level) as usize] };
    let high = if color > 12 { LEVELS[(0+level) as usize] } else { LEVELS[(4+level) as usize] };

    // Generate the square wave
    let mut signal = if incolor_phase(color, phase) { high } else { low };

    // When de-emphasis bits are set, some parts of the signal are attenuated:
    if ((emphasis & 0x01) > 0 && incolor_phase(0, phase)) ||
        ((emphasis & 0x02) > 0 && incolor_phase(4, phase)) ||
        ((emphasis & 0x04) > 0 && incolor_phase(8, phase)) {
            signal = signal * ATTENUATION;
        }

    signal
}

// The process of generating NTSC signal for a single pixel
fn ntsc_signal_levels_for_single_ouput(x: usize, pixel: u16, ppu_cycle: u64, signal_levels: &mut[f32]) {
    let phase = ppu_cycle.wrapping_mul(8).wrapping_rem_euclid(12);

    // Each pixel produces distinct 8 samples of NTSC signal.
    for p in 0..8 {
        let signal = (ntsc_signal(pixel, phase.wrapping_add(p)) - BLACK) / (WHITE - BLACK);
        signal_levels[(x*8)+(p as usize)] = signal;
    }
}

// calculate the signal levels for a scanline - (256 * 8) levels
fn ntsc_signal_levels(pixels: &[u16],  levels: &mut[f32], mut ppu_cycle: u64) {
    for x in 0..256 {
        ntsc_signal_levels_for_single_ouput(x, pixels[x], ppu_cycle, levels);
        ppu_cycle += 1;
    }
}

fn ntsc_decode_line(signal_levels: &[f32], ntsc_rgb: &mut [u32], ppu_cycle: u64) {
    // ppu_cycle: The ppu cycle at start of scanline
    let phase = ppu_cycle.wrapping_mul(8).wrapping_rem_euclid(12) as f32 + 3.9_f32;

    for x in 0..256 {
        let center: usize = x * (256 * 8) / 256 + 0;
        let begin: usize = center.saturating_sub(6);
        let end: usize = if (center+6) > (256*8) { 256 * 8 } else { center + 6 };
        let mut y =  0.0_f32; 
        let mut i = 0.0_f32;
        let mut q = 0.0_f32;

        for p in begin..end {
            let level = signal_levels[p] / 12.0_f32;
            y = y + level;
            i = i + level * (std::f32::consts::PI * (phase + (p as f32)) / 6.0_f32).cos();
            q = q + level * (std::f32::consts::PI * (phase + (p as f32)) / 6.0_f32).sin();
        }

        // convert yiq to rgb
        let rgb = 
        0x10000 * clamp((255.95_f32 * gamma_fix(y + 0.946882 * i +  0.623557_f32 * q)) as u32)
        + 0x00100 * clamp((255.95_f32 * gamma_fix(y + -0.274788 * i +  -0.635691 * q)) as u32)
        + 0x00001 * clamp((255.95_f32 * gamma_fix(y + -1.108545 * i +  1.709007 * q)) as u32);

        ntsc_rgb[x] = rgb;
    }
}

fn calc_ntsc_pixels(pixels: &[u16], ntsc_pixel_buffer: &mut [u32], frame_height: usize, mut ppu_cycle: u64) {
    // ppu_cycle: cycle at start of frame rendering
    let mut signal_levels: Vec<f32> = vec![0.0_f32; 256*8];
    let cycles_per_scanline = 341_u64;

    for scanline_index in 0..frame_height {
        let start = scanline_index * 256;
        let end = start + 256;

        ntsc_signal_levels(&pixels[start..end], &mut signal_levels, ppu_cycle);
        ntsc_decode_line(&mut signal_levels, &mut ntsc_pixel_buffer[start..end], ppu_cycle);
        ppu_cycle = ppu_cycle.wrapping_add(cycles_per_scanline);
    }
}

fn calc_ntsc_pixel(pixel: u16, saturation: f32, hue: f32, contrast: f32, brightness: f32, gamma: f32) -> u32 {
    // Calculate the luma and chroma by emulating the relevant circuits
    let mut y = 0.0_f32;
    let mut i = 0.0_f32;
    let mut q = 0.0_f32;
    
    // 12 clock cycles (samples) per pixel
    for phase in 0..12 {        
        // Normalize
        let mut v = (ntsc_signal(pixel, phase) - BLACK) / (WHITE - BLACK);

        // Ideal TV NTSC demodulator
        // Apply contrast/brightness
        v = (v - 0.5) * contrast + 0.5;
        v *= brightness / 12.0;

        y += v;
        i = i + v * (std::f32::consts::PI * (hue + (phase as f32)) / 6.0).cos();
        q = q + v * (std::f32::consts::PI * (hue + (phase as f32)) / 6.0).sin();
    }

    i *= saturation;
    q *= saturation;

    // convert yiq to rgb
    0x10000 * clamp((255.95 * gamma_fix(y + 0.946882 * i +  0.623557 * q)) as u32)
    + 0x00100 * clamp((255.95 * gamma_fix(y + -0.274788 * i +  -0.635691 * q)) as u32)
    + 0x00001 * clamp((255.95 * gamma_fix(y + -1.108545 * i +  1.709007 * q)) as u32)
}

pub fn generate_palette(saturation: f32, hue: f32, contrast: f32, brightness: f32, gamma: f32) -> Vec<u32> {
    (0..0x200).map(|pixel | calc_ntsc_pixel(pixel, saturation, hue, contrast, brightness, gamma)).collect()
}


