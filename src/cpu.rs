use std::ptr::null_mut;

use instruction::Instruction;
use rv64ua::*;
use rv64ud::*;
use rv64uf::*;
use rv64ui::*;
use rv64um::*;
use std::fmt::{Debug, Formatter};
use std::fmt;

pub mod instruction;
mod rv64ui;
mod rv64um;
mod rv64ua;
mod rv64uf;
mod rv64ud;

const CSR_CAPACITY: usize = 4096;
const _CSR_USTATUS_ADDRESS: u16 = 0x000;
const CSR_FFLAGS_ADDRESS: u16 = 0x001;
const CSR_FRM_ADDRESS: u16 = 0x002;
const CSR_FCSR_ADDRESS: u16 = 0x003;
const _CSR_UIE_ADDRESS: u16 = 0x004;
const _CSR_UTVEC_ADDRESS: u16 = 0x005;
const _CSR_USCRATCH_ADDRESS: u16 = 0x040;
const _CSR_UEPC_ADDRESS: u16 = 0x041;
const _CSR_UCAUSE_ADDRESS: u16 = 0x042;
const _CSR_UTVAL_ADDRESS: u16 = 0x043;
const _CSR_UIP_ADDRESS: u16 = 0x044;
const CSR_SSTATUS_ADDRESS: u16 = 0x100;
const _CSR_SEDELEG_ADDRESS: u16 = 0x102;
const _SR_SIDELEG_ADDRESS: u16 = 0x103;
const CSR_SIE_ADDRESS: u16 = 0x104;
const _CSR_STVEC_ADDRESS: u16 = 0x105;
const _CSR_SSCRATCH_ADDRESS: u16 = 0x140;
const _CSR_SEPC_ADDRESS: u16 = 0x141;
const _CSR_SCAUSE_ADDRESS: u16 = 0x142;
const _CSR_STVAL_ADDRESS: u16 = 0x143;
const CSR_SIP_ADDRESS: u16 = 0x144;
const _CSR_SATP_ADDRESS: u16 = 0x180;
const CSR_MSTATUS_ADDRESS: u16 = 0x300;
const _CSR_MISA_ADDRESS: u16 = 0x301;
const _CSR_MEDELEG_ADDRESS: u16 = 0x302;
const CSR_MIDELEG_ADDRESS: u16 = 0x303;
const CSR_MIE_ADDRESS: u16 = 0x304;

const _CSR_MTVEC_ADDRESS: u16 = 0x305;
const _CSR_MSCRATCH_ADDRESS: u16 = 0x340;
const CSR_MEPC_ADDRESS: u16 = 0x341;
const _CSR_MCAUSE_ADDRESS: u16 = 0x342;
const _CSR_MTVAL_ADDRESS: u16 = 0x343;
const CSR_MIP_ADDRESS: u16 = 0x344;
const _CSR_PMPCFG0_ADDRESS: u16 = 0x3a0;
const _CSR_PMPADDR0_ADDRESS: u16 = 0x3b0;
const _CSR_MCYCLE_ADDRESS: u16 = 0xb00;
const _CSR_CYCLE_ADDRESS: u16 = 0xc00;
const CSR_TIME_ADDRESS: u16 = 0xc01;
const _CSR_INSERT_ADDRESS: u16 = 0xc02;
const _CSR_MHARTID_ADDRESS: u16 = 0xf14;

#[derive(Clone, Debug)]
pub enum Xlen {
    Bit32,
    Bit64
}

