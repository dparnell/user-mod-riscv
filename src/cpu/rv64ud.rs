use crate::cpu::instruction;
use crate::cpu::instruction::Instruction;

pub const CANONICAL_NAN: u64 = 0x7ff8000000000000;
pub const SIGNALING_NAN: u64 = 0x7fff000000000000;


pub const FADD_D: Instruction = Instruction {
    name: "FADD.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.f[f.rs1] + cpu.f[f.rs2];
        Ok(())
    }
};

pub const FCVT_D_L: Instruction = Instruction {
    name: "FCVT.D.L",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.x[f.rs1] as f64;
        Ok(())
    }
};

pub const FCVT_D_LU: Instruction = Instruction {
    name: "FCVT.D.LU",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.x[f.rs1] as u64 as f64;
        Ok(())
    }
};

pub const FCVT_D_S: Instruction = Instruction {
    name: "FCVT.D.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v = cpu.get_f32(f.rs1);
        if v.is_nan() {
            cpu.f[f.rd] = f64::from_bits(CANONICAL_NAN);
        } else {
            cpu.f[f.rd] = v as f64;
        }
        Ok(())
    }
};

pub const FCVT_D_W: Instruction = Instruction {
    name: "FCVT.D.W",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.x[f.rs1] as i32 as f64;
        Ok(())
    }
};

pub const FCVT_D_WU: Instruction = Instruction {
    name: "FCVT.D.WU",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.x[f.rs1] as u32 as f64;
        Ok(())
    }
};

pub const FCVT_S_D: Instruction = Instruction {
    name: "FCVT.S.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd,cpu.f[f.rs1] as f32);
        Ok(())
    }
};

pub const FCVT_W_D: Instruction = Instruction {
    name: "FCVT.W.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v = cpu.f[f.rs1];

        if v.is_nan() {
            cpu.set_fcsr_nv();
            cpu.x[f.rd] = 0x7fffffff;
        } else {
            // println!("********** v={} i={}", v, v as i32 as i64);
            cpu.x[f.rd] = v as i32 as i64;
            if v.fract() != 0.0 {
                cpu.set_fcsr_nx();
            }
            if v > i32::MAX as f64 || v < i32::MIN as f64 {
                cpu.set_fcsr_nv();
            }
        }
        Ok(())
    }
};

pub const FDIV_D: Instruction = Instruction {
    name: "FDIV.D",
    operation: |cpu, _memory, word, _address| {
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
    operation: |cpu, _memory, word, _address| {
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
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.f[f.rs1];
        let v2 = cpu.f[f.rs2];
        if v1.is_nan() || v2.is_nan() {
            cpu.set_fcsr_nv();
        }

        cpu.x[f.rd] = match v1 <= v2 {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const FLT_D: Instruction = Instruction {
    name: "FLT.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.f[f.rs1];
        let v2 = cpu.f[f.rs2];
        if v1.is_nan() || v2.is_nan() {
            cpu.set_fcsr_nv();
        }

        cpu.x[f.rd] = match v1 < v2 {
            true => 1,
            false => 0
        };
        Ok(())
    }
};

pub const FMADD_D: Instruction = Instruction {
    name: "FMADD.D",
    operation: |cpu, _memory, word, _address| {
        // @TODO: Update fcsr if needed?
        let f = instruction::parse_format_r2(word);
        cpu.f[f.rd] = cpu.f[f.rs1] * cpu.f[f.rs2] + cpu.f[f.rs3];
        Ok(())
    }
};

pub const FMSUB_D: Instruction = Instruction {
    name: "FMSUB.D",
    operation: |cpu, _memory, word, _address| {
        // @TODO: Update fcsr if needed?
        let f = instruction::parse_format_r2(word);
        cpu.f[f.rd] = cpu.f[f.rs1] * cpu.f[f.rs2] - cpu.f[f.rs3];
        Ok(())
    }
};


pub const FMUL_D: Instruction = Instruction {
    name: "FMUL.D",
    operation: |cpu, _memory, word, _address| {
        // @TODO: Update fcsr if needed?
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = cpu.f[f.rs1] * cpu.f[f.rs2];
        Ok(())
    }
};

pub const FMV_D_X: Instruction = Instruction {
    name: "FMV.D.X",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.f[f.rd] = f64::from_bits(cpu.x[f.rs1] as u64);
        Ok(())
    }
};

pub const FMV_X_D: Instruction = Instruction {
    name: "FMV.X.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.x[f.rd] = cpu.f[f.rs1].to_bits() as i64;
        Ok(())
    }
};

pub const FNMADD_D: Instruction = Instruction {
    name: "FNMADD.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r2(word);
        cpu.f[f.rd] = -(cpu.f[f.rs1] * cpu.f[f.rs2]) - cpu.f[f.rs3];
        Ok(())
    }
};

