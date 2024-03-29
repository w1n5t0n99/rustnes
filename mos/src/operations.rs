use super::{Ctrl, Pinout};
use super::core::*;
use super::instructions::Instruction;
use super::bus::Bus;

const fn to_address(hb: u8, lb: u8) -> u16 {
    (hb as u16) << 8 | (lb as u16) 
}

#[inline]
fn is_nmi_asserted(cpu: &mut Context) -> bool {
    cpu.nmi_detected
}

#[inline]
fn is_irq_asserted(cpu: &mut Context, pinout: Pinout) -> bool {
    pinout.ctrl.contains(Ctrl::IRQ) == false && cpu.p.contains(StatusRegister::INT_DISABLE) == false
}

fn poll_interrupts(cpu: &mut Context, pinout: Pinout) {
    // nmi is edge detected, only needs to be held one cycle to set flag
    if is_nmi_asserted(cpu) {
        cpu.nmi_detected = false;
        cpu.ops.reset();
        cpu.ir.reset_to_nmi();
        cpu.int_vec_low = NMI_VEC_LOW;
    }
    // irq is level detected and must be held every cycle until handled
    else if is_irq_asserted(cpu, pinout) {
        cpu.ops.reset();
        cpu.ir.reset_to_irq();
        cpu.int_vec_low = IRQ_BRK_VEC_LOW;
    }
}

// interrupts are really polled on the second to last cycle
// this really only matters for SEI, CLI, PLP
fn delayed_poll_interrupts(cpu: &mut Context, irq_detected: bool) {
    // nmi is edge detected, only needs to be held one cycle to set flag
    if is_nmi_asserted(cpu) {
        cpu.nmi_detected = false;
        cpu.ops.reset();
        cpu.ir.reset_to_nmi();
        cpu.int_vec_low = NMI_VEC_LOW;
    }
    // irq is level detected and must be held every cycle until handled
    else if irq_detected {
        cpu.ops.reset();
        cpu.ir.reset_to_irq();
        cpu.int_vec_low = IRQ_BRK_VEC_LOW;
    }
}

//====================================================
// helper macros
//====================================================
macro_rules! first_cycle {
    ($cpu:ident, $bus:ident, $pinout:ident) => {
        // sync used to track first cycle of instruction
        $pinout.ctrl.set(Ctrl::SYNC, true);
        $pinout.ctrl.set(Ctrl::RW, true);
        $pinout.address = u16::from($cpu.pc);
        // always fetch opcode
        $pinout = $bus.read($pinout);
        // set instruction register to new opcode
        $cpu.ir.reset($pinout.data);
        $cpu.ops.reset();
        $cpu.pc.increment();
    }
}

macro_rules! second_cycle {
    ($cpu:ident, $bus:ident, $pinout:ident) => {
        $pinout.ctrl.set(Ctrl::SYNC, false);
        // instructions always read next byte after opcode
        $pinout.ctrl.set(Ctrl::RW, true);
        $pinout.address = u16::from($cpu.pc);
        // always fetch byte after opcode
        $pinout = $bus.read($pinout);
        $cpu.ops.dl = $pinout.data;

        $cpu.ir.increment();
    }
}

macro_rules! last_cycle {
    ($cpu:ident, $pinout:ident) => {
        poll_interrupts($cpu, $pinout);
        // instruction don't have more than 7 cycles so must've been set to interrupt
        if $cpu.ir.tm > 0xF {
            return $pinout;
        }
    }
}

macro_rules! delayed_int_last_cycle {
    ($cpu:ident, $irq_detected:ident,  $pinout:ident) => {
        delayed_poll_interrupts($cpu, $irq_detected);
        // instruction don't have more than 7 cycles so must've been set to interrupt
        if $cpu.ir.tm > 0xF {
            return $pinout;
        }
    }
}

macro_rules! read_cycle {
    ($cpu:ident, $bus:ident, $pinout:ident, $addr:expr) => {
        $pinout.ctrl.set(Ctrl::RW, true);
        $pinout.address = $addr;
        // get results from bus
        $pinout = $bus.read($pinout);
        $cpu.ops.dl = $pinout.data;

        $cpu.ir.increment();
    }
}

