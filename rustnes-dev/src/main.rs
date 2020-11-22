use nes::consoles::{Console, nes_ntsc::NesNtsc,};
use nes::JoypadInput;
use std::path::Path;
use std::time::Instant;
use ::minifb::{Key, Window, WindowOptions, Scale, ScaleMode};

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

 
    let now = Instant::now();
    nes.execute_frame(&mut fb);
    let duration = now.elapsed().as_millis();

    println!("Frame Execution ms: {}", duration);
    //window.limit_update_rate(Some(std::time::Duration::from_millis(16)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        nes.execute_frame(&mut fb);
        let mut jp1 = JoypadInput::from_bits_truncate(0x0);
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::Up => jp1.set(JoypadInput::UP, true),
                    Key::Down => jp1.set(JoypadInput::DOWN, true),
                    Key::Left => jp1.set(JoypadInput::LEFT, true),
                    Key::Right => jp1.set(JoypadInput::RIGHT, true),
                    Key::Enter =>  jp1.set(JoypadInput::START, true),
                    Key::Backspace =>  jp1.set(JoypadInput::SELECT, true),
                    Key::A =>  jp1.set(JoypadInput::A, true),
                    Key::B =>  jp1.set(JoypadInput::B, true),
                    _ => (),
                }
            }
        });

        // update controller state for last frame
        nes.set_joypad1_state(jp1);

        window.update_with_buffer(&fb, 256, 240).unwrap();
    }
}

fn main() {
    //debug_run("test_roms\\nestest.nes");
    //debug_run("test_roms\\donkey_kong.nes");
    debug_run("test_roms\\cpu_interrupts.nes");
}
