use std::ptr::null_mut;

#[derive(Clone)]
pub enum Xlen {
    Bit32,
    Bit64
}

pub struct Trap {
    pub trap_type: TrapType,
    pub value: u64 // Trap type specific value
}

#[allow(dead_code)]
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
    MachineExternalInterrupt
}

#[allow(unused_attributes)]
struct Instruction {
    operation: fn(cpu: &mut Cpu, word: u32, address: *const u8) -> Result<(), Trap>
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
    xlen: Xlen,
    x: [i64; 32],
    f: [f64; 32],
    pc: *mut u8,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            xlen: Xlen::Bit32,
            x: [0; 32],
            f: [0.0; 32],
            pc: null_mut()
        }
    }

    fn fetch(&mut self) -> u32 {
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

    pub fn get_pc(&self) -> usize {
        self.pc as usize
    }

    pub fn tick(&mut self) -> Result<(), Trap> {
        let instruction_address = self.pc;

        let word = self.fetch();
        if let Some(instruction) = Cpu::decode(word) {
            (instruction.operation)(self, word, instruction_address)
        } else {
            Err(Trap { trap_type: TrapType::IllegalInstruction, value: 0 })
        }
    }

    fn decode(word: u32) -> Option<&'static Instruction> {
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
                _ => None
            },

            0b0100011 => match (word >> 12) & 7 {
                0b000 => Some(&SB),
                0b001 => Some(&SH),
                0b010 => Some(&SW),
                _ => None
            },

            0b0010011 => match (word >> 12) & 7 {
                0b000 => Some(&ADDI),
                0b010 => Some(&SLTI),
                0b011 => Some(&SLTIU),
                0b100 => Some(&XORI),
                0b110 => Some(&ORI),
                0b111 => Some(&ANDI),
                0b001 => Some(&SLLI),
                0b101 => Some(&SRLI),
                0b101 => Some(&SRAI),
                _ => None
            },

            0b0010011 => match (word >> 12) & 7 {
                0b000 => Some(&ADD),
                0b010 => Some(&SLTI),
                0b011 => Some(&SLTIU),
                0b100 => Some(&XORI),
                0b110 => Some(&ORI),
                0b111 => Some(&ANDI),
                0b001 => match word >> 25 {
                    0b0000000 => Some(&SLLI),
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
                    0b0100000 => Some(&SUB),
                    _ => None
                },
                0b001 => match word >> 25 {
                    0b0000000 => Some(&SLL),
                    _ => None
                },
                0b010 => match word >> 25 {
                    0b0000000 => Some(&SLT),
                    _ => None
                },
                0b011 => match word >> 25 {
                    0b0000000 => Some(&SLTU),
                    _ => None
                },
                0b100 => match word >> 25 {
                    0b0000000 => Some(&XOR),
                    _ => None
                },
                0b101 => match word >> 25 {
                    0b0000000 => Some(&SRL),
                    0b0100000 => Some(&SRA),
                    _ => None
                },
                0b110 => match word >> 25 {
                    0b0000000 => Some(&OR),
                    _ => None
                },
                0b111 => match word >> 25 {
                    0b0000000 => Some(&AND),
                    _ => None
                },
                _ => None
            }
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

    fn sign_extend(&self, value: i64) -> i64 {
        match self.xlen {
            Xlen::Bit32 => value as i32 as i64,
            Xlen::Bit64 => value
        }
    }

    fn unsigned_data(&self, value: i64) -> u64 {
        match self.xlen {
            Xlen::Bit32 => (value & 0xffffffff) as u64,
            Xlen::Bit64 => value as u64
        }
    }
}

struct FormatR {
    rd: usize,
    rs1: usize,
    rs2: usize
}

fn parse_format_r(word: u32) -> FormatR {
    FormatR {
        rd: ((word >> 7) & 0x1f) as usize, // [11:7]
        rs1: ((word >> 15) & 0x1f) as usize, // [19:15]
        rs2: ((word >> 20) & 0x1f) as usize // [24:20]
    }
}

struct FormatU {
    rd: usize,
    imm: u64
}

fn parse_format_u(word: u32) -> FormatU {
    FormatU {
        rd: ((word >> 7) & 0x1f) as usize, // [11:7]
        imm: (
            match word & 0x80000000 {
                0x80000000 => 0xffffffff00000000,
                _ => 0
            } | // imm[63:32] = [31]
                ((word as u64) & 0xfffff000) // imm[31:12] = [31:12]
        ) as u64
    }
}