macro_rules! write_cycle {
    ($cpu:ident, $bus:ident, $pinout:ident, $addr:expr, $data: expr) => {
        $pinout.ctrl.set(Ctrl::RW, false);
        $pinout.address = $addr;
        $pinout.data = $data;
        // get results from bus
        $pinout = $bus.write($pinout);

        $cpu.ir.increment();
    }
}

//===================================================
// Reset
//====================================================
pub fn rst_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    cpu.a = 0xAA;
    cpu.p = StatusRegister::from_power_on();
    cpu.pc = ProgramCounter::from(0x00FF);
    read_cycle!(cpu, bus, pinout, 0x00FF);

    pinout
}

pub fn rst_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    read_cycle!(cpu, bus, pinout, 0x00FF);

    pinout
}

pub fn rst_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    read_cycle!(cpu, bus, pinout, 0x00FF);

    pinout
}

pub fn rst_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    read_cycle!(cpu, bus, pinout, 0x0100);

    pinout
}

pub fn rst_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    read_cycle!(cpu, bus, pinout, 0x01FF);

    pinout
}

pub fn rst_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    read_cycle!(cpu, bus, pinout, 0x01FE);
    cpu.sp = 0xFD;

    pinout
}

pub fn rst_c6<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    read_cycle!(cpu, bus, pinout, 0xFFFC);
    cpu.ops.adl = cpu.ops.dl;

    pinout
}

pub fn rst_c7<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    read_cycle!(cpu, bus, pinout, 0xFFFD);
    cpu.ops.adh = cpu.ops.dl;
    
    pinout
}

pub fn rst_c8<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // first cycle of next instruction 
    cpu.pc.pcl = cpu.ops.adl;
    cpu.pc.pch = cpu.ops.adh;
    // kludge to match nestest.log cycle timing after reset
    cpu.cycle = 7;
    first_cycle!(cpu, bus, pinout);

    pinout
}

//===================================================
// Break
//====================================================
pub fn brk_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // read instruction byte (discarded)
    second_cycle!(cpu, bus, pinout);
    cpu.pc.increment();
    pinout
}

pub fn brk_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write pch to stack
    write_cycle!(cpu, bus, pinout, to_address(0x01, cpu.sp), cpu.pc.pch);
    // decrement sp
    cpu.sp = cpu.sp.wrapping_sub(1);
    pinout
}

pub fn brk_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write pcl  to stack
    write_cycle!(cpu, bus, pinout, to_address(0x01, cpu.sp), cpu.pc.pcl);
    // decrement sp
    cpu.sp = cpu.sp.wrapping_sub(1);
    // check for hijack
    if is_nmi_asserted(cpu) {
        cpu.int_vec_low = NMI_VEC_LOW;
        cpu.nmi_detected = false;
        cpu.p.set(StatusRegister::INT_DISABLE, true);
    }
    else if is_irq_asserted(cpu, pinout) {
        cpu.int_vec_low = IRQ_BRK_VEC_LOW;
        cpu.p.set(StatusRegister::INT_DISABLE, true);
    }

    pinout
}

pub fn brk_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write status reg to stack
    write_cycle!(cpu, bus, pinout, to_address(0x01, cpu.sp), cpu.p.push_with_b());

    // decrement sp
    cpu.sp = cpu.sp.wrapping_sub(1);
    pinout
}

pub fn brk_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // set to_address to fetch pcl
    let addr = to_address(0xFF, cpu.int_vec_low);

    read_cycle!(cpu, bus, pinout, addr);
    cpu.pc.pcl = cpu.ops.dl;
    // set i flag
    cpu.p.set(StatusRegister::INT_DISABLE, true);
    pinout
}

pub fn brk_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // set to_address to fetch pch
    let addr = to_address(0xFF, cpu.int_vec_low + 1);

    read_cycle!(cpu, bus, pinout, addr);
    cpu.pc.pch = cpu.ops.dl;
    // set the interrupt vector back to the default with is the BRK vec
    cpu.int_vec_low = IRQ_BRK_VEC_LOW;
    pinout
}

