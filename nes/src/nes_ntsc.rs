/*
    Rust rewrite of Shay Green NES NTSC filtering library

    Reads NES pixels and writes RGB pixels (16-bit by
    default). The NES pixels are 6-bit raw palette values (0 to 0x3F). Edit
    nes_ntsc_config.h to change this
*/
type NesNtscRgbT = u32;
const NES_NTSC_ENTRY_SIZE: u32 = 128;
//TODO allow changing of palette size
const NES_NTSC_PALETTE_SIZE: u32 = 64 * 8;    // 6 bit color + 3 bit emphasis
// const NES_NTSC_PALETTE_SIZE: usize = 64;     // 6 bit  color only
const NES_NTSC_BURST_SIZE: u32 = NES_NTSC_ENTRY_SIZE / NES_NTSC_BURST_COUNT;

//TODO change to user mutable types
/* Interface for user-defined custom blitters */
const NES_NTSC_IN_CHUNK: u32 = 3;       /* number of input pixels read per chunk */
const NES_NTSC_OUT_CHUNK: u32 = 7;      /* number of output pixels generated per chunk */
const NES_NTSC_BLACK: u32 = 15;         /* palette index for black */
const NES_NTSC_BURST_COUNT: u32 = 3;    /* burst phase cycles through 0, 1, and 2 */
const NES_NTSC_RGB_BUILDER: u32 = (1 << 21) | (1 << 11) | (1 << 1);

/* Common implementation of NTSC filters */
const LUMA_CUTOFF: f32 = 0.20;
const GAMMA_SIZE: u32 = 1;

const RGB_BITS: u32 = 8;
const RGB_UNIT: u32 = 1 << RGB_BITS;
const RGB_OFFSET: f32 = (RGB_UNIT * 2) as f32 + 0.5;

const ARTIFACTS_MID: f32 = 1.0;
const ARTIFACTS_MAX: f32 = ARTIFACTS_MID * 1.5;

const FRINGING_MID: f32 = 1.0;
const FRINGING_MAX: f32 = FRINGING_MID * 2.0;

const STD_DECODER_HUE: f32 = -15.0;
const EXT_DECODER_HUE: f32 = STD_DECODER_HUE + 15.0;

const KERNEL_HALF: u32 = 16;
const KERNEL_SIZE: u32 = (KERNEL_HALF * 2) + 1;
const RESCALE_OUT: u32 = 7;
const RESCALE_IN: u32 = 8;
const ALIGNMENT_COUNT: u32 = 3;
const RGB_KERNEL_SIZE: u32 = BURST_SIZE / ALIGNMENT_COUNT;
const RGB_BIAS: u32 = RGB_UNIT * 2 * NES_NTSC_RGB_BUILDER;

const DEFAULT_DECODER: [f32; 6] = [0.956, 0.621, -0.272, -0.647, -1.105, 1.702];

macro_rules! pixel_negate{
    ($ntsc:expr) => {{ 1.0_f32 - (($ntsc as i32 + 100) & 2) as f32 }};
}

macro_rules! pixel_offset{
    ($ntsc:expr, $scaled:expr) => {{
        let ntsc_t = $ntsc as i32 - $scaled as i32 / RESCALE_OUT as i32 * RESCALE_IN as i32;
        let scaled_t = ($scaled as i32 + RESCALE_OUT as i32 * 10) % RESCALE_OUT as i32;

        KERNEL_SIZE as i32 / 2 + ntsc_t + 1 + (RESCALE_OUT as i32 - scaled_t) % RESCALE_OUT as i32 + (KERNEL_SIZE as i32 * 2 * scaled_t)
    }};
}

#[derive(Clone, Copy)]
struct Init {
    to_rgb: [f32; (NES_NTSC_BURST_COUNT * 6) as usize],
    to_float: [f32; GAMMA_SIZE as usize],
    contrast: f32,
    brightness: f32,
    artifacts: f32,
    fringing: f32,
    kernel: [f32; (RESCALE_OUT*KERNEL_SIZE*2) as usize],
}

impl Default for Init {
    fn default() -> Self {
         Init {
             to_rgb: [0.0; (NES_NTSC_BURST_COUNT * 6) as usize],
            to_float: [0.0; GAMMA_SIZE as usize],
            contrast: 0.0,
            brightness: 0.0,
            artifacts: 0.0,
            fringing: 0.0,
            kernel: [0.0; (RESCALE_OUT*KERNEL_SIZE*2) as usize]
         }
    }
}

