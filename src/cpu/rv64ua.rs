use crate::cpu::instruction;
use crate::cpu::instruction::Instruction;

pub const AMOADD_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const i64);
            *(cpu.x[f.rs1] as *mut u64) = cpu.x[f.rs2].wrapping_add(tmp) as u64;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOADD_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const i32) as i64;
            *(cpu.x[f.rs1] as *mut u32) = cpu.x[f.rs2].wrapping_add(tmp) as u32;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOAND_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const i64);
            *(cpu.x[f.rs1] as *mut u64) = (cpu.x[f.rs2] & tmp) as u64;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOAND_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const i32) as i64;
            *(cpu.x[f.rs1] as *mut u32) = (cpu.x[f.rs2] & tmp) as u32;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOMAX_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const i64);
            let max = match cpu.x[f.rs2] >=tmp {
                true => cpu.x[f.rs2],
                false => tmp as i64
            };
            *(cpu.x[f.rs1] as *mut i64) = max;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOMAX_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const i32);
            let max = match (cpu.x[f.rs2] as i32) >=tmp {
                true => cpu.x[f.rs2] as i32,
                false => tmp as i32
            };
            *(cpu.x[f.rs1] as *mut i32) = max;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOMAXU_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const u64);
            let max = match (cpu.x[f.rs2] as u64) >=tmp {
                true => cpu.x[f.rs2] as u64,
                false => tmp as u64
            };
            *(cpu.x[f.rs1] as *mut u64) = max;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOMAXU_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const u32);
            let max = match (cpu.x[f.rs2] as u32) >=tmp {
                true => cpu.x[f.rs2] as u32,
                false => tmp as u32
            };
            *(cpu.x[f.rs1] as *mut u32) = max;
            cpu.x[f.rd] = tmp as i32 as i64;
        }
        Ok(())
    }
};

pub const AMOMIN_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const i64);
            let min = match cpu.x[f.rs2] <=tmp {
                true => cpu.x[f.rs2],
                false => tmp as i64
            };
            *(cpu.x[f.rs1] as *mut i64) = min;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOMIN_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const i32);
            let min = match (cpu.x[f.rs2] as i32) <= tmp {
                true => cpu.x[f.rs2] as i32,
                false => tmp as i32
            };
            *(cpu.x[f.rs1] as *mut i32) = min;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOMINU_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const u64);
            let min = match (cpu.x[f.rs2] as u64) <= tmp {
                true => cpu.x[f.rs2] as u64,
                false => tmp as u64
            };
            *(cpu.x[f.rs1] as *mut u64) = min;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOMINU_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const u32);
            let min = match (cpu.x[f.rs2] as u32) <= tmp {
                true => cpu.x[f.rs2] as u32,
                false => tmp as u32
            };
            *(cpu.x[f.rs1] as *mut u32) = min;
            cpu.x[f.rd] = tmp as i32 as i64;
        }
        Ok(())
    }
};

pub const AMOOR_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const u64);
            *(cpu.x[f.rs1] as *mut u64) = ((cpu.x[f.rs2] as u64) | tmp) as u64;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOOR_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const u32);
            *(cpu.x[f.rs1] as *mut u32) = ((cpu.x[f.rs2] as u32) | tmp) as u32;
            cpu.x[f.rd] = tmp as i32 as i64;
        }
        Ok(())
    }
};

pub const AMOSWAP_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const u64);
            *(cpu.x[f.rs1] as *mut u64) = cpu.x[f.rs2] as u64;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOSWAP_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const u32);
            *(cpu.x[f.rs1] as *mut u32) = cpu.x[f.rs2] as u32;
            cpu.x[f.rd] = tmp as i32 as i64;
        }
        Ok(())
    }
};

pub const AMOXOR_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const u64);
            *(cpu.x[f.rs1] as *mut u64) = ((cpu.x[f.rs2] as u64) ^ tmp) as u64;
            cpu.x[f.rd] = tmp as i64;
        }
        Ok(())
    }
};

pub const AMOXOR_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        unsafe {
            let tmp = *(cpu.x[f.rs1] as *const u32);
            *(cpu.x[f.rs1] as *mut u32) = ((cpu.x[f.rs2] as u32) ^ tmp) as u32;
            cpu.x[f.rd] = tmp as i32 as i64;
        }
        Ok(())
    }
};

pub const LR_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        // @TODO: Implement properly
        unsafe {
            cpu.x[f.rd] = *(cpu.x[f.rs1] as *const i64);
        }
        cpu.is_reservation_set = true;
        cpu.reservation = cpu.x[f.rs1] as u64; // Is virtual address ok?
        Ok(())
    }
};

pub const LR_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        // @TODO: Implement properly
        unsafe {
            cpu.x[f.rd] = *(cpu.x[f.rs1] as *const u32) as i64;
        }
        cpu.is_reservation_set = true;
        cpu.reservation = cpu.x[f.rs1] as u64; // Is virtual address ok?
        Ok(())
    }
};

pub const SC_D: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        // @TODO: Implement properly
        cpu.x[f.rd] = match cpu.is_reservation_set && cpu.reservation == (cpu.x[f.rs1] as u64) {
            true => unsafe {
                *(cpu.x[f.rs1] as *mut u64) = cpu.x[f.rs2] as u64;
                cpu.is_reservation_set = false;
                0
            },
            false => 1
        };
        Ok(())
    }
};

pub const SC_W: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        // @TODO: Implement properly
        cpu.x[f.rd] = match cpu.is_reservation_set && cpu.reservation == (cpu.x[f.rs1] as u64) {
            true => unsafe {
                *(cpu.x[f.rs1] as *mut u32) = cpu.x[f.rs2] as u32;
                cpu.is_reservation_set = false;
                0
            },
            false => 1
        };
        Ok(())
    }
};
