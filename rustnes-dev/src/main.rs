use nes::consoles::{Console, nes_ntsc::NesNtsc,};
use nes::{JoypadInput, utils::time};
use std::path::Path;
use std::time::{Instant, Duration};
use ::minifb::{Menu, Key, Window, WindowOptions, Scale, ScaleMode, KeyRepeat};

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

const EMU_MODE: usize = 1;
const NORMAL: usize = 2;
const SINGLE_FRAME: usize = 3;

enum DebugMode {
    Normal,
    SingleFrame,
}

fn normal_run(nes: &mut NesNtsc, window: &mut Window, avg_frame_execution: &mut time::AvgDuration, frame_limit: &time::FrameLimit, fb: &mut [u32]) {
    avg_frame_execution.begin();
    let mut jp1 = JoypadInput::from_bits_truncate(0x0);
    nes.execute_frame(fb);

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
    window.update_with_buffer(fb, 256, 240).unwrap();
    avg_frame_execution.end();

    window.set_title(format!("RUSTNES --- avg frame execution {} us", avg_frame_execution.get_average_duration().as_micros()).as_str());
    frame_limit.end_of_frame(avg_frame_execution.get_current_duration());
}

fn single_frame_run(nes: &mut NesNtsc, window: &mut Window, avg_frame_execution: &mut time::AvgDuration, frame_limit: &time::FrameLimit, fb: &mut [u32]) {
    avg_frame_execution.begin();
    let mut jp1 = JoypadInput::from_bits_truncate(0x0);

    window.get_keys_pressed(KeyRepeat::No).map(|keys| {
        for t in keys {
            match t {
                Key::RightShift => {
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
                    
                    nes.execute_frame(fb);
                    // update controller state for last frame
                    nes.set_joypad1_state(jp1);
                    jp1 = JoypadInput::from_bits_truncate(0);
                 }
                _ => (),
            }
        }
    });

    window.update_with_buffer(fb, 256, 240).unwrap();
    avg_frame_execution.end();

    window.set_title(format!("RUSTNES --- frame {}", nes.get_frame_number()).as_str());
    frame_limit.end_of_frame(avg_frame_execution.get_current_duration());
}

fn main() {

    //debug_run("test_roms\\nestest.nes");
    //debug_run("test_roms\\donkey_kong.nes");
    //debug_run("test_roms\\2-nmi_and_brk.nes");
    //debug_run("test_roms\\Super Mario Bros (JU) (PRG 0).nes");
    //debug_run("test_roms\\color_test.nes");
    //debug_run("test_roms\\Mario Bros. (U) [!].nes");

    // window init code ============================
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

    let mut menu = Menu::new("Emulation Mode").unwrap();
    menu.add_item("Normal", NORMAL)
        .shortcut(Key::F1, 0)
        .build();
        menu.add_item("Single Frame", SINGLE_FRAME)
        .shortcut(Key::F2, 0)
        .build();

    let menu_handle = window.add_menu(&menu);

    // =============================================

    let mut fb: Vec<u32> = vec![0; 256*240];
    let mut debug_mode = DebugMode::Normal;
    let mut avg_frame_execution = time::AvgDuration::new();
    let frame_limit = time::FrameLimit::new(60);

    let mut nes = NesNtsc::new();
    nes.load_rom("test_roms\\test_ppu_read_buffer.nes");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // check menu
        window.is_menu_pressed().map(|menu_id| {
            match menu_id {
                NORMAL => {
                    debug_mode = DebugMode::Normal;
                }
                SINGLE_FRAME => {
                    debug_mode = DebugMode::SingleFrame;
                }
                _ => (),
            }
        });

        match debug_mode {
            DebugMode::Normal => {
                normal_run(&mut nes, &mut window, &mut avg_frame_execution, &frame_limit, &mut fb);
            }
            DebugMode::SingleFrame => {
                single_frame_run(&mut nes, &mut window, &mut avg_frame_execution, &frame_limit, &mut fb);
            }
        }
    }
}