#[inline]
fn rotate_iq(i: &mut f32, q: &mut f32, sin_b: f32, cos_b: f32) {
    let t = *i * cos_b - *q * sin_b;
    *q = *i * sin_b * *q * cos_b;
    *i = t;
}

#[derive(Clone, Copy)]
pub struct NesNtscSetup {
    /* Basic parameters */
    hue: f64,                   /* -1 = -180 degrees     +1 = +180 degrees */
    saturation: f64,            /* -1 = grayscale (0.0)  +1 = oversaturated colors (2.0) */
    contrast: f64,              /* -1 = dark (0.5)       +1 = light (1.5) */
    brightness: f64,            /* -1 = dark (0.5)       +1 = light (1.5) */
    sharpness: f64,             /* edge contrast enhancement/blurring */
    /* Advanced parameters */
    gamma: f64,                 /* -1 = dark (1.5)       +1 = light (0.5) */
    resolution: f64,            /* image resolution */
    artifacts: f64,             /* artifacts caused by color changes */
    fringing: f64,              /* color artifacts caused by brightness changes */
    bleed: f64,                 /* color bleed (color resolution reduction) */
    merge_fields: f64,          /* if 1, merges even and odd fields together to reduce flicker */
    decoder_matrix: [f32; 6],   /* optional RGB decoder matrix, 6 elements */
}

// Video format presets
const NES_NTSC_MONOCHROME: NesNtscSetup  = NesNtscSetup {
    hue: 0.0,           
    saturation: -1.0,   
    contrast: 0.0,      
    brightness: 0.0,    
    sharpness: 0.2,     
    gamma: 0.0,         
    resolution: 0.2,    
    artifacts: -0.2,    
    fringing: -0.2,     
    bleed: -1.0,       
    merge_fields: 1.0,  
    decoder_matrix: DEFAULT_DECODER,
};

const NES_NTSC_COMPOSITE: NesNtscSetup  = NesNtscSetup {
    hue: 0.0,           
    saturation: 0.0,   
    contrast: 0.0,      
    brightness: 0.0,    
    sharpness: 0.0,     
    gamma: 0.0,         
    resolution: 0.0,    
    artifacts: 0.0,    
    fringing: 0.0,     
    bleed: 0.0,       
    merge_fields: 1.0,
    decoder_matrix: DEFAULT_DECODER,  
};

const NES_NTSC_SVIDEO: NesNtscSetup  = NesNtscSetup {
    hue: 0.0,           
    saturation: 0.0,   
    contrast: 0.0,      
    brightness: 0.0,    
    sharpness: 0.2,     
    gamma: 0.0,         
    resolution: 0.2,    
    artifacts: -1.0,    
    fringing: -1.0,     
    bleed: 0.0,       
    merge_fields: 1.0,  
    decoder_matrix: DEFAULT_DECODER,
};

const NES_NTSC_RGB: NesNtscSetup  = NesNtscSetup {
    hue: 0.0,           
    saturation: 0.0,   
    contrast: 0.0,      
    brightness: 0.0,    
    sharpness: 0.2,     
    gamma: 0.0,         
    resolution: 0.7,    
    artifacts: -1.0,    
    fringing: -1.0,     
    bleed: -1.0,       
    merge_fields: 1.0,  
    decoder_matrix: DEFAULT_DECODER,
};

#[derive(Clone, Copy)]
pub struct NesNtsc {
    table: [[NesNtscRgbT; NES_NTSC_ENTRY_SIZE as usize]; NES_NTSC_PALETTE_SIZE as usize],
}

impl Default for NesNtsc {
    fn default() -> Self {
        NesNtsc {
            table: [[0; NES_NTSC_ENTRY_SIZE as usize]; NES_NTSC_PALETTE_SIZE as usize],    
        }
    }
}

#[derive(Clone, Copy)]
pub struct PixelInfo {
    offset: i32,
    negate: f32,
    kernel: [f32; 4],
}