pub fn brk_c6<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//===================================================
// IRQ
//====================================================
pub fn irq_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // read instruction byte (discarded)
    second_cycle!(cpu, bus, pinout);
    pinout
}

pub fn irq_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write pch to stack
    write_cycle!(cpu, bus, pinout, to_address(0x01, cpu.sp), cpu.pc.pch);
    // decrement sp
    cpu.sp = cpu.sp.wrapping_sub(1);
    pinout
}

pub fn irq_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write pcl  to stack
    write_cycle!(cpu, bus, pinout, to_address(0x01, cpu.sp), cpu.pc.pcl);
    // decrement sp
    cpu.sp = cpu.sp.wrapping_sub(1);
    // check for hijack
    if is_nmi_asserted(cpu) {
        cpu.int_vec_low = NMI_VEC_LOW;
        cpu.nmi_detected = false;
    }

    pinout
}

pub fn irq_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write status reg to stack
    write_cycle!(cpu, bus, pinout, to_address(0x01, cpu.sp), cpu.p.push_without_b());
    // after status write set I flag
    cpu.p.set(StatusRegister::INT_DISABLE, true);

    // decrement sp
    cpu.sp = cpu.sp.wrapping_sub(1);
    pinout
}

pub fn irq_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // set to_address to fetch pcl
    let addr = to_address(0xFF, cpu.int_vec_low);

    read_cycle!(cpu, bus, pinout, addr);
    cpu.pc.pcl = cpu.ops.dl;
    // set i flag
    cpu.p.set(StatusRegister::INT_DISABLE, true);
    pinout
}

pub fn irq_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // set to_address to fetch pch
    let addr = to_address(0xFF, cpu.int_vec_low + 1);

    read_cycle!(cpu, bus, pinout, addr);
    cpu.pc.pch = cpu.ops.dl;
    // set the interrupt vector back to the default with is the BRK vec
    cpu.int_vec_low = IRQ_BRK_VEC_LOW;
    pinout
}

pub fn irq_c6<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//===================================================
// NMI
//====================================================
pub fn nmi_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // read instruction byte (discarded)
    second_cycle!(cpu, bus, pinout);
    pinout
}

pub fn nmi_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write pch to stack
    write_cycle!(cpu, bus, pinout, to_address(0x01, cpu.sp), cpu.pc.pch);
    // decrement sp
    cpu.sp = cpu.sp.wrapping_sub(1);
    pinout
}

pub fn nmi_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write pcl  to stack
    write_cycle!(cpu, bus, pinout, to_address(0x01, cpu.sp), cpu.pc.pcl);
    // decrement sp
    cpu.sp = cpu.sp.wrapping_sub(1);

    pinout
}

pub fn nmi_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write status reg to stack
    write_cycle!(cpu, bus, pinout, to_address(0x01, cpu.sp), cpu.p.push_without_b());
    // after status write set I flag
    cpu.p.set(StatusRegister::INT_DISABLE, true);

    // decrement sp
    cpu.sp = cpu.sp.wrapping_sub(1);
    pinout
}

pub fn nmi_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // set to_address to fetch pcl
    let addr = to_address(0xFF, cpu.int_vec_low);

    read_cycle!(cpu, bus, pinout, addr);
    cpu.pc.pcl = cpu.ops.dl;
    // set i flag
    cpu.p.set(StatusRegister::INT_DISABLE, true);
    pinout
}

pub fn nmi_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // set to_address to fetch pch
    let addr = to_address(0xFF, cpu.int_vec_low + 1);

    read_cycle!(cpu, bus, pinout, addr);
    cpu.pc.pch = cpu.ops.dl;
    // set the interrupt vector back to the default with is the BRK vec
    cpu.int_vec_low = IRQ_BRK_VEC_LOW;
    pinout
}

pub fn nmi_c6<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//==========================================================
// single byte instructions
//===========================================================
pub fn single_byte_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    pinout
}

