use crate::cpu::instruction;
use crate::cpu::instruction::Instruction;

pub const FADD_D: Instruction = Instruction {
    name: "FADD.D",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.f[f.rs1] + cpu.f[f.rs2];
        Ok(())
    }
};

pub const FCVT_D_L: Instruction = Instruction {
    name: "FCVT.D.L",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.x[f.rs1] as f64;
        Ok(())
    }
};

pub const FCVT_D_S: Instruction = Instruction {
    name: "FCVT.D.S",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        // Is this implementation correct?
        cpu.f[f.rd] = f32::from_bits(cpu.f[f.rs1].to_bits() as u32) as f64;
        Ok(())
    }
};

pub const FCVT_D_W: Instruction = Instruction {
    name: "FCVT.D.W",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.x[f.rs1] as i32 as f64;
        Ok(())
    }
};

pub const FCVT_D_WU: Instruction = Instruction {
    name: "FCVT.D.WU",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.x[f.rs1] as u32 as f64;
        Ok(())
    }
};

pub const FCVT_S_D: Instruction = Instruction {
    name: "FCVT.S.D",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        // Is this implementation correct?
        cpu.f[f.rd] = cpu.f[f.rs1] as f32 as f64;
        Ok(())
    }
};

pub const FCVT_W_D: Instruction = Instruction {
    name: "FCVT.W.D",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        // Is this implementation correct?
        cpu.x[f.rd] = cpu.f[f.rs1] as u32 as i32 as i64;
        Ok(())
    }
};

pub const FDIV_D: Instruction = Instruction {
    name: "FDIV.D",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        let dividend = cpu.f[f.rs1];
        let divisor = cpu.f[f.rs2];
        // Is this implementation correct?
        if divisor == 0.0 {
            cpu.f[f.rd] = f64::INFINITY;
            cpu.set_fcsr_dz();
        } else if divisor == -0.0 {
            cpu.f[f.rd] = f64::NEG_INFINITY;
            cpu.set_fcsr_dz();
        } else {
            cpu.f[f.rd] = dividend / divisor;
        }
        Ok(())
    }
};

pub const FEQ_D: Instruction = Instruction {
    name: "FEQ.D",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.f[f.rs1] == cpu.f[f.rs2] {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const FLE_D: Instruction = Instruction {
    name: "FLE.D",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.f[f.rs1] <= cpu.f[f.rs2] {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const FLT_D: Instruction = Instruction {
    name: "FLT.D",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = match cpu.f[f.rs1] < cpu.f[f.rs2] {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const FMADD_D: Instruction = Instruction {
    name: "FMADD.D",
    operation: |cpu, word, _address| {
        // @TODO: Update fcsr if needed?
        let f = instruction::parse_format_r2(word);
        cpu.f[f.rd] = cpu.f[f.rs1] * cpu.f[f.rs2] + cpu.f[f.rs3];
        Ok(())
    }
};

pub const FMUL_D: Instruction = Instruction {
    name: "FMUL.D",
    operation: |cpu, word, _address| {
        // @TODO: Update fcsr if needed?
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.f[f.rs1] * cpu.f[f.rs2];
        Ok(())
    }
};

pub const FMV_D_X: Instruction = Instruction {
    name: "FMV.D.X",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = f64::from_bits(cpu.x[f.rs1] as u64);
        Ok(())
    }
};

pub const FMV_X_D: Instruction = Instruction {
    name: "FMV.X.D",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.f[f.rs1].to_bits() as i64;
        Ok(())
    }
};

pub const FNMSUB_D: Instruction = Instruction {
    name: "FNMSUB.D",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r2(word);
        cpu.f[f.rd] = -(cpu.f[f.rs1] * cpu.f[f.rs2]) + cpu.f[f.rs3];
        Ok(())
    }
};

pub const FSD: Instruction = Instruction {
    name: "FSD",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_s(word);
        unsafe {
            *(cpu.x[f.rs1].wrapping_add(f.imm) as *mut u64) = cpu.f[f.rs2].to_bits();
        }
        Ok(())
    }
};

pub const FLD: Instruction = Instruction {
    name: "FLD",
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        unsafe {
            cpu.f[f.rd] = f64::from_bits(*((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const u64));
        }
        Ok(())
    }
};