/* 3 input pixels -> 8 composite samples */
// macro expansion from original source
pub const NES_NTSC_PIXELS: [PixelInfo; ALIGNMENT_COUNT as usize] = [
    PixelInfo { 
        offset: pixel_offset!(-4, -9),
        negate: pixel_negate!(-4),
        kernel: [1.0, 1.0, 0.6667, 0.0],
    },
    
    PixelInfo {
        offset: pixel_offset!(-2, -7),
        negate: pixel_negate!(-2),
        kernel: [0.3333, 1.0, 1.0, 0.3333],
    },

    PixelInfo {
        offset: pixel_offset!(0, -5),
        negate: pixel_negate!(0),
        kernel: [0.0, 0.6667, 1.0, 1.0] 
    },
];

// TODO not sure whats for, prob for users to configure
const fn std_hue_condition(_setup: &NesNtscSetup) -> bool {
    true
}

/* Number of output pixels written by blitter for given input width. Width might
be rounded down slightly; use NES_NTSC_IN_WIDTH() on result to find rounded
value. Guaranteed not to round 256 down at all. */
const fn nes_ntsc_out_width(in_width: u32) -> u32 {
    ((in_width - 1) / (NES_NTSC_IN_CHUNK + 1)) * NES_NTSC_OUT_CHUNK
}

/* Number of input pixels that will fit within given output width. Might be
rounded down slightly; use NES_NTSC_OUT_WIDTH() on result to find rounded
value. */
const fn nes_ntsc_in_width(out_width: u32) -> u32 {
    (out_width / (NES_NTSC_OUT_CHUNK - 1)) * (NES_NTSC_IN_CHUNK + 1)
}

fn init_filters(imp: &mut Init, setup: &NesNtscSetup) {
    // TODO: as abitilty to switch kernel to user implementation
    let mut kernels: [f32; (KERNEL_SIZE * 2) as usize] = [0.0; (KERNEL_SIZE * 2) as usize];

    /* generate luma (y) filter using sinc kernel */
    {
        /* sinc with rolloff (dsf) */
        let rolloff: f32 = 1.0 + setup.sharpness as f32 * 0.032;
        let maxh: f32 = 32.0;
        let pow_a_n = rolloff.powf(maxh);
        let mut sum: f32;
        /* quadratic mapping to reduce negative (blurring) range */
        let t_to_angle: f32 = setup.resolution as f32 + 1.0;
        let to_angle: f32 = std::f32::consts::PI / maxh * LUMA_CUTOFF as f32 * (t_to_angle * t_to_angle + 1.0);
        kernels[(KERNEL_SIZE * 3 / 2) as usize] = maxh; /* default center value */
        
        for i in 0..(KERNEL_HALF * 2 + 1) {
            let x = i as i32 - KERNEL_HALF as i32;
            let angle = x as f32 * to_angle;
            /* instability occurs at center point with rolloff very close to 1.0 */
            if x > 0 || pow_a_n > 1.056 || pow_a_n < 0.981 {
                let rolloff_cos_a = rolloff * angle.cos();
                let num = 1.0 - rolloff_cos_a - (pow_a_n * (maxh * angle).cos()) + (pow_a_n * rolloff * ((maxh - 1.0) * angle).cos());
                let den = 1.0 - rolloff_cos_a - rolloff_cos_a + (rolloff * rolloff);
                let dsf = num / den;
                kernels[(KERNEL_SIZE * 3 / 2 - KERNEL_HALF + i) as usize] = dsf - 0.5;
            } 
        }

        /* apply blackman window and find sum */
        let mut sum: f32 = 0.0;
        for i in 0..(KERNEL_HALF * 2 + 1) {
            let x = std::f32::consts::PI * 2.0 / (KERNEL_HALF as f32 *2.0) * i as f32;
            let blackman: f32 = 0.42 - 0.5 * x.cos() + 0.08 * (x * 2.0).cos();
            // TODO check correctness
            kernels[(KERNEL_SIZE * 3 / 2 - KERNEL_HALF + i) as usize] *= blackman;
            sum += kernels[(KERNEL_SIZE * 3 / 2 - KERNEL_HALF + i) as usize];
        }
        
        /* normalize kernel */
        sum = 1.0 / sum;
        for i in 0..(KERNEL_HALF * 2 + 1) {
            let x = KERNEL_SIZE * 3 / 2 - KERNEL_HALF + i;
            kernels[x as usize] *= sum;
        }
    }
        
    /* generate chroma (iq) filter using gaussian kernel */
    {
        let cutoff_factor: f32 = -0.03125;
        let mut cutoff = setup.bleed as f32;
        let mut i: i32 = 0;

        if cutoff < 0.0 {
            /* keep extreme value accessible only near upper end of scale (1.0) */
            cutoff *= cutoff;
            cutoff *= cutoff;
            cutoff *= cutoff;
            cutoff *= -30.0 / 0.65;
        }
        cutoff = cutoff_factor - 0.65 * cutoff_factor * cutoff;
        
        let mut e: f32 = (KERNEL_HALF as f32) * -1.0;
        for i in 0..KERNEL_SIZE {
            kernels[i as usize] = ((e * e) as f32 * cutoff).exp();
            e += 1.0;
        }

        /* normalize even and odd phases separately */
        for i in 0..2 {
            let x = 0;
            let mut sum: f32 = 0.0;
            for x in (i..KERNEL_SIZE).step_by(2) {
                sum += kernels[x as usize];
            }

            sum = 1.0 / sum;
            for x in (i..KERNEL_SIZE).step_by(2) {
                kernels[x as usize] *= sum;
            }
        }
    }

    /* generate linear rescale kernels */
    {
        let mut weight: f32 = 1.0;
        let x = &mut imp.kernel;
        let mut kernel_index: usize = 0;
        for n in (0..RESCALE_OUT).rev() {
            let remain: f32 = 0.0;
            let mut i: usize = 0;
            weight -= 1.0 / (RESCALE_IN as f32);
            for i in 0..(KERNEL_SIZE * 2) {
                let cur = kernels[i as usize];
                let m = cur * weight;
                // TODO check correctness
                x[kernel_index] = m + remain;
                kernel_index += 1;
            }
        }
    }        
}