#[derive(Debug)]
pub struct Trap {
    pub trap_type: TrapType,
    pub value: u64 // Trap type specific value
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TrapType {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    UserSoftwareInterrupt,
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    UserTimerInterrupt,
    SupervisorTimerInterrupt,
    MachineTimerInterrupt,
    UserExternalInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt,
    Stop
}

/*

Register	ABI Name	Description	Saver
x0	        zero	    hardwired zero	-
x1	        ra	        return address	Caller
x2	        sp	        stack pointer	Callee
x3	        gp	        global pointer	-
x4	        tp	        thread pointer	-
x5-7	    t0-2	    temporary registers	Caller
x8	        s0 / fp	    Callee
x9	        s1	        saved register	Callee
x10-11	    a0-1	    function arguments / return values	Caller
x12-17	    a2-7	    function arguments	Caller
x18-27	    s2-11	    saved registers	Callee
x28-31	    t3-6	    temporary registers	Caller

 */

pub struct Cpu {
    pub pc: *mut u8,
    pub x: [i64; 32],
    pub f: [f64; 32],
    xlen: Xlen,
    pub csr: [u64; CSR_CAPACITY],
    reservation: u64, // @TODO: Should support multiple address reservations
    is_reservation_set: bool,
    ecall_handler: Option<Instruction>
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpu")
            .field("pc", &self.pc)
            .field("fcsr", &self.csr[CSR_FCSR_ADDRESS as usize])
            .field("x", &self.x)
            .field("f", &self.f)
            .finish()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: null_mut(),
            x: [0; 32],
            f: [0.0; 32],
            xlen: Xlen::Bit64,
            csr: [0; CSR_CAPACITY],
            reservation: 0,
            is_reservation_set: false,
            ecall_handler: None
        }
    }

    pub fn fetch(&mut self) -> u32 {
        unsafe {
            let result = *(self.pc as *const u32);
            match result & 3 {
                3 => {
                    self.pc = self.pc.add(4);

                    result
                },
                _ => {
                    self.pc = self.pc.add(2);

                    Cpu::uncompress(result & 0xffff)
                }
            }
        }
    }

    pub fn update_pc(&mut self, new_pc: *mut u32) {
        self.pc = new_pc as *mut u8;
    }

    pub fn set_ecall_handler(&mut self, handler: Option<Instruction>) {
        self.ecall_handler = handler;
    }

    pub fn get_pc(&self) -> usize {
        self.pc as usize
    }

    pub fn tick(&mut self) -> Result<(), Trap> {
        let instruction_address = self.pc;
        self.csr[CSR_TIME_ADDRESS as usize] = self.csr[CSR_TIME_ADDRESS as usize].wrapping_add(1);

        let word = self.fetch();
        if let Some(instruction) = Cpu::decode(word) {
            let result = (instruction.operation)(self, word, instruction_address);
            self.x[0] = 0; // make sure x0 is still zero!

            result
        } else {
            Err(Trap { trap_type: TrapType::IllegalInstruction, value: word as u64 })
        }
    }

    pub fn get_f32(&self, reg: usize) -> f32 {
        // only consider the bottom 32 bits of the register
        f32::from_bits(self.f[reg].to_bits() as u32)
    }

    pub fn set_f32(&mut self, reg: usize, f: f32) {
        // the Risc V spec says that setting the f register with a 32 bit float should
        // set the top 32 bits of the register to 1
        self.f[reg] = f64::from_bits(0xffffffff00000000 | f.to_bits() as u64);
    }

    pub fn decode(word: u32) -> Option<&'static Instruction> {
        match word & 0x7f {
            0b0110111 => Some(&LUI),

            0b0010111 => Some(&AUIPC),

            0b1101111 => Some(&JAL),

            0b1100111 => Some(&JALR),

            0b1100011 => match (word >> 12) & 7 {
                0b000 => Some(&BEQ),
                0b001 => Some(&BNE),
                0b100 => Some(&BLT),
                0b101 => Some(&BGE),
                0b110 => Some(&BLTU),
                0b111 => Some(&BGEU),
                _ => None
            },

            0b0000011 => match (word >> 12) & 7 {
                0b000 => Some(&LB),
                0b001 => Some(&LH),
                0b010 => Some(&LW),
                0b100 => Some(&LBU),
                0b101 => Some(&LHU),
                0b110 => Some(&LWU),
                0b011 => Some(&LD),
                _ => None
            },

            0b0100011 => match (word >> 12) & 7 {
                0b000 => Some(&SB),
                0b001 => Some(&SH),
                0b010 => Some(&SW),
                0b011 => Some(&SD),
                _ => None
            },

            0b0010011 => match (word >> 12) & 7 {
                0b000 => Some(&ADDI),
                0b010 => Some(&SLTI),
                0b011 => Some(&SLTIU),
                0b100 => Some(&XORI),
                0b110 => Some(&ORI),
                0b111 => Some(&ANDI),
                0b001 => match word >> 25 {
                    0b0000000 => Some(&SLLI),
                    0b0000001 => Some(&SLLI),
                    _ => None
                },
                0b101 => match word >> 25 {
                    0b0000000 => Some(&SRLI),
                    0b0100000 => Some(&SRAI),
                    _ => None
                },
                _ => None
            },

            0b0110011 => match (word >> 12) & 7 {
                0b000 => match word >> 25 {
                    0b0000000 => Some(&ADD),
                    0b0000001 => Some(&MUL),
                    0b0100000 => Some(&SUB),
                    _ => None
                },
                0b001 => match word >> 25 {
                    0b0000000 => Some(&SLL),
                    0b0000001 => Some(&MULH),
                    _ => None
                },
                0b010 => match word >> 25 {
                    0b0000000 => Some(&SLT),
                    0b0000001 => Some(&MULHSU),
                    _ => None
                },
                0b011 => match word >> 25 {
                    0b0000000 => Some(&SLTU),
                    0b0000001 => Some(&MULHU),
                    _ => None
                },
                0b100 => match word >> 25 {
                    0b0000000 => Some(&XOR),
                    0b0000001 => Some(&DIV),
                    _ => None
                ,}
                0b111 => match word >> 25 {
                    0b0000000 => Some(&AND),
                    0b0000001 => Some(&REMU),
                    _ => None
                },
                0b101 => match word >> 25 {
                    0b0000000 => Some(&SRL),
                    0b0000001 => Some(&DIVU),
                    0b0100000 => Some(&SRA),
                    _ => None
                },
                0b110 => match word >> 25 {
                    0b0000000 => Some(&OR),
                    0b0000001 => Some(&REM),
                    _ => None
                },
                _ => None
            },

            0b0011011 => match (word >> 12) & 7 {
                0b000 => Some(&ADDIW),
                0b001 => match word >> 25 {
                    0b0000000 =>Some(&SLLIW),
                    _ => None
                },
                0b101 => match word >> 25 {
                    0b0000000 => Some(&SRLIW),
                    0b0100000 => Some(&SRAIW),
                    _ => None
                },
                _ => None
            },

            0b0111011 => match (word >> 12) & 7 {
                0b000 => match word >> 25 {
                    0b0000000 => Some(&ADDW),
                    0b0000001 => Some(&MULW),
                    0b0100000 => Some(&SUBW),
                    _ => None
                },
                0b001 => match word >> 25 {
                    0b0000000 => Some(&SLLW),
                    _ => None
                },
                0b101 => match word >> 25 {
                    0b0000000 => Some(&SRLW),
                    0b0000001 => Some(&DIVUW),
                    0b0100000 => Some(&SRAW),
                    _ => None
                },
                0b100 => match word >> 25 {
                    0b0000001 => Some(&DIVW),
                    _ => None
                },
                0b110 => match word >> 25 {
                    0b0000001 => Some(&REMW),
                    _ => None
                },
                0b111 => match word >> 25 {
                    0b0000001 => Some(&REMUW),
                    _ => None
                },
                _ => None
            },

            0b0000111 => match (word >> 12) & 7 {
                0b010 => Some(&FLW),
                _ => None
            },

            0b0100111 => match (word >> 12) & 7 {
                0b010 => Some(&FSW),
                _ => None
            },

            0b1010011 => match word >> 25 {
                0b0000000 => Some(&FADD_S),
                0b0000100 => Some(&FSUB_S),
                0b0001000 => Some(&FMUL_S),
                0b0001100 => Some(&FDIV_S),
                0b0101100 => match (word >> 20) & 31 {
                    0b00000 => Some(&FSQRT_S),
                    _ => None
                },
                0b0010000 => match (word >> 12) & 3 {
                    0b000 => Some(&FSGNJ_S),
                    0b001 => Some(&FSGNJN_S),
                    0b010 => Some(&FSGNJX_S),
                    _ => None
                },
                0b0010100 => match (word >> 12) & 3 {
                    0b000 => Some(&UNIMPLEMENTED), // FMIN_S
                    0b001 => Some(&UNIMPLEMENTED), // FMAX_S
                    _ => None
                },
                0b1100000 => match (word >> 20) & 31 {
                    0b00000 => Some(&UNIMPLEMENTED), // FCVT_W_S
                    0b00001 => Some(&UNIMPLEMENTED), // FCVT_WU_S
                    0b00010 => Some(&FCVT_L_S),
                    0b00011 => Some(&FCVT_LU_S),
                    _ => None
                },
                0b1110000 => match (word >> 20) & 31 {
                    0b00000 => match (word >> 12) & 3 {
                        0b000 => Some(&FMV_X_W),
                        _ => None
                    },
                    _ => None
                },
                0b1010000 => match (word >> 12) & 3 {
                    0b010 => Some(&FEQ_S),
                    0b001 => Some(&FLT_S),
                    0b000 => Some(&FLE_S),
                    _ => None
                },
                0b1111000 => match (word >> 20) & 31 {
                    0b00000 => match (word >> 12) & 3 {
                        0b000 => Some(&FMV_W_X),
                        0b001 => Some(&UNIMPLEMENTED), // FCLASS_S
                        _ => None
                    },
                    _ => None
                },
                0b1101000 => match (word >> 20) & 31 {
                    0b00000 => Some(&FCVT_S_W),
                    0b00001 => Some(&FCVT_S_WU),
                    0b00010 => Some(&FCVT_S_L),
                    0b00011 => Some(&FCVT_S_LU),
                    _ => None
                }
                _ => None
            },

            0b1000011 => match (word >> 25) & 3 {
                0b00 => Some(&UNIMPLEMENTED), // FMADD_S
                _ => None
            },

            0b1000111 => match (word >> 25) & 3 {
                0b00 => Some(&UNIMPLEMENTED), // FMSUB_S
                _ => None
            },

            0b1001011 => match (word >> 25) & 3 {
                0b00 => Some(&UNIMPLEMENTED), // FNMSUB_S
                _ => None
            },

            0b1001111 => match (word >> 25) & 3 {
                0b00 => Some(&UNIMPLEMENTED), // FNMADD_S
                _ => None
            },

            0b0001111 => match (word >> 12) & 7 {
                0b000 => Some(&FENCE),
                0b001 => Some(&FENCE_I),
                _ => None
            },

            0b0101111 => match (word >> 12) & 7 {
                0b010 => match word >> 27 {
                    0b00010 => match (word >> 20) & 0x1f {
                        0b00000 => Some(&LR_W),
                        _ => None
                    },
                    0b00011 => Some(&SC_W),
                    0b00001 => Some(&AMOSWAP_W),
                    0b00000 => Some(&AMOADD_W),
                    0b00100 => Some(&AMOXOR_W),
                    0b01100 => Some(&AMOAND_W),
                    0b01000 => Some(&AMOOR_W),
                    0b10000 => Some(&AMOMIN_W),
                    0b10100 => Some(&AMOMAX_W),
                    0b11000 => Some(&AMOMINU_W),
                    0b11100 => Some(&AMOMAXU_W),
                    _ => None
                },
                0b011 => match word >> 27 {
                    0b00010 => match (word >> 20) & 0x1f {
                        0b00000 => Some(&LR_D),
                        _ => None
                    },
                    0b00011 => Some(&SC_D),
                    0b00001 => Some(&AMOSWAP_D),
                    0b00000 => Some(&AMOADD_D),
                    0b00100 => Some(&AMOXOR_D),
                    0b01100 => Some(&AMOAND_D),
                    0b01000 => Some(&AMOOR_D),
                    0b10000 => Some(&AMOMIN_D),
                    0b10100 => Some(&AMOMAX_D),
                    0b11000 => Some(&AMOMINU_D),
                    0b11100 => Some(&AMOMAXU_D),
                    _ => None
                },
                _ => None
            },

            0b1110011 => match (word >> 12) & 7 {
                0b000 => match word {
                    0b00000000000000000000000001110011 => Some(&ECALL),
                    0b00000000000100000000000001110011 => Some(&EBREAK),
                    0b00110000001000000000000001110011 => Some(&MRET),
                    _ => None
                },
                0b001 => Some(&CSRRW),
                0b010 => Some(&CSRRS),
                0b011 => Some(&CSRRC),
                0b101 => Some(&CSRRWI),
                0b110 => Some(&CSRRSI),
                0b111 => Some(&CSRRCI),
                _ => None
            },

            _ => None
        }
    }

    fn uncompress(halfword: u32) -> u32 {
        let op = halfword & 0x3; // [1:0]
        let funct3 = (halfword >> 13) & 0x7; // [15:13]

        match op {
            0 => match funct3 {
                0 => {
                    // C.ADDI4SPN
                    // addi rd+8, x2, nzuimm
                    let rd = (halfword >> 2) & 0x7; // [4:2]
                    let nzuimm =
                        ((halfword >> 7) & 0x30) | // nzuimm[5:4] <= [12:11]
                            ((halfword >> 1) & 0x3c0) | // nzuimm{9:6] <= [10:7]
                            ((halfword >> 4) & 0x4) | // nzuimm[2] <= [6]
                            ((halfword >> 2) & 0x8); // nzuimm[3] <= [5]
                    // nzuimm == 0 is reserved instruction
                    if nzuimm != 0 {
                        return (nzuimm << 20) | (2 << 15) | ((rd + 8) << 7) | 0x13;
                    }
                },
                1 => {
                    // @TODO: Support C.LQ for 128-bit
                    // C.FLD for 32, 64-bit
                    // fld rd+8, offset(rs1+8)
                    let rd = (halfword >> 2) & 0x7; // [4:2]
                    let rs1 = (halfword >> 7) & 0x7; // [9:7]
                    let offset =
                        ((halfword >> 7) & 0x38) | // offset[5:3] <= [12:10]
                            ((halfword << 1) & 0xc0); // offset[7:6] <= [6:5]
                    return (offset << 20) | ((rs1 + 8) << 15) | (3 << 12) | ((rd + 8) << 7) | 0x7;
                },
                2 => {
                    // C.LW
                    // lw rd+8, offset(rs1+8)
                    let rs1 = (halfword >> 7) & 0x7; // [9:7]
                    let rd = (halfword >> 2) & 0x7; // [4:2]
                    let offset =
                        ((halfword >> 7) & 0x38) | // offset[5:3] <= [12:10]
                            ((halfword >> 4) & 0x4) | // offset[2] <= [6]
                            ((halfword << 1) & 0x40); // offset[6] <= [5]
                    return (offset << 20) | ((rs1 + 8) << 15) | (2 << 12) | ((rd + 8) << 7) | 0x3;
                },
                3 => {
                    // @TODO: Support C.FLW in 32-bit mode
                    // C.LD in 64-bit mode
                    // ld rd+8, offset(rs1+8)
                    let rs1 = (halfword >> 7) & 0x7; // [9:7]
                    let rd = (halfword >> 2) & 0x7; // [4:2]
                    let offset =
                        ((halfword >> 7) & 0x38) | // offset[5:3] <= [12:10]
                            ((halfword << 1) & 0xc0); // offset[7:6] <= [6:5]
                    return (offset << 20) | ((rs1 + 8) << 15) | (3 << 12) | ((rd + 8) << 7) | 0x3;
                },
                4 => {
                    // Reserved
                },
                5 => {
                    // C.FSD
                    // fsd rs2+8, offset(rs1+8)
                    let rs1 = (halfword >> 7) & 0x7; // [9:7]
                    let rs2 = (halfword >> 2) & 0x7; // [4:2]
                    let offset =
                        ((halfword >> 7) & 0x38) | // uimm[5:3] <= [12:10]
                            ((halfword << 1) & 0xc0); // uimm[7:6] <= [6:5]
                    let imm11_5 = (offset >> 5) & 0x7f;
                    let imm4_0 = offset & 0x1f;
                    return (imm11_5 << 25) | ((rs2 + 8) << 20) | ((rs1 + 8) << 15) | (3 << 12) | (imm4_0 << 7) | 0x27;
                },
                6 => {
                    // C.SW
                    // sw rs2+8, offset(rs1+8)
                    let rs1 = (halfword >> 7) & 0x7; // [9:7]
                    let rs2 = (halfword >> 2) & 0x7; // [4:2]
                    let offset =
                        ((halfword >> 7) & 0x38) | // offset[5:3] <= [12:10]
                            ((halfword << 1) & 0x40) | // offset[6] <= [5]
                            ((halfword >> 4) & 0x4); // offset[2] <= [6]
                    let imm11_5 = (offset >> 5) & 0x7f;
                    let imm4_0 = offset & 0x1f;
                    return (imm11_5 << 25) | ((rs2 + 8) << 20) | ((rs1 + 8) << 15) | (2 << 12) | (imm4_0 << 7) | 0x23;
                },
                7 => {
                    // @TODO: Support C.FSW in 32-bit mode
                    // C.SD
                    // sd rs2+8, offset(rs1+8)
                    let rs1 = (halfword >> 7) & 0x7; // [9:7]
                    let rs2 = (halfword >> 2) & 0x7; // [4:2]
                    let offset =
                        ((halfword >> 7) & 0x38) | // uimm[5:3] <= [12:10]
                            ((halfword << 1) & 0xc0); // uimm[7:6] <= [6:5]
                    let imm11_5 = (offset >> 5) & 0x7f;
                    let imm4_0 = offset & 0x1f;
                    return (imm11_5 << 25) | ((rs2 + 8) << 20) | ((rs1 + 8) << 15) | (3 << 12) | (imm4_0 << 7) | 0x23;
                },
                _ => {} // Not happens
            },
            1 => {
                match funct3 {
                    0 => {
                        let r = (halfword >> 7) & 0x1f; // [11:7]
                        let imm = match halfword & 0x1000 {
                            0x1000 => 0xffffffc0,
                            _ => 0
                        } | // imm[31:6] <= [12]
                            ((halfword >> 7) & 0x20) | // imm[5] <= [12]
                            ((halfword >> 2) & 0x1f); // imm[4:0] <= [6:2]
                        if r == 0 && imm == 0 {
                            // C.NOP
                            // addi x0, x0, 0
                            return 0x13;
                        } else if r != 0 {
                            // C.ADDI
                            // addi r, r, imm
                            return (imm << 20) | (r << 15) | (r << 7) | 0x13;
                        }
                        // @TODO: Support HINTs
                        // r == 0 and imm != 0 is HINTs
                    },
                    1 => {
                        // @TODO: Support C.JAL in 32-bit mode
                        // C.ADDIW
                        // addiw r, r, imm
                        let r = (halfword >> 7) & 0x1f;
                        let imm = match halfword & 0x1000 {
                            0x1000 => 0xffffffc0,
                            _ => 0
                        } | // imm[31:6] <= [12]
                            ((halfword >> 7) & 0x20) | // imm[5] <= [12]
                            ((halfword >> 2) & 0x1f); // imm[4:0] <= [6:2]
                        if r != 0 {
                            return (imm << 20) | (r << 15) | (r << 7) | 0x1b;
                        }
                        // r == 0 is reserved instruction
                    },
                    2 => {
                        // C.LI
                        // addi rd, x0, imm
                        let r = (halfword >> 7) & 0x1f;
                        let imm = match halfword & 0x1000 {
                            0x1000 => 0xffffffc0,
                            _ => 0
                        } | // imm[31:6] <= [12]
                            ((halfword >> 7) & 0x20) | // imm[5] <= [12]
                            ((halfword >> 2) & 0x1f); // imm[4:0] <= [6:2]
                        if r != 0 {
                            return (imm << 20) | (r << 7) | 0x13;
                        }
                        // @TODO: Support HINTs
                        // r == 0 is for HINTs
                    },
                    3 => {
                        let r = (halfword >> 7) & 0x1f; // [11:7]
                        if r == 2 {
                            // C.ADDI16SP
                            // addi r, r, nzimm
                            let imm = match halfword & 0x1000 {
                                0x1000 => 0xfffffc00,
                                _ => 0
                            } | // imm[31:10] <= [12]
                                ((halfword >> 3) & 0x200) | // imm[9] <= [12]
                                ((halfword >> 2) & 0x10) | // imm[4] <= [6]
                                ((halfword << 1) & 0x40) | // imm[6] <= [5]
                                ((halfword << 4) & 0x180) | // imm[8:7] <= [4:3]
                                ((halfword << 3) & 0x20); // imm[5] <= [2]
                            if imm != 0 {
                                return (imm << 20) | (r << 15) | (r << 7) | 0x13;
                            }
                            // imm == 0 is for reserved instruction
                        }
                        if r != 0 && r != 2 {
                            // C.LUI
                            // lui r, nzimm
                            let nzimm = match halfword & 0x1000 {
                                0x1000 => 0xfffc0000,
                                _ => 0
                            } | // nzimm[31:18] <= [12]
                                ((halfword << 5) & 0x20000) | // nzimm[17] <= [12]
                                ((halfword << 10) & 0x1f000); // nzimm[16:12] <= [6:2]
                            if nzimm != 0 {
                                return nzimm | (r << 7) | 0x37;
                            }
                            // nzimm == 0 is for reserved instruction
                        }
                    },
                    4 => {
                        let funct2 = (halfword >> 10) & 0x3; // [11:10]
                        match funct2 {
                            0 => {
                                // C.SRLI
                                // c.srli rs1+8, rs1+8, shamt
                                let shamt =
                                    ((halfword >> 7) & 0x20) | // shamt[5] <= [12]
                                        ((halfword >> 2) & 0x1f); // shamt[4:0] <= [6:2]
                                let rs1 = (halfword >> 7) & 0x7; // [9:7]
                                return (shamt << 20) | ((rs1 + 8) << 15) | (5 << 12) | ((rs1 + 8) << 7) | 0x13;
                            },
                            1 => {
                                // C.SRAI
                                // srai rs1+8, rs1+8, shamt
                                let shamt =
                                    ((halfword >> 7) & 0x20) | // shamt[5] <= [12]
                                        ((halfword >> 2) & 0x1f); // shamt[4:0] <= [6:2]
                                let rs1 = (halfword >> 7) & 0x7; // [9:7]
                                return (0x20 << 25) | (shamt << 20) | ((rs1 + 8) << 15) | (5 << 12) | ((rs1 + 8) << 7) | 0x13;
                            },
                            2 => {
                                // C.ANDI
                                // andi, r+8, r+8, imm
                                let r = (halfword >> 7) & 0x7; // [9:7]
                                let imm = match halfword & 0x1000 {
                                    0x1000 => 0xffffffc0,
                                    _ => 0
                                } | // imm[31:6] <= [12]
                                    ((halfword >> 7) & 0x20) | // imm[5] <= [12]
                                    ((halfword >> 2) & 0x1f); // imm[4:0] <= [6:2]
                                return (imm << 20) | ((r + 8) << 15) | (7 << 12) | ((r + 8) << 7) | 0x13;
                            },
                            3 => {
                                let funct1 = (halfword >> 12) & 1; // [12]
                                let funct2_2 = (halfword >> 5) & 0x3; // [6:5]
                                let rs1 = (halfword >> 7) & 0x7;
                                let rs2 = (halfword >> 2) & 0x7;
                                match funct1 {
                                    0 => match funct2_2 {
                                        0 => {
                                            // C.SUB
                                            // sub rs1+8, rs1+8, rs2+8
                                            return (0x20 << 25) | ((rs2 + 8) << 20) | ((rs1 + 8) << 15) | ((rs1 + 8) << 7) | 0x33;
                                        },
                                        1 => {
                                            // C.XOR
                                            // xor rs1+8, rs1+8, rs2+8
                                            return ((rs2 + 8) << 20) | ((rs1 + 8) << 15) | (4 << 12) | ((rs1 + 8) << 7) | 0x33;
                                        },
                                        2 => {
                                            // C.OR
                                            // or rs1+8, rs1+8, rs2+8
                                            return ((rs2 + 8) << 20) | ((rs1 + 8) << 15) | (6 << 12) | ((rs1 + 8) << 7) | 0x33;
                                        },
                                        3 => {
                                            // C.AND
                                            // and rs1+8, rs1+8, rs2+8
                                            return ((rs2 + 8) << 20) | ((rs1 + 8) << 15) | (7 << 12) | ((rs1 + 8) << 7) | 0x33;
                                        },
                                        _ => {} // Not happens
                                    },
                                    1 => match funct2_2 {
                                        0 => {
                                            // C.SUBW
                                            // subw r1+8, r1+8, r2+8
                                            return (0x20 << 25) | ((rs2 + 8) << 20) | ((rs1 + 8) << 15) | ((rs1 + 8) << 7) | 0x3b;
                                        },
                                        1 => {
                                            // C.ADDW
                                            // addw r1+8, r1+8, r2+8
                                            return ((rs2 + 8) << 20) | ((rs1 + 8) << 15) | ((rs1 + 8) << 7) | 0x3b;
                                        },
                                        2 => {
                                            // Reserved
                                        },
                                        3 => {
                                            // Reserved
                                        },
                                        _ => {} // Not happens
                                    },
                                    _ => {} // No happens
                                };
                            },
                            _ => {} // not happens
                        };
                    },
                    5 => {
                        // C.J
                        // jal x0, imm
                        let offset =
                            match halfword & 0x1000 {
                                0x1000 => 0xfffff000,
                                _ => 0
                            } | // offset[31:12] <= [12]
                                ((halfword >> 1) & 0x800) | // offset[11] <= [12]
                                ((halfword >> 7) & 0x10) | // offset[4] <= [11]
                                ((halfword >> 1) & 0x300) | // offset[9:8] <= [10:9]
                                ((halfword << 2) & 0x400) | // offset[10] <= [8]
                                ((halfword >> 1) & 0x40) | // offset[6] <= [7]
                                ((halfword << 1) & 0x80) | // offset[7] <= [6]
                                ((halfword >> 2) & 0xe) | // offset[3:1] <= [5:3]
                                ((halfword << 3) & 0x20); // offset[5] <= [2]
                        let imm =
                            ((offset >> 1) & 0x80000) | // imm[19] <= offset[20]
                                ((offset << 8) & 0x7fe00) | // imm[18:9] <= offset[10:1]
                                ((offset >> 3) & 0x100) | // imm[8] <= offset[11]
                                ((offset >> 12) & 0xff); // imm[7:0] <= offset[19:12]
                        return (imm << 12) | 0x6f;
                    },
                    6 => {
                        // C.BEQZ
                        // beq r+8, x0, offset
                        let r = (halfword >> 7) & 0x7;
                        let offset =
                            match halfword & 0x1000 {
                                0x1000 => 0xfffffe00,
                                _ => 0
                            } | // offset[31:9] <= [12]
                                ((halfword >> 4) & 0x100) | // offset[8] <= [12]
                                ((halfword >> 7) & 0x18) | // offset[4:3] <= [11:10]
                                ((halfword << 1) & 0xc0) | // offset[7:6] <= [6:5]
                                ((halfword >> 2) & 0x6) | // offset[2:1] <= [4:3]
                                ((halfword << 3) & 0x20); // offset[5] <= [2]
                        let imm2 =
                            ((offset >> 6) & 0x40) | // imm2[6] <= [12]
                                ((offset >> 5) & 0x3f); // imm2[5:0] <= [10:5]
                        let imm1 =
                            (offset & 0x1e) | // imm1[4:1] <= [4:1]
                                ((offset >> 11) & 0x1); // imm1[0] <= [11]
                        return (imm2 << 25) | ((r + 8) << 20) | (imm1 << 7) | 0x63;
                    },
                    7 => {
                        // C.BNEZ
                        // bne r+8, x0, offset
                        let r = (halfword >> 7) & 0x7;
                        let offset =
                            match halfword & 0x1000 {
                                0x1000 => 0xfffffe00,
                                _ => 0
                            } | // offset[31:9] <= [12]
                                ((halfword >> 4) & 0x100) | // offset[8] <= [12]
                                ((halfword >> 7) & 0x18) | // offset[4:3] <= [11:10]
                                ((halfword << 1) & 0xc0) | // offset[7:6] <= [6:5]
                                ((halfword >> 2) & 0x6) | // offset[2:1] <= [4:3]
                                ((halfword << 3) & 0x20); // offset[5] <= [2]
                        let imm2 =
                            ((offset >> 6) & 0x40) | // imm2[6] <= [12]
                                ((offset >> 5) & 0x3f); // imm2[5:0] <= [10:5]
                        let imm1 =
                            (offset & 0x1e) | // imm1[4:1] <= [4:1]
                                ((offset >> 11) & 0x1); // imm1[0] <= [11]
                        return (imm2 << 25) | ((r + 8) << 20) | (1 << 12) | (imm1 << 7) | 0x63;
                    },
                    _ => {} // No happens
                };
            },
            2 => {
                match funct3 {
                    0 => {
                        // C.SLLI
                        // slli r, r, shamt
                        let r = (halfword >> 7) & 0x1f;
                        let shamt =
                            ((halfword >> 7) & 0x20) | // imm[5] <= [12]
                                ((halfword >> 2) & 0x1f); // imm[4:0] <= [6:2]
                        if r != 0 {
                            return (shamt << 20) | (r << 15) | (1 << 12) | (r << 7) | 0x13;
                        }
                        // r == 0 is reserved instruction?
                    },
                    1 => {
                        // C.FLDSP
                        // fld rd, offset(x2)
                        let rd = (halfword >> 7) & 0x1f;
                        let offset =
                            ((halfword >> 7) & 0x20) | // offset[5] <= [12]
                                ((halfword >> 2) & 0x18) | // offset[4:3] <= [6:5]
                                ((halfword << 4) & 0x1c0); // offset[8:6] <= [4:2]
                        if rd != 0 {
                            return (offset << 20) | (2 << 15) | (3 << 12) | (rd << 7) | 0x7;
                        }
                        // rd == 0 is reseved instruction
                    },
                    2 => {
                        // C.LWSP
                        // lw r, offset(x2)
                        let r = (halfword >> 7) & 0x1f;
                        let offset =
                            ((halfword >> 7) & 0x20) | // offset[5] <= [12]
                                ((halfword >> 2) & 0x1c) | // offset[4:2] <= [6:4]
                                ((halfword << 4) & 0xc0); // offset[7:6] <= [3:2]
                        if r != 0 {
                            return (offset << 20) | (2 << 15) | (2 << 12) | (r << 7) | 0x3;
                        }
                        // r == 0 is reseved instruction
                    },
                    3 => {
                        // @TODO: Support C.FLWSP in 32-bit mode
                        // C.LDSP
                        // ld rd, offset(x2)
                        let rd = (halfword >> 7) & 0x1f;
                        let offset =
                            ((halfword >> 7) & 0x20) | // offset[5] <= [12]
                                ((halfword >> 2) & 0x18) | // offset[4:3] <= [6:5]
                                ((halfword << 4) & 0x1c0); // offset[8:6] <= [4:2]
                        if rd != 0 {
                            return (offset << 20) | (2 << 15) | (3 << 12) | (rd << 7) | 0x3;
                        }
                        // rd == 0 is reseved instruction
                    },
                    4 => {
                        let funct1 = (halfword >> 12) & 1; // [12]
                        let rs1 = (halfword >> 7) & 0x1f; // [11:7]
                        let rs2 = (halfword >> 2) & 0x1f; // [6:2]
                        match funct1 {
                            0 => {
                                if rs1 != 0 && rs2 == 0 {
                                    // C.JR
                                    // jalr x0, 0(rs1)
                                    return (rs1 << 15) | 0x67;
                                }
                                // rs1 == 0 is reserved instruction
                                if rs1 != 0 && rs2 != 0 {
                                    // C.MV
                                    // add rs1, x0, rs2
                                    // println!("C.MV RS1:{:x} RS2:{:x}", rs1, rs2);
                                    return (rs2 << 20) | (rs1 << 7) | 0x33;
                                }
                                // rs1 == 0 && rs2 != 0 is Hints
                                // @TODO: Support Hints
                            },
                            1 => {
                                if rs1 == 0 && rs2 == 0 {
                                    // C.EBREAK
                                    // ebreak
                                    return 0x00100073;
                                }
                                if rs1 != 0 && rs2 == 0 {
                                    // C.JALR
                                    // jalr x1, 0(rs1)
                                    return (rs1 << 15) | (1 << 7) | 0x67;
                                }
                                if rs1 != 0 && rs2 != 0 {
                                    // C.ADD
                                    // add rs1, rs1, rs2
                                    return (rs2 << 20) | (rs1 << 15) | (rs1 << 7) | 0x33;
                                }
                                // rs1 == 0 && rs2 != 0 is Hists
                                // @TODO: Supports Hinsts
                            },
                            _ => {} // Not happens
                        };
                    },
                    5 => {
                        // @TODO: Implement
                        // C.FSDSP
                        // fsd rs2, offset(x2)
                        let rs2 = (halfword >> 2) & 0x1f; // [6:2]
                        let offset =
                            ((halfword >> 7) & 0x38) | // offset[5:3] <= [12:10]
                                ((halfword >> 1) & 0x1c0); // offset[8:6] <= [9:7]
                        let imm11_5 = (offset >> 5) & 0x3f;
                        let imm4_0 = offset & 0x1f;
                        return (imm11_5 << 25) | (rs2 << 20) | (2 << 15) | (3 << 12) | (imm4_0 << 7) | 0x27;
                    },
                    6 => {
                        // C.SWSP
                        // sw rs2, offset(x2)
                        let rs2 = (halfword >> 2) & 0x1f; // [6:2]
                        let offset =
                            ((halfword >> 7) & 0x3c) | // offset[5:2] <= [12:9]
                                ((halfword >> 1) & 0xc0); // offset[7:6] <= [8:7]
                        let imm11_5 = (offset >> 5) & 0x3f;
                        let imm4_0 = offset & 0x1f;
                        return (imm11_5 << 25) | (rs2 << 20) | (2 << 15) | (2 << 12) | (imm4_0 << 7) | 0x23;
                    },
                    7 => {
                        // @TODO: Support C.FSWSP in 32-bit mode
                        // C.SDSP
                        // sd rs, offset(x2)
                        let rs2 = (halfword >> 2) & 0x1f; // [6:2]
                        let offset =
                            ((halfword >> 7) & 0x38) | // offset[5:3] <= [12:10]
                                ((halfword >> 1) & 0x1c0); // offset[8:6] <= [9:7]
                        let imm11_5 = (offset >> 5) & 0x3f;
                        let imm4_0 = offset & 0x1f;
                        return (imm11_5 << 25) | (rs2 << 20) | (2 << 15) | (3 << 12) | (imm4_0 << 7) | 0x23;
                    },
                    _ => {} // Never happens
                };
            },
            _ => {} // Never happens
        };
        0xffffffff // Return invalid value
    }

    pub fn sign_extend(&self, value: i64) -> i64 {
        match self.xlen {
            Xlen::Bit32 => value as i32 as i64,
            Xlen::Bit64 => value
        }
    }

    pub fn unsigned_data(&self, value: i64) -> u64 {
        match self.xlen {
            Xlen::Bit32 => (value & 0xffffffff) as u64,
            Xlen::Bit64 => value as u64
        }
    }

    pub fn most_negative(&self) -> i64 {
        match self.xlen {
            Xlen::Bit32 => i32::MIN as i64,
            Xlen::Bit64 => i64::MIN
        }
    }

    pub fn read_csr(&self, address: u16) -> u64 {
        match address {
            // @TODO: Mask should consider of 32-bit mode
            CSR_FFLAGS_ADDRESS => self.read_fflags(),
            CSR_FRM_ADDRESS => (self.csr[CSR_FCSR_ADDRESS as usize] >> 5) & 0x7,
            CSR_SSTATUS_ADDRESS => self.csr[CSR_MSTATUS_ADDRESS as usize] & 0x80000003000de162,
            CSR_SIE_ADDRESS => self.csr[CSR_MIE_ADDRESS as usize] & 0x222,
            CSR_SIP_ADDRESS => self.csr[CSR_MIP_ADDRESS as usize] & 0x222,
            CSR_FCSR_ADDRESS => self.csr[CSR_FCSR_ADDRESS as usize] & 0xff,
            _ => self.csr[address as usize]
        }
    }

    pub fn write_csr(&mut self, address: u16, value: u64) {
        match address {
            CSR_FFLAGS_ADDRESS => self.write_fflags(value),
            CSR_FRM_ADDRESS => {
                self.csr[CSR_FCSR_ADDRESS as usize] &= !0xe0;
                self.csr[CSR_FCSR_ADDRESS as usize] |= (value << 5) & 0xe0;
            },
            CSR_SSTATUS_ADDRESS => {
                self.csr[CSR_MSTATUS_ADDRESS as usize] &= !0x80000003000de162;
                self.csr[CSR_MSTATUS_ADDRESS as usize] |= value & 0x80000003000de162;
            },
            CSR_SIE_ADDRESS => {
                self.csr[CSR_MIE_ADDRESS as usize] &= !0x222;
                self.csr[CSR_MIE_ADDRESS as usize] |= value & 0x222;
            },
            CSR_SIP_ADDRESS => {
                self.csr[CSR_MIP_ADDRESS as usize] &= !0x222;
                self.csr[CSR_MIP_ADDRESS as usize] |= value & 0x222;
            },
            CSR_MIDELEG_ADDRESS => {
                self.csr[address as usize] = value & 0x666; // from qemu
            },
            _ => {
                self.csr[address as usize] = value;
            }
        };
    }

    pub fn set_fcsr_dz(&mut self) {
        let flags = self.read_fflags();
        self.write_fflags(flags | 8);
    }

    pub fn set_fcsr_nv(&mut self) {
        let flags = self.read_fflags();
        self.write_fflags(flags | 16);
    }

    #[cfg(target_arch = "x86_64")]
    fn read_fflags(&self) -> u64 {
        use core::arch::x86_64::*;
        let intel = unsafe { _mm_getcsr() };

        let inexact = match intel & _MM_EXCEPT_INEXACT {
            _MM_EXCEPT_INEXACT => 1,
            _ => 0
        };

        let underflow = match intel & _MM_EXCEPT_UNDERFLOW {
            _MM_EXCEPT_UNDERFLOW => 2,
            _ => 0
        };

        let overflow = match intel & _MM_EXCEPT_OVERFLOW {
            _MM_EXCEPT_OVERFLOW => 4,
            _ => 0
        };

        let div_by_zero = match intel & _MM_EXCEPT_DIV_ZERO {
            _MM_EXCEPT_DIV_ZERO => 8,
            _ => 0
        };

        let invalid_op = match intel & _MM_EXCEPT_INVALID {
            _MM_EXCEPT_INVALID => 16,
            _ => 0
        };

        let flags = self.csr[CSR_FCSR_ADDRESS as usize] & !0x1f;
        flags | inexact | underflow | overflow | div_by_zero | invalid_op
    }

    #[cfg(not(target_arch = "x86_64"))]
    fn read_fflags(&self) -> u64 {
        self.csr[CSR_FCSR_ADDRESS as usize] & 0x1f
    }

    #[cfg(target_arch = "x86_64")]
    fn write_fflags(&mut self, value: u64) {
        use core::arch::x86_64::*;
        let mut flags = unsafe { _mm_getcsr() } & !_MM_EXCEPT_MASK;

        if value & 1 == 1 {
            flags = flags | _MM_EXCEPT_INEXACT;
        }
        if value & 2 == 2 {
            flags = flags | _MM_EXCEPT_OVERFLOW;
        }
        if value & 4 == 4 {
            flags = flags | _MM_EXCEPT_OVERFLOW;
        }
        if value & 8 == 8 {
            flags = flags | _MM_EXCEPT_DIV_ZERO;
        }
        if value & 16 == 16 {
            flags = flags | _MM_EXCEPT_INVALID;
        }

        unsafe { _mm_setcsr(flags); }
    }

    #[cfg(not(target_arch = "x86_64"))]
    fn write_fflags(&mut self, value: u64) {
        self.csr[CSR_FCSR_ADDRESS as usize] &= !0x1f;
        self.csr[CSR_FCSR_ADDRESS as usize] |= value & 0x1f;
    }
}


