use super::core::*;
use super::operations::*;
use super::instructions::*;
use super::{Ctrl, Pinout};
use super::bus::Bus;
use std::fmt;

pub struct Rp2a03 {
    cpu: Context,
}

impl Rp2a03 {
    pub fn from_power_on() -> (Rp2a03, Pinout) {
        let mut cpu_context = Context::new();
        cpu_context.ir.opcode = 0x00;
        cpu_context.ir.tm = 0x10;

        let cpu = Rp2a03 { cpu: cpu_context };
        let cpu_pinout = Pinout::new();
        
        (cpu, cpu_pinout)
    }

    pub fn tick<B: Bus>(&mut self, bus: &mut B, mut pinout: Pinout) -> Pinout {
		//default RW pin to 1
        pinout.ctrl.set(Ctrl::RW, true);
        
        if pinout.ctrl.contains(Ctrl::NMI) == false {
            self.cpu.nmi_detected = true;
        }
		
        match u16::from(self.cpu.ir) {
            // reset
            0x0010 =>  pinout = rst_c0(&mut self.cpu, bus, pinout),
            0x0011 =>  pinout = rst_c1(&mut self.cpu, bus, pinout),
            0x0012 =>  pinout = rst_c2(&mut self.cpu, bus, pinout),
            0x0013 =>  pinout = rst_c3(&mut self.cpu, bus, pinout),
            0x0014 =>  pinout = rst_c4(&mut self.cpu, bus, pinout),
            0x0015 =>  pinout = rst_c5(&mut self.cpu, bus, pinout),
            0x0016 =>  pinout = rst_c6(&mut self.cpu, bus, pinout),
            0x0017 =>  pinout = rst_c7(&mut self.cpu, bus, pinout),
            0x0018 =>  pinout = rst_c8(&mut self.cpu, bus, pinout),
            // brk
            0x0000 => pinout = brk_c0( &mut self.cpu, bus, pinout),
            0x0001 => pinout = brk_c1( &mut self.cpu, bus, pinout),
            0x0002 => pinout = brk_c2( &mut self.cpu, bus, pinout),
            0x0003 => pinout = brk_c3( &mut self.cpu, bus, pinout),
            0x0004 => pinout = brk_c4( &mut self.cpu, bus, pinout),
            0x0005 => pinout = brk_c5( &mut self.cpu, bus, pinout),
            0x0006 => pinout = brk_c6( &mut self.cpu, bus, pinout),
            // jmp absolute
            0x4C00 =>  pinout = jmp_absolute_c0(&mut self.cpu, bus, pinout),
            0x4C01 =>  pinout = jmp_absolute_c1(&mut self.cpu, bus, pinout),
            0x4C02 =>  pinout = jmp_absolute_c2(&mut self.cpu, bus, pinout),
            // jmp indirect
            0x6C00 =>  pinout = jmp_indirect_c0(&mut self.cpu, bus, pinout),
            0x6C01 =>  pinout = jmp_indirect_c1(&mut self.cpu, bus, pinout),
            0x6C02 =>  pinout = jmp_indirect_c2(&mut self.cpu, bus, pinout),
            0x6C03 =>  pinout = jmp_indirect_c3(&mut self.cpu, bus, pinout),
            0x6C04 =>  pinout = jmp_indirect_c4(&mut self.cpu, bus, pinout),
            // rts
            0x6000 =>  pinout = rts_c0(&mut self.cpu, bus, pinout),
            0x6001 =>  pinout = rts_c1(&mut self.cpu, bus, pinout),
            0x6002 =>  pinout = rts_c2(&mut self.cpu, bus, pinout),
            0x6003 =>  pinout = rts_c3(&mut self.cpu, bus, pinout),
            0x6004 =>  pinout = rts_c4(&mut self.cpu, bus, pinout),
            0x6005 =>  pinout = rts_c5(&mut self.cpu, bus, pinout),
            // rti
            0x4000 =>  pinout = rti_c0(&mut self.cpu, bus, pinout),
            0x4001 =>  pinout = rti_c1(&mut self.cpu, bus, pinout),
            0x4002 =>  pinout = rti_c2(&mut self.cpu, bus, pinout),
            0x4003 =>  pinout = rti_c3(&mut self.cpu, bus, pinout),
            0x4004 =>  pinout = rti_c4(&mut self.cpu, bus, pinout),
            0x4005 =>  pinout = rti_c5(&mut self.cpu, bus, pinout),
            // jsr
            0x2000 =>  pinout = jsr_c0(&mut self.cpu, bus, pinout),
            0x2001 =>  pinout = jsr_c1(&mut self.cpu, bus, pinout),
            0x2002 => pinout = jsr_c2( &mut self.cpu, bus, pinout),
            0x2003 => pinout = jsr_c3( &mut self.cpu, bus, pinout),
            0x2004 =>  pinout = jsr_c4(&mut self.cpu, bus, pinout),
            0x2005 =>  pinout = jsr_c5(&mut self.cpu, bus, pinout),
            // php
            0x0800 =>  pinout = php_c0(&mut self.cpu, bus, pinout),
            0x0801 => pinout = php_c1( &mut self.cpu, bus, pinout),
            0x0802 =>  pinout = php_c2(&mut self.cpu, bus, pinout),
            // pha
            0x4800 =>  pinout = pha_c0(&mut self.cpu, bus, pinout),
            0x4801 => pinout = pha_c1( &mut self.cpu, bus, pinout),
            0x4802 =>  pinout = pha_c2(&mut self.cpu, bus, pinout),
            // plp
            0x2800 =>  pinout = plp_c0(&mut self.cpu, bus, pinout),
            0x2801 =>  pinout = plp_c1(&mut self.cpu, bus, pinout),
            0x2802 =>  pinout = plp_c2(&mut self.cpu, bus, pinout),
            0x2803 =>  pinout = plp_c3(&mut self.cpu, bus, pinout),
            // pla
            0x6800 =>  pinout = pla_c0(&mut self.cpu, bus, pinout),
            0x6801 =>  pinout = pla_c1(&mut self.cpu, bus, pinout),
            0x6802 =>  pinout = pla_c2(&mut self.cpu, bus, pinout),
            0x6803 =>  pinout = pla_c3(&mut self.cpu, bus, pinout),
            // AdcNoDec immediate
            0x6900 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x6901 =>  pinout = immediate_read_c1::<B, AdcNoDec>(&mut self.cpu, bus, pinout),
            // AdcNoDec zero page read
            0x6500 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0x6501 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0x6502 =>  pinout = zeropage_read_c2::<B, AdcNoDec>(&mut self.cpu, bus, pinout),
            // AdcNoDec zero page x read
            0x7500 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0x7501 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0x7502 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0x7503 =>  pinout = zeropage_x_read_c3::<B, AdcNoDec>(&mut self.cpu, bus, pinout),
            // AdcNoDec absolute read
            0x6D00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0x6D01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0x6D02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0x6D03 =>  pinout = absolute_read_c3::<B, AdcNoDec>(&mut self.cpu, bus, pinout),
            // AdcNoDec absolute x read
            0x7D00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0x7D01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0x7D02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0x7D03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0x7D04 =>  pinout = absolute_x_read_c4::<B, AdcNoDec>(&mut self.cpu, bus, pinout),
            // AdcNoDec absolute y read
            0x7900 =>  pinout = absolute_y_read_c0(&mut self.cpu, bus, pinout),
            0x7901 =>  pinout = absolute_y_read_c1(&mut self.cpu, bus, pinout),
            0x7902 =>  pinout = absolute_y_read_c2(&mut self.cpu, bus, pinout),
            0x7903 =>  pinout = absolute_y_read_c3(&mut self.cpu, bus, pinout),
            0x7904 =>  pinout = absolute_y_read_c4::<B, AdcNoDec>(&mut self.cpu, bus, pinout),
            // AdcNoDec indirect x read
            0x6100 =>  pinout = indirect_x_read_c0(&mut self.cpu, bus, pinout),
            0x6101 =>  pinout = indirect_x_read_c1(&mut self.cpu, bus, pinout),
            0x6102 =>  pinout = indirect_x_read_c2(&mut self.cpu, bus, pinout),
            0x6103 =>  pinout = indirect_x_read_c3(&mut self.cpu, bus, pinout),
            0x6104 =>  pinout = indirect_x_read_c4(&mut self.cpu, bus, pinout),
            0x6105 =>  pinout = indirect_x_read_c5::<B, AdcNoDec>(&mut self.cpu, bus, pinout),
            // AdcNoDec indirect y read
            0x7100 =>  pinout = indirect_y_read_c0(&mut self.cpu, bus, pinout),
            0x7101 =>  pinout = indirect_y_read_c1(&mut self.cpu, bus, pinout),
            0x7102 =>  pinout = indirect_y_read_c2(&mut self.cpu, bus, pinout),
            0x7103 =>  pinout = indirect_y_read_c3(&mut self.cpu, bus, pinout),
            0x7104 =>  pinout = indirect_y_read_c4(&mut self.cpu, bus, pinout),
            0x7105 =>  pinout = indirect_y_read_c5::<B, AdcNoDec>(&mut self.cpu, bus, pinout),
            // And immediate
            0x2900 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x2901 =>  pinout = immediate_read_c1::<B, And>(&mut self.cpu, bus, pinout),
            // And zero page read
            0x2500 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0x2501 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0x2502 =>  pinout = zeropage_read_c2::<B, And>(&mut self.cpu, bus, pinout),
            // And zero page x read
            0x3500 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0x3501 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0x3502 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0x3503 =>  pinout = zeropage_x_read_c3::<B, And>(&mut self.cpu, bus, pinout),
            // And absolute read
            0x2D00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0x2D01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0x2D02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0x2D03 =>  pinout = absolute_read_c3::<B, And>(&mut self.cpu, bus, pinout),
            // And absolute x read
            0x3D00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0x3D01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0x3D02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0x3D03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0x3D04 =>  pinout = absolute_x_read_c4::<B, And>(&mut self.cpu, bus, pinout),
            // And absolute y read
            0x3900 =>  pinout = absolute_y_read_c0(&mut self.cpu, bus, pinout),
            0x3901 =>  pinout = absolute_y_read_c1(&mut self.cpu, bus, pinout),
            0x3902 =>  pinout = absolute_y_read_c2(&mut self.cpu, bus, pinout),
            0x3903 =>  pinout = absolute_y_read_c3(&mut self.cpu, bus, pinout),
            0x3904 =>  pinout = absolute_y_read_c4::<B, And>(&mut self.cpu, bus, pinout),
            // And indirect x read
            0x2100 =>  pinout = indirect_x_read_c0(&mut self.cpu, bus, pinout),
            0x2101 =>  pinout = indirect_x_read_c1(&mut self.cpu, bus, pinout),
            0x2102 =>  pinout = indirect_x_read_c2(&mut self.cpu, bus, pinout),
            0x2103 =>  pinout = indirect_x_read_c3(&mut self.cpu, bus, pinout),
            0x2104 =>  pinout = indirect_x_read_c4(&mut self.cpu, bus, pinout),
            0x2105 =>  pinout = indirect_x_read_c5::<B, And>(&mut self.cpu, bus, pinout),
            // And indirect y read
            0x3100 =>  pinout = indirect_y_read_c0(&mut self.cpu, bus, pinout),
            0x3101 =>  pinout = indirect_y_read_c1(&mut self.cpu, bus, pinout),
            0x3102 =>  pinout = indirect_y_read_c2(&mut self.cpu, bus, pinout),
            0x3103 =>  pinout = indirect_y_read_c3(&mut self.cpu, bus, pinout),
            0x3104 =>  pinout = indirect_y_read_c4(&mut self.cpu, bus, pinout),
            0x3105 =>  pinout = indirect_y_read_c5::<B, And>(&mut self.cpu, bus, pinout),
            // AslAccum single byte
            0x0A00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x0A01 =>  pinout = single_byte_c1::<B, AslAccum>(&mut self.cpu, bus, pinout),
            // Asl zero page modify
            0x0600 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0x0601 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0x0602 => pinout = zeropage_modify_c2::<B, Asl>(&mut self.cpu, bus, pinout),
            0x0603 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0x0604 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Asl zero page x modify
            0x1600 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0x1601 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0x1602 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0x1603 => pinout = zeropage_x_modify_c3::<B, Asl>(&mut self.cpu, bus, pinout),
            0x1604 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0x1605 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Asl absolute modify
            0x0E00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0x0E01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0x0E02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0x0E03 => pinout = absolute_modify_c3::<B, Asl>(&mut self.cpu, bus, pinout),
            0x0E04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0x0E05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Asl absolute x modify
            0x1E00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0x1E01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0x1E02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0x1E03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0x1E04 => pinout = absolute_x_modify_c4::<B, Asl>(&mut self.cpu, bus, pinout),
            0x1E05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0x1E06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // Bcc branch
            0x9000 =>  pinout = branch_c0(&mut self.cpu, bus, pinout),
            0x9001 =>  pinout = branch_c1::<B, Bcc>(&mut self.cpu, bus, pinout),
            0x9002 =>  pinout = branch_c2(&mut self.cpu, bus, pinout),
            0x9003 =>  pinout = branch_c3(&mut self.cpu, bus, pinout),
            // Bcs branch
            0xB000 =>  pinout = branch_c0(&mut self.cpu, bus, pinout),
            0xB001 =>  pinout = branch_c1::<B, Bcs>(&mut self.cpu, bus, pinout),
            0xB002 =>  pinout = branch_c2(&mut self.cpu, bus, pinout),
            0xB003 =>  pinout = branch_c3(&mut self.cpu, bus, pinout),
            // Beq branch
            0xF000 =>  pinout = branch_c0(&mut self.cpu, bus, pinout),
            0xF001 =>  pinout = branch_c1::<B, Beq>(&mut self.cpu, bus, pinout),
            0xF002 =>  pinout = branch_c2(&mut self.cpu, bus, pinout),
            0xF003 =>  pinout = branch_c3(&mut self.cpu, bus, pinout),
            // Bmi branch
            0x3000 =>  pinout = branch_c0(&mut self.cpu, bus, pinout),
            0x3001 =>  pinout = branch_c1::<B, Bmi>(&mut self.cpu, bus, pinout),
            0x3002 =>  pinout = branch_c2(&mut self.cpu, bus, pinout),
            0x3003 =>  pinout = branch_c3(&mut self.cpu, bus, pinout),
            // Bne branch
            0xD000 =>  pinout = branch_c0(&mut self.cpu, bus, pinout),
            0xD001 =>  pinout = branch_c1::<B, Bne>(&mut self.cpu, bus, pinout),
            0xD002 =>  pinout = branch_c2(&mut self.cpu, bus, pinout),
            0xD003 =>  pinout = branch_c3(&mut self.cpu, bus, pinout),
            // Bpl branch
            0x1000 =>  pinout = branch_c0(&mut self.cpu, bus, pinout),
            0x1001 =>  pinout = branch_c1::<B, Bpl>(&mut self.cpu, bus, pinout),
            0x1002 =>  pinout = branch_c2(&mut self.cpu, bus, pinout),
            0x1003 =>  pinout = branch_c3(&mut self.cpu, bus, pinout),
            // Bvc branch
            0x5000 =>  pinout = branch_c0(&mut self.cpu, bus, pinout),
            0x5001 =>  pinout = branch_c1::<B, Bvc>(&mut self.cpu, bus, pinout),
            0x5002 =>  pinout = branch_c2(&mut self.cpu, bus, pinout),
            0x5003 =>  pinout = branch_c3(&mut self.cpu, bus, pinout),
            // Bvs branch
            0x7000 =>  pinout = branch_c0(&mut self.cpu, bus, pinout),
            0x7001 =>  pinout = branch_c1::<B, Bvs>(&mut self.cpu, bus, pinout),
            0x7002 =>  pinout = branch_c2(&mut self.cpu, bus, pinout),
            0x7003 =>  pinout = branch_c3(&mut self.cpu, bus, pinout),
            // Bit zero page read
            0x2400 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0x2401 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0x2402 =>  pinout = zeropage_read_c2::<B, Bit>(&mut self.cpu, bus, pinout),
            // Bit absolute read
            0x2C00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0x2C01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0x2C02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0x2C03 =>  pinout = absolute_read_c3::<B, Bit>(&mut self.cpu, bus, pinout),
            // Clc single byte
            0x1800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x1801 =>  pinout = single_byte_c1::<B, Clc>(&mut self.cpu, bus, pinout),
            // Cld single byte
            0xD800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xD801 =>  pinout = single_byte_c1::<B, Cld>(&mut self.cpu, bus, pinout),
            // Cli single byte
            0x5800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x5801 =>  pinout = single_byte_c1::<B, Cli>(&mut self.cpu, bus, pinout),
            // Clv single byte
            0xB800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xB801 =>  pinout = single_byte_c1::<B, Clv>(&mut self.cpu, bus, pinout),
            // Cmp immediate
            0xC900 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xC901 =>  pinout = immediate_read_c1::<B, Cmp>(&mut self.cpu, bus, pinout),
            // Cmp zero page read
            0xC500 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0xC501 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0xC502 =>  pinout = zeropage_read_c2::<B, Cmp>(&mut self.cpu, bus, pinout),
            // Cmp zero page x read
            0xD500 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0xD501 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0xD502 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0xD503 =>  pinout = zeropage_x_read_c3::<B, Cmp>(&mut self.cpu, bus, pinout),
            // Cmp absolute read
            0xCD00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0xCD01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0xCD02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0xCD03 =>  pinout = absolute_read_c3::<B, Cmp>(&mut self.cpu, bus, pinout),
            // Cmp absolute x read
            0xDD00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0xDD01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0xDD02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0xDD03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0xDD04 =>  pinout = absolute_x_read_c4::<B, Cmp>(&mut self.cpu, bus, pinout),
            // Cmp absolute y read
            0xD900 =>  pinout = absolute_y_read_c0(&mut self.cpu, bus, pinout),
            0xD901 =>  pinout = absolute_y_read_c1(&mut self.cpu, bus, pinout),
            0xD902 =>  pinout = absolute_y_read_c2(&mut self.cpu, bus, pinout),
            0xD903 =>  pinout = absolute_y_read_c3(&mut self.cpu, bus, pinout),
            0xD904 =>  pinout = absolute_y_read_c4::<B, Cmp>(&mut self.cpu, bus, pinout),
            // Cmp indirect x read
            0xC100 =>  pinout = indirect_x_read_c0(&mut self.cpu, bus, pinout),
            0xC101 =>  pinout = indirect_x_read_c1(&mut self.cpu, bus, pinout),
            0xC102 =>  pinout = indirect_x_read_c2(&mut self.cpu, bus, pinout),
            0xC103 =>  pinout = indirect_x_read_c3(&mut self.cpu, bus, pinout),
            0xC104 =>  pinout = indirect_x_read_c4(&mut self.cpu, bus, pinout),
            0xC105 =>  pinout = indirect_x_read_c5::<B, Cmp>(&mut self.cpu, bus, pinout),
            // Cmp indirect y read
            0xD100 =>  pinout = indirect_y_read_c0(&mut self.cpu, bus, pinout),
            0xD101 =>  pinout = indirect_y_read_c1(&mut self.cpu, bus, pinout),
            0xD102 =>  pinout = indirect_y_read_c2(&mut self.cpu, bus, pinout),
            0xD103 =>  pinout = indirect_y_read_c3(&mut self.cpu, bus, pinout),
            0xD104 =>  pinout = indirect_y_read_c4(&mut self.cpu, bus, pinout),
            0xD105 =>  pinout = indirect_y_read_c5::<B, Cmp>(&mut self.cpu, bus, pinout),
            // Ldx immediate
            0xA200 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xA201 =>  pinout = immediate_read_c1::<B, Ldx>(&mut self.cpu, bus, pinout),
            // Ldx zero page read
            0xA600 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0xA601 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0xA602 =>  pinout = zeropage_read_c2::<B, Ldx>(&mut self.cpu, bus, pinout),
            // Ldx zero page y read
            0xB600 =>  pinout = zeropage_y_read_c0(&mut self.cpu, bus, pinout),
            0xB601 =>  pinout = zeropage_y_read_c1(&mut self.cpu, bus, pinout),
            0xB602 =>  pinout = zeropage_y_read_c2(&mut self.cpu, bus, pinout),
            0xB603 =>  pinout = zeropage_y_read_c3::<B, Ldx>(&mut self.cpu, bus, pinout),
            // Ldx absolute read
            0xAE00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0xAE01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0xAE02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0xAE03 =>  pinout = absolute_read_c3::<B, Ldx>(&mut self.cpu, bus, pinout),
            // Ldx absolute y read
            0xBE00 =>  pinout = absolute_y_read_c0(&mut self.cpu, bus, pinout),
            0xBE01 =>  pinout = absolute_y_read_c1(&mut self.cpu, bus, pinout),
            0xBE02 =>  pinout = absolute_y_read_c2(&mut self.cpu, bus, pinout),
            0xBE03 =>  pinout = absolute_y_read_c3(&mut self.cpu, bus, pinout),
            0xBE04 =>  pinout = absolute_y_read_c4::<B, Ldx>(&mut self.cpu, bus, pinout),
            // Ldy immediate
            0xA000 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xA001 =>  pinout = immediate_read_c1::<B, Ldy>(&mut self.cpu, bus, pinout),
            // Ldy zero page read
            0xA400 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0xA401 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0xA402 =>  pinout = zeropage_read_c2::<B, Ldy>(&mut self.cpu, bus, pinout),
            // Ldy zero page x read
            0xB400 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0xB401 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0xB402 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0xB403 =>  pinout = zeropage_x_read_c3::<B, Ldy>(&mut self.cpu, bus, pinout),
            // Ldy absolute read
            0xAC00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0xAC01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0xAC02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0xAC03 =>  pinout = absolute_read_c3::<B, Ldy>(&mut self.cpu, bus, pinout),
            // Ldy absolute x read
            0xBC00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0xBC01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0xBC02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0xBC03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0xBC04 =>  pinout = absolute_x_read_c4::<B, Ldy>(&mut self.cpu, bus, pinout),
            // Cpx immediate
            0xE000 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xE001 =>  pinout = immediate_read_c1::<B, Cpx>(&mut self.cpu, bus, pinout),
            // Cpx zero page read
            0xE400 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0xE401 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0xE402 =>  pinout = zeropage_read_c2::<B, Cpx>(&mut self.cpu, bus, pinout),
            // Cpx absolute read
            0xEC00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0xEC01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0xEC02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0xEC03 =>  pinout = absolute_read_c3::<B, Cpx>(&mut self.cpu, bus, pinout),
            // Cpy immediate
            0xC000 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xC001 =>  pinout = immediate_read_c1::<B, Cpy>(&mut self.cpu, bus, pinout),
            // Cpy zero page read
            0xC400 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0xC401 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0xC402 =>  pinout = zeropage_read_c2::<B, Cpy>(&mut self.cpu, bus, pinout),
            // Cpy absolute read
            0xCC00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0xCC01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0xCC02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0xCC03 =>  pinout = absolute_read_c3::<B, Cpy>(&mut self.cpu, bus, pinout),
            // Dec zero page modify
            0xC600 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0xC601 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0xC602 => pinout = zeropage_modify_c2::<B, Dec>(&mut self.cpu, bus, pinout),
            0xC603 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0xC604 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Dec zero page x modify
            0xD600 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0xD601 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0xD602 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0xD603 => pinout = zeropage_x_modify_c3::<B, Dec>(&mut self.cpu, bus, pinout),
            0xD604 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0xD605 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Dec absolute modify
            0xCE00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0xCE01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0xCE02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0xCE03 => pinout = absolute_modify_c3::<B, Dec>(&mut self.cpu, bus, pinout),
            0xCE04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0xCE05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Dec absolute x modify
            0xDE00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0xDE01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0xDE02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0xDE03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0xDE04 => pinout = absolute_x_modify_c4::<B, Dec>(&mut self.cpu, bus, pinout),
            0xDE05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0xDE06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // Dex single byte
            0xCA00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xCA01 =>  pinout = single_byte_c1::<B, Dex>(&mut self.cpu, bus, pinout),
            // Dey single byte
            0x8800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x8801 =>  pinout = single_byte_c1::<B, Dey>(&mut self.cpu, bus, pinout),
            // Eor immediate
            0x4900 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x4901 =>  pinout = immediate_read_c1::<B, Eor>(&mut self.cpu, bus, pinout),
            // Eor zero page read
            0x4500 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0x4501 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0x4502 =>  pinout = zeropage_read_c2::<B, Eor>(&mut self.cpu, bus, pinout),
            // Eor zero page x read
            0x5500 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0x5501 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0x5502 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0x5503 =>  pinout = zeropage_x_read_c3::<B, Eor>(&mut self.cpu, bus, pinout),
            // Eor absolute read
            0x4D00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0x4D01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0x4D02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0x4D03 =>  pinout = absolute_read_c3::<B, Eor>(&mut self.cpu, bus, pinout),
            // Eor absolute x read
            0x5D00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0x5D01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0x5D02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0x5D03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0x5D04 =>  pinout = absolute_x_read_c4::<B, Eor>(&mut self.cpu, bus, pinout),
            // Eor absolute y read
            0x5900 =>  pinout = absolute_y_read_c0(&mut self.cpu, bus, pinout),
            0x5901 =>  pinout = absolute_y_read_c1(&mut self.cpu, bus, pinout),
            0x5902 =>  pinout = absolute_y_read_c2(&mut self.cpu, bus, pinout),
            0x5903 =>  pinout = absolute_y_read_c3(&mut self.cpu, bus, pinout),
            0x5904 =>  pinout = absolute_y_read_c4::<B, Eor>(&mut self.cpu, bus, pinout),
            // Eor indirect x read
            0x4100 =>  pinout = indirect_x_read_c0(&mut self.cpu, bus, pinout),
            0x4101 =>  pinout = indirect_x_read_c1(&mut self.cpu, bus, pinout),
            0x4102 =>  pinout = indirect_x_read_c2(&mut self.cpu, bus, pinout),
            0x4103 =>  pinout = indirect_x_read_c3(&mut self.cpu, bus, pinout),
            0x4104 =>  pinout = indirect_x_read_c4(&mut self.cpu, bus, pinout),
            0x4105 =>  pinout = indirect_x_read_c5::<B, Eor>(&mut self.cpu, bus, pinout),
            // Eor indirect y read
            0x5100 =>  pinout = indirect_y_read_c0(&mut self.cpu, bus, pinout),
            0x5101 =>  pinout = indirect_y_read_c1(&mut self.cpu, bus, pinout),
            0x5102 =>  pinout = indirect_y_read_c2(&mut self.cpu, bus, pinout),
            0x5103 =>  pinout = indirect_y_read_c3(&mut self.cpu, bus, pinout),
            0x5104 =>  pinout = indirect_y_read_c4(&mut self.cpu, bus, pinout),
            0x5105 =>  pinout = indirect_y_read_c5::<B, Eor>(&mut self.cpu, bus, pinout),
            // Inc zero page modify
            0xE600 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0xE601 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0xE602 => pinout = zeropage_modify_c2::<B, Inc>(&mut self.cpu, bus, pinout),
            0xE603 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0xE604 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Inc zero page x modify
            0xF600 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0xF601 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0xF602 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0xF603 => pinout = zeropage_x_modify_c3::<B, Inc>(&mut self.cpu, bus, pinout),
            0xF604 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0xF605 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Inc absolute modify
            0xEE00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0xEE01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0xEE02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0xEE03 => pinout = absolute_modify_c3::<B, Inc>(&mut self.cpu, bus, pinout),
            0xEE04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0xEE05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Inc absolute x modify
            0xFE00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0xFE01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0xFE02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0xFE03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0xFE04 => pinout = absolute_x_modify_c4::<B, Inc>(&mut self.cpu, bus, pinout),
            0xFE05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0xFE06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // Inx single byte
            0xE800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xE801 =>  pinout = single_byte_c1::<B, Inx>(&mut self.cpu, bus, pinout),
            // Iny single byte
            0xC800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xC801 =>  pinout = single_byte_c1::<B, Iny>(&mut self.cpu, bus, pinout),
            // Lda immediate
            0xA900 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xA901 =>  pinout = immediate_read_c1::<B, Lda>(&mut self.cpu, bus, pinout),
            // Lda zero page read
            0xA500 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0xA501 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0xA502 =>  pinout = zeropage_read_c2::<B, Lda>(&mut self.cpu, bus, pinout),
            // Lda zero page x read
            0xB500 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0xB501 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0xB502 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0xB503 =>  pinout = zeropage_x_read_c3::<B, Lda>(&mut self.cpu, bus, pinout),
            // Lda absolute read
            0xAD00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0xAD01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0xAD02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0xAD03 =>  pinout = absolute_read_c3::<B, Lda>(&mut self.cpu, bus, pinout),
            // Lda absolute x read
            0xBD00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0xBD01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0xBD02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0xBD03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0xBD04 =>  pinout = absolute_x_read_c4::<B, Lda>(&mut self.cpu, bus, pinout),
            // Lda absolute y read
            0xB900 =>  pinout = absolute_y_read_c0(&mut self.cpu, bus, pinout),
            0xB901 =>  pinout = absolute_y_read_c1(&mut self.cpu, bus, pinout),
            0xB902 =>  pinout = absolute_y_read_c2(&mut self.cpu, bus, pinout),
            0xB903 =>  pinout = absolute_y_read_c3(&mut self.cpu, bus, pinout),
            0xB904 =>  pinout = absolute_y_read_c4::<B, Lda>(&mut self.cpu, bus, pinout),
            // Lda indirect x read
            0xA100 =>  pinout = indirect_x_read_c0(&mut self.cpu, bus, pinout),
            0xA101 =>  pinout = indirect_x_read_c1(&mut self.cpu, bus, pinout),
            0xA102 =>  pinout = indirect_x_read_c2(&mut self.cpu, bus, pinout),
            0xA103 =>  pinout = indirect_x_read_c3(&mut self.cpu, bus, pinout),
            0xA104 =>  pinout = indirect_x_read_c4(&mut self.cpu, bus, pinout),
            0xA105 =>  pinout = indirect_x_read_c5::<B, Lda>(&mut self.cpu, bus, pinout),
            // Lda indirect y read
            0xB100 =>  pinout = indirect_y_read_c0(&mut self.cpu, bus, pinout),
            0xB101 =>  pinout = indirect_y_read_c1(&mut self.cpu, bus, pinout),
            0xB102 =>  pinout = indirect_y_read_c2(&mut self.cpu, bus, pinout),
            0xB103 =>  pinout = indirect_y_read_c3(&mut self.cpu, bus, pinout),
            0xB104 =>  pinout = indirect_y_read_c4(&mut self.cpu, bus, pinout),
            0xB105 =>  pinout = indirect_y_read_c5::<B, Lda>(&mut self.cpu, bus, pinout),
            // LsrAccum single byte
            0x4A00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x4A01 =>  pinout = single_byte_c1::<B, LsrAccum>(&mut self.cpu, bus, pinout),
            // Lsr zero page modify
            0x4600 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0x4601 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0x4602 => pinout = zeropage_modify_c2::<B, Lsr>(&mut self.cpu, bus, pinout),
            0x4603 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0x4604 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Lsr zero page x modify
            0x5600 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0x5601 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0x5602 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0x5603 => pinout = zeropage_x_modify_c3::<B, Lsr>(&mut self.cpu, bus, pinout),
            0x5604 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0x5605 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Lsr absolute modify
            0x4E00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0x4E01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0x4E02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0x4E03 => pinout = absolute_modify_c3::<B, Lsr>(&mut self.cpu, bus, pinout),
            0x4E04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0x4E05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Lsr absolute x modify
            0x5E00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0x5E01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0x5E02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0x5E03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0x5E04 => pinout = absolute_x_modify_c4::<B, Lsr>(&mut self.cpu, bus, pinout),
            0x5E05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0x5E06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // Nop single byte
            0xEA00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xEA01 =>  pinout = single_byte_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Ora immediate
            0x0900 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x0901 =>  pinout = immediate_read_c1::<B, Ora>(&mut self.cpu, bus, pinout),
            // Ora zero page read
            0x0500 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0x0501 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0x0502 =>  pinout = zeropage_read_c2::<B, Ora>(&mut self.cpu, bus, pinout),
            // Ora zero page x read
            0x1500 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0x1501 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0x1502 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0x1503 =>  pinout = zeropage_x_read_c3::<B, Ora>(&mut self.cpu, bus, pinout),
            // Ora absolute read
            0x0D00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0x0D01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0x0D02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0x0D03 =>  pinout = absolute_read_c3::<B, Ora>(&mut self.cpu, bus, pinout),
            // Ora absolute x read
            0x1D00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0x1D01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0x1D02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0x1D03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0x1D04 =>  pinout = absolute_x_read_c4::<B, Ora>(&mut self.cpu, bus, pinout),
            // Ora absolute y read
            0x1900 =>  pinout = absolute_y_read_c0(&mut self.cpu, bus, pinout),
            0x1901 =>  pinout = absolute_y_read_c1(&mut self.cpu, bus, pinout),
            0x1902 =>  pinout = absolute_y_read_c2(&mut self.cpu, bus, pinout),
            0x1903 =>  pinout = absolute_y_read_c3(&mut self.cpu, bus, pinout),
            0x1904 =>  pinout = absolute_y_read_c4::<B, Ora>(&mut self.cpu, bus, pinout),
            // Ora indirect x read
            0x0100 =>  pinout = indirect_x_read_c0(&mut self.cpu, bus, pinout),
            0x0101 =>  pinout = indirect_x_read_c1(&mut self.cpu, bus, pinout),
            0x0102 =>  pinout = indirect_x_read_c2(&mut self.cpu, bus, pinout),
            0x0103 =>  pinout = indirect_x_read_c3(&mut self.cpu, bus, pinout),
            0x0104 =>  pinout = indirect_x_read_c4(&mut self.cpu, bus, pinout),
            0x0105 =>  pinout = indirect_x_read_c5::<B, Ora>(&mut self.cpu, bus, pinout),
            // Ora indirect y read
            0x1100 =>  pinout = indirect_y_read_c0(&mut self.cpu, bus, pinout),
            0x1101 =>  pinout = indirect_y_read_c1(&mut self.cpu, bus, pinout),
            0x1102 =>  pinout = indirect_y_read_c2(&mut self.cpu, bus, pinout),
            0x1103 =>  pinout = indirect_y_read_c3(&mut self.cpu, bus, pinout),
            0x1104 =>  pinout = indirect_y_read_c4(&mut self.cpu, bus, pinout),
            0x1105 =>  pinout = indirect_y_read_c5::<B, Ora>(&mut self.cpu, bus, pinout),
            // RolAccum single byte
            0x2A00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x2A01 =>  pinout = single_byte_c1::<B, RolAccum>(&mut self.cpu, bus, pinout),
            // Rol zero page modify
            0x2600 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0x2601 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0x2602 => pinout = zeropage_modify_c2::<B, Rol>(&mut self.cpu, bus, pinout),
            0x2603 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0x2604 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Rol zero page x modify
            0x3600 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0x3601 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0x3602 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0x3603 => pinout = zeropage_x_modify_c3::<B, Rol>(&mut self.cpu, bus, pinout),
            0x3604 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0x3605 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Rol absolute modify
            0x2E00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0x2E01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0x2E02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0x2E03 => pinout = absolute_modify_c3::<B, Rol>(&mut self.cpu, bus, pinout),
            0x2E04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0x2E05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Rol absolute x modify
            0x3E00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0x3E01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0x3E02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0x3E03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0x3E04 => pinout = absolute_x_modify_c4::<B, Rol>(&mut self.cpu, bus, pinout),
            0x3E05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0x3E06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // RorAccum single byte
            0x6A00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x6A01 =>  pinout = single_byte_c1::<B, RorAccum>(&mut self.cpu, bus, pinout),
            // Ror zero page modify
            0x6600 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0x6601 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0x6602 => pinout = zeropage_modify_c2::<B, Ror>(&mut self.cpu, bus, pinout),
            0x6603 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0x6604 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Ror zero page x modify
            0x7600 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0x7601 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0x7602 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0x7603 => pinout = zeropage_x_modify_c3::<B, Ror>(&mut self.cpu, bus, pinout),
            0x7604 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0x7605 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Ror absolute modify
            0x6E00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0x6E01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0x6E02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0x6E03 => pinout = absolute_modify_c3::<B, Ror>(&mut self.cpu, bus, pinout),
            0x6E04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0x6E05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Ror absolute x modify
            0x7E00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0x7E01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0x7E02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0x7E03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0x7E04 => pinout = absolute_x_modify_c4::<B, Ror>(&mut self.cpu, bus, pinout),
            0x7E05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0x7E06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // SbcNoDec immediate
            0xE900 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xE901 =>  pinout = immediate_read_c1::<B, SbcNoDec>(&mut self.cpu, bus, pinout),
            // SbcNoDec zero page read
            0xE500 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0xE501 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0xE502 =>  pinout = zeropage_read_c2::<B, SbcNoDec>(&mut self.cpu, bus, pinout),
            // SbcNoDec zero page x read
            0xF500 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0xF501 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0xF502 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0xF503 =>  pinout = zeropage_x_read_c3::<B, SbcNoDec>(&mut self.cpu, bus, pinout),
            // SbcNoDec absolute read
            0xED00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0xED01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0xED02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0xED03 =>  pinout = absolute_read_c3::<B, SbcNoDec>(&mut self.cpu, bus, pinout),
            // SbcNoDec absolute x read
            0xFD00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0xFD01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0xFD02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0xFD03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0xFD04 =>  pinout = absolute_x_read_c4::<B, SbcNoDec>(&mut self.cpu, bus, pinout),
            // SbcNoDec absolute y read
            0xF900 =>  pinout = absolute_y_read_c0(&mut self.cpu, bus, pinout),
            0xF901 =>  pinout = absolute_y_read_c1(&mut self.cpu, bus, pinout),
            0xF902 =>  pinout = absolute_y_read_c2(&mut self.cpu, bus, pinout),
            0xF903 =>  pinout = absolute_y_read_c3(&mut self.cpu, bus, pinout),
            0xF904 =>  pinout = absolute_y_read_c4::<B, SbcNoDec>(&mut self.cpu, bus, pinout),
            // SbcNoDec indirect x read
            0xE100 =>  pinout = indirect_x_read_c0(&mut self.cpu, bus, pinout),
            0xE101 =>  pinout = indirect_x_read_c1(&mut self.cpu, bus, pinout),
            0xE102 =>  pinout = indirect_x_read_c2(&mut self.cpu, bus, pinout),
            0xE103 =>  pinout = indirect_x_read_c3(&mut self.cpu, bus, pinout),
            0xE104 =>  pinout = indirect_x_read_c4(&mut self.cpu, bus, pinout),
            0xE105 =>  pinout = indirect_x_read_c5::<B, SbcNoDec>(&mut self.cpu, bus, pinout),
            // SbcNoDec indirect y read
            0xF100 =>  pinout = indirect_y_read_c0(&mut self.cpu, bus, pinout),
            0xF101 =>  pinout = indirect_y_read_c1(&mut self.cpu, bus, pinout),
            0xF102 =>  pinout = indirect_y_read_c2(&mut self.cpu, bus, pinout),
            0xF103 =>  pinout = indirect_y_read_c3(&mut self.cpu, bus, pinout),
            0xF104 =>  pinout = indirect_y_read_c4(&mut self.cpu, bus, pinout),
            0xF105 =>  pinout = indirect_y_read_c5::<B, SbcNoDec>(&mut self.cpu, bus, pinout),
            // Sec single byte
            0x3800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x3801 =>  pinout = single_byte_c1::<B, Sec>(&mut self.cpu, bus, pinout),
            // Sed single byte
            0xF800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xF801 =>  pinout = single_byte_c1::<B, Sed>(&mut self.cpu, bus, pinout),
            // Sei single byte
            0x7800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x7801 =>  pinout = single_byte_c1::<B, Sei>(&mut self.cpu, bus, pinout),
            // Sta zero page store
            0x8500 =>  pinout = zeropage_store_c0(&mut self.cpu, bus, pinout),
            0x8501 => pinout = zeropage_store_c1::<B, Sta>(&mut self.cpu, bus, pinout),
            0x8502 =>  pinout = zeropage_store_c2(&mut self.cpu, bus, pinout),
            // Sta zero page x store
            0x9500 =>  pinout = zeropage_x_store_c0(&mut self.cpu, bus, pinout),
            0x9501 =>  pinout = zeropage_x_store_c1(&mut self.cpu, bus, pinout),
            0x9502 => pinout = zeropage_x_store_c2::<B, Sta>(&mut self.cpu, bus, pinout),
            0x9503 =>  pinout = zeropage_x_store_c3(&mut self.cpu, bus, pinout),
            // Sta absolute store
            0x8D00 =>  pinout = absolute_store_c0(&mut self.cpu, bus, pinout),
            0x8D01 =>  pinout = absolute_store_c1(&mut self.cpu, bus, pinout),
            0x8D02 => pinout = absolute_store_c2::<B, Sta>(&mut self.cpu, bus, pinout),
            0x8D03 =>  pinout = absolute_store_c3(&mut self.cpu, bus, pinout),
            // Sta absolute x store
            0x9D00 =>  pinout = absolute_x_store_c0(&mut self.cpu, bus, pinout),
            0x9D01 =>  pinout = absolute_x_store_c1(&mut self.cpu, bus, pinout),
            0x9D02 =>  pinout = absolute_x_store_c2(&mut self.cpu, bus, pinout),
            0x9D03 => pinout = absolute_x_store_c3::<B, Sta>(&mut self.cpu, bus, pinout),
            0x9D04 =>  pinout = absolute_x_store_c4(&mut self.cpu, bus, pinout),
            // Sta absolute y store
            0x9900 =>  pinout = absolute_y_store_c0(&mut self.cpu, bus, pinout),
            0x9901 =>  pinout = absolute_y_store_c1(&mut self.cpu, bus, pinout),
            0x9902 =>  pinout = absolute_y_store_c2(&mut self.cpu, bus, pinout),
            0x9903 => pinout = absolute_y_store_c3::<B, Sta>(&mut self.cpu, bus, pinout),
            0x9904 =>  pinout = absolute_y_store_c4(&mut self.cpu, bus, pinout),
            // Sta indirect x store
            0x8100 =>  pinout = indirect_x_store_c0(&mut self.cpu, bus, pinout),
            0x8101 =>  pinout = indirect_x_store_c1(&mut self.cpu, bus, pinout),
            0x8102 =>  pinout = indirect_x_store_c2(&mut self.cpu, bus, pinout),
            0x8103 =>  pinout = indirect_x_store_c3(&mut self.cpu, bus, pinout),
            0x8104 => pinout = indirect_x_store_c4::<B, Sta>(&mut self.cpu, bus, pinout),
            0x8105 =>  pinout = indirect_x_store_c5(&mut self.cpu, bus, pinout),
            // Sta indirect y store
            0x9100 =>  pinout = indirect_y_store_c0(&mut self.cpu, bus, pinout),
            0x9101 =>  pinout = indirect_y_store_c1(&mut self.cpu, bus, pinout),
            0x9102 =>  pinout = indirect_y_store_c2(&mut self.cpu, bus, pinout),
            0x9103 =>  pinout = indirect_y_store_c3(&mut self.cpu, bus, pinout),
            0x9104 => pinout = indirect_y_store_c4::<B, Sta>(&mut self.cpu, bus, pinout),
            0x9105 =>  pinout = indirect_y_store_c5(&mut self.cpu, bus, pinout),
            // Stx zero page store
            0x8600 =>  pinout = zeropage_store_c0(&mut self.cpu, bus, pinout),
            0x8601 => pinout = zeropage_store_c1::<B, Stx>(&mut self.cpu, bus, pinout),
            0x8602 =>  pinout = zeropage_store_c2(&mut self.cpu, bus, pinout),
            // Stx zero page y store
            0x9600 =>  pinout = zeropage_y_store_c0(&mut self.cpu, bus, pinout),
            0x9601 =>  pinout = zeropage_y_store_c1(&mut self.cpu, bus, pinout),
            0x9602 => pinout = zeropage_y_store_c2::<B, Stx>(&mut self.cpu, bus, pinout),
            0x9603 =>  pinout = zeropage_y_store_c3(&mut self.cpu, bus, pinout),
            // Stx absolute store
            0x8E00 =>  pinout = absolute_store_c0(&mut self.cpu, bus, pinout),
            0x8E01 =>  pinout = absolute_store_c1(&mut self.cpu, bus, pinout),
            0x8E02 => pinout = absolute_store_c2::<B, Stx>(&mut self.cpu, bus, pinout),
            0x8E03 =>  pinout = absolute_store_c3(&mut self.cpu, bus, pinout),
            // Sty zero page store
            0x8400 =>  pinout = zeropage_store_c0(&mut self.cpu, bus, pinout),
            0x8401 => pinout = zeropage_store_c1::<B, Sty>(&mut self.cpu, bus, pinout),
            0x8402 =>  pinout = zeropage_store_c2(&mut self.cpu, bus, pinout),
            // Sty zero page x store
            0x9400 =>  pinout = zeropage_x_store_c0(&mut self.cpu, bus, pinout),
            0x9401 =>  pinout = zeropage_x_store_c1(&mut self.cpu, bus, pinout),
            0x9402 => pinout = zeropage_x_store_c2::<B, Sty>(&mut self.cpu, bus, pinout),
            0x9403 =>  pinout = zeropage_x_store_c3(&mut self.cpu, bus, pinout),
            // Sty absolute store
            0x8C00 =>  pinout = absolute_store_c0(&mut self.cpu, bus, pinout),
            0x8C01 =>  pinout = absolute_store_c1(&mut self.cpu, bus, pinout),
            0x8C02 => pinout = absolute_store_c2::<B, Sty>(&mut self.cpu, bus, pinout),
            0x8C03 =>  pinout = absolute_store_c3(&mut self.cpu, bus, pinout),
            // Tax single byte
            0xAA00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xAA01 =>  pinout = single_byte_c1::<B, Tax>(&mut self.cpu, bus, pinout),
            // Tay single byte
            0xA800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xA801 =>  pinout = single_byte_c1::<B, Tay>(&mut self.cpu, bus, pinout),
            // Tsx single byte
            0xBA00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xBA01 =>  pinout = single_byte_c1::<B, Tsx>(&mut self.cpu, bus, pinout),
            // Txa single byte
            0x8A00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x8A01 =>  pinout = single_byte_c1::<B, Txa>(&mut self.cpu, bus, pinout),
            // Txs single byte
            0x9A00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x9A01 =>  pinout = single_byte_c1::<B, Txs>(&mut self.cpu, bus, pinout),
            // Tya single byte
            0x9800 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x9801 =>  pinout = single_byte_c1::<B, Tya>(&mut self.cpu, bus, pinout),
            // Aac immediate
            0x0B00 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x0B01 =>  pinout = immediate_read_c1::<B, Aac>(&mut self.cpu, bus, pinout),
            // Aac immediate
            0x2B00 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x2B01 =>  pinout = immediate_read_c1::<B, Aac>(&mut self.cpu, bus, pinout),
            // Arr immediate
            0x6B00 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x6B01 =>  pinout = immediate_read_c1::<B, Arr>(&mut self.cpu, bus, pinout),
            // Asr immediate
            0x4B00 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x4B01 =>  pinout = immediate_read_c1::<B, Asr>(&mut self.cpu, bus, pinout),
            // Atx immediate
            0xAB00 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xAB01 =>  pinout = immediate_read_c1::<B, Atx>(&mut self.cpu, bus, pinout),
            // Axs immediate
            0xCB00 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xCB01 =>  pinout = immediate_read_c1::<B, Axs>(&mut self.cpu, bus, pinout),
            // Aax zero page store
            0x8700 =>  pinout = zeropage_store_c0(&mut self.cpu, bus, pinout),
            0x8701 => pinout = zeropage_store_c1::<B, Aax>(&mut self.cpu, bus, pinout),
            0x8702 =>  pinout = zeropage_store_c2(&mut self.cpu, bus, pinout),
            // Aax zero page y store
            0x9700 =>  pinout = zeropage_y_store_c0(&mut self.cpu, bus, pinout),
            0x9701 =>  pinout = zeropage_y_store_c1(&mut self.cpu, bus, pinout),
            0x9702 => pinout = zeropage_y_store_c2::<B, Aax>(&mut self.cpu, bus, pinout),
            0x9703 =>  pinout = zeropage_y_store_c3(&mut self.cpu, bus, pinout),
            // Aax indirect x store
            0x8300 =>  pinout = indirect_x_store_c0(&mut self.cpu, bus, pinout),
            0x8301 =>  pinout = indirect_x_store_c1(&mut self.cpu, bus, pinout),
            0x8302 =>  pinout = indirect_x_store_c2(&mut self.cpu, bus, pinout),
            0x8303 =>  pinout = indirect_x_store_c3(&mut self.cpu, bus, pinout),
            0x8304 => pinout = indirect_x_store_c4::<B, Aax>(&mut self.cpu, bus, pinout),
            0x8305 =>  pinout = indirect_x_store_c5(&mut self.cpu, bus, pinout),
            // Aax absolute store
            0x8F00 =>  pinout = absolute_store_c0(&mut self.cpu, bus, pinout),
            0x8F01 =>  pinout = absolute_store_c1(&mut self.cpu, bus, pinout),
            0x8F02 => pinout = absolute_store_c2::<B, Aax>(&mut self.cpu, bus, pinout),
            0x8F03 =>  pinout = absolute_store_c3(&mut self.cpu, bus, pinout),
            // Axa absolute y store
            0x9F00 =>  pinout = absolute_y_store_c0(&mut self.cpu, bus, pinout),
            0x9F01 =>  pinout = absolute_y_store_c1(&mut self.cpu, bus, pinout),
            0x9F02 =>  pinout = absolute_y_store_c2(&mut self.cpu, bus, pinout),
            0x9F03 => pinout = absolute_y_store_c3::<B, Axa>(&mut self.cpu, bus, pinout),
            0x9F04 =>  pinout = absolute_y_store_c4(&mut self.cpu, bus, pinout),
            // Axa indirect y store
            0x9300 =>  pinout = indirect_y_store_c0(&mut self.cpu, bus, pinout),
            0x9301 =>  pinout = indirect_y_store_c1(&mut self.cpu, bus, pinout),
            0x9302 =>  pinout = indirect_y_store_c2(&mut self.cpu, bus, pinout),
            0x9303 =>  pinout = indirect_y_store_c3(&mut self.cpu, bus, pinout),
            0x9304 => pinout = indirect_y_store_c4::<B, Axa>(&mut self.cpu, bus, pinout),
            0x9305 =>  pinout = indirect_y_store_c5(&mut self.cpu, bus, pinout),
            // Dcp zero page modify
            0xC700 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0xC701 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0xC702 => pinout = zeropage_modify_c2::<B, Dcp>(&mut self.cpu, bus, pinout),
            0xC703 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0xC704 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Dcp zero page x modify
            0xD700 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0xD701 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0xD702 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0xD703 => pinout = zeropage_x_modify_c3::<B, Dcp>(&mut self.cpu, bus, pinout),
            0xD704 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0xD705 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Dcp absolute modify
            0xCF00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0xCF01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0xCF02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0xCF03 => pinout = absolute_modify_c3::<B, Dcp>(&mut self.cpu, bus, pinout),
            0xCF04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0xCF05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Dcp absolute x modify
            0xDF00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0xDF01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0xDF02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0xDF03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0xDF04 => pinout = absolute_x_modify_c4::<B, Dcp>(&mut self.cpu, bus, pinout),
            0xDF05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0xDF06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // Dcp undocumented absolute y modify
            0xDB00 =>  pinout = undoc_absolute_y_c0(&mut self.cpu, bus, pinout),
            0xDB01 =>  pinout = undoc_absolute_y_c1(&mut self.cpu, bus, pinout),
            0xDB02 =>  pinout = undoc_absolute_y_c2(&mut self.cpu, bus, pinout),
            0xDB03 =>  pinout = undoc_absolute_y_c3(&mut self.cpu, bus, pinout),
            0xDB04 => pinout = undoc_absolute_y_c4::<B, Dcp>( &mut self.cpu, bus, pinout),
            0xDB05 => pinout = undoc_absolute_y_c5( &mut self.cpu, bus, pinout),
            0xDB06 =>  pinout = undoc_absolute_y_c6(&mut self.cpu, bus, pinout),
            // Dcp undocumented indirect x modify
            0xC300 =>  pinout = undoc_indirect_x_c0(&mut self.cpu, bus, pinout),
            0xC301 =>  pinout = undoc_indirect_x_c1(&mut self.cpu, bus, pinout),
            0xC302 =>  pinout = undoc_indirect_x_c2(&mut self.cpu, bus, pinout),
            0xC303 =>  pinout = undoc_indirect_x_c3(&mut self.cpu, bus, pinout),
            0xC304 =>  pinout = undoc_indirect_x_c4(&mut self.cpu, bus, pinout),
            0xC305 => pinout = undoc_indirect_x_c5::<B, Dcp>( &mut self.cpu, bus, pinout),
            0xC306 => pinout = undoc_indirect_x_c6( &mut self.cpu, bus, pinout),
            0xC307 =>  pinout = undoc_indirect_x_c7(&mut self.cpu, bus, pinout),
            // Dcp undocumented indirect y modify
            0xD300 =>  pinout = undoc_indirect_y_c0(&mut self.cpu, bus, pinout),
            0xD301 =>  pinout = undoc_indirect_y_c1(&mut self.cpu, bus, pinout),
            0xD302 =>  pinout = undoc_indirect_y_c2(&mut self.cpu, bus, pinout),
            0xD303 =>  pinout = undoc_indirect_y_c3(&mut self.cpu, bus, pinout),
            0xD304 =>  pinout = undoc_indirect_y_c4(&mut self.cpu, bus, pinout),
            0xD305 => pinout = undoc_indirect_y_c5::<B, Dcp>( &mut self.cpu, bus, pinout),
            0xD306 => pinout = undoc_indirect_y_c6( &mut self.cpu, bus, pinout),
            0xD307 =>  pinout = undoc_indirect_y_c7(&mut self.cpu, bus, pinout),
            // Nop zero page read
            0x0400 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0x0401 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0x0402 =>  pinout = zeropage_read_c2::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop zero page read
            0x4400 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0x4401 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0x4402 =>  pinout = zeropage_read_c2::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop zero page read
            0x6400 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0x6401 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0x6402 =>  pinout = zeropage_read_c2::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop zero page x read
            0x1400 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0x1401 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0x1402 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0x1403 =>  pinout = zeropage_x_read_c3::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop zero page x read
            0x3400 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0x3401 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0x3402 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0x3403 =>  pinout = zeropage_x_read_c3::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop zero page x read
            0x5400 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0x5401 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0x5402 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0x5403 =>  pinout = zeropage_x_read_c3::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop zero page x read
            0x7400 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0x7401 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0x7402 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0x7403 =>  pinout = zeropage_x_read_c3::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop zero page x read
            0xD400 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0xD401 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0xD402 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0xD403 =>  pinout = zeropage_x_read_c3::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop zero page x read
            0xF400 =>  pinout = zeropage_x_read_c0(&mut self.cpu, bus, pinout),
            0xF401 =>  pinout = zeropage_x_read_c1(&mut self.cpu, bus, pinout),
            0xF402 =>  pinout = zeropage_x_read_c2(&mut self.cpu, bus, pinout),
            0xF403 =>  pinout = zeropage_x_read_c3::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop immediate
            0x8000 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x8001 =>  pinout = immediate_read_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop immediate
            0x8200 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x8201 =>  pinout = immediate_read_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop immediate
            0x8900 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x8901 =>  pinout = immediate_read_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop immediate
            0xC200 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xC201 =>  pinout = immediate_read_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop immediate
            0xE200 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xE201 =>  pinout = immediate_read_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Isc zero page modify
            0xE700 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0xE701 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0xE702 => pinout = zeropage_modify_c2::<B, Isc>(&mut self.cpu, bus, pinout),
            0xE703 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0xE704 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Isc zero page x modify
            0xF700 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0xF701 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0xF702 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0xF703 => pinout = zeropage_x_modify_c3::<B, Isc>(&mut self.cpu, bus, pinout),
            0xF704 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0xF705 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Isc absolute modify
            0xEF00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0xEF01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0xEF02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0xEF03 => pinout = absolute_modify_c3::<B, Isc>(&mut self.cpu, bus, pinout),
            0xEF04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0xEF05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Isc absolute x modify
            0xFF00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0xFF01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0xFF02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0xFF03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0xFF04 => pinout = absolute_x_modify_c4::<B, Isc>(&mut self.cpu, bus, pinout),
            0xFF05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0xFF06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // Isc undocumented absolute y modify
            0xFB00 =>  pinout = undoc_absolute_y_c0(&mut self.cpu, bus, pinout),
            0xFB01 =>  pinout = undoc_absolute_y_c1(&mut self.cpu, bus, pinout),
            0xFB02 =>  pinout = undoc_absolute_y_c2(&mut self.cpu, bus, pinout),
            0xFB03 =>  pinout = undoc_absolute_y_c3(&mut self.cpu, bus, pinout),
            0xFB04 => pinout = undoc_absolute_y_c4::<B, Isc>( &mut self.cpu, bus, pinout),
            0xFB05 => pinout = undoc_absolute_y_c5( &mut self.cpu, bus, pinout),
            0xFB06 =>  pinout = undoc_absolute_y_c6(&mut self.cpu, bus, pinout),
            // Isc undocumented indirect x modify
            0xE300 =>  pinout = undoc_indirect_x_c0(&mut self.cpu, bus, pinout),
            0xE301 =>  pinout = undoc_indirect_x_c1(&mut self.cpu, bus, pinout),
            0xE302 =>  pinout = undoc_indirect_x_c2(&mut self.cpu, bus, pinout),
            0xE303 =>  pinout = undoc_indirect_x_c3(&mut self.cpu, bus, pinout),
            0xE304 =>  pinout = undoc_indirect_x_c4(&mut self.cpu, bus, pinout),
            0xE305 => pinout = undoc_indirect_x_c5::<B, Isc>( &mut self.cpu, bus, pinout),
            0xE306 => pinout = undoc_indirect_x_c6( &mut self.cpu, bus, pinout),
            0xE307 =>  pinout = undoc_indirect_x_c7(&mut self.cpu, bus, pinout),
            // Isc undocumented indirect y modify
            0xF300 =>  pinout = undoc_indirect_y_c0(&mut self.cpu, bus, pinout),
            0xF301 =>  pinout = undoc_indirect_y_c1(&mut self.cpu, bus, pinout),
            0xF302 =>  pinout = undoc_indirect_y_c2(&mut self.cpu, bus, pinout),
            0xF303 =>  pinout = undoc_indirect_y_c3(&mut self.cpu, bus, pinout),
            0xF304 =>  pinout = undoc_indirect_y_c4(&mut self.cpu, bus, pinout),
            0xF305 => pinout = undoc_indirect_y_c5::<B, Isc>( &mut self.cpu, bus, pinout),
            0xF306 => pinout = undoc_indirect_y_c6( &mut self.cpu, bus, pinout),
            0xF307 =>  pinout = undoc_indirect_y_c7(&mut self.cpu, bus, pinout),
            // Kil single byte
            0x0200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x0201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0x1200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x1201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0x2200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x2201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0x3200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x3201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0x4200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x4201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0x5200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x5201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0x6200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x6201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0x7200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x7201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0x9200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x9201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0xB200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xB201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0xD200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xD201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Kil single byte
            0xF200 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xF201 =>  pinout = single_byte_c1::<B, Kil>(&mut self.cpu, bus, pinout),
            // Lar absolute y read
            0xBB00 =>  pinout = absolute_y_read_c0(&mut self.cpu, bus, pinout),
            0xBB01 =>  pinout = absolute_y_read_c1(&mut self.cpu, bus, pinout),
            0xBB02 =>  pinout = absolute_y_read_c2(&mut self.cpu, bus, pinout),
            0xBB03 =>  pinout = absolute_y_read_c3(&mut self.cpu, bus, pinout),
            0xBB04 =>  pinout = absolute_y_read_c4::<B, Lar>(&mut self.cpu, bus, pinout),
            // Lax zero page read
            0xA700 =>  pinout = zeropage_read_c0(&mut self.cpu, bus, pinout),
            0xA701 =>  pinout = zeropage_read_c1(&mut self.cpu, bus, pinout),
            0xA702 =>  pinout = zeropage_read_c2::<B, Lax>(&mut self.cpu, bus, pinout),
            // Lax zero page y read
            0xB700 =>  pinout = zeropage_y_read_c0(&mut self.cpu, bus, pinout),
            0xB701 =>  pinout = zeropage_y_read_c1(&mut self.cpu, bus, pinout),
            0xB702 =>  pinout = zeropage_y_read_c2(&mut self.cpu, bus, pinout),
            0xB703 =>  pinout = zeropage_y_read_c3::<B, Lax>(&mut self.cpu, bus, pinout),
            // Lax absolute read
            0xAF00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0xAF01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0xAF02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0xAF03 =>  pinout = absolute_read_c3::<B, Lax>(&mut self.cpu, bus, pinout),
            // Lax absolute y read
            0xBF00 =>  pinout = absolute_y_read_c0(&mut self.cpu, bus, pinout),
            0xBF01 =>  pinout = absolute_y_read_c1(&mut self.cpu, bus, pinout),
            0xBF02 =>  pinout = absolute_y_read_c2(&mut self.cpu, bus, pinout),
            0xBF03 =>  pinout = absolute_y_read_c3(&mut self.cpu, bus, pinout),
            0xBF04 =>  pinout = absolute_y_read_c4::<B, Lax>(&mut self.cpu, bus, pinout),
            // Lax indirect x read
            0xA300 =>  pinout = indirect_x_read_c0(&mut self.cpu, bus, pinout),
            0xA301 =>  pinout = indirect_x_read_c1(&mut self.cpu, bus, pinout),
            0xA302 =>  pinout = indirect_x_read_c2(&mut self.cpu, bus, pinout),
            0xA303 =>  pinout = indirect_x_read_c3(&mut self.cpu, bus, pinout),
            0xA304 =>  pinout = indirect_x_read_c4(&mut self.cpu, bus, pinout),
            0xA305 =>  pinout = indirect_x_read_c5::<B, Lax>(&mut self.cpu, bus, pinout),
            // Lax indirect y read
            0xB300 =>  pinout = indirect_y_read_c0(&mut self.cpu, bus, pinout),
            0xB301 =>  pinout = indirect_y_read_c1(&mut self.cpu, bus, pinout),
            0xB302 =>  pinout = indirect_y_read_c2(&mut self.cpu, bus, pinout),
            0xB303 =>  pinout = indirect_y_read_c3(&mut self.cpu, bus, pinout),
            0xB304 =>  pinout = indirect_y_read_c4(&mut self.cpu, bus, pinout),
            0xB305 =>  pinout = indirect_y_read_c5::<B, Lax>(&mut self.cpu, bus, pinout),
            // Nop single byte
            0x1A00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x1A01 =>  pinout = single_byte_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop single byte
            0x3A00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x3A01 =>  pinout = single_byte_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop single byte
            0x5A00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x5A01 =>  pinout = single_byte_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop single byte
            0x7A00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0x7A01 =>  pinout = single_byte_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop single byte
            0xDA00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xDA01 =>  pinout = single_byte_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop single byte
            0xFA00 =>  pinout = single_byte_c0(&mut self.cpu, bus, pinout),
            0xFA01 =>  pinout = single_byte_c1::<B, Nop>(&mut self.cpu, bus, pinout),
            // Rla zero page modify
            0x2700 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0x2701 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0x2702 => pinout = zeropage_modify_c2::<B, Rla>(&mut self.cpu, bus, pinout),
            0x2703 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0x2704 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Rla zero page x modify
            0x3700 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0x3701 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0x3702 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0x3703 => pinout = zeropage_x_modify_c3::<B, Rla>(&mut self.cpu, bus, pinout),
            0x3704 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0x3705 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Rla absolute modify
            0x2F00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0x2F01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0x2F02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0x2F03 => pinout = absolute_modify_c3::<B, Rla>(&mut self.cpu, bus, pinout),
            0x2F04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0x2F05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Rla absolute x modify
            0x3F00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0x3F01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0x3F02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0x3F03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0x3F04 => pinout = absolute_x_modify_c4::<B, Rla>(&mut self.cpu, bus, pinout),
            0x3F05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0x3F06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // Rla undocumented absolute y modify
            0x3B00 =>  pinout = undoc_absolute_y_c0(&mut self.cpu, bus, pinout),
            0x3B01 =>  pinout = undoc_absolute_y_c1(&mut self.cpu, bus, pinout),
            0x3B02 =>  pinout = undoc_absolute_y_c2(&mut self.cpu, bus, pinout),
            0x3B03 =>  pinout = undoc_absolute_y_c3(&mut self.cpu, bus, pinout),
            0x3B04 => pinout = undoc_absolute_y_c4::<B, Rla>( &mut self.cpu, bus, pinout),
            0x3B05 => pinout = undoc_absolute_y_c5( &mut self.cpu, bus, pinout),
            0x3B06 =>  pinout = undoc_absolute_y_c6(&mut self.cpu, bus, pinout),
            // Rla undocumented indirect x modify
            0x2300 =>  pinout = undoc_indirect_x_c0(&mut self.cpu, bus, pinout),
            0x2301 =>  pinout = undoc_indirect_x_c1(&mut self.cpu, bus, pinout),
            0x2302 =>  pinout = undoc_indirect_x_c2(&mut self.cpu, bus, pinout),
            0x2303 =>  pinout = undoc_indirect_x_c3(&mut self.cpu, bus, pinout),
            0x2304 =>  pinout = undoc_indirect_x_c4(&mut self.cpu, bus, pinout),
            0x2305 => pinout = undoc_indirect_x_c5::<B, Rla>( &mut self.cpu, bus, pinout),
            0x2306 => pinout = undoc_indirect_x_c6( &mut self.cpu, bus, pinout),
            0x2307 =>  pinout = undoc_indirect_x_c7(&mut self.cpu, bus, pinout),
            // Rla undocumented indirect y modify
            0x3300 =>  pinout = undoc_indirect_y_c0(&mut self.cpu, bus, pinout),
            0x3301 =>  pinout = undoc_indirect_y_c1(&mut self.cpu, bus, pinout),
            0x3302 =>  pinout = undoc_indirect_y_c2(&mut self.cpu, bus, pinout),
            0x3303 =>  pinout = undoc_indirect_y_c3(&mut self.cpu, bus, pinout),
            0x3304 =>  pinout = undoc_indirect_y_c4(&mut self.cpu, bus, pinout),
            0x3305 => pinout = undoc_indirect_y_c5::<B, Rla>( &mut self.cpu, bus, pinout),
            0x3306 => pinout = undoc_indirect_y_c6( &mut self.cpu, bus, pinout),
            0x3307 =>  pinout = undoc_indirect_y_c7(&mut self.cpu, bus, pinout),
            // Rra zero page modify
            0x6700 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0x6701 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0x6702 => pinout = zeropage_modify_c2::<B, Rra>(&mut self.cpu, bus, pinout),
            0x6703 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0x6704 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Rra zero page x modify
            0x7700 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0x7701 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0x7702 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0x7703 => pinout = zeropage_x_modify_c3::<B, Rra>(&mut self.cpu, bus, pinout),
            0x7704 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0x7705 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Rra absolute modify
            0x6F00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0x6F01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0x6F02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0x6F03 => pinout = absolute_modify_c3::<B, Rra>(&mut self.cpu, bus, pinout),
            0x6F04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0x6F05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Rra absolute x modify
            0x7F00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0x7F01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0x7F02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0x7F03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0x7F04 => pinout = absolute_x_modify_c4::<B, Rra>(&mut self.cpu, bus, pinout),
            0x7F05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0x7F06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // Rra undocumented absolute y modify
            0x7B00 =>  pinout = undoc_absolute_y_c0(&mut self.cpu, bus, pinout),
            0x7B01 =>  pinout = undoc_absolute_y_c1(&mut self.cpu, bus, pinout),
            0x7B02 =>  pinout = undoc_absolute_y_c2(&mut self.cpu, bus, pinout),
            0x7B03 =>  pinout = undoc_absolute_y_c3(&mut self.cpu, bus, pinout),
            0x7B04 => pinout = undoc_absolute_y_c4::<B, Rra>( &mut self.cpu, bus, pinout),
            0x7B05 => pinout = undoc_absolute_y_c5( &mut self.cpu, bus, pinout),
            0x7B06 =>  pinout = undoc_absolute_y_c6(&mut self.cpu, bus, pinout),
            // Rra undocumented indirect x modify
            0x6300 =>  pinout = undoc_indirect_x_c0(&mut self.cpu, bus, pinout),
            0x6301 =>  pinout = undoc_indirect_x_c1(&mut self.cpu, bus, pinout),
            0x6302 =>  pinout = undoc_indirect_x_c2(&mut self.cpu, bus, pinout),
            0x6303 =>  pinout = undoc_indirect_x_c3(&mut self.cpu, bus, pinout),
            0x6304 =>  pinout = undoc_indirect_x_c4(&mut self.cpu, bus, pinout),
            0x6305 => pinout = undoc_indirect_x_c5::<B, Rra>( &mut self.cpu, bus, pinout),
            0x6306 => pinout = undoc_indirect_x_c6( &mut self.cpu, bus, pinout),
            0x6307 =>  pinout = undoc_indirect_x_c7(&mut self.cpu, bus, pinout),
            // Rra undocumented indirect y modify
            0x7300 =>  pinout = undoc_indirect_y_c0(&mut self.cpu, bus, pinout),
            0x7301 =>  pinout = undoc_indirect_y_c1(&mut self.cpu, bus, pinout),
            0x7302 =>  pinout = undoc_indirect_y_c2(&mut self.cpu, bus, pinout),
            0x7303 =>  pinout = undoc_indirect_y_c3(&mut self.cpu, bus, pinout),
            0x7304 =>  pinout = undoc_indirect_y_c4(&mut self.cpu, bus, pinout),
            0x7305 => pinout = undoc_indirect_y_c5::<B, Rra>( &mut self.cpu, bus, pinout),
            0x7306 => pinout = undoc_indirect_y_c6( &mut self.cpu, bus, pinout),
            0x7307 =>  pinout = undoc_indirect_y_c7(&mut self.cpu, bus, pinout),
            // SbcNoDec immediate
            0xEB00 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0xEB01 =>  pinout = immediate_read_c1::<B, SbcNoDec>(&mut self.cpu, bus, pinout),
            // Slo zero page modify
            0x0700 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0x0701 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0x0702 => pinout = zeropage_modify_c2::<B, Slo>(&mut self.cpu, bus, pinout),
            0x0703 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0x0704 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Slo zero page x modify
            0x1700 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0x1701 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0x1702 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0x1703 => pinout = zeropage_x_modify_c3::<B, Slo>(&mut self.cpu, bus, pinout),
            0x1704 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0x1705 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Slo absolute modify
            0x0F00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0x0F01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0x0F02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0x0F03 => pinout = absolute_modify_c3::<B, Slo>(&mut self.cpu, bus, pinout),
            0x0F04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0x0F05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Slo absolute x modify
            0x1F00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0x1F01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0x1F02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0x1F03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0x1F04 => pinout = absolute_x_modify_c4::<B, Slo>(&mut self.cpu, bus, pinout),
            0x1F05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0x1F06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // Slo undocumented absolute y modify
            0x1B00 =>  pinout = undoc_absolute_y_c0(&mut self.cpu, bus, pinout),
            0x1B01 =>  pinout = undoc_absolute_y_c1(&mut self.cpu, bus, pinout),
            0x1B02 =>  pinout = undoc_absolute_y_c2(&mut self.cpu, bus, pinout),
            0x1B03 =>  pinout = undoc_absolute_y_c3(&mut self.cpu, bus, pinout),
            0x1B04 => pinout = undoc_absolute_y_c4::<B, Slo>( &mut self.cpu, bus, pinout),
            0x1B05 => pinout = undoc_absolute_y_c5( &mut self.cpu, bus, pinout),
            0x1B06 =>  pinout = undoc_absolute_y_c6(&mut self.cpu, bus, pinout),
            // Slo undocumented indirect x modify
            0x0300 =>  pinout = undoc_indirect_x_c0(&mut self.cpu, bus, pinout),
            0x0301 =>  pinout = undoc_indirect_x_c1(&mut self.cpu, bus, pinout),
            0x0302 =>  pinout = undoc_indirect_x_c2(&mut self.cpu, bus, pinout),
            0x0303 =>  pinout = undoc_indirect_x_c3(&mut self.cpu, bus, pinout),
            0x0304 =>  pinout = undoc_indirect_x_c4(&mut self.cpu, bus, pinout),
            0x0305 => pinout = undoc_indirect_x_c5::<B, Slo>( &mut self.cpu, bus, pinout),
            0x0306 => pinout = undoc_indirect_x_c6( &mut self.cpu, bus, pinout),
            0x0307 =>  pinout = undoc_indirect_x_c7(&mut self.cpu, bus, pinout),
            // Slo undocumented indirect y modify
            0x1300 =>  pinout = undoc_indirect_y_c0(&mut self.cpu, bus, pinout),
            0x1301 =>  pinout = undoc_indirect_y_c1(&mut self.cpu, bus, pinout),
            0x1302 =>  pinout = undoc_indirect_y_c2(&mut self.cpu, bus, pinout),
            0x1303 =>  pinout = undoc_indirect_y_c3(&mut self.cpu, bus, pinout),
            0x1304 =>  pinout = undoc_indirect_y_c4(&mut self.cpu, bus, pinout),
            0x1305 => pinout = undoc_indirect_y_c5::<B, Slo>( &mut self.cpu, bus, pinout),
            0x1306 => pinout = undoc_indirect_y_c6( &mut self.cpu, bus, pinout),
            0x1307 =>  pinout = undoc_indirect_y_c7(&mut self.cpu, bus, pinout),
            // Sre zero page modify
            0x4700 =>  pinout = zeropage_modify_c0(&mut self.cpu, bus, pinout),
            0x4701 =>  pinout = zeropage_modify_c1(&mut self.cpu, bus, pinout),
            0x4702 => pinout = zeropage_modify_c2::<B, Sre>(&mut self.cpu, bus, pinout),
            0x4703 => pinout = zeropage_modify_c3(&mut self.cpu, bus, pinout),
            0x4704 =>  pinout = zeropage_modify_c4(&mut self.cpu, bus, pinout),
            // Sre zero page x modify
            0x5700 =>  pinout = zeropage_x_modify_c0(&mut self.cpu, bus, pinout),
            0x5701 =>  pinout = zeropage_x_modify_c1(&mut self.cpu, bus, pinout),
            0x5702 =>  pinout = zeropage_x_modify_c2(&mut self.cpu, bus, pinout),
            0x5703 => pinout = zeropage_x_modify_c3::<B, Sre>(&mut self.cpu, bus, pinout),
            0x5704 => pinout = zeropage_x_modify_c4(&mut self.cpu, bus, pinout),
            0x5705 =>  pinout = zeropage_x_modify_c5(&mut self.cpu, bus, pinout),
            // Sre absolute modify
            0x4F00 =>  pinout = absolute_modify_c0(&mut self.cpu, bus, pinout),
            0x4F01 =>  pinout = absolute_modify_c1(&mut self.cpu, bus, pinout),
            0x4F02 =>  pinout = absolute_modify_c2(&mut self.cpu, bus, pinout),
            0x4F03 => pinout = absolute_modify_c3::<B, Sre>(&mut self.cpu, bus, pinout),
            0x4F04 => pinout = absolute_modify_c4(&mut self.cpu, bus, pinout),
            0x4F05 =>  pinout = absolute_modify_c5(&mut self.cpu, bus, pinout),
            // Sre absolute x modify
            0x5F00 =>  pinout = absolute_x_modify_c0(&mut self.cpu, bus, pinout),
            0x5F01 =>  pinout = absolute_x_modify_c1(&mut self.cpu, bus, pinout),
            0x5F02 =>  pinout = absolute_x_modify_c2(&mut self.cpu, bus, pinout),
            0x5F03 =>  pinout = absolute_x_modify_c3(&mut self.cpu, bus, pinout),
            0x5F04 => pinout = absolute_x_modify_c4::<B, Sre>(&mut self.cpu, bus, pinout),
            0x5F05 => pinout = absolute_x_modify_c5(&mut self.cpu, bus, pinout),
            0x5F06 =>  pinout = absolute_x_modify_c6(&mut self.cpu, bus, pinout),
            // Sre undocumented absolute y modify
            0x5B00 =>  pinout = undoc_absolute_y_c0(&mut self.cpu, bus, pinout),
            0x5B01 =>  pinout = undoc_absolute_y_c1(&mut self.cpu, bus, pinout),
            0x5B02 =>  pinout = undoc_absolute_y_c2(&mut self.cpu, bus, pinout),
            0x5B03 =>  pinout = undoc_absolute_y_c3(&mut self.cpu, bus, pinout),
            0x5B04 => pinout = undoc_absolute_y_c4::<B, Sre>( &mut self.cpu, bus, pinout),
            0x5B05 => pinout = undoc_absolute_y_c5( &mut self.cpu, bus, pinout),
            0x5B06 =>  pinout = undoc_absolute_y_c6(&mut self.cpu, bus, pinout),
            // Sre undocumented indirect x modify
            0x4300 =>  pinout = undoc_indirect_x_c0(&mut self.cpu, bus, pinout),
            0x4301 =>  pinout = undoc_indirect_x_c1(&mut self.cpu, bus, pinout),
            0x4302 =>  pinout = undoc_indirect_x_c2(&mut self.cpu, bus, pinout),
            0x4303 =>  pinout = undoc_indirect_x_c3(&mut self.cpu, bus, pinout),
            0x4304 =>  pinout = undoc_indirect_x_c4(&mut self.cpu, bus, pinout),
            0x4305 => pinout = undoc_indirect_x_c5::<B, Sre>( &mut self.cpu, bus, pinout),
            0x4306 => pinout = undoc_indirect_x_c6( &mut self.cpu, bus, pinout),
            0x4307 =>  pinout = undoc_indirect_x_c7(&mut self.cpu, bus, pinout),
            // Sre undocumented indirect y modify
            0x5300 =>  pinout = undoc_indirect_y_c0(&mut self.cpu, bus, pinout),
            0x5301 =>  pinout = undoc_indirect_y_c1(&mut self.cpu, bus, pinout),
            0x5302 =>  pinout = undoc_indirect_y_c2(&mut self.cpu, bus, pinout),
            0x5303 =>  pinout = undoc_indirect_y_c3(&mut self.cpu, bus, pinout),
            0x5304 =>  pinout = undoc_indirect_y_c4(&mut self.cpu, bus, pinout),
            0x5305 => pinout = undoc_indirect_y_c5::<B, Sre>( &mut self.cpu, bus, pinout),
            0x5306 => pinout = undoc_indirect_y_c6( &mut self.cpu, bus, pinout),
            0x5307 =>  pinout = undoc_indirect_y_c7(&mut self.cpu, bus, pinout),
            // Sxa absolute y store
            0x9E00 =>  pinout = absolute_y_store_c0(&mut self.cpu, bus, pinout),
            0x9E01 =>  pinout = absolute_y_store_c1(&mut self.cpu, bus, pinout),
            0x9E02 =>  pinout = absolute_y_store_c2(&mut self.cpu, bus, pinout),
            0x9E03 => pinout = absolute_y_store_c3::<B, Sxa>(&mut self.cpu, bus, pinout),
            0x9E04 =>  pinout = absolute_y_store_c4(&mut self.cpu, bus, pinout),
            // Sya absolute x store
            0x9C00 =>  pinout = absolute_x_store_c0(&mut self.cpu, bus, pinout),
            0x9C01 =>  pinout = absolute_x_store_c1(&mut self.cpu, bus, pinout),
            0x9C02 =>  pinout = absolute_x_store_c2(&mut self.cpu, bus, pinout),
            0x9C03 => pinout = absolute_x_store_c3::<B, Sya>(&mut self.cpu, bus, pinout),
            0x9C04 =>  pinout = absolute_x_store_c4(&mut self.cpu, bus, pinout),
            // Nop absolute read
            0x0C00 =>  pinout = absolute_read_c0(&mut self.cpu, bus, pinout),
            0x0C01 =>  pinout = absolute_read_c1(&mut self.cpu, bus, pinout),
            0x0C02 =>  pinout = absolute_read_c2(&mut self.cpu, bus, pinout),
            0x0C03 =>  pinout = absolute_read_c3::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop absolute x read
            0x1C00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0x1C01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0x1C02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0x1C03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0x1C04 =>  pinout = absolute_x_read_c4::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop absolute x read
            0x3C00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0x3C01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0x3C02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0x3C03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0x3C04 =>  pinout = absolute_x_read_c4::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop absolute x read
            0x5C00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0x5C01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0x5C02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0x5C03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0x5C04 =>  pinout = absolute_x_read_c4::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop absolute x read
            0x7C00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0x7C01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0x7C02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0x7C03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0x7C04 =>  pinout = absolute_x_read_c4::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop absolute x read
            0xDC00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0xDC01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0xDC02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0xDC03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0xDC04 =>  pinout = absolute_x_read_c4::<B, Nop>(&mut self.cpu, bus, pinout),
            // Nop absolute x read
            0xFC00 =>  pinout = absolute_x_read_c0(&mut self.cpu, bus, pinout),
            0xFC01 =>  pinout = absolute_x_read_c1(&mut self.cpu, bus, pinout),
            0xFC02 =>  pinout = absolute_x_read_c2(&mut self.cpu, bus, pinout),
            0xFC03 =>  pinout = absolute_x_read_c3(&mut self.cpu, bus, pinout),
            0xFC04 =>  pinout = absolute_x_read_c4::<B, Nop>(&mut self.cpu, bus, pinout),
            // Xaa immediate
            0x8B00 =>  pinout = immediate_read_c0(&mut self.cpu, bus, pinout),
            0x8B01 =>  pinout = immediate_read_c1::<B, Xaa>(&mut self.cpu, bus, pinout),
            // Xas absolute y store
            0x9B00 =>  pinout = absolute_y_store_c0(&mut self.cpu, bus, pinout),
            0x9B01 =>  pinout = absolute_y_store_c1(&mut self.cpu, bus, pinout),
            0x9B02 =>  pinout = absolute_y_store_c2(&mut self.cpu, bus, pinout),
            0x9B03 => pinout = absolute_y_store_c3::<B, Xas>(&mut self.cpu, bus, pinout),
            0x9B04 =>  pinout = absolute_y_store_c4(&mut self.cpu, bus, pinout),
            _ => panic!("{}: is an invalid opcode", u16::from(self.cpu.ir)),
        }

        // "pull up" input pins. these must be asserted every cycle they wish to remain active
        pinout.ctrl.set(Ctrl::NMI, true);
        pinout.ctrl.set(Ctrl::IRQ, true);
        pinout.ctrl.set(Ctrl::RDY, true);
        pinout.ctrl.set(Ctrl::HALT, true);

        self.cpu.cycle += 1;
        pinout
    }