fn init(imp: &mut Init, setup: &NesNtscSetup) {
    imp.brightness = setup.brightness as f32 * (0.5 * RGB_UNIT as f32) + RGB_OFFSET;
    imp.contrast = setup.contrast as f32 * (0.5 * RGB_UNIT as f32) + RGB_UNIT as f32;
    // TODO default palette contrast

    imp.artifacts = setup.artifacts as f32;
    if imp.artifacts > 0.0 { imp.artifacts *= ARTIFACTS_MAX - ARTIFACTS_MID; }
    imp.artifacts = imp.artifacts * ARTIFACTS_MID + ARTIFACTS_MID;

    imp.fringing = setup.fringing as f32;
    if imp.fringing > 0.0 { imp.fringing *= FRINGING_MAX - FRINGING_MID; }
    imp.fringing = imp.fringing * imp.fringing + FRINGING_MID;

    init_filters(imp, setup);

    /* generate gamma table */
    if GAMMA_SIZE > 1 {
        // TODO check correctness
        let to_float: f32 = 1.0 / (GAMMA_SIZE as f32 - (GAMMA_SIZE - 1) as f32);
        let gamma : f32 = 1.1333 - setup.gamma as f32 * 0.5;
        /* match common PC's 2.2 gamma to TV's 2.65 gamma */
        for i in 0..GAMMA_SIZE {
            imp.to_float[i as usize] = (i as f32 * to_float).powf(gamma) * imp.contrast + imp.brightness;
        }
    }

    /* setup decoder matricies */
    {
        let hue: f32 = (setup.hue as f32 * std::f32::consts::PI + std::f32::consts::PI / 180.0 * EXT_DECODER_HUE) + 
            ( std::f32::consts::PI / 180.0 * (STD_DECODER_HUE - EXT_DECODER_HUE));
        let sat: f32 = setup.sharpness as f32 + 1.0;
        
        //TODO add optional decoder support
        let mut s: f32 = hue.sin() * sat;
        let mut c: f32 = hue.cos() * sat;

        let mut x = 0;
        for n in 0..NES_NTSC_BURST_COUNT {
            let mut y = 0;
            for n in 0..3 {
                let i = setup.decoder_matrix[y];
                y += 1;
                let q = setup.decoder_matrix[y];
                y += 1;

                imp.to_rgb[x] = i * c - q * s;
                x += 1;
                imp.to_rgb[x] = i * s + q * c;
                x += 1;
            }

            rotate_iq(&mut s, &mut c, 0.866025, -0.5);
        }
    }
}

