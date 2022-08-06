use crate::cpu::instruction;
use crate::cpu::instruction::Instruction;

pub const AMOADD_D: Instruction = Instruction {
    name: "AMOADD.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);

        let tmp = memory.read_i64(cpu.x[f.rs1] as usize)?;
        memory.write_u64(cpu.x[f.rs1] as usize, cpu.x[f.rs2].wrapping_add(tmp) as u64)?;
        cpu.x[f.rd] = tmp;
        Ok(())
    }
};

pub const AMOADD_W: Instruction = Instruction {
    name: "AMOADD.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_i32(cpu.x[f.rs1] as usize)? as i64;
        memory.write_u32(cpu.x[f.rs1] as usize, cpu.x[f.rs2].wrapping_add(tmp) as u32)?;
        cpu.x[f.rd] = tmp;
        Ok(())
    }
};

pub const AMOAND_D: Instruction = Instruction {
    name: "AMOAND.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_i64(cpu.x[f.rs1] as usize)?;
        memory.write_u64(cpu.x[f.rs1] as usize, (cpu.x[f.rs2] & tmp) as u64)?;
        cpu.x[f.rd] = tmp;
        Ok(())
    }
};

pub const AMOAND_W: Instruction = Instruction {
    name: "AMOAND.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_i32(cpu.x[f.rs1] as usize)? as i64;
        memory.write_u32(cpu.x[f.rs1] as usize, (cpu.x[f.rs2] & tmp) as u32)?;
        cpu.x[f.rd] = tmp;
        Ok(())
    }
};

pub const AMOMAX_D: Instruction = Instruction {
    name: "AMOMAX.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);

        let tmp = memory.read_i64(cpu.x[f.rs1] as usize)?;
        let max = match cpu.x[f.rs2] >=tmp {
            true => cpu.x[f.rs2],
            false => tmp as i64
        };
        memory.write_u64(cpu.x[f.rs1] as usize, max as u64)?;
        cpu.x[f.rd] = tmp;

        Ok(())
    }
};

pub const AMOMAX_W: Instruction = Instruction {
    name: "AMOMAX.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_i32(cpu.x[f.rs1] as usize)?;
        let max = match (cpu.x[f.rs2] as i32) >=tmp {
            true => cpu.x[f.rs2] as i32,
            false => tmp as i32
        };
        memory.write_u32(cpu.x[f.rs1] as usize, max as u32)?;
        cpu.x[f.rd] = tmp as i64;
        Ok(())
    }
};

pub const AMOMAXU_D: Instruction = Instruction {
    name: "AMOMAXU.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_u64(cpu.x[f.rs1] as usize)?;
        let max = match (cpu.x[f.rs2] as u64) >=tmp {
            true => cpu.x[f.rs2] as u64,
            false => tmp as u64
        };
        memory.write_u64(cpu.x[f.rs1] as usize, max)?;
        cpu.x[f.rd] = tmp as i64;
        Ok(())
    }
};

pub const AMOMAXU_W: Instruction = Instruction {
    name: "AMOMAXU.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_u32(cpu.x[f.rs1] as usize)?;
        let max = match (cpu.x[f.rs2] as u32) >= tmp {
            true => cpu.x[f.rs2] as u32,
            false => tmp as u32
        };
        memory.write_u32(cpu.x[f.rs1] as usize, max)?;
        cpu.x[f.rd] = tmp as i32 as i64;
        Ok(())
    }
};

pub const AMOMIN_D: Instruction = Instruction {
    name: "AMOMIN.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);

        let tmp = memory.read_i64(cpu.x[f.rs1] as usize)?;
        let min = match cpu.x[f.rs2] <=tmp {
            true => cpu.x[f.rs2],
            false => tmp as i64
        };
        memory.write_u64(cpu.x[f.rs1] as usize, min as u64)?;
        cpu.x[f.rd] = tmp as i64;
        Ok(())
    }
};

pub const AMOMIN_W: Instruction = Instruction {
    name: "AMOMIN.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_i32(cpu.x[f.rs1] as usize)?;
        let min = match (cpu.x[f.rs2] as i32) <= tmp {
            true => cpu.x[f.rs2] as i32,
            false => tmp as i32
        };
        memory.write_u32(cpu.x[f.rs1] as usize, min as u32)?;
        cpu.x[f.rd] = tmp as i64;
        Ok(())
    }
};

