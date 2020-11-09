pub mod nes_ntsc;

use std::path::Path;

const WIDTH: u32 = 256;
const PADDED_WIDTH: u32 = 282;
const HEIGHT: u32 = 240;

pub trait Console {
    fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P);
    fn power_on(&mut self);
    fn restart(& mut self);
    fn execute_frame(&mut self, frame_buffer: &mut [u32]);
    // TODO handle inputs, handle audio
    // TODO implement error handling on load rom
}