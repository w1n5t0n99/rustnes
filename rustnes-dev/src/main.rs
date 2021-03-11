use nes::consoles::{Console, nes_ntsc::NesNtsc,};
use nes::{JoypadInput, utils};
use std::path::Path;
use std::time::{Instant, Duration};
use ::minifb::{Key, Window, WindowOptions, Scale, ScaleMode};


pub fn debug_run<P: AsRef<Path>>(file_path: P) {
    let mut fb: Vec<u32> = vec![0; 256*240];
    let mut nes = NesNtsc::new();
    nes.load_rom(file_path);


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

 
    let mut avg_frame_execution = utils::AvgDuration::new();
    let frame_limit = utils::FrameLimit::new(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
       
        avg_frame_execution.begin();

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

        avg_frame_execution.end();

        window.set_title(format!("RUSTNES --- avg frame execution {} us", avg_frame_execution.get_average_duration().as_micros()).as_str());

        frame_limit.end_of_frame(avg_frame_execution.get_current_duration());
    }
}

fn main() {
    //debug_run("test_roms\\nestest.nes");
    //debug_run("test_roms\\donkey_kong.nes");
    //debug_run("test_roms\\Super Mario Bros (JU) (PRG 0).nes");
    //debug_run("test_roms\\color_test.nes");
    debug_run("test_roms\\Mario Bros. (U) [!].nes");
    //debug_run("test_roms\\scroll.nes");
}
