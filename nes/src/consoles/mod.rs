pub mod nes_ntsc;

use crate::controllers::JoypadInput;
use std::path::Path;
use std::io::Write;

pub trait Console {
    fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P);
    fn power_on_console(&mut self);
    fn restart_console(& mut self);

    fn get_frame_number(&self) -> u64;
    fn get_index_buffer(&self) -> &[u16];

    fn execute_frame(&mut self);

    fn input_joypad1_state(&mut self, joypad: JoypadInput);
    fn input_joypad2_state(&mut self, joypad: JoypadInput);

    fn output_pixel_buffer(&mut self, frame_buffer: &mut [u32]);
    fn output_log<W: Write>(&mut self , w: &mut W);    
}