#[inline]
fn yiq_to_rgb_int(y: f32, i: f32, q: f32, to_rgb: &[f32]) -> (u32, u32, u32) {
    let r = (y + to_rgb[0] * i + to_rgb[1] * q) as u32;
    let g = (y + to_rgb[2] * i + to_rgb[3] * q) as u32;
    let b = (y + to_rgb[4] * i + to_rgb[5] * q) as u32;
    (r, g, b)
}

#[inline]
fn yiq_to_rgb_float(y: f32, i: f32, q: f32, to_rgb: &[f32]) -> (f32, f32, f32) {
    let r = y + to_rgb[0] * i + to_rgb[1] * q;
    let g = y + to_rgb[2] * i + to_rgb[3] * q;
    let b = y + to_rgb[4] * i + to_rgb[5] * q;
    (r, g, b)
}

#[inline]
fn rgb_to_yiq(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let y = r * 0.299 + g * 0.587 + b * 0.114;
    let i = r * 0.596 - g * 0.275 - b * 0.321;
    let q = r * 0.212 - g * 0.523 + b * 0.311;
    (y, i, q)
}



#[inline]
fn pack_rgb(r: u32, g: u32, b: u32) -> u32 {
    (r << 21) | (g << 11) | (b << 1)
}

/* Generate pixel at all burst phases and column alignments */
fn gen_kernel(imp: &mut Init, mut y: f32, mut i: f32, mut q: f32, out: &mut [NesNtscRgbT]) {
    /* generate for each scanline burst phase */
    let mut to_rgb_index: usize = 0;
    let mut out_index: usize = 0;
    let mut burst_remain = NES_NTSC_BURST_COUNT;
    y -= RGB_OFFSET;

    loop {
        /* Encode yiq into *two* composite signals (to allow control over artifacting).
		Convolve these with kernels which: filter respective components, apply
		sharpening, and rescale horizontally. Convert resulting yiq to rgb and pack
        into integer. Based on algorithm by NewRisingSun. */
        let mut pixel_info_index: usize = 0;
        let mut alignment_remain = ALIGNMENT_COUNT;
        loop {
            /* negate is -1 when composite starts at odd multiple of 2 */
            let yy = y * imp.fringing * NES_NTSC_PIXELS[pixel_info_index].negate;
            let ic0 = (i + yy) * NES_NTSC_PIXELS[pixel_info_index].kernel[0];
            let qc1 = (q + yy) * NES_NTSC_PIXELS[pixel_info_index].kernel[1];
            let ic2 = (i - yy) * NES_NTSC_PIXELS[pixel_info_index].kernel[2];
            let qc3 = (q - yy) * NES_NTSC_PIXELS[pixel_info_index].kernel[3];

            let factor = imp.artifacts * NES_NTSC_PIXELS[pixel_info_index].negate;
            let ii  = i * factor;
            let yc0 = (y + ii) * NES_NTSC_PIXELS[pixel_info_index].kernel[0];
            let yc2 = (y - ii) * NES_NTSC_PIXELS[pixel_info_index].kernel[2];

            let qq = q * factor;
            let yc1 = (y + qq) * NES_NTSC_PIXELS[pixel_info_index].kernel[1];
            let yc3 = (y - qq) * NES_NTSC_PIXELS[pixel_info_index].kernel[3];

            let mut kernel_index = NES_NTSC_PIXELS[pixel_info_index].offset as usize;
            pixel_info_index += 1;

            for n in (0..RGB_KERNEL_SIZE).rev() {
                let i = imp.kernel[kernel_index+0] * ic0 + imp.kernel[kernel_index+2] * ic2;
                let q = imp.kernel[kernel_index+1] * qc1 + imp.kernel[kernel_index+3] * qc3;
                let y = imp.kernel[KERNEL_SIZE as usize+0] * yc0 + imp.kernel[KERNEL_SIZE as usize+1] * yc1 + 
                imp.kernel[KERNEL_SIZE as usize+2] * yc2 + imp.kernel[KERNEL_SIZE as usize+3] * yc3 + RGB_OFFSET;

                if RESCALE_OUT <= 1 { kernel_index -= 1; }
                else if kernel_index <  (KERNEL_SIZE * 2 * (RESCALE_OUT - 1)) as usize { kernel_index += (KERNEL_SIZE as usize * 2 -1); }
                else { kernel_index -= KERNEL_SIZE as usize * 2 * (RESCALE_OUT as usize - 1) + 2; }

                let (r, g, b) = yiq_to_rgb_int(y, i, q, &imp.to_rgb[to_rgb_index..]);
                out[out_index] = pack_rgb(r, g, b) - RGB_BIAS;
                out_index += 1;
            }

            alignment_remain -= 1;
            if alignment_remain == 0 { break; }
        }

        to_rgb_index += 6;
        rotate_iq(&mut i, &mut q, -0.866025_f32, -0.5_f32);  /* -120 degrees */

       burst_remain -= 1;
       if burst_remain == 0 { break; } 
    }
}

