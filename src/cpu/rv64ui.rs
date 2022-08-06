use crate::cpu::{instruction, Xlen};
use crate::cpu::instruction::Instruction;

pub const ADD: Instruction = Instruction {
    name: "ADD",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_add(cpu.x[f.rs2]));
        Ok(())
    }
};

pub const ADDI: Instruction = Instruction {
    name: "ADDI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_add(f.imm));
        Ok(())
    }
};

pub const ADDIW: Instruction = Instruction {
    name: "ADDIW",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = cpu.x[f.rs1].wrapping_add(f.imm) as i32 as i64;
        Ok(())
    }
};

pub const ADDW: Instruction = Instruction {
    name: "ADDW",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.x[f.rs1].wrapping_add(cpu.x[f.rs2]) as i32 as i64;
        Ok(())
    }
};

pub const AND: Instruction = Instruction {
    name: "AND",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] & cpu.x[f.rs2]);
        Ok(())
    }
};

pub const ANDI: Instruction = Instruction {
    name: "ANDI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] & f.imm);
        Ok(())
    }
};

pub const AUIPC: Instruction = Instruction {
    name: "AUIPC",
    operation: |cpu, _memory, word, address| {
        let f = instruction::parse_format_u(word);
        cpu.x[f.rd] = cpu.sign_extend(address.wrapping_add(f.imm as usize) as i64);
        Ok(())
    }
};

pub const BEQ: Instruction = Instruction {
    name: "BEQ",
    operation: |cpu, _memory, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) == cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize);
        }
        Ok(())
    }
};

pub const BGE: Instruction = Instruction {
    name: "BGE",
    operation: |cpu, _memory, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) >= cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize);
        }
        Ok(())
    }
};

pub const BGEU: Instruction = Instruction {
    name: "BGEU",
    operation: |cpu, _memory, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.unsigned_data(cpu.x[f.rs1]) >= cpu.unsigned_data(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize);
        }
        Ok(())
    }
};

pub const BLT: Instruction = Instruction {
    name: "BLT",
    operation: |cpu, _memory, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) < cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize);
        }
        Ok(())
    }
};

pub const BLTU: Instruction = Instruction {
    name: "BLTU",
    operation: |cpu, _memory, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.unsigned_data(cpu.x[f.rs1]) < cpu.unsigned_data(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize);
        }
        Ok(())
    }
};

pub const BNE: Instruction = Instruction {
    name: "BNE",
    operation: |cpu, _memory, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) != cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize);
        }
        Ok(())
    }
};

pub const FENCE: Instruction = Instruction {
    name: "FENCE",
    operation: |_cpu, _memory, _word, _address| {
        // Do nothing
        Ok(())
    }
};


pub const JAL: Instruction = Instruction {
    name: "JAL",
    operation: |cpu, _memory, word, address| {
        let f = instruction::parse_format_j(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.pc as i64);
        cpu.pc = address.wrapping_add(f.imm as usize);
        Ok(())
    }
};

pub const JALR: Instruction = Instruction {
    name: "JALR",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_i(word);
        let tmp = cpu.sign_extend(cpu.pc as i64);
        cpu.pc = (cpu.x[f.rs1] as u64).wrapping_add(f.imm as u64) as usize;
        cpu.x[f.rd] = tmp;
        Ok(())
    }
};

pub const LB: Instruction = Instruction {
    name: "LB",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = memory.read_i8(cpu.x[f.rs1].wrapping_add(f.imm) as usize)? as i64;
        Ok(())
    }
};

pub const LBU: Instruction = Instruction {
    name: "LBU",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = memory.read_u8(cpu.x[f.rs1].wrapping_add(f.imm) as usize)? as i64;
        Ok(())
    }
};

pub const LD: Instruction = Instruction {
    name: "LD",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = memory.read_i64(cpu.x[f.rs1].wrapping_add(f.imm) as usize)?;
        Ok(())
    }
};

pub const LH: Instruction = Instruction {
    name: "LH",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = memory.read_i16(cpu.x[f.rs1].wrapping_add(f.imm) as usize)? as i64;
        Ok(())
    }
};

pub const LHU: Instruction = Instruction {
    name: "LHU",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = memory.read_u16(cpu.x[f.rs1].wrapping_add(f.imm) as usize)? as i64;
        Ok(())
    }
};

