
mod single_frame_mode;
mod normal_mode;

use single_frame_mode::*;
use normal_mode::*;

use nes::consoles::{Console, nes_ntsc::NesNtsc,};
use nes::{JoypadInput, utils::time};
use std::path::Path;
use std::time::{Instant, Duration};
use ::minifb::{Menu, Key, Window, WindowOptions, Scale, ScaleMode, KeyRepeat};

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

    let menu_handle = window.add_menu(&menu);

    // =============================================

    let mut emu_mode = EmuMode::Normal;
    let mut emu_pause = false;

    let mut avg_frame_execution = time::AvgDuration::new();
    let frame_limit = time::FrameLimit::new(60);

    let mut fb: Vec<u32> = vec![0; WIDTH*HEIGHT];  
    let mut nes = NesNtsc::new();
    nes.load_rom("test_roms\\Mario Bros. (U) [!].nes");
    let mut jp1 = JoypadInput::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        avg_frame_execution.begin();

        // check menu
        window.is_menu_pressed().map(|menu_id| {
            match menu_id {
                NORMAL => {
                    emu_mode = EmuMode::Normal;
                }
                SINGLE_FRAME => {
                    emu_mode = EmuMode::SingleFrame;
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
        
        nes.set_joypad1_state(jp1);

        match emu_mode {
            EmuMode::Normal => {
                normal_execute(&mut window, &mut nes, jp1, &mut fb);
            }
            EmuMode::SingleFrame => {
                single_frame_execute(&mut window, &mut nes, jp1, &mut fb);
            }
        }

        window.update_with_buffer(&fb, WIDTH, HEIGHT).unwrap();
        avg_frame_execution.end();

        //window.set_title(format!("RUSTNES --- avg frame execution {} us", avg_frame_execution.get_average_duration().as_micros()).as_str());
        frame_limit.end_of_frame(avg_frame_execution.get_current_duration());
    }
}