pub const AMOMINU_D: Instruction = Instruction {
    name: "AMOMINU.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_u64(cpu.x[f.rs1] as usize)?;
        let min = match (cpu.x[f.rs2] as u64) <= tmp {
            true => cpu.x[f.rs2] as u64,
            false => tmp as u64
        };
        memory.write_u64(cpu.x[f.rs1] as usize, min)?;
        cpu.x[f.rd] = tmp as i64;
        Ok(())
    }
};

pub const AMOMINU_W: Instruction = Instruction {
    name: "AMOMINU.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_u32(cpu.x[f.rs1] as usize)?;
        let min = match (cpu.x[f.rs2] as u32) <= tmp {
            true => cpu.x[f.rs2] as u32,
            false => tmp as u32
        };
        memory.write_u32(cpu.x[f.rs1] as usize, min)?;
        cpu.x[f.rd] = tmp as i32 as i64;
        Ok(())
    }
};

pub const AMOOR_D: Instruction = Instruction {
    name: "AMOOR.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_u64(cpu.x[f.rs1] as usize)?;
        memory.write_u64(cpu.x[f.rs1] as usize, (cpu.x[f.rs2] as u64) | tmp)?;
        cpu.x[f.rd] = tmp as i64;
        Ok(())
    }
};

pub const AMOOR_W: Instruction = Instruction {
    name: "AMOOR.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_u32(cpu.x[f.rs1] as usize)?;
        memory.write_u32(cpu.x[f.rs1] as usize, (cpu.x[f.rs2] as u32) | tmp)?;
        cpu.x[f.rd] = tmp as i32 as i64;
        Ok(())
    }
};

pub const AMOSWAP_D: Instruction = Instruction {
    name: "AMOSWAP.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_u64(cpu.x[f.rs1] as usize)?;
        memory.write_u64(cpu.x[f.rs1] as usize, cpu.x[f.rs2] as u64)?;
        cpu.x[f.rd] = tmp as i64;
        Ok(())
    }
};

pub const AMOSWAP_W: Instruction = Instruction {
    name: "AMOSWAP.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_u32(cpu.x[f.rs1] as usize)?;
        memory.write_u32(cpu.x[f.rs1] as usize, cpu.x[f.rs2] as u32)?;
        cpu.x[f.rd] = tmp as i32 as i64;
        Ok(())
    }
};

pub const AMOXOR_D: Instruction = Instruction {
    name: "AMOXOR.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_u64(cpu.x[f.rs1] as usize)?;
        memory.write_u64(cpu.x[f.rs1] as usize, cpu.x[f.rs2] as u64 ^ tmp)?;
        cpu.x[f.rd] = tmp as i64;
        Ok(())
    }
};

pub const AMOXOR_W: Instruction = Instruction {
    name: "AMOXOR.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let tmp = memory.read_u32(cpu.x[f.rs1] as usize)?;
        memory.write_u32(cpu.x[f.rs1] as usize, cpu.x[f.rs2] as u32 ^ tmp)?;
        cpu.x[f.rd] = tmp as i32 as i64;
        Ok(())
    }
};

pub const LR_D: Instruction = Instruction {
    name: "LR.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        // @TODO: Implement properly
        cpu.x[f.rd] = memory.read_i64(cpu.x[f.rs1] as usize)?;
        cpu.is_reservation_set = true;
        cpu.reservation = cpu.x[f.rs1] as u64; // Is virtual address ok?
        Ok(())
    }
};

pub const LR_W: Instruction = Instruction {
    name: "LR.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        // @TODO: Implement properly
        cpu.x[f.rd] = memory.read_u32(cpu.x[f.rs1] as usize)? as i64;
        cpu.is_reservation_set = true;
        cpu.reservation = cpu.x[f.rs1] as u64; // Is virtual address ok?
        Ok(())
    }
};

pub const SC_D: Instruction = Instruction {
    name: "SC.D",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        // @TODO: Implement properly
        cpu.x[f.rd] = match cpu.is_reservation_set && cpu.reservation == (cpu.x[f.rs1] as u64) {
            true => {
                memory.write_u64(cpu.x[f.rs1] as usize, cpu.x[f.rs2] as u64)?;
                cpu.is_reservation_set = false;
                0
            },
            false => 1
        };
        Ok(())
    }
};

pub const SC_W: Instruction = Instruction {
    name: "SC.W",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_r(word);
        // @TODO: Implement properly
        cpu.x[f.rd] = match cpu.is_reservation_set && cpu.reservation == (cpu.x[f.rs1] as u64) {
            true => {
                memory.write_u32(cpu.x[f.rs1] as usize, cpu.x[f.rs2] as u32)?;
                cpu.is_reservation_set = false;
                0
            },
            false => 1
        };
        Ok(())
    }
};
