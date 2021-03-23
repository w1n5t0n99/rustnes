pub const RST_VEC_LOW: u8 = 0xFC;
pub const NMI_VEC_LOW: u8 = 0xFA;
pub const IRQ_BRK_VEC_LOW: u8 = 0xFE;

const RST_TM: u8 = 0x10;
const NMI_TM: u8 = 0x20;
const IRQ_TM: u8 = 0x30;

bitflags! {
    // 7  bit  0
    // ---- ----
    // NVss DIZC
    // |||| ||||
    // |||| |||+- Carry
    // |||| ||+-- Zero
    // |||| |+--- Interrupt Disable
    // |||| +---- Decimal
    // ||++------ No CPU effect, see: the B flag
    // |+-------- Overflow
    // +--------- Negative
    pub struct StatusRegister: u8 {
        const CARRY              = 0b00000001; 
        const ZERO               = 0b00000010;
        const INT_DISABLE        = 0b00000100;
        const DECIMAL            = 0b00001000;
        const OVERFLOW           = 0b01000000;
        const NEGATIVE           = 0b10000000;
    }
}

impl StatusRegister {
    pub fn from_power_on() -> StatusRegister {
        // IRQ disabled
        StatusRegister::INT_DISABLE
    }

    pub fn push_with_b(&mut self) -> u8 {
        self.bits() | 0b00110000
    }

    pub fn push_without_b(&mut self) -> u8 {
        self.bits() | 0b00100000
    }
}

/*
internally the 16bit program counter was 2 8bit registers, requiring 
two cycles to set both the high and low address bytes
*/
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ProgramCounter {
    pub pcl: u8,
    pub pch: u8,
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter {
            pch: 0,
            pcl: 0,
        }
    }

    #[inline]
    pub fn increment(&mut self) {
        let mut pc: u16 = u16::from(*self);
        pc += 1;
        *self = ProgramCounter::from(pc);
    }
}

impl std::convert::From<ProgramCounter> for u16 {
    fn from(pc: ProgramCounter) -> u16 {
        (pc.pch as u16) << 8 | (pc.pcl as u16) 
    }
}

impl std::convert::From<u16> for ProgramCounter {
    fn from(b: u16) -> ProgramCounter {
        ProgramCounter {
            pcl: b as u8,
            pch: (b >> 8) as u8,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct InstructionRegister {
    pub opcode: u8,
    pub tm: u8,
}

impl InstructionRegister {
    pub fn new() -> InstructionRegister {
        InstructionRegister {
            opcode: 0,
            tm: 0,
        }
    }

    #[inline]
    pub fn reset(&mut self, opcode: u8) {
        self.opcode = opcode;
        self.tm = 0;
    }

    #[inline]
    pub fn reset_to_rst(&mut self) {
        self.opcode = 0x00;
        self.tm = RST_TM;
    }

    #[inline]
    pub fn reset_to_nmi(&mut self) {
        self.opcode = 0x00;
        self.tm = NMI_TM;
    }

    #[inline]
    pub fn reset_to_irq(&mut self) {
        self.opcode = 0x00;
        self.tm = IRQ_TM;
    }

    #[inline]
    pub fn increment(&mut self) {
        self.tm = self.tm.wrapping_add(1);
    }
}

impl std::convert::From<InstructionRegister> for u16 {
    fn from(ir: InstructionRegister) -> u16 {
        (ir.opcode as u16) << 8 | (ir.tm as u16) 
    }
}

impl std::convert::From<u16> for InstructionRegister {
    fn from(b: u16) -> InstructionRegister {
        InstructionRegister {
            tm: b as u8,
            opcode: (b >> 8) as u8,
        }
    }
}

/*
Holds intermediate data during cycle operations, the 6502 would have used internal registers
or the ALU to hold this data
 */
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct OpState {
    //base address - address in index addressing modes that specifies index location
    pub bal: u8,
    pub bah: u8,
    // effective address - destination in memory where data is found
    pub adl: u8,
    pub adh: u8,
    // indirect address - address found in operand of instruction using (Indirect),Y
    pub ial: u8,
    pub iah: u8,
    // offset address
    pub offset: u8,
    pub offset_carry: bool,
    pub offset_neg: bool,
    pub branch_taken: bool,
    // data input latch
    pub dl: u8,
}

impl OpState {
    pub fn new() -> OpState {
        OpState {
            bal: 0,
            bah: 0,
            adl: 0,
            adh: 0,
            ial: 0,
            iah: 0,
            offset: 0,
            offset_carry: false,
            offset_neg: false,
            branch_taken: false,
            dl: 0,
        }
    }

    pub fn reset(&mut self) {
        self.bal = 0;
        self.bah = 0;
        self.adl = 0;
        self.adh = 0;
        self.ial = 0;
        self.iah = 0;
        self.offset = 0;
        self.offset_carry = false;
        self.offset_neg = false;
        self.branch_taken = false;
        self.dl = 0;
    }
}

//internal state of cpu
#[derive(Debug, Clone)]
pub struct Context
{
    pub cycle: u64,
    pub ops: OpState,
    pub ir: InstructionRegister,
    pub p: StatusRegister,
    pub pc: ProgramCounter,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,   
    pub int_vec_low: u8, 
    pub nmi_detected: bool,
}

impl Context
{
    pub fn new() -> Context
    {
        Context
        {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            cycle: 0,
            ir: InstructionRegister::new(),
            p: StatusRegister::from_power_on(),
            pc: ProgramCounter::new(),
            ops: OpState::new(),
            int_vec_low: IRQ_BRK_VEC_LOW,
            nmi_detected: false,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0;
        self.cycle = 0;
        self.ir = InstructionRegister::new();
        self.p = self.p | StatusRegister::INT_DISABLE;
        self.ops = OpState::new();
        self.int_vec_low = IRQ_BRK_VEC_LOW;
        self.nmi_detected = false;
    }
}

pub fn opcode_to_mnemonic(opcode: u8, tm: u8) -> &'static str {
    match opcode {
        0x00 => {
            match tm {
                0x10..=0x18 => "RST",
                0x00..=0x06 => "BRK",
                0x30..=0x36 => "IRQ",
                0x20..=0x26 => "NMI",
                _ => panic!("interrupt timing out of bounds"),
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