pub const FNMSUB_D: Instruction = Instruction {
    name: "FNMSUB.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r2(word);
        cpu.f[f.rd] = -(cpu.f[f.rs1] * cpu.f[f.rs2]) + cpu.f[f.rs3];
        Ok(())
    }
};

pub const FSD: Instruction = Instruction {
    name: "FSD",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_s(word);
        memory.write_u64(cpu.x[f.rs1].wrapping_add(f.imm) as usize, cpu.f[f.rs2].to_bits())
    }
};

pub const FLD: Instruction = Instruction {
    name: "FLD",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_i(word);
        cpu.f[f.rd] = f64::from_bits(memory.read_u64(cpu.x[f.rs1].wrapping_add(f.imm) as usize)?);
        Ok(())
    }
};

pub const FSUB_D: Instruction = Instruction {
    name: "FSUB.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.f[f.rs1];
        let v2 = cpu.f[f.rs2];

        if v1.is_infinite() && v2.is_infinite() {
            cpu.f[f.rd] = f64::from_bits(CANONICAL_NAN);
            cpu.set_fcsr_nv();
        } else {
            cpu.f[f.rd] = v1 - v2;
        }

        Ok(())
    }
};

pub const FSQRT_D: Instruction = Instruction {
    name: "FSQRT.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);

        let v = cpu.f[f.rs1];
        if v >= 0.0 {
            cpu.f[f.rd] = v.sqrt();
        } else {
            cpu.f[f.rd] = f64::from_bits(CANONICAL_NAN);
            cpu.set_fcsr_nv();
        }
        Ok(())
    }
};

pub const FSGNJ_D: Instruction = Instruction {
    name: "FSGNJ.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let rs1_bits = cpu.f[f.rs1].to_bits();
        let rs2_bits = cpu.f[f.rs2].to_bits();
        let sign_bit = rs2_bits & 0x8000000000000000;
        cpu.f[f.rd] = f64::from_bits(sign_bit | (rs1_bits & 0x7fffffffffffffff));
        Ok(())
    }
};

pub const FSGNJN_D: Instruction = Instruction {
    name: "FSGNJN.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let rs1_bits = cpu.f[f.rs1].to_bits();
        let rs2_bits = cpu.f[f.rs2].to_bits();
        let sign_bit = (rs2_bits & 0x8000000000000000) ^ 0x8000000000000000;
        cpu.f[f.rd] = f64::from_bits(sign_bit | (rs1_bits & 0x7fffffffffffffff));
        Ok(())
    }
};

pub const FSGNJX_D: Instruction = Instruction {
    name: "FSGNJX.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let rs1_bits = cpu.f[f.rs1].to_bits();
        let rs2_bits = cpu.f[f.rs2].to_bits();
        let sign_bit = (rs1_bits ^ rs2_bits) & 0x8000000000000000;

        cpu.f[f.rd] = f64::from_bits(sign_bit | (rs1_bits & 0x7fffffffffffffff));
        Ok(())
    }
};

