use crate::cpu::instruction;
use crate::cpu::instruction::Instruction;

pub const FADD_S: Instruction = Instruction {
    name: "FADD.S",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);

        cpu.set_f32(f.rd, cpu.get_f32(f.rs1) + cpu.get_f32(f.rs2));
        Ok(())
    }
};

pub const FSUB_S: Instruction = Instruction {
    name: "FSUB.S",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);

        cpu.set_f32(f.rd, cpu.get_f32(f.rs1) - cpu.get_f32(f.rs2));
        Ok(())
    }
};

pub const FLW: Instruction = Instruction {
    name: "FLW",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        unsafe {
            // this seems a bit odd to me
            cpu.f[f.rd] = f64::from_bits(*((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const i32) as i64 as u64);
        }
        Ok(())
    }
};

pub const FMUL_S: Instruction = Instruction {
    name: "FMUL.S",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd, cpu.get_f32(f.rs1) * cpu.get_f32(f.rs2));
        Ok(())
    }
};

pub const FMV_X_W: Instruction = Instruction {
    name: "FMV.X.W",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.f[f.rs1].to_bits() as i32 as i64;
        Ok(())
    }
};

pub const FMV_W_X: Instruction = Instruction {
    name: "FMV.W.X",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd, f32::from_bits(cpu.x[f.rs1] as u32));
        Ok(())
    }
};

pub const FSW: Instruction = Instruction {
    name: "FSW",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_s(word);
        unsafe {
            *(cpu.x[f.rs1].wrapping_add(f.imm) as *mut u32) = cpu.f[f.rs2].to_bits() as u32;
        }
        Ok(())
    }
};

pub const FSGNJ_S: Instruction = Instruction {
    name: "FSGNJ.S",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let rs1_bits = cpu.get_f32(f.rs1).to_bits();
        let rs2_bits = cpu.get_f32(f.rs2).to_bits();
        let sign_bit = rs2_bits & 0x80000000;
        cpu.set_f32(f.rd, f32::from_bits(sign_bit | (rs1_bits & 0x7fffffff)));
        Ok(())
    }
};

pub const FSGNJN_S: Instruction = Instruction {
    name: "FSGNJN.S",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let rs1_bits = cpu.get_f32(f.rs1).to_bits();
        let rs2_bits = cpu.get_f32(f.rs2).to_bits();
        let sign_bit = (rs2_bits & 0x80000000) ^ 0x80000000;
        cpu.set_f32(f.rd, f32::from_bits(sign_bit | (rs1_bits & 0x7fffffff)));
        Ok(())
    }
};

pub const FSGNJX_S: Instruction = Instruction {
    name: "FSGNJX.S",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let rs1_bits = cpu.get_f32(f.rs1).to_bits();
        let rs2_bits = cpu.get_f32(f.rs2).to_bits();
        let sign_bit = (rs1_bits ^ rs2_bits) & 0x80000000;

        cpu.set_f32(f.rd, f32::from_bits(sign_bit | rs1_bits & 0x7fffffff));
        Ok(())
    }
};

pub const FEQ_S: Instruction = Instruction {
    name: "FEQ.S",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.get_f32(f.rs1) == cpu.get_f32(f.rs2) {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const FLE_S: Instruction = Instruction {
    name: "FLE.S",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.get_f32(f.rs1) <= cpu.get_f32(f.rs2) {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const FLT_S: Instruction = Instruction {
    name: "FLT.S",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.get_f32(f.rs1) < cpu.get_f32(f.rs2) {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const FCVT_S_W: Instruction = Instruction {
    name: "FCVT.S.W",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd, cpu.x[f.rs1] as i32 as f32);
        Ok(())
    }
};

pub const FCVT_S_WU: Instruction = Instruction {
    name: "FCVT.S.WU",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd, cpu.x[f.rs1] as u32 as f32);
        Ok(())
    }
};

