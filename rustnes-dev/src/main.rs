use nes::{Nes, NesError};
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::time::Instant;
use ::minifb::{Key, Window, WindowOptions, Scale, ScaleMode};

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

pub fn execute_nestest_cpu_only<P: AsRef<Path>>(file_path: P) -> Result<(), NesError> {
    let mut nes = Nes::from_power_on();
    nes.load_rom(file_path)?;
    nes.debug_reset(0xC000);
    
    let mut cpu_ofile = File::create("nestest_rustnes_log.txt").expect("unable to create file");
    let now = Instant::now();
    let cycles = 27000;
    for _i in 0..cycles {
        cpu_ofile.write_all(format!("{}\n", nes).as_bytes()).unwrap();
        nes.execute_cycle();
    }

    println!("new nes {:?} cpu cycles in {:?}", cycles, now.elapsed());

    Ok(())
}

pub fn display_rom_chr<P: AsRef<Path>>(file_path: P) -> Result<(), NesError> {
    let mut nes = Nes::from_power_on();
    nes.load_rom(file_path)?;
    nes.debug_reset(0xC000);
    let mut chr_buffer = nes.chr_framebuffer();
    
    let chr_window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X2,
        scale_mode: ScaleMode::AspectRatioStretch,
        topmost: true,
        transparency: false,
    };

    let chr_width: usize = 32 * 8;
    let chr_height: usize = chr_buffer.len() / chr_width;
    println!("{} Width {} Height {} Size",chr_width, chr_height, chr_buffer.len());

    let mut window = Window::new(
        "Test - ESC to exit",
        chr_width,
        chr_height,
        chr_window_options,
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&chr_buffer, chr_width, chr_height)
            .unwrap();
        
            chr_buffer = nes.chr_framebuffer();
    }

    Ok(())
}

pub fn ppu_debug<P: AsRef<Path>>(file_path: P) {
    let mut nes = Nes::from_power_on();
    nes.load_debug_rom();

    nes.execute_debug_frame(file_path);
}

fn main() -> Result<(), NesError> {
    //execute_nestest_cpu_only("test_roms\\nestest.nes")?;
    display_rom_chr("test_roms\\nestest.nes")?;
    ppu_debug("ppu_log.txt");

    Ok(())
}
