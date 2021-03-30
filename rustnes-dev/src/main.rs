use nes::consoles::{Console, nes_ntsc::NesNtsc,};
use nes::JoypadInput;
use nes::utils::{frame_limiter, average_duration};

use ::minifb::{Menu, Key, Window, WindowOptions, Scale, ScaleMode, KeyRepeat};

use std::{io::Write, time::{Instant, Duration}};
use std::fs::File;
use std::path::Path; 
use std::io::BufWriter;

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

const MODE_MENU: usize = 1;
const NORMAL: usize = 2;
const SINGLE_FRAME: usize = 3;
const DEBUG_MENU: usize = 4;
const BEGIN_LOG: usize = 5;
const END_LOG: usize = 6;

enum EmuMode {
    Normal,
    SingleFrame,
}

pub fn normal_execute<C: Console>(nes: &mut C, jp1: JoypadInput, fb: &mut [u32]) -> Duration {
    let start_instant = Instant::now();
    nes.input_joypad1_state(jp1);                 
    nes.execute_frame();
    nes.output_pixel_buffer(fb);
    Instant::now() - start_instant
}

fn main() {

    //debug_run("test_roms\\nestest.nes");
    //debug_run("test_roms\\donkey_kong.nes");
    //debug_run("test_roms\\2-nmi_and_brk.nes");
    //debug_run("test_roms\\Super Mario Bros (JU) (PRG 0).nes");
    //debug_run("test_roms\\color_test.nes");
    //debug_run("test_roms\\Mario Bros. (U) [!].nes");
    //debug_run("test_roms\\test_ppu_read_buffer.nes");

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
        WIDTH,
        HEIGHT,
        window_options,
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut menu = Menu::new("Menu").unwrap();
    let mut mode_menu = Menu::new("Emulation Mode").unwrap();
    let mut debug_menu = Menu::new("Debug").unwrap();

    mode_menu.add_item("Normal", NORMAL)
        .shortcut(Key::F1, 0)
        .build();
    mode_menu.add_item("Single Frame", SINGLE_FRAME)
        .shortcut(Key::F2, 0)
        .build();

    debug_menu.add_item("Begin Log", BEGIN_LOG)
        .shortcut(Key::F7, 0)
        .build();
    debug_menu.add_item("End Log", END_LOG)
        .shortcut(Key::F8, 0)
        .build();

    menu.add_sub_menu("Emu Mode", &mode_menu);
    //menu.add_separator();
    menu.add_sub_menu("Debug", &debug_menu);

    let _menu_handle = window.add_menu(&menu);

    // =============================================

    let mut emu_mode = EmuMode::Normal;
    let mut emu_pause = false;
    let mut exec_frame = false;
    let mut enable_trace_log = false;
    let mut old_enable_trace_log = false;

    let mut average_duration = average_duration::AverageDuration::new();
    let mut frame_limiter = frame_limiter::FrameLimiter::new(60);

    let mut fb: Vec<u32> = vec![0; WIDTH*HEIGHT];  
    let mut nes = NesNtsc::new();
    nes.load_rom("test_roms\\Mario Bros. (U) [!].nes");
    let mut jp1 = JoypadInput::new();
 
    while window.is_open() && !window.is_key_down(Key::Escape) {
        frame_limiter.start();

        // check menu
        window.is_menu_pressed().map(|menu_id| {
            match menu_id {
                NORMAL => {
                    emu_mode = EmuMode::Normal;
                }
                SINGLE_FRAME => {
                    emu_mode = EmuMode::SingleFrame;
                }
                BEGIN_LOG => {
                    enable_trace_log = true;
                }
                END_LOG => {
                    enable_trace_log = false;
                }
                _ => (),
            }
        });

        jp1.clear();
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
        
        window.get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                match t {
                    Key::Period => exec_frame = true,
                    Key::P => emu_pause = !emu_pause,
                    _ => (),
                }
            }
        });
        
        if emu_pause == false {
            match emu_mode {
                EmuMode::Normal => {
                    average_duration.update(normal_execute(&mut nes, jp1, &mut fb));
                }
                EmuMode::SingleFrame => {
                    if exec_frame {
                        average_duration.update(normal_execute(&mut nes, jp1, &mut fb));
                        // trace logs quickly grow huge, only really useful if going frame by frame
                        if enable_trace_log {
                            let log_file = File::create(format!("logs\\trace-frame-{}.log", nes.get_frame_number())).unwrap();
                            let mut log_writer = BufWriter::new(log_file);  
                            nes.output_log(&mut log_writer);
                        }
                    }
                }
            }
        }

        exec_frame = false;
        window.update_with_buffer(&fb, WIDTH, HEIGHT).unwrap();

        window.set_title(format!("RUSTNES --- avg frame execution {} us", average_duration.get_average_duration().as_micros()).as_str());
        frame_limiter.wait();
    }
}