pub const LW: Instruction = Instruction {
    name: "LW",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = memory.read_i32(cpu.x[f.rs1].wrapping_add(f.imm) as usize)? as i64;
        Ok(())
    }
};

pub const LWU: Instruction = Instruction {
    name: "LWU",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = memory.read_u32(cpu.x[f.rs1].wrapping_add(f.imm) as usize)? as i64;
        Ok(())
    }
};

pub const LUI: Instruction = Instruction {
    name: "LUI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_u(word);
        cpu.x[f.rd] = f.imm as i64;
        Ok(())
    }
};

pub const OR: Instruction = Instruction {
    name: "OR",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] | cpu.x[f.rs2]);
        Ok(())
    }
};

pub const ORI: Instruction = Instruction {
    name: "ORI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] | f.imm);
        Ok(())
    }
};

pub const SB: Instruction = Instruction {
    name: "SB",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_s(word);
        memory.write_u8(cpu.x[f.rs1].wrapping_add(f.imm) as usize, cpu.x[f.rs2] as u8)
    }
};

pub const SD: Instruction = Instruction {
    name: "SD",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_s(word);
        memory.write_u64(cpu.x[f.rs1].wrapping_add(f.imm) as usize, cpu.x[f.rs2] as u64)
    }
};

pub const SH: Instruction = Instruction {
    name: "SH",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_s(word);
        memory.write_u16(cpu.x[f.rs1].wrapping_add(f.imm) as usize, cpu.x[f.rs2] as u16)
    }
};

pub const SLL: Instruction = Instruction {
    name: "SLL",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_shl(cpu.x[f.rs2] as u32));
        Ok(())
    }
};

pub const SLLI: Instruction = Instruction {
    name: "SLLI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let mask = match cpu.xlen {
            Xlen::Bit32 => 0x1f,
            Xlen::Bit64 => 0x3f
        };
        let shamt = (word >> 20) & mask;
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] << shamt);
        Ok(())
    }
};

pub const SLLIW: Instruction = Instruction {
    name: "SLLIW",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let shamt = f.rs2 as u32;
        cpu.x[f.rd] = (cpu.x[f.rs1] << shamt) as i32 as i64;
        Ok(())
    }
};

pub const SLLW: Instruction = Instruction {
    name: "SLLW",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = (cpu.x[f.rs1] as u32).wrapping_shl(cpu.x[f.rs2] as u32) as i32 as i64;
        Ok(())
    }
};

pub const SLTI: Instruction = Instruction {
    name: "SLTI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = match cpu.x[f.rs1] < f.imm {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const SLT: Instruction = Instruction {
    name: "SLT",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.x[f.rs1] < cpu.x[f.rs2] {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const SLTIU: Instruction = Instruction {
    name: "SLTIU",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = match cpu.unsigned_data(cpu.x[f.rs1]) < cpu.unsigned_data(f.imm) {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const SLTU: Instruction = Instruction {
    name: "SLTU",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.unsigned_data(cpu.x[f.rs1]) < cpu.unsigned_data(cpu.x[f.rs2]) {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const SRA: Instruction = Instruction {
    name: "SRA",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_shr(cpu.x[f.rs2] as u32));
        Ok(())
    }
};

pub const SRAI: Instruction = Instruction {
    name: "SRAI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let mask = match cpu.xlen {
            Xlen::Bit32 => 0x1f,
            Xlen::Bit64 => 0x3f
        };
        let shamt = (word >> 20) & mask;
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] >> shamt);
        Ok(())
    }
};

pub const SRAIW: Instruction = Instruction {
    name: "SRAIW",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let shamt = ((word >> 20) & 0x1f) as u32;
        cpu.x[f.rd] = ((cpu.x[f.rs1] as i32) >> shamt) as i64;
        Ok(())
    }
};

pub const SRAW: Instruction = Instruction {
    name: "SRAW",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = (cpu.x[f.rs1] as i32).wrapping_shr(cpu.x[f.rs2] as u32) as i64;
        Ok(())
    }
};


pub const SRL: Instruction = Instruction {
    name: "SRL",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.unsigned_data(cpu.x[f.rs1]).wrapping_shr(cpu.x[f.rs2] as u32) as i64);
        Ok(())
    }
};

