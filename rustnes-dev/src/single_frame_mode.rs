use nes::consoles::Console;
use nes::JoypadInput;
use ::minifb::{Key, Window, KeyRepeat};

pub fn single_frame_execute<C: Console>(window: &mut Window, nes: &mut C, jp1: JoypadInput, fb: &mut [u32]) {
    window.get_keys_pressed(KeyRepeat::No).map(|keys| {
        for t in keys {
            match t {
                Key::Period => {   
                    nes.set_joypad1_state(jp1);                 
                    nes.execute_frame(fb);
                 }
                _ => (),
            }
        }
    });
}