pub mod nes_ntsc;

use crate::controllers::JoypadInput;
use std::path::Path;
use std::io::Write;

pub trait Console {
    fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P);
    fn power_on_console(&mut self);
    fn restart_console(& mut self);
    fn execute_frame(&mut self, frame_buffer: &mut [u32]);
    fn execute_frame_debug<W: Write>(&mut self ,w: &mut W, frame_buffer: &mut [u32]);
    fn execute_scanline(&mut self, frame_buffer: &mut [u32]);
    fn execute_cycle(&mut self, frame_buffer: &mut [u32]);
    fn set_joypad1_state(&mut self, joypad: JoypadInput);
    fn set_joypad2_state(&mut self, joypad: JoypadInput);
    fn get_frame_number(&mut self) -> u64;
}