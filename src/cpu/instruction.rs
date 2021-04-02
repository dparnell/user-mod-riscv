use crate::cpu::{Cpu, Trap};
use std::fmt::{Debug, Formatter};
use std::fmt;

pub struct Instruction {
    pub name: &'static str,
    pub operation: fn(cpu: &mut Cpu, word: u32, address: *const u8) -> Result<(), Trap>
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("name", &self.name)
            .finish()
    }
}

pub struct FormatR {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize
}

pub fn parse_format_r(word: u32) -> FormatR {
    FormatR {
        rd: ((word >> 7) & 0x1f) as usize, // [11:7]
        rs1: ((word >> 15) & 0x1f) as usize, // [19:15]
        rs2: ((word >> 20) & 0x1f) as usize // [24:20]
    }
}

pub struct FormatU {
    pub rd: usize,
    pub imm: u64
}

pub fn parse_format_u(word: u32) -> FormatU {
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

pub struct FormatI {
    pub rd: usize,
    pub rs1: usize,
    pub imm: i64
}

pub fn parse_format_i(word: u32) -> FormatI {
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

pub struct FormatJ {
    pub rd: usize,
    pub imm: u64
}

pub fn parse_format_j(word: u32) -> FormatJ {
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

pub struct FormatB {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u64
}

pub fn parse_format_b(word: u32) -> FormatB {
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

pub struct FormatS {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: i64
}

pub fn parse_format_s(word: u32) -> FormatS {
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

pub struct FormatCSR {
    pub csr: u16,
    pub rs: usize,
    pub rd: usize
}

pub fn parse_format_csr(word: u32) -> FormatCSR {
    FormatCSR {
        csr: ((word >> 20) & 0xfff) as u16, // [31:20]
        rs: ((word >> 15) & 0x1f) as usize, // [19:15], also uimm
        rd: ((word >> 7) & 0x1f) as usize // [11:7]
    }
}

// has rs3
pub struct FormatR2 {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub rs3: usize
}

pub fn parse_format_r2(word: u32) -> FormatR2 {
    FormatR2 {
        rd: ((word >> 7) & 0x1f) as usize, // [11:7]
        rs1: ((word >> 15) & 0x1f) as usize, // [19:15]
        rs2: ((word >> 20) & 0x1f) as usize, // [24:20]
        rs3: ((word >> 27) & 0x1f) as usize // [31:27]
    }
}

