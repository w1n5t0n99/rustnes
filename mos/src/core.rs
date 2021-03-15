
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InterruptState {
    None,
    BrkHijack,
    IrqHijack,
    Irq,
    Nmi,
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
    pub ints: InterruptState,
    pub ir: InstructionRegister,
    pub p: StatusRegister,
    pub pc: ProgramCounter,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,    
    pub nmi_detected: bool,
    pub first_cycle: bool,
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
            ints: InterruptState::None,
            nmi_detected: false,
            first_cycle: false,
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
        self.ints = InterruptState::None;
        self.nmi_detected = false;
    }
}

