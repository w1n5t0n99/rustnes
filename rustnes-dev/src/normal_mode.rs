use nes::consoles::Console;
use nes::JoypadInput;
use ::minifb::Window;

pub fn normal_execute<C: Console>(window: &mut Window, nes: &mut C, jp1: JoypadInput, fb: &mut [u32]) {
    nes.set_joypad1_state(jp1);                 
    nes.execute_frame(fb);
}