pub fn single_byte_c1<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    let irq_detected = is_irq_asserted(cpu, pinout);

    T::execute(cpu);
    
    delayed_int_last_cycle!(cpu, irq_detected, pinout);
    // if no interrupt do first cycle
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// immediate read
//========================================================================
pub fn immediate_read_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.pc.increment();
    pinout
}

pub fn immediate_read_c1<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    T::execute(cpu);
    last_cycle!(cpu, pinout);
    // if no interrupt do first cycle
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// zero page read
//========================================================================
pub fn zeropage_read_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.adl = cpu.ops.dl;
    pinout
}

pub fn zeropage_read_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.adl));

    cpu.pc.increment();
    pinout
}

pub fn zeropage_read_c2<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    T::execute(cpu);
    last_cycle!(cpu, pinout);
    // if no interrupt do first cycle
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// absolute read
//========================================================================
pub fn absolute_read_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.adl = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_read_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.adh = cpu.ops.dl;

    pinout
}

pub fn absolute_read_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));

    cpu.pc.increment();
    pinout
}

pub fn absolute_read_c3<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    T::execute(cpu);
    last_cycle!(cpu, pinout);
    // if no interrupt do first cycle
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// indirect x read
//========================================================================
pub fn indirect_x_read_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn indirect_x_read_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // read discarded - still perform read for "open bus behaivor"
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal));
    pinout
}

pub fn indirect_x_read_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal.wrapping_add(cpu.x)));
    cpu.ops.adl = cpu.ops.dl;
    pinout
}

pub fn indirect_x_read_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal.wrapping_add(cpu.x).wrapping_add(1)));
    cpu.ops.adh = cpu.ops.dl;
    pinout
}

pub fn indirect_x_read_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    pinout
}

pub fn indirect_x_read_c5<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    T::execute(cpu);
    last_cycle!(cpu, pinout);
    // if no interrupt do first cycle
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// absolute x read
//========================================================================
pub fn absolute_x_read_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_x_read_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.bah = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_x_read_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    let adl = cpu.ops.bal.overflowing_add(cpu.x);
    cpu.ops.adl = adl.0;
    cpu.ops.adh = cpu.ops.bah;
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    // if no page boundry crossed skip next cycle
    if adl.1 == false { cpu.ir.increment(); }
    pinout
}

pub fn absolute_x_read_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    cpu.ops.adh = cpu.ops.adh.wrapping_add(1);
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    pinout
}

pub fn absolute_x_read_c4<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    T::execute(cpu);
    last_cycle!(cpu, pinout);
    // if no interrupt do first cycle
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// absolute y read
//========================================================================
pub fn absolute_y_read_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_y_read_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.bah = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_y_read_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    let adl = cpu.ops.bal.overflowing_add(cpu.y);
    cpu.ops.adl = adl.0;
    cpu.ops.adh = cpu.ops.bah;
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    // if no page boundry crossed skip next cycle
    if adl.1 == false { cpu.ir.increment(); }
    pinout
}

pub fn absolute_y_read_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    cpu.ops.adh = cpu.ops.adh.wrapping_add(1);
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    pinout
}

pub fn absolute_y_read_c4<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    T::execute(cpu);
    last_cycle!(cpu, pinout);
    // if no interrupt do first cycle
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// zero page x read
//========================================================================
pub fn zeropage_x_read_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn zeropage_x_read_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal));
    pinout
}

pub fn zeropage_x_read_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal.wrapping_add(cpu.x)));
    pinout
}

pub fn zeropage_x_read_c3<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    T::execute(cpu);
    last_cycle!(cpu, pinout);
    // if no interrupt do first cycle
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// zero page y read
//========================================================================
pub fn zeropage_y_read_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn zeropage_y_read_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal));
    pinout
}

pub fn zeropage_y_read_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal.wrapping_add(cpu.y)));
    pinout
}

pub fn zeropage_y_read_c3<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    T::execute(cpu);
    last_cycle!(cpu, pinout);
    // if no interrupt do first cycle
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// indirect y read
//========================================================================
pub fn indirect_y_read_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.ial = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn indirect_y_read_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.ial));
    cpu.ops.bal = cpu.ops.dl;

    pinout
}

