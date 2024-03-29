pub mod rp2c02;
mod ppu_registers;
mod ppu_operations;
pub mod background;
pub mod sprites;
mod ssprites;
mod bus;
mod palette_ram;
mod scanline_prerender;
mod scanline_render;
mod scanline_postrender;
mod scanline_vblank;

bitflags! {
    pub struct Ctrl: u8 {
        const RD =   0b00000001;     // /RD read from VRAM. This is asserted when reading from palette
        const WR =   0b00000010;     // /WR write to VRAM. This is NOT asserted when writing to palette
        const ALE =  0b00000100;     // ALE goes high at beggining of VRAM access
    }
}

impl Default for Ctrl {
    fn default() -> Ctrl {
        Ctrl::WR | Ctrl::RD
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pinout {
    pub ctrl: Ctrl,
    pub address: u16,
    pub data: u8,
}

impl Pinout {
    pub fn new() -> Self {
        Pinout {
            ctrl: Ctrl::default(),
            address: 0,
            data: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Context {
    pub cycle: u64,
    pub frame: u64,
    pub read_2002_cycle: u64,                           // Used to track NMI race condition
    pub addr_reg: ppu_registers::AddrReg,
    pub control_reg: ppu_registers::ControlRegister,
    pub prev_control_reg: ppu_registers::ControlRegister,   // used to trigger multiple NMI during vblank
    pub mask_reg: ppu_registers::MaskRegister,
    pub status_reg: ppu_registers::StatusRegister,
    pub vpos: u16,
    pub hpos: u16, 
    pub io_db: u8,                                      // Simulate latch created by long traces of data bus
    pub odd_frame: bool,
    pub write_block: bool,
    pub last_frame_cycle: bool,
}

impl Context {
    pub fn new() -> Self {
        Context {
            cycle: 0,
            frame: 0,
            read_2002_cycle: 0,
            addr_reg: ppu_registers::AddrReg::new(),
            control_reg: ppu_registers::ControlRegister::new(),
            prev_control_reg: ppu_registers::ControlRegister::new(),
            mask_reg: ppu_registers::MaskRegister::new(),
            status_reg: ppu_registers::StatusRegister::new(),
            vpos: 261,
            hpos: 0,
            io_db: 0,
            odd_frame: false,
            write_block: true,
            last_frame_cycle: false,
        }
    }
}
