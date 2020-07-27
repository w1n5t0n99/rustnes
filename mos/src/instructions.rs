use super::core::*;

#[inline]
fn set_zero(x: u8) -> bool {
    if x == 0 { true } else { false }
}

#[inline]
fn set_negative(x: u8) -> bool {
    if (x & 0x80) == 0x80 { true } else { false }
}

pub trait Instruction {
    fn execute(cpu: &mut Context);
}

//=====================================================
// official opcodes
//====================================================== 

pub struct AdcNoDec {}
impl Instruction for AdcNoDec {
    fn execute(cpu: &mut Context) {
        let sum = (cpu.a as u16) + (cpu.ops.dl as u16) + (cpu.p.carry as u16); 
        cpu.p.carry = if sum > 255 { true } else {false };

        let result = sum as u8;
        cpu.a = result;
       // cpu.p.overflow =  if (signed_sum < -128) || (signed_sum > 127) { true } else { false };
        cpu.p.overflow =  if ((cpu.ops.dl ^ result) & (cpu.a & result) & 0x80) == 0x80 { true } else { false };
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);

    }
}

pub struct Adc {}
impl Instruction for Adc {
    fn execute(cpu: &mut Context) {
        if cpu.p.decimal == false {
            let sum = (cpu.a as u16) + (cpu.ops.dl as u16) + (cpu.p.carry as u16); 
            cpu.p.carry = if sum > 255 { true } else {false };

            let result = sum as u8;
            cpu.a = result;
            cpu.p.overflow =  if ((cpu.ops.dl ^ result) & (cpu.a & result) & 0x80) == 0x80 { true } else { false };
            cpu.p.zero = set_zero(cpu.a);
            cpu.p.negative = set_negative(cpu.a);
        }
        else {
            // decimal mode (MAME implementation)
            let c: u8 = if cpu.p.carry == true {1} else {0};
            cpu.p.carry = false;
            cpu.p.overflow = false;
            cpu.p.negative = false;
            cpu.p.zero = false;

            let mut al = (cpu.a & 0x0F) + (cpu.ops.dl & 0x0F) + c;
            if al > 9 { al += 6; }

            let mut ah = (cpu.a >> 4) + (cpu.ops.dl >> 4) + ((al > 0x0F) as u8);

            if (cpu.a.wrapping_add(cpu.ops.dl)).wrapping_add(c) == 0 {
                cpu.p.zero = true;
            }
            else if (ah & 0x8) > 0 {
                cpu.p.negative = true;
            }

            if (!(cpu.a ^ cpu.ops.dl) & (cpu.a ^ (ah << 4)) & 0x80) > 0 {
                cpu.p.overflow = true;
            }

            if ah > 9 { ah += 6; }
            if ah > 15 { cpu.p.carry = true; }

            cpu.a = (ah << 4) | (al & 0x0F);
        }
    }
}

pub struct And {}
impl Instruction for And {
    fn execute(cpu: &mut Context) {
        let a = cpu.a & cpu.ops.dl;
        cpu.a = a;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Asl {}
impl Instruction for Asl {
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x80) > 0 { true } else { false };
        cpu.ops.dl = cpu.ops.dl.wrapping_mul(2);

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct AslAccum {}
impl Instruction for AslAccum {
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.a & 0x80) > 0 { true } else { false };
        cpu.a = cpu.a.wrapping_mul(2);

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Bcc {}
impl Instruction for Bcc {
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.carry == false { true } else { false };
    }
}

pub struct Bcs {}
impl Instruction for Bcs {
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.carry == true { true } else { false };
    }
}

pub struct Beq {}
impl Instruction for Beq {
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.zero == true { true } else { false };
    }
}

pub struct Bit {}
impl Instruction for Bit {
    fn execute(cpu: &mut Context) {
        let x = cpu.a & cpu.ops.dl;
        cpu.p.negative = if (cpu.ops.dl & 0x80) == 0x80 { true } else { false };
        cpu.p.overflow = if (cpu.ops.dl & 0x40) == 0x40 { true } else { false };
        cpu.p.zero = if x == 0 { true } else { false };
    }
}

pub struct Bmi {}
impl Instruction for Bmi {
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.negative == true { true } else { false };
    }
}

