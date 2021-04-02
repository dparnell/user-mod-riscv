use crate::cpu::{instruction, Xlen};
use crate::cpu::instruction::Instruction;

pub const DIV: Instruction = Instruction {
    name: "DIV",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let dividend = cpu.x[f.rs1];
        let divisor = cpu.x[f.rs2];
        if divisor == 0 {
            cpu.x[f.rd] = -1;
        } else if dividend == cpu.most_negative() && divisor == -1 {
            cpu.x[f.rd] = dividend;
        } else {
            cpu.x[f.rd] = cpu.sign_extend(dividend.wrapping_div(divisor))
        }
        Ok(())
    }
};

pub const DIVU: Instruction = Instruction {
    name: "DIVU",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let dividend = cpu.unsigned_data(cpu.x[f.rs1]);
        let divisor = cpu.unsigned_data(cpu.x[f.rs2]);
        if divisor == 0 {
            cpu.x[f.rd] = -1;
        } else {
            cpu.x[f.rd] = cpu.sign_extend(dividend.wrapping_div(divisor) as i64)
        }
        Ok(())
    }
};

pub const DIVUW: Instruction = Instruction {
    name: "DIVUW",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let dividend = cpu.unsigned_data(cpu.x[f.rs1]) as u32;
        let divisor = cpu.unsigned_data(cpu.x[f.rs2]) as u32;
        if divisor == 0 {
            cpu.x[f.rd] = -1;
        } else {
            cpu.x[f.rd] = dividend.wrapping_div(divisor) as i32 as i64
        }
        Ok(())
    }
};

pub const DIVW: Instruction = Instruction {
    name: "DIVW",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let dividend = cpu.x[f.rs1] as i32;
        let divisor = cpu.x[f.rs2] as i32;
        if divisor == 0 {
            cpu.x[f.rd] = -1;
        } else if dividend == std::i32::MIN && divisor == -1 {
            cpu.x[f.rd] = dividend as i32 as i64;
        } else {
            cpu.x[f.rd] = dividend.wrapping_div(divisor) as i32 as i64
        }
        Ok(())
    }
};

pub const MUL: Instruction = Instruction {
    name: "MUL",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_mul(cpu.x[f.rs2]));
        Ok(())
    }
};

pub const MULH: Instruction = Instruction {
    name: "MULH",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.xlen {
            Xlen::Bit32 => {
                cpu.sign_extend((cpu.x[f.rs1] * cpu.x[f.rs2]) >> 32)
            },
            Xlen::Bit64 => {
                ((cpu.x[f.rs1] as i128) * (cpu.x[f.rs2] as i128) >> 64) as i64
            }
        };
        Ok(())
    }
};

pub const MULHU: Instruction = Instruction {
    name: "MULHU",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.xlen {
            Xlen::Bit32 => {
                cpu.sign_extend((((cpu.x[f.rs1] as u32 as u64) * (cpu.x[f.rs2] as u32 as u64)) >> 32) as i64)
            },
            Xlen::Bit64 => {
                ((cpu.x[f.rs1] as u64 as u128).wrapping_mul(cpu.x[f.rs2] as u64 as u128) >> 64) as i64
            }
        };
        Ok(())
    }
};

pub const MULHSU: Instruction = Instruction {
    name: "MULHSU",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.xlen {
            Xlen::Bit32 => {
                cpu.sign_extend(((cpu.x[f.rs1] as i64).wrapping_mul(cpu.x[f.rs2] as u32 as i64) >> 32) as i64)
            },
            Xlen::Bit64 => {
                ((cpu.x[f.rs1] as u128).wrapping_mul(cpu.x[f.rs2] as u64 as u128) >> 64) as i64
            }
        };
        Ok(())
    }
};

pub const MULW: Instruction = Instruction {
    name: "MULW",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend((cpu.x[f.rs1] as i32).wrapping_mul(cpu.x[f.rs2] as i32) as i64);
        Ok(())
    }
};

pub const REM: Instruction = Instruction {
    name: "REM",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let dividend = cpu.x[f.rs1];
        let divisor = cpu.x[f.rs2];
        if divisor == 0 {
            cpu.x[f.rd] = dividend;
        } else if dividend == cpu.most_negative() && divisor == -1 {
            cpu.x[f.rd] = 0;
        } else {
            cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_rem(cpu.x[f.rs2]));
        }
        Ok(())
    }
};

pub const REMU: Instruction = Instruction {
    name: "REMU",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let dividend = cpu.unsigned_data(cpu.x[f.rs1]);
        let divisor = cpu.unsigned_data(cpu.x[f.rs2]);
        cpu.x[f.rd] = match divisor {
            0 => cpu.sign_extend(dividend as i64),
            _ => cpu.sign_extend(dividend.wrapping_rem(divisor) as i64)
        };
        Ok(())
    }
};

pub const REMUW: Instruction = Instruction {
    name: "REMUW",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let dividend = cpu.x[f.rs1] as u32;
        let divisor = cpu.x[f.rs2] as u32;
        cpu.x[f.rd] = match divisor {
            0 => dividend as i32 as i64,
            _ => dividend.wrapping_rem(divisor) as i32 as i64
        };
        Ok(())
    }
};

pub const REMW: Instruction = Instruction {
    name: "REMW",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let dividend = cpu.x[f.rs1] as i32;
        let divisor = cpu.x[f.rs2] as i32;
        if divisor == 0 {
            cpu.x[f.rd] = dividend as i64;
        } else if dividend == std::i32::MIN && divisor == -1 {
            cpu.x[f.rd] = 0;
        } else {
            cpu.x[f.rd] = dividend.wrapping_rem(divisor) as i64;
        }
        Ok(())
    }
};
