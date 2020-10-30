use super::ppu_registers::{AddrReg, StatusRegister, ControlRegister, MaskRegister};

#[derive(Debug, Clone, Copy)]
pub struct Context {
    addr_reg: AddrReg,
    control_reg: ControlRegister,
    mask_reg: MaskRegister,
    status_reg: StatusRegister,
}