pub struct Bne {}
impl Instruction for Bne {
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.zero == false { true } else { false };
    }
}

pub struct Bpl {}
impl Instruction for Bpl {
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.negative == false { true } else { false };
    }
}

pub struct Bvc {}
impl Instruction for Bvc {
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.overflow == false { true } else { false };
    }
}

pub struct Bvs {}
impl Instruction for Bvs {
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.overflow == true { true } else { false };
    }
}

pub struct Clc {}
impl Instruction for Clc {
    fn execute(cpu: &mut Context) {
        cpu.p.carry = false;
    }
}

pub struct Cld {}
impl Instruction for Cld {
    fn execute(cpu: &mut Context) {
        cpu.p.decimal = false;
    }
}

pub struct Cli {}
impl Instruction for Cli {
    fn execute(cpu: &mut Context) {
        cpu.p.interrupt_disable = false;
    }
}

pub struct Clv {}
impl Instruction for Clv {
    fn execute(cpu: &mut Context) {
        cpu.p.overflow = false;
    }
}

pub struct Cmp {}
impl Instruction for Cmp {
    fn execute(cpu: &mut Context) {
        cpu.p.carry = if cpu.a >= cpu.ops.dl { true } else { false };
        cpu.p.zero = if cpu.a == cpu.ops.dl { true } else {false };
        cpu.p.negative = set_negative(cpu.a.wrapping_sub(cpu.ops.dl));
    }
}

pub struct Cpx {}
impl Instruction for Cpx {
    fn execute(cpu: &mut Context) {
        cpu.p.carry = if cpu.x >= cpu.ops.dl { true } else { false };
        cpu.p.zero = if cpu.x == cpu.ops.dl { true } else {false };
        cpu.p.negative = set_negative(cpu.x.wrapping_sub(cpu.ops.dl));
    }
}

pub struct Cpy {}
impl Instruction for Cpy {
    fn execute(cpu: &mut Context) {
        cpu.p.carry = if cpu.y >= cpu.ops.dl { true } else { false };
        cpu.p.zero = if cpu.y == cpu.ops.dl { true } else {false };
        cpu.p.negative = set_negative(cpu.y.wrapping_sub(cpu.ops.dl));
    }
}

pub struct Dec {}
impl Instruction for Dec {
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.ops.dl.wrapping_sub(1);
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct Dex {}
impl Instruction for Dex {
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.x.wrapping_sub(1);
        cpu.p.zero = set_zero(cpu.x);
        cpu.p.negative = set_negative(cpu.x);
    }
}

pub struct Dey {}
impl Instruction for Dey {
    fn execute(cpu: &mut Context) {
        cpu.y = cpu.y.wrapping_sub(1);
        cpu.p.zero = set_zero(cpu.y);
        cpu.p.negative = set_negative(cpu.y);
    }
}

pub struct Eor {}
impl Instruction for Eor {
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.a ^ cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Inc {}
impl Instruction for Inc {
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.ops.dl.wrapping_add(1);
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct Inx {}
impl Instruction for Inx {
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.x.wrapping_add(1);
        cpu.p.zero = set_zero(cpu.x);
        cpu.p.negative = set_negative(cpu.x);
    }
}

pub struct Iny {}
impl Instruction for Iny {
    fn execute(cpu: &mut Context) {
        cpu.y = cpu.y.wrapping_add(1);
        cpu.p.zero = set_zero(cpu.y);
        cpu.p.negative = set_negative(cpu.y);
    }
}

pub struct Lda {}
impl Instruction for Lda {
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Ldx {}
impl Instruction for Ldx {
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.x);
        cpu.p.negative = set_negative(cpu.x);
    }
}

pub struct Ldy {}
impl Instruction for Ldy {
    fn execute(cpu: &mut Context) {
        cpu.y = cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.y);
        cpu.p.negative = set_negative(cpu.y);
    }
}