pub const FMIN_D: Instruction = Instruction {
    name: "FMIN.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.f[f.rs1];
        let v2 = cpu.f[f.rs2];

        let mut result = match (v1.is_sign_positive(), v2.is_sign_positive()) {
            (true, true) => match v1 < v2 {
                true => v1,
                false => v2
            },
            (true, false) => v2,
            (false, true) => v1,
            (false, false) => match v1 < v2 {
                true => v1,
                false => v2
            }
        };

        if v1.is_nan() || v2.is_nan() {
            if v1.is_nan() && v2.is_nan() {
                if v1.to_bits() == SIGNALING_NAN || v2.to_bits() == SIGNALING_NAN {
                    cpu.set_fcsr_nv();
                }
                result = f64::from_bits(CANONICAL_NAN);
            } else if v1.is_nan() {
                result = v2;
            } else {
                result = v1;
            }
        }

        cpu.f[f.rd] = result;
        Ok(())
    }
};


pub const FMAX_D: Instruction = Instruction {
    name: "FMAX.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.f[f.rs1];
        let v2 = cpu.f[f.rs2];

        let mut result = match (v1.is_sign_positive(), v2.is_sign_positive()) {
            (true, true) => match v1 > v2 {
                true => v1,
                false => v2
            },
            (true, false) => v1,
            (false, true) => v2,
            (false, false) => match v1 > v2 {
                true => v1,
                false => v2
            }
        };

        if v1.is_nan() || v2.is_nan() {
            if v1.is_nan() && v2.is_nan() {
                if v1.to_bits() == SIGNALING_NAN || v2.to_bits() == SIGNALING_NAN {
                    cpu.set_fcsr_nv();
                }
                result = f64::from_bits(CANONICAL_NAN);
            } else if v1.is_nan() {
                result = v2;
            } else {
                result = v1;
            }
        }

        cpu.f[f.rd] = result;
        Ok(())
    }
};

pub const FCVT_WU_D: Instruction = Instruction {
    name: "FCVT.WU.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v = cpu.f[f.rs1];

        if v.is_nan() || v <= -1.0 {
            cpu.set_fcsr_nv();

            if v.is_nan() {
                cpu.x[f.rd] = -1;
            } else {
                cpu.x[f.rd] = 0;
            }
        } else {
            let u = v as u32;

            // apparently we need to sign extend the value
            let upper: u64 = match u & 0x80000000 {
                0 => 0,
                _ => 0xffffffff00000000
            };

            cpu.x[f.rd] = (u as u64 | upper) as i64;
            if v.fract() != 0.0 {
                cpu.set_fcsr_nx();
            }
        }
        Ok(())
    }
};

pub const FCVT_L_D: Instruction = Instruction {
    name: "FCVT.L.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v = cpu.f[f.rs1];

        if v.is_nan() {
            cpu.set_fcsr_nv();
            cpu.x[f.rd] = 0x7fffffffffffffff;
        } else {
            cpu.x[f.rd] = v as i64;
            if v.fract() != 0.0 {
                cpu.set_fcsr_nx();
            }
        }
        Ok(())
    }
};

pub const FCVT_LU_D: Instruction = Instruction {
    name: "FCVT.LU.D",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v = cpu.f[f.rs1];

        if v.is_nan() || v <= -1.0 {
            cpu.set_fcsr_nv();
            if v.is_nan() {
                cpu.x[f.rd] = -1;
            } else {
                cpu.x[f.rd] = 0;
            }
        } else {
            let flags = cpu.read_fflags();
            // it seems the conversion of float values to u64 is setting the NX flag on Intel for
            // things like 1.0 when on RiscV it does not, so we can not rely on the native flag in this case
            cpu.x[f.rd] = v as u64 as i64;
            if v.fract() != 0.0 {
                cpu.set_fcsr_nx();
            } else {
                let new_flags = cpu.read_fflags();

                if new_flags & 1 == 1 && flags & 1 == 0 {
                    cpu.write_fflags(flags);
                }
            }
        }
        Ok(())
    }
};