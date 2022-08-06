use crate::cpu::instruction;
use crate::cpu::instruction::Instruction;

pub const CANONICAL_NAN: u32 = 0x7fc00000;
pub const SIGNALING_NAN: u32 = 0x7fff0000;

pub const FADD_S: Instruction = Instruction {
    name: "FADD.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.get_f32(f.rs1);
        let v2 = cpu.get_f32(f.rs2);

        cpu.set_f32(f.rd, v1 + v2);
        Ok(())
    }
};

pub const FDIV_S: Instruction = Instruction {
    name: "FDIV.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let dividend = cpu.get_f32(f.rs1);
        let divisor = cpu.get_f32(f.rs2);
        // Is this implementation correct?
        if divisor == 0.0 {
            cpu.set_f32(f.rd, f32::INFINITY);
            cpu.set_fcsr_dz();
        } else if divisor == -0.0 {
            cpu.set_f32(f.rd, f32::NEG_INFINITY);
            cpu.set_fcsr_dz();
        } else {
            cpu.set_f32(f.rd, dividend / divisor);
        }

        Ok(())
    }
};

pub const FSUB_S: Instruction = Instruction {
    name: "FSUB.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.get_f32(f.rs1);
        let v2 = cpu.get_f32(f.rs2);

        cpu.set_f32(f.rd, v1 - v2);
        Ok(())
    }
};

pub const FSQRT_S: Instruction = Instruction {
    name: "FSQRT.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v = cpu.get_f32(f.rs1);
        if v >= 0.0 {
            cpu.set_f32(f.rd, v.sqrt());
        } else {
            cpu.set_f32(f.rd, f32::from_bits(CANONICAL_NAN));
            cpu.set_fcsr_nv();
        }
        Ok(())
    }
};

pub const FLW: Instruction = Instruction {
    name: "FLW",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_i(word);
        let value = f32::from_bits(memory.read_u32(cpu.x[f.rs1].wrapping_add(f.imm) as usize)?);
        cpu.set_f32(f.rd, value);
        Ok(())
    }
};

pub const FMUL_S: Instruction = Instruction {
    name: "FMUL.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.get_f32(f.rs1);
        let v2 = cpu.get_f32(f.rs2);

        cpu.set_f32(f.rd, v1 * v2);
        Ok(())
    }
};

pub const FMV_X_W: Instruction = Instruction {
    name: "FMV.X.W",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let value = cpu.f[f.rs1].to_bits() as i32;

        if value as u32 == 0xffc00000 {
            cpu.x[f.rd] = CANONICAL_NAN as i64;
        } else {
            cpu.x[f.rd] = value as i64;
        }
        Ok(())
    }
};

pub const FMV_W_X: Instruction = Instruction {
    name: "FMV.W.X",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd, f32::from_bits(cpu.x[f.rs1] as u32));
        Ok(())
    }
};

pub const FSW: Instruction = Instruction {
    name: "FSW",
    operation: |cpu, memory, word, _address| {
        let f = instruction::parse_format_s(word);
        memory.write_u32(cpu.x[f.rs1].wrapping_add(f.imm) as usize, cpu.f[f.rs2].to_bits() as u32)
    }
};

