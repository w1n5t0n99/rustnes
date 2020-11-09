use nes::consoles::{Console, nes_ntsc::NesNtsc};
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::time::Instant;
use ::minifb::{Key, Window, WindowOptions, Scale, ScaleMode};

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

pub fn execute_nestest_cpu_only<P: AsRef<Path>>(file_path: P) {
    let mut nes = NesNtsc::new();
    nes.load_rom(file_path);
    nes.set_entry(0xC000);
    let mut fb: Vec<u16> = vec![0; 256*240];
    let mut log_file = File::create("nes_log.txt").expect("Unable to open log file");

    
    let mut cpu_ofile = File::create("nestest_rustnes_log.txt").expect("unable to create file");
    let now = Instant::now();
    let cycles = 27000;
    for _i in 0..cycles {
        cpu_ofile.write_all(format!("{}\n", nes).as_bytes()).unwrap();
        //nes.execute_cycle(&mut fb, &mut log_file);
    }

    println!("new nes {:?} cpu cycles in {:?}", cycles, now.elapsed());
}

pub fn ppu_debug<P: AsRef<Path>>(file_path: P) {
    let mut fb: Vec<u32> = vec![0; 256*240];
    let mut nes = NesNtsc::new();
    //nes.load_rom(file_path);
    nes.load_debug_rom();
    let now = Instant::now();

    nes.nametable_framebuffer(&mut fb);
    println!("FRAME TIME: {}", now.elapsed().as_millis());

    let window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X2,
        scale_mode: ScaleMode::AspectRatioStretch,
        topmost: true,
        transparency: false,
    };

    let mut window = Window::new(
        "NES Test - ESC to exit",
        256,
        240,
        window_options,
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_millis(16)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        //let rgb_buffer: Vec<u32> = fb.iter().map(|pixel| palette[*pixel as usize]).collect();
        window.update_with_buffer(&fb, 256, 240).unwrap();
    }
}

pub fn debug_run<P: AsRef<Path>>(file_path: P) {
    let mut fb: Vec<u32> = vec![0; 256*240];
    let mut nes = NesNtsc::new();
    nes.load_rom(file_path);
    let now = Instant::now();

    nes.nametable_framebuffer(&mut fb);

    let window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X2,
        scale_mode: ScaleMode::AspectRatioStretch,
        topmost: true,
        transparency: false,
    };

    let mut window = Window::new(
        "NES Test - ESC to exit",
        256,
        240,
        window_options,
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut log_file = File::create("nes_log.txt").expect("Unable to open log file");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        //nes.execute_frame(&mut fb);
        for _i in 0..(29781*10) { log_file.write_all(format!("{}\n", nes).as_bytes()).unwrap(); }
        window.update_with_buffer(&fb, 256, 240).unwrap();
    }
}

fn main() {
    //execute_nestest_cpu_only("test_roms\\nestest.nes")?;
    //ppu_debug("test_roms\\donkey_kong.nes");
    debug_run("test_roms\\donkey_kong.nes");
}