pub fn indirect_y_read_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.ial.wrapping_add(1)));
    cpu.ops.bah = cpu.ops.dl;

    pinout
}

pub fn indirect_y_read_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    let adl = cpu.ops.bal.overflowing_add(cpu.y);
    cpu.ops.adl = adl.0;
    cpu.ops.adh = cpu.ops.bah;
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    // if no page boundry crossed skip next cycle
    if adl.1 == false { cpu.ir.increment(); }

    pinout
}

pub fn indirect_y_read_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    cpu.ops.adh = cpu.ops.adh.wrapping_add(1);
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));

    pinout
}

pub fn indirect_y_read_c5<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    T::execute(cpu);
    last_cycle!(cpu, pinout);
    // if no interrupt do first cycle
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// zero page store
//========================================================================
pub fn zeropage_store_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.adl = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn zeropage_store_c1<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    T::execute(cpu);
    write_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn zeropage_store_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// absolute store
//========================================================================
pub fn absolute_store_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.adl = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_store_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.adh = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_store_c2<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    T::execute(cpu);
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn absolute_store_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// indirect x store
//========================================================================
pub fn indirect_x_store_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn indirect_x_store_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // data discarded
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal));

    pinout
}

pub fn indirect_x_store_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal.wrapping_add(cpu.x)));
    cpu.ops.adl = cpu.ops.dl;

    pinout
}

pub fn indirect_x_store_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal.wrapping_add(cpu.x).wrapping_add(1)));
    cpu.ops.adh = cpu.ops.dl;

    pinout
}

pub fn indirect_x_store_c4<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    T::execute(cpu);
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn indirect_x_store_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// absolute x store
//========================================================================
pub fn absolute_x_store_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_x_store_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.bah = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_x_store_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    let bal = cpu.ops.bal.overflowing_add(cpu.x);
    cpu.ops.adl = bal.0;
    cpu.ops.adh = cpu.ops.bah;
    // data discarded
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    cpu.ops.adh = cpu.ops.bah.wrapping_add(bal.1 as u8);


    pinout
}

pub fn absolute_x_store_c3<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    T::execute(cpu);
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn absolute_x_store_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// absolute y store
//========================================================================
pub fn absolute_y_store_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_y_store_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.bah = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_y_store_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    let bal = cpu.ops.bal.overflowing_add(cpu.y);
    cpu.ops.adl = bal.0;
    cpu.ops.adh = cpu.ops.bah;
    // data discarded
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    cpu.ops.adh = cpu.ops.bah.wrapping_add(bal.1 as u8);

    pinout
}

pub fn absolute_y_store_c3<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    T::execute(cpu);
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn absolute_y_store_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// zero page x store
//========================================================================
pub fn zeropage_x_store_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn zeropage_x_store_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // data discarded
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal));
    pinout
}

pub fn zeropage_x_store_c2<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    T::execute(cpu);
    write_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal.wrapping_add(cpu.x)), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn zeropage_x_store_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// zero page y store
//========================================================================
pub fn zeropage_y_store_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn zeropage_y_store_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // data discarded
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal));
    pinout
}

pub fn zeropage_y_store_c2<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    T::execute(cpu);
    write_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal.wrapping_add(cpu.y)), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn zeropage_y_store_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// indirect y store
//========================================================================
pub fn indirect_y_store_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.ial = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn indirect_y_store_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.ial));
    cpu.ops.bal = cpu.ops.dl;

    pinout
}

pub fn indirect_y_store_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.ial.wrapping_add(1)));
    cpu.ops.bah = cpu.ops.dl;

    pinout
}

pub fn indirect_y_store_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    cpu.ops.adl = cpu.ops.bal.wrapping_add(cpu.y);
    cpu.ops.adh = cpu.ops.bah;
    // data discarded
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));

    pinout
}

pub fn indirect_y_store_c4<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    T::execute(cpu);
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn indirect_y_store_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// zero page modify
//========================================================================
pub fn zeropage_modify_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.adl = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn zeropage_modify_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.adl));
    pinout
}