pub const SRLI: Instruction = Instruction {
    name: "SRLI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let mask = match cpu.xlen {
            Xlen::Bit32 => 0x1f,
            Xlen::Bit64 => 0x3f
        };
        let shamt = (word >> 20) & mask;
        cpu.x[f.rd] = cpu.sign_extend((cpu.unsigned_data(cpu.x[f.rs1]) >> shamt) as i64);
        Ok(())
    }
};

pub const SRLIW: Instruction = Instruction {
    name: "SRLIW",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let mask = match cpu.xlen {
            Xlen::Bit32 => 0x1f,
            Xlen::Bit64 => 0x3f
        };
        let shamt = (word >> 20) & mask;
        cpu.x[f.rd] = ((cpu.x[f.rs1] as u32) >> shamt) as i32 as i64;
        Ok(())
    }
};

pub const SRLW: Instruction = Instruction {
    name: "SRLW",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = (cpu.x[f.rs1] as u32).wrapping_shr(cpu.x[f.rs2] as u32) as i32 as i64;
        Ok(())
    }
};

pub const SUB: Instruction = Instruction {
    name: "SUB",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_sub(cpu.x[f.rs2]));
        Ok(())
    }
};

pub const SUBW: Instruction = Instruction {
    name: "SUBW",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.x[f.rs1].wrapping_sub(cpu.x[f.rs2]) as i32 as i64;
        Ok(())
    }
};

pub const SW: Instruction = Instruction {
    name: "SW",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_s(word);
        memory.write_u32(cpu.x[f.rs1].wrapping_add(f.imm) as usize, cpu.x[f.rs2] as u32)
    }
};

pub const XOR: Instruction = Instruction {
    name: "XOR",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] ^ cpu.x[f.rs2]);
        Ok(())
    }
};

pub const XORI: Instruction = Instruction {
    name: "XORI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] ^ f.imm);
        Ok(())
    }
};

pub const CSRRC: Instruction = Instruction {
    name: "CSRRC",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr) as i64;
        let tmp = cpu.x[f.rs];
        cpu.x[f.rd] = cpu.sign_extend(data);
        cpu.write_csr(f.csr, (cpu.x[f.rd] & !tmp) as u64);

        Ok(())
    }
};

pub const CSRRCI: Instruction = Instruction {
    name: "CSRRCI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr);
        cpu.x[f.rd] = cpu.sign_extend(data as i64);
        cpu.write_csr(f.csr, (cpu.x[f.rd] & !(f.rs as i64)) as u64);
        Ok(())
    }
};

pub const CSRRS: Instruction = Instruction {
    name: "CSRRS",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr);
        let tmp = cpu.x[f.rs];
        cpu.x[f.rd] = cpu.sign_extend(data as i64);
        cpu.write_csr(f.csr, cpu.unsigned_data(cpu.x[f.rd] | tmp));
        Ok(())
    }
};

pub const CSRRSI: Instruction = Instruction {
    name: "CSRRSI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr);
        cpu.x[f.rd] = cpu.sign_extend(data as i64);
        cpu.write_csr(f.csr, cpu.unsigned_data(cpu.x[f.rd] | (f.rs as i64)));
        Ok(())
    }
};

pub const CSRRW: Instruction = Instruction {
    name: "CSRRW",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr);
        let tmp = cpu.x[f.rs];
        cpu.x[f.rd] = cpu.sign_extend(data as i64);
        cpu.write_csr(f.csr, cpu.unsigned_data(tmp));
        Ok(())
    }
};

pub const CSRRWI: Instruction = Instruction {
    name: "CSRRWI",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr);
        cpu.x[f.rd] = cpu.sign_extend(data as i64);
        cpu.write_csr(f.csr, f.rs as u64);
        Ok(())
    }
};

pub const EBREAK: Instruction = Instruction {
    name: "EBREAK",
    operation: |_cpu, _memory, _word, _address| {
        // TODO: implement debugger?
        Ok(())
    }
};

pub const ECALL: Instruction = Instruction {
    name: "ECALL",
    operation: |cpu, memory, word, address| {
        if let Some(handler) = &cpu.ecall_handler {
            (handler.operation)(cpu, memory, word, address)
        } else {
            Ok(())
        }
    }
};

pub const FENCE_I: Instruction = Instruction {
    name: "FENCE.I",
    operation: |_cpu, _memory, _word, _address| {
        // Do nothing
        Ok(())
    }
};