pub struct Lsr {}
impl Instruction for Lsr {
    fn execute(cpu: &mut Context) {
        let old_carry = if (cpu.ops.dl & 0x01) > 0 { true } else { false };

        cpu.ops.dl = cpu.ops.dl.wrapping_div(2);
        // clear bit 7
        cpu.ops.dl &= 0b01111111;

        cpu.p.carry = old_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct LsrAccum {}
impl Instruction for LsrAccum {
    fn execute(cpu: &mut Context) {
        let old_carry = if (cpu.a & 0x01) > 0 { true } else { false };

        cpu.a = cpu.a.wrapping_div(2);
        // clear bit 7
        cpu.a &= 0b01111111;

        cpu.p.carry = old_carry;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Nop {}
impl Instruction for Nop {
    fn execute(_cpu: &mut Context) {
        // causes no changes to processor
    }
}

pub struct Ora {}
impl Instruction for Ora {
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.a | cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Rol {}
impl Instruction for Rol {
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x80) > 0 { true } else { false };
        cpu.ops.dl = cpu.ops.dl.wrapping_mul(2);
        cpu.ops.dl |= cpu.p.carry as u8;

        cpu.p.carry = new_carry;
        cpu.p.negative = set_negative(cpu.ops.dl);
        cpu.p.zero = set_zero(cpu.ops.dl);
    }
}

pub struct RolAccum {}
impl Instruction for RolAccum {
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.a & 0x80) > 0 { true } else { false };
        cpu.a = cpu.a.wrapping_mul(2);
        cpu.a |= cpu.p.carry as u8;

        cpu.p.carry = new_carry;
        cpu.p.negative = set_negative(cpu.a);
        cpu.p.zero = set_zero(cpu.a);
    }
}

pub struct Ror {}
impl Instruction for Ror {
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x01) > 0 { true } else { false };

        cpu.ops.dl = cpu.ops.dl.wrapping_div(2);
        cpu.ops.dl |= (cpu.p.carry as u8) << 7;

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct RorAccum {}
impl Instruction for RorAccum {
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.a & 0x01) > 0 { true } else { false };

        cpu.a = cpu.a.wrapping_div(2);
        cpu.a |= (cpu.p.carry as u8) << 7;

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct SbcNoDec {}
impl Instruction for SbcNoDec {
    fn execute(cpu: &mut Context) {
        let dl = cpu.ops.dl ^ 0xFF;
        //let sum = cpu.a.wrapping_add(dl).wrapping_add(cpu.p.carry as u8);
        let sum = (cpu.a as u16) + (dl as u16) + cpu.p.carry as u16;
        let result = (sum & 0xFF) as u8;
        cpu.p.carry = if sum > 255 { true } else { false };
        cpu.p.overflow = if ((cpu.a ^ result) & (dl ^ result) & 0x80) != 0 { true } else { false };
        cpu.a = result;
        cpu.p.negative = set_negative(cpu.a);
        cpu.p.zero = set_zero(cpu.a);  
    }
}

pub struct Sbc {}
impl Instruction for Sbc {
    fn execute(cpu: &mut Context) {
        if cpu.p.decimal == false {
            let dl = cpu.ops.dl ^ 0xFF;
            //let sum = cpu.a.wrapping_add(dl).wrapping_add(cpu.p.carry as u8);
            let sum = (cpu.a as u16) + (dl as u16) + cpu.p.carry as u16;
            let result = (sum & 0xFF) as u8;
            cpu.p.carry = if sum > 255 { true } else { false };
            cpu.p.overflow = if ((cpu.a ^ result) & (dl ^ result) & 0x80) != 0 { true } else { false };
            cpu.a = result;
            cpu.p.negative = set_negative(cpu.a);
            cpu.p.zero = set_zero(cpu.a);
        }
        else {
            // decimal mode (MAME implementation)
            let c: u8 = if cpu.p.carry == true {1} else {0};
            cpu.p.carry = false;
            cpu.p.overflow = false;
            cpu.p.negative = false;
            cpu.p.zero = false;

            let diff: u16 = ((cpu.a as u16).wrapping_sub(cpu.ops.dl as u16)).wrapping_sub(c as u16);
            let mut al = ((cpu.a & 0x0F).wrapping_sub(cpu.ops.dl & 0x0F)).wrapping_sub(c);

            if  (al as i8) < 0 {
                al -= 6;
            }

            let mut ah = ((cpu.a >> 4).wrapping_sub(cpu.ops.dl >> 4)).wrapping_sub(((al as i8) < 0) as u8);

            if (diff as u8) == 0 {
                cpu.p.zero = true;
            }
            else if (diff & 0x80) > 0 {
                cpu.p.negative = true;
            }

            if ((cpu.a as u16 ^ cpu.ops.dl as u16) & (cpu.a as u16 ^ diff) & 0x80) > 0 {
                cpu.p.overflow = true;
            }

            if (!(diff & 0xFF00)) > 0 { cpu.p.carry = true; }
            if (ah & 0x80) > 0 { ah -= 6; }

            cpu.a = (ah << 4) | (al & 0x0F);
        }    
    }
}

pub struct Sec {}
impl Instruction for Sec {
    fn execute(cpu: &mut Context) {
        cpu.p.carry = true;
    }
}

pub struct Sed {}
impl Instruction for Sed {
    fn execute(cpu: &mut Context) {
        cpu.p.decimal = true;
    }
}

pub struct Sei {}
impl Instruction for Sei {
    fn execute(cpu: &mut Context) {
        cpu.p.interrupt_disable = true;
    }
}

pub struct Sta {}
impl Instruction for Sta {
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.a;
    }
}

