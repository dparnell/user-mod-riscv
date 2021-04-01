use crate::cpu::{instruction, Xlen};
use crate::cpu::instruction::Instruction;

pub const ADD: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_add(cpu.x[f.rs2]));
        Ok(())
    }
};

pub const ADDI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_add(f.imm));
        Ok(())
    }
};

pub const ADDIW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = cpu.x[f.rs1].wrapping_add(f.imm) as i32 as i64;
        Ok(())
    }
};

pub const ADDW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.x[f.rs1].wrapping_add(cpu.x[f.rs2]) as i32 as i64;
        Ok(())
    }
};

pub const AND: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] & cpu.x[f.rs2]);
        Ok(())
    }
};

pub const ANDI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] & f.imm);
        Ok(())
    }
};

pub const AUIPC: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = instruction::parse_format_u(word);
        cpu.x[f.rd] = cpu.sign_extend(address.wrapping_add(f.imm as usize) as i64);
        Ok(())
    }
};

pub const BEQ: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) == cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

pub const BGE: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) >= cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

pub const BGEU: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.unsigned_data(cpu.x[f.rs1]) >= cpu.unsigned_data(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

pub const BLT: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) < cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

pub const BLTU: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.unsigned_data(cpu.x[f.rs1]) < cpu.unsigned_data(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

pub const BNE: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = instruction::parse_format_b(word);
        if cpu.sign_extend(cpu.x[f.rs1]) != cpu.sign_extend(cpu.x[f.rs2]) {
            cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        }
        Ok(())
    }
};

pub const FENCE: Instruction = Instruction {
    operation: |_cpu, _word, _address| {
        // Do nothing
        Ok(())
    }
};


pub const JAL: Instruction = Instruction {
    operation: |cpu, word, address| {
        let f = instruction::parse_format_j(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.pc as i64);
        cpu.pc = address.wrapping_add(f.imm as usize) as *mut u8;
        Ok(())
    }
};

pub const JALR: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        let tmp = cpu.sign_extend(cpu.pc as i64);
        cpu.pc = (cpu.x[f.rs1] as u64).wrapping_add(f.imm as u64) as *mut u8;
        cpu.x[f.rd] = tmp;
        Ok(())
    }
};

pub const LB: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const i8) as i64;
        }
        Ok(())
    }
};

pub const LBU: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const u8) as i64;
        }
        Ok(())
    }
};

pub const LD: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const i64);
        }
        Ok(())
    }
};

pub const LH: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const i16) as i64;
        }
        Ok(())
    }
};

pub const LHU: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const u16) as i64;
        }
        Ok(())
    }
};

pub const LW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const i32) as i64;
        }
        Ok(())
    }
};

pub const LWU: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        unsafe {
            cpu.x[f.rd] = *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const u32) as i64;
        }
        Ok(())
    }
};

pub const LUI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_u(word);
        cpu.x[f.rd] = f.imm as i64;
        Ok(())
    }
};

pub const OR: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] | cpu.x[f.rs2]);
        Ok(())
    }
};

pub const ORI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] | f.imm);
        Ok(())
    }
};

pub const SB: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_s(word);

        unsafe {
            *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *mut u8) = cpu.x[f.rs2] as u8;
        }
        Ok(())
    }
};

pub const SD: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_s(word);

        unsafe {
            *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *mut u64) = cpu.x[f.rs2] as u64;
        }
        Ok(())
    }
};

pub const SH: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_s(word);

        unsafe {
            *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *mut u16) = cpu.x[f.rs2] as u16;
        }
        Ok(())
    }
};

pub const SLL: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_shl(cpu.x[f.rs2] as u32));
        Ok(())
    }
};

pub const SLLI: Instruction = Instruction {
    operation: |cpu, word, _address| {
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
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let shamt = f.rs2 as u32;
        cpu.x[f.rd] = (cpu.x[f.rs1] << shamt) as i32 as i64;
        Ok(())
    }
};

pub const SLLW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = (cpu.x[f.rs1] as u32).wrapping_shl(cpu.x[f.rs2] as u32) as i32 as i64;
        Ok(())
    }
};

