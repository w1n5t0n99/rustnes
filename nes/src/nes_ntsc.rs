/*
    Rust rewrite of Shay Green NES NTSC filtering library

    Reads NES pixels and writes RGB pixels (16-bit by
    default). The NES pixels are 6-bit raw palette values (0 to 0x3F). Edit
    nes_ntsc_config.h to change this
*/
type NesNtscRgb = u32;
const NES_NTSC_ENTRY_SIZE: u32 = 128;
const NES_NTSC_PALETTE_SIZE: u32 = 64 * 8;    // 6 bit color + 3 bit emphasis
// const NES_NTSC_PALETTE_SIZE: usize = 64;     // 6 bit  color only

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

const BURST_SIZE: u32 = NES_NTSC_ENTRY_SIZE / NES_NTSC_BURST_COUNT;
const KERNEL_HALF: u32 = 16;
const KERNEL_SIZE: u32 = (KERNEL_HALF * 2) + 1;
const RESCALE_OUT: u32 = 7;
const RESCALE_IN: u32 = 8;
const ALIGNMENT_COUNT: u32 = 3;
const RGB_KERNEL_SIZE: u32 = BURST_SIZE / ALIGNMENT_COUNT;
const RGB_BIAS: u32 = RGB_UNIT * 2 * NES_NTSC_RGB_BUILDER;

const DEFAULT_DECODER: [f32; 6] = [0.956, 0.621, -0.272, -0.647, -1.105, 1.702];

macro_rules! pixel_negate{
    ($ntsc:expr) => { 1.0_f32 - (($ntsc as i32 + 100) & 2) as f32 };
}

const fn pixel_offset(mut ntsc: i32, mut scaled: i32) -> i32 {
    ntsc = ntsc - scaled / RESCALE_OUT as i32 * RESCALE_IN as i32;
    scaled = (scaled + RESCALE_OUT as i32 * 10) % RESCALE_OUT as i32;

    KERNEL_SIZE as i32 / 2 + ntsc + 1 + (RESCALE_OUT as i32 - scaled) % RESCALE_OUT as i32 + (KERNEL_SIZE as i32 * 2 * scaled)
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

fn rotate_iq(i: &mut f32, q: &mut f32, sin_b: f32, cos_b: f32) {
    let t = *i * cos_b - *q * sin_b;
    *q = *i * sin_b * *q * cos_b;
    *i = t;
}

#[derive(Debug, Clone, Copy)]
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
pub struct PixelInfo {
    offset: i32,
    negate: f32,
    kernel: [f32; 4],
}

/* 3 input pixels -> 8 composite samples */
// macro expansion from original source
pub const NES_NTSC_PIXELS: [PixelInfo; ALIGNMENT_COUNT as usize] = [
    PixelInfo { 
        offset: pixel_offset(-4, -9),
        negate: pixel_negate!(-4),
        kernel: [1.0, 1.0, 0.6667, 0.0],
    },
    
    PixelInfo {
        offset: pixel_offset(-2, -7),
        negate: pixel_negate!(-2),
        kernel: [0.3333, 1.0, 1.0, 0.3333],
    },

    PixelInfo {
        offset: pixel_offset(0, -5),
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

fn init_filters(init: &mut Init, setup: &NesNtscSetup) {
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
        let x = &mut init.kernel;
        let mut kernel_index: usize = 0;
        for n in (0..RESCALE_OUT).rev() {
            let mut remain: f32 = 0.0;
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

fn init(init: &mut Init, setup: &NesNtscSetup) {
    init.brightness = setup.brightness as f32 * (0.5 * RGB_UNIT as f32) + RGB_OFFSET;
    init.contrast = setup.contrast as f32 * (0.5 * RGB_UNIT as f32) + RGB_UNIT as f32;
    // TODO default palette contrast

    init.artifacts = setup.artifacts as f32;
    if init.artifacts > 0.0 { init.artifacts *= ARTIFACTS_MAX - ARTIFACTS_MID; }
    init.artifacts = init.artifacts * ARTIFACTS_MID + ARTIFACTS_MID;

    init.fringing = setup.fringing as f32;
    if init.fringing > 0.0 { init.fringing *= FRINGING_MAX - FRINGING_MID; }
    init.fringing = init.fringing * init.fringing + FRINGING_MID;

    init_filters(init, setup);

    /* generate gamma table */
    if GAMMA_SIZE > 1 {
        // TODO check correctness
        let to_float: f32 = 1.0 / (GAMMA_SIZE as f32 - (GAMMA_SIZE - 1) as f32);
        let gamma : f32 = 1.1333 - setup.gamma as f32 * 0.5;
        /* match common PC's 2.2 gamma to TV's 2.65 gamma */
        for i in 0..GAMMA_SIZE {
            init.to_float[i as usize] = (i as f32 * to_float).powf(gamma) * init.contrast + init.brightness;
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

                init.to_rgb[x] = i * c - q * s;
                x += 1;
                init.to_rgb[x] = i * s + q * c;
                x += 1;
            }

            rotate_iq(&mut s, &mut c, 0.866025, -0.5);
        }
    }
}

/* Generate pixel at all burst phases and column alignments */
fn gen_kernel(init: &mut Init, y: f32, i: f32, q: f32, out: &mut NesNtscRgb) {
    
}