    pub fn cycle_count(&self) -> u64 {
        self.cpu.cycle
    }

    pub fn reset(&mut self) {
        self.cpu = Context::new();

        self.cpu.ir.opcode = 0x00;
        self.cpu.ir.tm = 0x10;
    }

    fn mnemonic_lookup(&self) -> &str {
        match self.cpu.ir.opcode {
            0x00 => {
                match self.cpu.ints {
                    InterruptState::None if self.cpu.ir.tm > 0x9 => "RST",
                    InterruptState::None => "BRK",
                    InterruptState::BrkHijack => "BRK Hijacked",
                    InterruptState::IrqHijack => "IRQ Hijacked",
                    InterruptState::Irq => "IRQ",
                    InterruptState::Nmi => "NMI",
                }
            }
            // AdcNoDec
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => "Adc",
            // AND
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => "AND",
            // ASL
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => "ASL",
            // BCC
            0x90 => "BCC",
            // BCS
            0xB0 => "BCS",
            // BEQ
            0xF0 => "BEQ",
            // BIT
            0x24 => "BIT",
            0x2C => "BIT",
            // BMI
            0x30 => "BMI",
            // BNE
            0xD0 => "BNE",
            // BPL
            0x10 => "BPL",
            // BVC
            0x50 => "BVC",
            // BVS
            0x70 => "BVS",
            // CLC
            0x18 => "CLC",
            // CLD
            0xD8 => "CLD",
            // CLI
            0x58 => "CLI",
            // CLV
            0xB8 => "CLV",
            // CMP
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => "CMP",
            // CPX
            0xE0 | 0xE4 | 0xEC => "CPX",
            // CPY
            0xC0 | 0xC4 | 0xCC => "CPY",
            // DEC
            0xC6 | 0xD6 | 0xCE | 0xDE => "DEC",
            // DEX
            0xCA => "DEX",
            // DEY
            0x88 => "DEY",
            // EOR
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => "EOR",
            // INC
            0xE6 | 0xF6 | 0xEE | 0xFE => "INC",
            // INX
            0xE8 => "INX",
            // INY
            0xC8 => "INY",
            // JMP
            0x4C => "JMP",
            0x6C => "JMP",
            // JSR
            0x20 => "JSR",
            // LDA
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => "LDA",
            // LDX
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => "LDX",
            // LDY
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => "LDY",
            // LSR
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => "LSR",
            // NOP
            0xEA => "NOP",
            // ORA
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => "ORA",
            // PHA
            0x48 => "PHA",
            // PHP
            0x08 => "PHP",
            // PLA
            0x68 => "PLA",
            // PLP
            0x28 => "PLP",
            // ROL
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => "ROL",
            // ROR
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E => "ROR",
            // RTI
            0x40 => "RTI",
            // RTS
            0x60 => "RTS",
            // SBC
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => "SBC",
            // SEC
            0x38 => "SEC",
            // SED
            0xF8 => "SED",
            // SEI
            0x78 => "SEI",
            // STA
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => "STA",
            // STX
            0x86 | 0x96 | 0x8E => "STX",
            // STY
            0x84 | 0x94 | 0x8C => "STY",
            // TAX
            0xAA => "TAX",
            // TAY
            0xA8 => "TAY",
            // TSX
            0xBA => "TSX",
            // TXA
            0x8A => "TXA",
            // TXS
            0x9A => "TXS",
            // TYA 
            0x98 => "TYA",
            // Undocumented opcodes
            // AAC
            0x0B | 0x2B => "*AAC*",
            // AAX
            0x87 | 0x97 | 0x83 | 0x8F => "*AAX*",
            // ARR
            0x6B => "*ARR*",
            // ASR
            0x4B => "*ASR*",
            // ATX
            0xAB => "*ATX*",
            // AXA
            0x9F | 0x93 => "*AXA*",
            // AXS
            0xCB => "*AXS*",
            // DCP
            0xC7 | 0xD7 | 0xCF | 0xDF | 0xDB | 0xC3 | 0xD3 => "*DCP*",
            // DOP - double NOP
            0x04 | 0x14 | 0x34 | 0x44 | 0x54 | 0x64 | 0x74 | 0x80 | 0x82 | 0x89 | 0xC2 | 0xD4 | 0xE2 | 0xF4 => "*DOP*",
            //ISC
            0xE7 | 0xF7 | 0xEF | 0xFF | 0xFB | 0xE3 | 0xF3 => "*ISC*",
            // KIL
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => "*KIL*",
            // LAR
            0xBB => "*LAR*",
            // LAX
            0xA7 | 0xB7 | 0xAF | 0xBF | 0xA3 | 0xB3 => "*LAX*",
            // NOP
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => "*NOP*",
            // RLA
            0x27 | 0x37 | 0x2F | 0x3F | 0x3B | 0x23 | 0x33 => "*RLA*",
            // RRA
            0x67 | 0x77 | 0x6F | 0x7F | 0x7B | 0x63 | 0x73 => "*RRA*",
            // SBC
            0xEB => "*SBC*",
            // SLO
            0x07 | 0x17 | 0x0F | 0x1F | 0x1B | 0x03 | 0x13 => "*SLO*",
            // SRE
            0x47 | 0x57 | 0x4F | 0x5F | 0x5B | 0x43 | 0x53 => "*SRE*",
            // SXA
            0x9E => "*SXA*",
            // SYA
            0x9C => "*SYA*",
            // TOP
            0x0C | 0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => "*TOP*",
            // XAA
            0x8B => "*XAA*",
            // XAS
            0x9B => "XAS",
        }
    }
}

impl fmt::Display for Rp2a03 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#X}  IR:{:#X} TM:{:#X} SYNC:{} {} - A:{:#X} X:{:#X} Y:{:#X} P:{:#X} SP:{:#X} CYC: {}",
        u16::from(self.cpu.pc), self.cpu.ir.opcode, self.cpu.ir.tm, self.cpu.first_cycle, self.mnemonic_lookup(), self.cpu.a,
        self.cpu.x, self.cpu.y, u8::from(self.cpu.p), self.cpu.sp, self.cpu.cycle)
    }
}