fn correct_errors(color: NesNtscRgbT, out: &mut [NesNtscRgbT]) {
    for n in (0..NES_NTSC_BURST_COUNT).rev() {
        for i in 0..(RGB_KERNEL_SIZE as usize/2) {
            let error = color - out[i] - out[(i+12)%14+24] - out[(i+10)%14+28] -
                out[i+7] - out[i+5+14] - out [i+3+28];
            
            // Distribute error
            let mut fourth = (error + 2 * NES_NTSC_RGB_BUILDER) >> 2;
            fourth &= (RGB_BIAS >> 1) - NES_NTSC_RGB_BUILDER;
            fourth -= RGB_BIAS >> 2;
            out[i+3+28] += fourth;
            out[i+5+14] += fourth;
            out[i+7] += error - (fourth*3);            
        }
    }
}

fn merge_kernel_fields(io: &mut [NesNtscRgbT]) {
    let mut io_index: usize = 0;
    for _n in (0..BURST_SIZE).rev() {
        let p0 = io[io_index + (BURST_SIZE as usize * 0)] + RGB_BIAS;
        let p1 = io[io_index + (BURST_SIZE as usize * 1)] + RGB_BIAS;
        let p2 = io[io_index + (BURST_SIZE as usize * 2)] + RGB_BIAS;
        /* merge colors without losing precision */
        io[io_index + (BURST_SIZE as usize * 0)] = ((p0 + p1 - ((p0 ^ p1) & NES_NTSC_RGB_BUILDER)) >> 1) - RGB_BIAS;
        io[io_index + (BURST_SIZE as usize * 1)] = ((p1 + p2 - ((p1 ^ p2) & NES_NTSC_RGB_BUILDER)) >> 1) - RGB_BIAS;
        io[io_index + (BURST_SIZE as usize * 2)] = ((p2 + p0 - ((p2 ^ p0) & NES_NTSC_RGB_BUILDER)) >> 1) - RGB_BIAS;
        io_index += 1;
    }
}