pub const FSGNJ_S: Instruction = Instruction {
    name: "FSGNJ.S",
    operation: |cpu, _memory, word, _address| {
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
    operation: |cpu, _memory, word, _address| {
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
    operation: |cpu, _memory, word, _address| {
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
    operation: |cpu, _memory, word, _address| {
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
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.get_f32(f.rs1);
        let v2 = cpu.get_f32(f.rs2);
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

pub const FLT_S: Instruction = Instruction {
    name: "FLT.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.get_f32(f.rs1);
        let v2 = cpu.get_f32(f.rs2);
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

pub const FCVT_S_W: Instruction = Instruction {
    name: "FCVT.S.W",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd, cpu.x[f.rs1] as i32 as f32);
        Ok(())
    }
};

pub const FCVT_S_WU: Instruction = Instruction {
    name: "FCVT.S.WU",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd, cpu.x[f.rs1] as u32 as f32);
        Ok(())
    }
};

pub const FCVT_L_S: Instruction = Instruction {
    name: "FCVT.L.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v = cpu.get_f32(f.rs1);

        if v.is_nan() {
            cpu.set_fcsr_nv();
            cpu.x[f.rd] = 0x7fffffffffffffff;
        } else {
            let flags = cpu.read_fflags();
            // it seems the conversion of float values to u64 is setting the NX flag on Intel for
            // things like 1.0 when on RiscV it does not, so we can not rely on the native flag in this case
            cpu.x[f.rd] = v as i64;
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

pub const FCVT_LU_S: Instruction = Instruction {
    name: "FCVT.LU.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v = cpu.get_f32(f.rs1);

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

pub const FCVT_S_L: Instruction = Instruction {
    name: "FCVT.S.L",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd, cpu.x[f.rs1] as f32);
        Ok(())
    }
};

pub const FCVT_S_LU: Instruction = Instruction {
    name: "FCVT.S.LU",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd, cpu.x[f.rs1] as u64 as f32);

        Ok(())
    }
};

pub const FCVT_W_S: Instruction = Instruction {
    name: "FCVT.W.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v = cpu.get_f32(f.rs1);

        if v.is_nan() {
            cpu.set_fcsr_nv();
            cpu.x[f.rd] = 0x7fffffff;
        } else {
            cpu.x[f.rd] = v as i32 as i64;
            if v.fract() != 0.0 {
                cpu.set_fcsr_nx();
            }
        }
        Ok(())
    }
};

pub const FCVT_WU_S: Instruction = Instruction {
    name: "FCVT.WU.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v = cpu.get_f32(f.rs1);

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

pub const FMIN_S: Instruction = Instruction {
    name: "FMIN.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.get_f32(f.rs1);
        let v2 = cpu.get_f32(f.rs2);

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
                result = f32::from_bits(CANONICAL_NAN);
            } else if v1.is_nan() {
                result = v2;
            } else {
                result = v1;
            }
        }

        cpu.set_f32(f.rd, result);
        Ok(())
    }
};


pub const FMAX_S: Instruction = Instruction {
    name: "FMAX.S",
    operation: |cpu, _memory, word, _address| {
        let f = instruction::parse_format_r(word);
        let v1 = cpu.get_f32(f.rs1);
        let v2 = cpu.get_f32(f.rs2);

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
                result = f32::from_bits(CANONICAL_NAN);
            } else if v1.is_nan() {
                result = v2;
            } else {
                result = v1;
            }
        }

        cpu.set_f32(f.rd, result);
        Ok(())
    }
};

pub const FMADD_S: Instruction = Instruction {
    name: "FMADD.S",
    operation: |cpu, _memory, word, _address| {
        // @TODO: Update fcsr if needed?
        let f = instruction::parse_format_r2(word);
        let v = cpu.get_f32(f.rs1) * cpu.get_f32(f.rs2) + cpu.get_f32(f.rs3);
        cpu.set_f32(f.rd, v);
        Ok(())
    }
};

pub const FMSUB_S: Instruction = Instruction {
    name: "FMSUB.S",
    operation: |cpu, _memory, word, _address| {
        // @TODO: Update fcsr if needed?
        let f = instruction::parse_format_r2(word);
        let v = cpu.get_f32(f.rs1) * cpu.get_f32(f.rs2) - cpu.get_f32(f.rs3);
        cpu.set_f32(f.rd, v);
        Ok(())
    }
};

pub const FNMADD_S: Instruction = Instruction {
    name: "FNMADD.S",
    operation: |cpu, _memory, word, _address| {
        // @TODO: Update fcsr if needed?
        let f = instruction::parse_format_r2(word);
        let v = -(cpu.get_f32(f.rs1) * cpu.get_f32(f.rs2)) - cpu.get_f32(f.rs3);
        cpu.set_f32(f.rd, v);
        Ok(())
    }
};

pub const FNMSUB_S: Instruction = Instruction {
    name: "FNMSUB.S",
    operation: |cpu, _memory, word, _address| {
        // @TODO: Update fcsr if needed?
        let f = instruction::parse_format_r2(word);
        let v = -(cpu.get_f32(f.rs1) * cpu.get_f32(f.rs2)) + cpu.get_f32(f.rs3);
        cpu.set_f32(f.rd, v);
        Ok(())
    }
};