struct FormatI {
    rd: usize,
    rs1: usize,
    imm: i64
}

fn parse_format_i(word: u32) -> FormatI {
    FormatI {
        rd: ((word >> 7) & 0x1f) as usize, // [11:7]
        rs1: ((word >> 15) & 0x1f) as usize, // [19:15]
        imm: (
            match word & 0x80000000 { // imm[31:11] = [31]
                0x80000000 => 0xfffff800,
                _ => 0
            } |
                ((word >> 20) & 0x000007ff) // imm[10:0] = [30:20]
        ) as i32 as i64
    }
}

struct FormatJ {
    rd: usize,
    imm: u64
}

fn parse_format_j(word: u32) -> FormatJ {
    FormatJ {
        rd: ((word >> 7) & 0x1f) as usize, // [11:7]
        imm: (
            match word & 0x80000000 { // imm[31:20] = [31]
                0x80000000 => 0xfff00000,
                _ => 0
            } |
                (word & 0x000ff000) | // imm[19:12] = [19:12]
                ((word & 0x00100000) >> 9) | // imm[11] = [20]
                ((word & 0x7fe00000) >> 20) // imm[10:1] = [30:21]
        ) as i32 as i64 as u64
    }
}

struct FormatB {
    rs1: usize,
    rs2: usize,
    imm: u64
}

fn parse_format_b(word: u32) -> FormatB {
    FormatB {
        rs1: ((word >> 15) & 0x1f) as usize, // [19:15]
        rs2: ((word >> 20) & 0x1f) as usize, // [24:20]
        imm: (
            match word & 0x80000000 { // imm[31:12] = [31]
                0x80000000 => 0xfffff000,
                _ => 0
            } |
                ((word << 4) & 0x00000800) | // imm[11] = [7]
                ((word >> 20) & 0x000007e0) | // imm[10:5] = [30:25]
                ((word >> 7) & 0x0000001e) // imm[4:1] = [11:8]
        ) as i32 as i64 as u64
    }
}

struct FormatS {
    rs1: usize,
    rs2: usize,
    imm: i64
}

fn parse_format_s(word: u32) -> FormatS {
    FormatS {
        rs1: ((word >> 15) & 0x1f) as usize, // [19:15]
        rs2: ((word >> 20) & 0x1f) as usize, // [24:20]
        imm: (
            match word & 0x80000000 {
                0x80000000 => 0xfffff000,
                _ => 0
            } | // imm[31:12] = [31]
                ((word >> 20) & 0xfe0) | // imm[11:5] = [31:25]
                ((word >> 7) & 0x1f) // imm[4:0] = [11:7]
        ) as i32 as i64
    }
}

const ADD: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_add(cpu.x[f.rs2]));
        Ok(())
    }
};

const ADDI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_add(f.imm));
        Ok(())
    }
};

const AND: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] & cpu.x[f.rs2]);
        Ok(())
    }
};

const ANDI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] & f.imm);
        Ok(())
    }
};

const AUIPC: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = parse_format_u(word);
        cpu.x[f.rd] = cpu.sign_extend(address.wrapping_add(f.imm as usize) as i64);
        Ok(())
    }
};

const BEQ: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) == cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

const BGE: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) >= cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

const BGEU: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = parse_format_b(word);
        if cpu.unsigned_data(cpu.x[f.rs1]) >= cpu.unsigned_data(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

const BLT: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) < cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

const BLTU: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = parse_format_b(word);
        if cpu.unsigned_data(cpu.x[f.rs1]) < cpu.unsigned_data(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

const BNE: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) != cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

const JAL: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = parse_format_j(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.pc as i64);
        cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        Ok(())
    }
};

const JALR: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        let tmp = cpu.sign_extend(cpu.pc as i64);
        cpu.pc = (cpu.x[f.rs1] as u64).wrapping_add(f.imm as u64) as *mut u8;
        cpu.x[f.rd] = tmp;
        Ok(())
    }
};

const LB: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const i8) as i64;
        }
        Ok(())
    }
};

const LBU: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const u8) as i64;
        }
        Ok(())
    }
};

const LD: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const i64);
        }
        Ok(())
    }
};

const LH: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const i16) as i64;
        }
        Ok(())
    }
};