pub fn nes_ntsc_init(ntsc: &mut NesNtsc, setup: Option<NesNtscSetup>) {
    let mut imp: Init = Default::default();
    let setup = setup.unwrap_or(NES_NTSC_COMPOSITE);
    init(&mut imp, &setup);
    /* setup fast gamma */
    let gamma_factor = {
        let mut gamma = setup.gamma as f32 * -0.5;
        if std_hue_condition(&setup) {
            gamma += 0.1333;
        }

        let mut gf =  gamma.abs().powf(0.73);
        if gamma < 0.0 {
            gf = -gf;
        }

        gf
    };

    let merge_fields = if setup.artifacts <= -1.0 && setup.fringing <= -1.0 {
        1.0
    }
    else {
        setup.merge_fields
    };

    for entry in 0..NES_NTSC_PALETTE_SIZE {
        /* Base 64-color generation */
        static LO_LEVELS: [f32; 4] = [-0.12, 0.00, 0.31, 0.72];
        static HIGH_LEVELS: [f32; 4] = [0.40, 0.68, 1.00, 1.00];

        let level = entry >> 4 & 0x03;
        let mut lo = LO_LEVELS[level as usize];
        let mut hi = HIGH_LEVELS[level as usize];

        let color = entry & 0x0F;
        if color == 0 { lo = hi; }
        if color == 0x0D { hi = lo; }
        if color > 0x0D { hi = 0.0; lo = 0.0;}

        {
            let PHASES: [f32; 0x10+3] = [
                -1.0, -0.866025, -0.5, 0.0,  0.5,  0.866025,
				 1.0,  0.866025,  0.5, 0.0, -0.5, -0.866025,
				-1.0, -0.866025, -0.5, 0.0,  0.5,  0.866025,
				 1.0
            ];
            
            // #define TO_ANGLE_SIN( color )   phases [color]
            // #define TO_ANGLE_COS( color )   phases [(color) + 3]
            
            /* Convert raw waveform to YIQ */
            let sat = (hi - lo) * 0.5;
            let mut i  = PHASES[color as usize] * sat;
            let mut q = PHASES[(color + 3) as usize] * sat;
            let mut y = (hi + lo) * 0.5;

            /* Apply color emphasis */
            // #ifdef NES_NTSC_EMPHASIS

            // upper 3 bits of 9 bit emphasis + color
            let tint = entry >> 6 & 7;
            
            if tint > 0 && color <= 0x0D {
                static ATTEN_MUL: f32 = 0.79399;
                static ATTEN_SUB: f32 = 0.0782838;

                if tint == 7 {
                    y = y * (ATTEN_MUL * 1.13) - (ATTEN_SUB * 1.13);
                }
                else {
                    static TINTS: [u32; 8] = [ 0, 6, 10, 8, 2, 4, 0, 0 ];
                    let tint_color = TINTS[tint as usize];
                    let mut sat = hi * (0.5 - ATTEN_MUL * 0.5) + ATTEN_SUB * 0.5;
                    y -= sat * 0.5;
                    if tint >= 3 && tint != 4 {
                        /* combined tint bits */
                        sat *= 0.6;
                        y -= sat;
                    } 

                    i += PHASES[tint_color as usize] * sat;
                    q += PHASES[(tint_color + 3) as usize] * sat;
                }
            }

            /* Apply brightness, contrast, and gamma */
            y *= setup.contrast as f32 * 0.5 + 1.0;
            /* adjustment reduces error when using input palette */
            y += setup.brightness as f32 * 0.5 - 0.5 / 256.0;

            {
                let (mut r, mut g, mut b) = yiq_to_rgb_float(y, i, q, &DEFAULT_DECODER);
                /* fast approximation of n = pow( n, gamma ) */
                r = (r * gamma_factor - gamma_factor) * r + r;
                g = (g * gamma_factor - gamma_factor) * g + g;
                b = (b * gamma_factor - gamma_factor) * b + b;
                let (yy, ii, qq) = rgb_to_yiq(r, g, b);
                y =yy;
                i =ii;
                q = qq;
            }

            i *= RGB_UNIT as f32;
			q *= RGB_UNIT as f32;
			y *= RGB_UNIT as f32;
            y += RGB_OFFSET;
            
            /* Generate kernel */
            {
                let (r, g, mut b) = yiq_to_rgb_int(y, i, q, &imp.to_rgb);
                /* blue tends to overflow, so clamp it */
                b = if b < 0x3E0 { 0x3E0 } else { b };
                let rgb: NesNtscRgbT = pack_rgb(r, g, b);

                gen_kernel(&mut imp, y, i, q, &mut ntsc.table[entry as usize]);
                if merge_fields > 0.0 { merge_kernel_fields(&mut ntsc.table[entry as usize]); }
                correct_errors(rgb, &mut ntsc.table[entry as usize]);
            }
        }
    }
}



/*
    Custom Blitter
    --------------
    row_width - number of pixels to get to next row (may be greater than width)
    For example if you want to blit 257 input
    pixels on a row (for whatever odd reason)
*/
pub fn nes_ntsc_blit(ntsc: &mut NesNtsc, input: &[u16],  row_width: u32, burst_phase: u32, in_width: u32, in_height: u32, rgb_out: &mut[u32], long_pitch: u32) {
    let chunk_count = (in_width -1) / NES_NTSC_IN_CHUNK;
    for _i in 0..in_height {

    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_info() {
        for n in NES_NTSC_PIXELS.iter() {
            assert!(n.offset >= 0);
            println!("pixel offset {}", n.offset);
        }
    }
}

