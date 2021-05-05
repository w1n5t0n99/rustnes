use nes::consoles::{Console, EmuError};
use nes::consoles::nes_ntsc::NesNtsc;
use nes::JoypadInput;
use nes::utils::{frame_limiter, average_duration};

use ::minifb::{Menu, Key, Window, WindowOptions, Scale, ScaleMode, KeyRepeat};

use std::{io::Write, time::{Instant, Duration}};
use std::fs::File;
use std::path::Path; 
use std::io::BufWriter;

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

const MENU_NORMAL: usize = 2;
const MENU_SINGLE_FRAME: usize = 3;
const MENU_BEGIN_LOG: usize = 5;
const MENU_END_LOG: usize = 6;
const MENU_POWERON: usize = 8;
const MENU_RESTART: usize = 9;

enum EmuMode {
    Normal,
    SingleFrame,
}

pub fn normal_execute<C: Console>(nes: &mut C, jp1: JoypadInput, fb: &mut [u32]) -> Duration {
    let start_instant = Instant::now();
    nes.input_joypad1_state(jp1);                 
    nes.execute_frame();
    let emu_res = nes.output_pixel_buffer(fb);

    match emu_res {
        Ok(_) => { }
        Err(emu_err) => {
            let log_file = File::create(format!("logs\\trace-frame-{}-error.log", nes.get_frame_number())).unwrap();
            let mut log_writer = BufWriter::new(log_file);  
            nes.output_log(&mut log_writer);
        }
    }

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
    //debug_run("test_roms\\ppu_sprite_hit.nes");
    //debug_run("test_roms\\blargg_ppu_tests\\palette_ram.nes");
    //nes.load_rom("test_roms\\sprite_hit_tests\\07.screen_bottom.nes");

    // window init code ============================
    let window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X2,
        scale_mode: ScaleMode::AspectRatioStretch,
        topmost: true,
        transparency: false,
        none: false,
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
    let mut console_menu = Menu::new("Console").unwrap();

    mode_menu.add_item("Normal", MENU_NORMAL)
        .shortcut(Key::F1, 0)
        .build();
    mode_menu.add_item("Single Frame", MENU_SINGLE_FRAME)
        .shortcut(Key::F2, 0)
        .build();

    debug_menu.add_item("Begin Log", MENU_BEGIN_LOG)
        .shortcut(Key::F7, 0)
        .build();
    debug_menu.add_item("End Log", MENU_END_LOG)
        .shortcut(Key::F8, 0)
        .build();

    menu.add_sub_menu("Emu Mode", &mode_menu);
    //menu.add_separator();
    menu.add_sub_menu("Debug", &debug_menu);

    console_menu.add_item("Power On", MENU_POWERON)
        .build();
    console_menu.add_item("Restart", MENU_RESTART)
        .build();

    window.add_menu(&menu);
    window.add_menu(&console_menu);

    // =============================================

    let mut emu_mode = EmuMode::Normal;
    let mut emu_pause = false;
    let mut exec_frame = false;
    let mut enable_trace_log = false;

    let mut average_duration = average_duration::AverageDuration::new();
    let mut frame_limiter = frame_limiter::FrameLimiter::new(60);

    let mut fb: Vec<u32> = vec![0; WIDTH*HEIGHT];  
    let mut nes = NesNtsc::new();
    let mut jp1 = JoypadInput::new();

    nes.load_rom("test_roms\\instr_test-v5\\rom_singles\\16-special.nes");
 
    while window.is_open() && !window.is_key_down(Key::Escape) {
        frame_limiter.start();

        // check menu
        window.is_menu_pressed().map(|menu_id| {
            match menu_id {
                MENU_NORMAL => {
                    emu_mode = EmuMode::Normal;
                }
                MENU_SINGLE_FRAME => {
                    emu_mode = EmuMode::SingleFrame;
                }
                MENU_BEGIN_LOG => {
                    enable_trace_log = true;
                }
                MENU_END_LOG => {
                    enable_trace_log = false;
                }
                MENU_POWERON => {
                    nes.power_on_console();
                }
                MENU_RESTART => {
                    nes.restart_console();
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