pub const UNIMPLEMENTED: Instruction = Instruction {
    name: "UNIMP",
    operation: |_cpu, word, _address| {
        Err(Trap{
            trap_type: TrapType::IllegalInstruction,
            value: word as u64
        })
    }
};

// while this is a "machine" mode instruction it is needed for the official tests to pass
const MRET: Instruction = Instruction {
    name: "MRET",
    operation: |cpu, _word, _address| {
        cpu.pc = cpu.read_csr(CSR_MEPC_ADDRESS) as *mut u8;

        let status = cpu.read_csr(CSR_MSTATUS_ADDRESS);
        let mpie = (status >> 7) & 1;
        //let mpp = (status >> 11) & 0x3;
        let mprv = 0;
        // Override MIE[3] with MPIE[7], set MPIE[7] to 1, set MPP[12:11] to 0
        // and override MPRV[17]
        let new_status = (status & !0x21888) | (mprv << 17) | (mpie << 3) | (1 << 7);
        cpu.write_csr(CSR_MSTATUS_ADDRESS, new_status);
        Ok(())
    }
};

#[cfg(test)]
mod test_cpu {
    use super::*;

    #[test]
    fn babys_first_instruction() {
        let mut cpu = Cpu::new();
        let mut instruction= vec![0x00000505]; // addi a0,a0,1
        cpu.update_pc(&mut instruction[0]);
        let pc1 = cpu.get_pc();
        assert_eq!(cpu.x[10], 0);
        cpu.tick().ok().expect("cpu failure");
        assert_eq!(cpu.x[10], 1);
        let pc2 = cpu.get_pc();
        assert_eq!(2, pc2 - pc1);
    }

    #[test]
    fn two_compressed_instruction() {
        let mut cpu = Cpu::new();
        let mut instruction= vec![0x05050505];
        cpu.update_pc(&mut instruction[0]);
        let pc1 = cpu.get_pc();
        assert_eq!(cpu.x[10], 0);
        cpu.tick().ok().expect("cpu failure");
        assert_eq!(cpu.x[10], 1);
        cpu.tick().ok().expect("cpu failure");
        assert_eq!(cpu.x[10], 2);
        let pc2 = cpu.get_pc();
        assert_eq!(4, pc2 - pc1);
    }
}