pub struct Stx {}
impl Instruction for Stx {
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.x;
    }
}

pub struct Sty {}
impl Instruction for Sty {
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.y;
    }
}

pub struct Tax {}
impl Instruction for Tax {
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.a;

        cpu.p.zero = set_zero(cpu.x);
        cpu.p.negative = set_negative(cpu.x);
    }
}

pub struct Tay {}
impl Instruction for Tay {
    fn execute(cpu: &mut Context) {
        cpu.y = cpu.a;

        cpu.p.zero = set_zero(cpu.y);
        cpu.p.negative = set_negative(cpu.y);
    }
}

pub struct Tsx {}
impl Instruction for Tsx {
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.sp;

        cpu.p.zero = set_zero(cpu.x);
        cpu.p.negative = set_negative(cpu.x);
    }
}

pub struct Txa {}
impl Instruction for Txa {
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.x;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Txs {}
impl Instruction for Txs {
    fn execute(cpu: &mut Context) {
        cpu.sp = cpu.x;
    }
}

pub struct Tya {}
impl Instruction for Tya {
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.y;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

//====================================================
// undocumented opcodes
//====================================================
pub struct Aac {}
impl Instruction for Aac {
    fn execute(cpu: &mut Context) {
        cpu.a &= cpu.ops.dl;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
        cpu.p.carry = if cpu.p.negative == true { true } else { false };
    }
}

pub struct Aax {}
impl Instruction for Aax {
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.a & cpu.x;
        