pub fn zeropage_modify_c2<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write original data back
    write_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.adl), cpu.ops.dl);
    // instruction executed, changing data 
    T::execute(cpu);

    pinout
}

pub fn zeropage_modify_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write modified data
    write_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn zeropage_modify_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// absolute modify
//========================================================================
pub fn absolute_modify_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.adl = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_modify_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.adh = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_modify_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    pinout
}

pub fn absolute_modify_c3<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write original data back
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    // instruction executed, changing data 
    T::execute(cpu);

    pinout
}

pub fn absolute_modify_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write modified data
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn absolute_modify_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// zero page x modify
//========================================================================
pub fn zeropage_x_modify_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn zeropage_x_modify_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // data discarded
    cpu.ops.adl = cpu.ops.bal.wrapping_add(cpu.x);
    read_cycle!(cpu, bus,pinout, to_address(0, cpu.ops.adl));

    pinout
}

pub fn zeropage_x_modify_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.adl));
    pinout
}

pub fn zeropage_x_modify_c3<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write original data back
    write_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.adl), cpu.ops.dl);
    // instruction executed, changing data 
    T::execute(cpu);

    pinout
}

pub fn zeropage_x_modify_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write modified data
    write_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn zeropage_x_modify_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// absolute x modify
//========================================================================
pub fn absolute_x_modify_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_x_modify_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.bah = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn absolute_x_modify_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // data discarded
    let adl =  cpu.ops.bal.overflowing_add(cpu.x);
    cpu.ops.adl = adl.0;
    cpu.ops.adh = cpu.ops.bah + (adl.1 as u8);
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));

    pinout
}

pub fn absolute_x_modify_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    pinout
}

pub fn absolute_x_modify_c4<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write original data back
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    // instruction executed, changing data 
    T::execute(cpu);

    pinout
}

pub fn absolute_x_modify_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write modified data
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);

    pinout
}

pub fn absolute_x_modify_c6<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// php
//========================================================================
pub fn php_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    // data discarded
    pinout
}

pub fn php_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    cpu.ops.dl = cpu.p.push_with_b();
    write_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp), cpu.ops.dl);
    // decrement stack pointer
    cpu.sp = cpu.sp.wrapping_sub(1);
    last_cycle!(cpu, pinout);
    pinout
}

pub fn php_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// pha
//========================================================================
pub fn pha_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    // data discarded
    pinout
}

pub fn pha_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    cpu.ops.dl = cpu.a;
    write_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp), cpu.ops.dl);
    // decrement stack pointer
    cpu.sp = cpu.sp.wrapping_sub(1);
    last_cycle!(cpu, pinout);
    pinout
}

pub fn pha_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// plp
//========================================================================
pub fn plp_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    // data discarded
    pinout
}

pub fn plp_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // data discarded
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));
    pinout
}

pub fn plp_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    cpu.sp = cpu.sp.wrapping_add(1);
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));

    let irq_detected = is_irq_asserted(cpu, pinout);

    cpu.p = StatusRegister::from_bits_truncate(cpu.ops.dl);

    delayed_int_last_cycle!(cpu, irq_detected, pinout);
    pinout
}

pub fn plp_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// pla
//========================================================================
pub fn pla_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    // data discarded
    pinout
}

pub fn pla_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // data discarded
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));
    pinout
}

pub fn pla_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    cpu.sp = cpu.sp.wrapping_add(1);
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));
    cpu.a = cpu.ops.dl;

    if cpu.a == 0 { cpu.p.set(StatusRegister::ZERO, true) } else {cpu.p.set(StatusRegister::ZERO, false) };
    if (cpu.a & 0x80) == 0x80 { cpu.p.set(StatusRegister::NEGATIVE, true) } else { cpu.p.set(StatusRegister::NEGATIVE, false) };

    last_cycle!(cpu, pinout);
    pinout
}

pub fn pla_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// jsr
//========================================================================
pub fn jsr_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.adl = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn jsr_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // read from sp - data discarded
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));

    pinout
}

pub fn jsr_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    write_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp), cpu.pc.pch);
    cpu.sp = cpu.sp.wrapping_sub(1);
    
    pinout
}