const LHU: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const u16) as i64;
        }
        Ok(())
    }
};

const LUI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_u(word);
        cpu.x[f.rd] = f.imm as i64;
        Ok(())
    }
};

const LW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const i32) as i64;
        }
        Ok(())
    }
};

const LWU: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const u32) as i64;
        }
        Ok(())
    }
};

const OR: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] | cpu.x[f.rs2]);
        Ok(())
    }
};

const ORI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] | f.imm);
        Ok(())
    }
};

const SB: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_s(word);

        unsafe {
            *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *mut u8) = cpu.x[f.rs2] as u8;
        }
        Ok(())
    }
};

const SD: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_s(word);

        unsafe {
            *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *mut u64) = cpu.x[f.rs2] as u64;
        }
        Ok(())
    }
};

const SH: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_s(word);

        unsafe {
            *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *mut u16) = cpu.x[f.rs2] as u16;
        }
        Ok(())
    }
};

const SLL: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_shl(cpu.x[f.rs2] as u32));
        Ok(())
    }
};

const SLLI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        let mask = match cpu.xlen {
            Xlen::Bit32 => 0x1f,
            Xlen::Bit64 => 0x3f
        };
        let shamt = (word >> 20) & mask;
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] << shamt);
        Ok(())
    }
};

const SLLIW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        let shamt = f.rs2 as u32;
        cpu.x[f.rd] = (cpu.x[f.rs1] << shamt) as i32 as i64;
        Ok(())
    }
};

const SLLW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = (cpu.x[f.rs1] as u32).wrapping_shl(cpu.x[f.rs2] as u32) as i32 as i64;
        Ok(())
    }
};

const SLTI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        cpu.x[f.rd] = match cpu.x[f.rs1] < f.imm {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

const SLT: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = match cpu.x[f.rs1] < cpu.x[f.rs2] {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

const SLTIU: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        cpu.x[f.rd] = match cpu.unsigned_data(cpu.x[f.rs1]) < cpu.unsigned_data(f.imm) {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

const SLTU: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = match cpu.unsigned_data(cpu.x[f.rs1]) < cpu.unsigned_data(cpu.x[f.rs2]) {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

const SRA: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_shr(cpu.x[f.rs2] as u32));
        Ok(())
    }
};

const SRAI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        let mask = match cpu.xlen {
            Xlen::Bit32 => 0x1f,
            Xlen::Bit64 => 0x3f
        };
        let shamt = (word >> 20) & mask;
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] >> shamt);
        Ok(())
    }
};

const SRL: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.unsigned_data(cpu.x[f.rs1]).wrapping_shr(cpu.x[f.rs2] as u32) as i64);
        Ok(())
    }
};

const SRLI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        let mask = match cpu.xlen {
            Xlen::Bit32 => 0x1f,
            Xlen::Bit64 => 0x3f
        };
        let shamt = (word >> 20) & mask;
        cpu.x[f.rd] = cpu.sign_extend((cpu.unsigned_data(cpu.x[f.rs1]) >> shamt) as i64);
        Ok(())
    }
};

const SRLIW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        let mask = match cpu.xlen {
            Xlen::Bit32 => 0x1f,
            Xlen::Bit64 => 0x3f
        };
        let shamt = (word >> 20) & mask;
        cpu.x[f.rd] = ((cpu.x[f.rs1] as u32) >> shamt) as i32 as i64;
        Ok(())
    }
};

const SRLW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = (cpu.x[f.rs1] as u32).wrapping_shr(cpu.x[f.rs2] as u32) as i32 as i64;
        Ok(())
    }
};

const SUB: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_sub(cpu.x[f.rs2]));
        Ok(())
    }
};

const SUBW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = cpu.x[f.rs1].wrapping_sub(cpu.x[f.rs2]) as i32 as i64;
        Ok(())
    }
};

const SW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_s(word);

        unsafe {
            *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *mut u32) = cpu.x[f.rs2] as u32;
        }
        Ok(())
    }
};

const XOR: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] ^ cpu.x[f.rs2]);
        Ok(())
    }
};

const XORI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] ^ f.imm);
        Ok(())
    }
};

#[cfg(test)]
mod test_cpu {
    use super::*;

    #[test]
    fn babys_first_instruction() {
        let mut cpu = Cpu::new();
        let mut instruction= vec![0x00000505]; /// addi a0,a0,1
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