        //cpu.p.zero = set_zero(cpu.ops.dl);
        //cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct Arr {}
impl Instruction for Arr {
    fn execute(cpu: &mut Context) {
        cpu.a &= cpu.ops.dl;
        // rotate right
        cpu.a = cpu.a.wrapping_div(2);
        cpu.a |= (cpu.p.carry as u8) << 7;

        let mask = cpu.a & 0b01100000;
        match mask {
            0b01100000 => {
                cpu.p.carry = true;
                cpu.p.overflow = false;
            }
            0b00000000 => {
                cpu.p.carry = false;
                cpu.p.overflow = false;
            }
            0b00100000 => {
                cpu.p.carry = false;
                cpu.p.overflow = true;
            }
            0b01000000 => {
                cpu.p.carry = true;
                cpu.p.overflow = true;
            }
            _ => panic!("Arr instruction"),
        }
    }
}

pub struct Asr {}
impl Instruction for Asr {
    fn execute(cpu: &mut Context) {
        
        let old_carry = if (cpu.a & 0x01) > 0 { true } else { false };
        cpu.a &= cpu.ops.dl;
        // rotate right
        cpu.a = cpu.a.wrapping_div(2);

        cpu.p.carry = old_carry;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Atx {}
impl Instruction for Atx {
    fn execute(cpu: &mut Context) {
        cpu.a &= cpu.ops.dl;
        cpu.x = cpu.a;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Axa {}
impl Instruction for Axa {
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.a & cpu.x & (cpu.ops.adh.wrapping_add(1));

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Axs {}
impl Instruction for Axs {
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.a & cpu.x;
        let (x, c) = cpu.x.overflowing_sub(cpu.ops.dl);
        cpu.x = x;
        cpu.p.carry = c;
    }
}

pub struct Dcp {}
impl Instruction for Dcp {
    fn execute(cpu: &mut Context) {
        let (x, _c) = cpu.ops.dl.overflowing_sub(1);
        cpu.ops.dl = x;
        let result = cpu.a.wrapping_sub(cpu.ops.dl);
        cpu.p.carry = if cpu.a >= cpu.ops.dl { true } else { false };
        cpu.p.zero = if cpu.a == cpu.ops.dl { true } else { false }; 
        cpu.p.negative = set_negative(result);
    }
}

pub struct Isc {}
impl Instruction for Isc {
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.ops.dl.wrapping_add(1);
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
        
        let dl = cpu.ops.dl ^ 0xFF;
        let sum = (cpu.a as u16) + (dl as u16) + cpu.p.carry as u16;
        let result = (sum & 0xFF) as u8;
        cpu.p.carry = if sum > 255 { true } else { false };
        cpu.p.overflow = if ((cpu.a ^ result) & (dl ^ result) & 0x80) != 0 { true } else { false };
        cpu.a = result;
        cpu.p.negative = set_negative(cpu.a);
        cpu.p.zero = set_zero(cpu.a);
    }
}

pub struct Kil {}
impl Instruction for Kil {
    fn execute(cpu: &mut Context) {
        let mut addr = u16::from(cpu.pc);
       // halt pc, lock up cpu
        addr -= 1;
        cpu.pc = ProgramCounter::from(addr);
    }
}

pub struct Lar {}
impl Instruction for Lar {
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.sp & cpu.ops.dl;
        cpu.x = cpu.a;
        cpu.sp = cpu.a;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Lax {}
impl Instruction for Lax {
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.ops.dl;
        cpu.x = cpu.ops.dl;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Rla {}
impl Instruction for Rla {
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x80) > 0 { true } else { false };
        cpu.ops.dl = cpu.ops.dl.wrapping_mul(2);
        cpu.ops.dl |= cpu.p.carry as u8;

        cpu.p.carry = new_carry;
        cpu.p.negative = set_negative(cpu.ops.dl);
        cpu.p.zero = set_zero(cpu.ops.dl);

        cpu.a = cpu.a & cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Rra {}
impl Instruction for Rra {
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x01) > 0 { true } else { false };

        cpu.ops.dl = cpu.ops.dl.wrapping_div(2);
        cpu.ops.dl |= (cpu.p.carry as u8) << 7;

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);

        let sum = (cpu.a as u16) + (cpu.ops.dl as u16) + (cpu.p.carry as u16); 
        cpu.p.carry = if sum > 255 { true } else {false };

        let result = sum as u8;
        cpu.a = result;
       // cpu.p.overflow =  if (signed_sum < -128) || (signed_sum > 127) { true } else { false };
        cpu.p.overflow =  if ((cpu.ops.dl ^ result) & (cpu.a & result) & 0x80) == 0x80 { true } else { false };
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);

    }
}

pub struct Slo {}
impl Instruction for Slo {
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x80) > 0 { true } else { false };
        cpu.ops.dl = cpu.ops.dl.wrapping_mul(2);

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);

        cpu.a = cpu.a | cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Sre {}
impl Instruction for Sre {
    fn execute(cpu: &mut Context) {
        let old_carry = if (cpu.ops.dl & 0x01) > 0 { true } else { false };

        cpu.ops.dl = cpu.ops.dl.wrapping_div(2);
        // clear bit 7
        cpu.ops.dl &= 0b01111111;

        cpu.p.carry = old_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);

        cpu.a = cpu.a ^ cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);    
    }
}

pub struct Sxa {}
impl Instruction for Sxa {
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.x & (cpu.ops.adh + 1);  
    }
}

pub struct Sya {}
impl Instruction for Sya {
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.y & (cpu.ops.adh + 1);  
    }
}

pub struct Xaa {}
impl Instruction for Xaa {
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.x;
        // TODO: A is anded with some unkown immediate value
        cpu.a &= cpu.ops.dl;
    }
}

pub struct Xas {}
impl Instruction for Xas {
    fn execute(cpu: &mut Context) {
        cpu.sp = cpu.x & cpu.a;
        cpu.ops.dl = cpu.sp & (cpu.ops.adh + 1);
    }
}
