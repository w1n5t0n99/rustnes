use nes::consoles::{Console, nes_ntsc::NesNtsc};
use nes::{StandardInput, ZapperInput};
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::time::{Instant, Duration};
use ::minifb::{Key, Window, WindowOptions, Scale, ScaleMode};

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

pub fn execute_nestest_cpu_only<P: AsRef<Path>>(file_path: P) {
    let mut nes = NesNtsc::new();
    nes.load_rom(file_path);
    nes.set_entry(0xC000);
    let mut fb: Vec<u16> = vec![0; 256*240];
    let mut log_file = File::create("nestest_log.txt").expect("Unable to open log file");

    let now = Instant::now();
    let cycles = 27000;
    for _i in 0..cycles {
        log_file.write_all(format!("{}", nes).as_bytes()).unwrap(); 
        nes.execute_cycle();
    }
}

pub fn debug_run<P: AsRef<Path>>(file_path: P) {
    let mut fb: Vec<u32> = vec![0; 256*240];
    let mut nes = NesNtsc::new();
    nes.load_rom(file_path);

    //nes.nametable_framebuffer(&mut fb);

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
    let ten_millis = Duration::from_millis(10);
    let mut now = Instant::now();

    //std::thread::sleep(ten_millis);
    nes.execute_frame(&mut fb);

    let mut duration = now.elapsed().as_millis();

    println!("Frame Execution ms: {}", duration);
    let ctr1 = StandardInput::from_bits_truncate(0x0);
    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        nes.execute_frame(&mut fb);
        let mut ctr1 = StandardInput::from_bits_truncate(0x0);
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::Up => ctr1.set(StandardInput::Up, true),
                    Key::Down => ctr1.set(StandardInput::Down, true),
                    Key::Left => ctr1.set(StandardInput::Left, true),
                    Key::Right => ctr1.set(StandardInput::Right, true),
                    Key::Enter =>  ctr1.set(StandardInput::Start, true),
                    Key::Backspace =>  ctr1.set(StandardInput::Select, true),
                    Key::A =>  ctr1.set(StandardInput::A, true),
                    Key::B =>  ctr1.set(StandardInput::B, true),
                    _ => (),
                }
            }
        });

        // update controller state for last frame
        nes.update_controller1(ctr1);

        window.update_with_buffer(&fb, 256, 240).unwrap();
    }
}

fn main() {
    //execute_nestest_cpu_only("test_roms\\nestest.nes");
    //debug_run("test_roms\\donkey_kong.nes");
    debug_run("test_roms\\nestest.nes");
}
