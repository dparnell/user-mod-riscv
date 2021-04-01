use crate::cpu::instruction;
use crate::cpu::instruction::Instruction;

pub const FADD_S: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_r(word);
        cpu.set_f32(f.rd, cpu.get_f32(f.rs1) + cpu.get_f32(f.rs2));
        Ok(())
    }
};

pub const FLW: Instruction = Instruction {
    operation: |cpu, word, _address| {
        let f = instruction::parse_format_i(word);
        unsafe {
            // this seems a bit odd to me
            cpu.f[f.rd] = f64::from_bits(*((cpu.x[f.rs1].wrapping_add(f.imm) as u64) as *const i32) as i64 as u64);
        }
        Ok(())
    }
};