pub fn jsr_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    write_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp), cpu.pc.pcl);
    cpu.sp = cpu.sp.wrapping_sub(1);
    
    pinout
}

pub fn jsr_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.adh = cpu.ops.dl;
    cpu.pc.pcl = cpu.ops.adl;
    cpu.pc.pch = cpu.ops.adh;

    last_cycle!(cpu, pinout);
    pinout
}

pub fn jsr_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// rti
//========================================================================
pub fn rti_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    // data discarded

    pinout
}

pub fn rti_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // read from sp - data discarded
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));
    cpu.sp = cpu.sp.wrapping_add(1);

    pinout
}

pub fn rti_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));
    cpu.p = StatusRegister::from_bits_truncate(cpu.ops.dl);
    cpu.sp = cpu.sp.wrapping_add(1);
    
    pinout
}

pub fn rti_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));
    cpu.pc.pcl = cpu.ops.dl;
    cpu.sp = cpu.sp.wrapping_add(1);
    
    pinout
}

pub fn rti_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));
    cpu.pc.pch = cpu.ops.dl;

    last_cycle!(cpu, pinout);
    pinout
}

pub fn rti_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// jump absolute
//========================================================================
pub fn jmp_absolute_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.adl = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn jmp_absolute_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout,  u16::from(cpu.pc));
    cpu.ops.adh = cpu.ops.dl;

    cpu.pc.pcl = cpu.ops.adl;
    cpu.pc.pch = cpu.ops.adh;

    last_cycle!(cpu, pinout);
    pinout
}

pub fn jmp_absolute_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// jump indirect
//========================================================================
pub fn jmp_indirect_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.ial = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn jmp_indirect_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.iah = cpu.ops.dl;

    pinout
}

pub fn jmp_indirect_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.iah, cpu.ops.ial));
    cpu.ops.adl = cpu.ops.dl;

    pinout
}

pub fn jmp_indirect_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.iah, cpu.ops.ial.wrapping_add(1)));
    cpu.ops.adh = cpu.ops.dl;

    cpu.pc.pcl =cpu.ops.adl;
    cpu.pc.pch = cpu.ops.adh;

    last_cycle!(cpu, pinout);
    pinout
}

pub fn jmp_indirect_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// rts
//========================================================================
pub fn rts_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    // data discarded
    pinout
}

pub fn rts_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // read from sp - data discarded
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));
    cpu.sp = cpu.sp.wrapping_add(1);
    pinout
}

pub fn rts_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));
    cpu.pc.pcl = cpu.ops.dl;
    cpu.sp = cpu.sp.wrapping_add(1);
    pinout
}

pub fn rts_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0x1, cpu.sp));
    cpu.pc.pch = cpu.ops.dl;
    pinout
}

pub fn rts_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // data discarded
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.pc.increment();
    last_cycle!(cpu, pinout);
    pinout
}

pub fn rts_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//=======================================================================
// branch
// the mcs65000 hardware manual has a typo in the branch instruction cycle timing
// http://forum.6502.org/viewtopic.php?t=1634
//========================================================================
pub fn branch_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.offset = cpu.ops.dl;
    cpu.pc.increment();
    // branch always checks for interrupts on this cycle
    last_cycle!(cpu, pinout);
    pinout
}

pub fn branch_c1<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    T::execute(cpu);
    // fetch next opcode 
    if cpu.ops.branch_taken == false {
        // first cycle of next instruction
        first_cycle!(cpu, bus, pinout);
    }
    // add offset to pcl
    else {
        // offset is signed check if negative
        if cpu.ops.offset > 0x7F {
            cpu.ops.offset_neg = true;
            cpu.ops.offset = !cpu.ops.offset + 1;
            let pcl_temp = cpu.pc.pcl.overflowing_sub(cpu.ops.offset);
            if pcl_temp.1 == true { cpu.ops.offset_carry = true; }
            cpu.pc.pcl = pcl_temp.0;
        }
        else {
            let pcl_temp = cpu.pc.pcl.overflowing_add(cpu.ops.offset);
            if pcl_temp.1 == true { cpu.ops.offset_carry = true; }
            cpu.pc.pcl = pcl_temp.0;
        }

        read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    }
    pinout
}