pub const SLTI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = match cpu.x[f.rs1] < f.imm {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const SLT: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.x[f.rs1] < cpu.x[f.rs2] {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const SLTIU: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = match cpu.unsigned_data(cpu.x[f.rs1]) < cpu.unsigned_data(f.imm) {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const SLTU: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.unsigned_data(cpu.x[f.rs1]) < cpu.unsigned_data(cpu.x[f.rs2]) {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const SRA: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_shr(cpu.x[f.rs2] as u32));
        Ok(())
    }
};

pub const SRAI: Instruction = Instruction {
    operation: |cpu, word, _address| {
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
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let shamt = ((word >> 20) & 0x1f) as u32;
        cpu.x[f.rd] = ((cpu.x[f.rs1] as i32) >> shamt) as i64;
        Ok(())
    }
};

pub const SRAW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = (cpu.x[f.rs1] as i32).wrapping_shr(cpu.x[f.rs2] as u32) as i64;
        Ok(())
    }
};


pub const SRL: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.unsigned_data(cpu.x[f.rs1]).wrapping_shr(cpu.x[f.rs2] as u32) as i64);
        Ok(())
    }
};

pub const SRLI: Instruction = Instruction {
    operation: |cpu, word, _address| {
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
    operation: |cpu, word, _address| {
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
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = (cpu.x[f.rs1] as u32).wrapping_shr(cpu.x[f.rs2] as u32) as i32 as i64;
        Ok(())
    }
};

pub const SUB: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1].wrapping_sub(cpu.x[f.rs2]));
        Ok(())
    }
};

pub const SUBW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.x[f.rs1].wrapping_sub(cpu.x[f.rs2]) as i32 as i64;
        Ok(())
    }
};

pub const SW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_s(word);

        unsafe {
            *((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *mut u32) = cpu.x[f.rs2] as u32;
        }
        Ok(())
    }
};

pub const XOR: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] ^ cpu.x[f.rs2]);
        Ok(())
    }
};

pub const XORI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.x[f.rd] = cpu.sign_extend(cpu.x[f.rs1] ^ f.imm);
        Ok(())
    }
};

pub const CSRRC: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr) as i64;
        let tmp = cpu.x[f.rs];
        cpu.x[f.rd] = cpu.sign_extend(data);
        cpu.write_csr(f.csr, (cpu.x[f.rd] & !tmp) as u64);

        Ok(())
    }
};

pub const CSRRCI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr);
        cpu.x[f.rd] = cpu.sign_extend(data as i64);
        cpu.write_csr(f.csr, (cpu.x[f.rd] & !(f.rs as i64)) as u64);
        Ok(())
    }
};

pub const CSRRS: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr);
        let tmp = cpu.x[f.rs];
        cpu.x[f.rd] = cpu.sign_extend(data as i64);
        cpu.write_csr(f.csr, cpu.unsigned_data(cpu.x[f.rd] | tmp));
        Ok(())
    }
};

pub const CSRRSI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr);
        cpu.x[f.rd] = cpu.sign_extend(data as i64);
        cpu.write_csr(f.csr, cpu.unsigned_data(cpu.x[f.rd] | (f.rs as i64)));
        Ok(())
    }
};

pub const CSRRW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr);
        let tmp = cpu.x[f.rs];
        cpu.x[f.rd] = cpu.sign_extend(data as i64);
        cpu.write_csr(f.csr, cpu.unsigned_data(tmp));
        Ok(())
    }
};

pub const CSRRWI: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_csr(word);
        let data = cpu.read_csr(f.csr);
        cpu.x[f.rd] = cpu.sign_extend(data as i64);
        cpu.write_csr(f.csr, f.rs as u64);
        Ok(())
    }
};

pub const EBREAK: Instruction = Instruction {
    operation: |_cpu, _word, _address| {
        // TODO: implement debugger?
        Ok(())
    }
};

pub const ECALL: Instruction = Instruction {
    operation: |cpu, word, address| {
        if let Some(handler) = &cpu.ecall_handler {
            (handler.operation)(cpu, word, address)
        } else {
            Ok(())
        }
    }
};

pub const FENCE_I: Instruction = Instruction {
    operation: |_cpu, _word, _address| {
        // Do nothing
        Ok(())
    }
};