pub fn branch_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // check if page boundry crossed
    if cpu.ops.offset_carry == true {
        if cpu.ops.offset_neg == true { cpu.pc.pch = cpu.pc.pch.wrapping_sub(1); }
        else { cpu.pc.pch = cpu.pc.pch.wrapping_add(1); }
        read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
        last_cycle!(cpu, pinout);
    }
    else {
        last_cycle!(cpu, pinout);
        // if no interrupt do first cycle
        first_cycle!(cpu, bus, pinout);
    }
    
    pinout
}

pub fn branch_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//================================================================
// undocumented indirect x
//================================================================
pub fn undoc_indirect_x_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn undoc_indirect_x_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    // read discarded - still perform read for "open bus behaivor"
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal));
    pinout
}

pub fn undoc_indirect_x_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal + cpu.x));
    cpu.ops.adl = cpu.ops.dl;
    pinout
}

pub fn undoc_indirect_x_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.bal + cpu.x + 1));
    cpu.ops.adh = cpu.ops.dl;
    pinout
}

pub fn undoc_indirect_x_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    pinout
}

pub fn undoc_indirect_x_c5<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write original data back
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    T::execute(cpu);
    pinout
}

pub fn undoc_indirect_x_c6<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write modified data back
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);
    pinout
}

pub fn undoc_indirect_x_c7<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//================================================================
// undocumented indirect y
//================================================================
pub fn undoc_indirect_y_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.ial = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn undoc_indirect_y_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.ial));
    cpu.ops.bal = cpu.ops.dl;
    pinout
}

pub fn undoc_indirect_y_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(0, cpu.ops.ial.wrapping_add(1)));
    cpu.ops.bah = cpu.ops.dl;
    pinout
}

pub fn undoc_indirect_y_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    let adl = cpu.ops.bal.overflowing_add(cpu.y);
    cpu.ops.adl = adl.0;
    cpu.ops.adh = cpu.ops.bah;
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    // the udocumented version does not seem to have a page boundry skip
    pinout
}

pub fn undoc_indirect_y_c4<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    let adl = cpu.ops.bal.overflowing_add(cpu.y);
    cpu.ops.adl = adl.0;
    cpu.ops.adh = cpu.ops.adh.wrapping_add(adl.1 as u8);
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));

    pinout
}

pub fn undoc_indirect_y_c5<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write original data back
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    T::execute(cpu);
    pinout
}

pub fn undoc_indirect_y_c6<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write modified data back
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);
    pinout
}

pub fn undoc_indirect_y_c7<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}

//================================================================
// undocumented absolute y
//================================================================
pub fn undoc_absolute_y_c0<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    second_cycle!(cpu, bus, pinout);
    cpu.ops.bal = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn undoc_absolute_y_c1<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, u16::from(cpu.pc));
    cpu.ops.bah = cpu.ops.dl;

    cpu.pc.increment();
    pinout
}

pub fn undoc_absolute_y_c2<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    let adl = cpu.ops.bal.overflowing_add(cpu.y);
    cpu.ops.adl = adl.0;
    cpu.ops.adh = cpu.ops.bah.wrapping_add(adl.1 as u8);
    // data discarded
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));

    pinout
}

pub fn undoc_absolute_y_c3<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    read_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl));
    pinout
}

pub fn undoc_absolute_y_c4<B: Bus, T: Instruction>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write original data back
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    T::execute(cpu);
    pinout
}

pub fn undoc_absolute_y_c5<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    // write modified data back
    write_cycle!(cpu, bus, pinout, to_address(cpu.ops.adh, cpu.ops.adl), cpu.ops.dl);
    last_cycle!(cpu, pinout);
    pinout
}

pub fn undoc_absolute_y_c6<B: Bus>(cpu: &mut Context, bus: &mut B, mut pinout: Pinout) -> Pinout {
    if pinout.ctrl.contains(Ctrl::RDY) == false { return pinout; }
    first_cycle!(cpu, bus, pinout);
    pinout
}