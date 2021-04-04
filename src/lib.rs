pub mod cpu;

#[cfg(test)]
mod test {
    extern crate elfloader;

    use super::cpu::*;
    use super::cpu::instruction::Instruction;

    use elfloader::*;

    const IMG_BASE: u64 = 0x80000000;
    const MAX_SIZE: usize = 1024 * 32;
    struct RVTestElfLoader {
        target: [u8; MAX_SIZE],
    }

    impl RVTestElfLoader {
        pub fn new() -> Self {
            RVTestElfLoader {
                target: [0; MAX_SIZE]
            }
        }

        pub fn get_target(&mut self) -> *mut u32 {
            unsafe {
                std::mem::transmute::<&u8, *mut u32>(&self.target[0])
            }
        }
    }

    impl ElfLoader for RVTestElfLoader {
        fn allocate(&mut self, _load_headers: LoadableHeaders) -> Result<(), &'static str> {
            /*
            for header in load_headers {
                println!(
                    "allocate base = {:#x} size = {:#x} flags = {}",
                    header.virtual_addr(),
                    header.mem_size(),
                    header.flags()
                );
            }
            */
            Ok(())
        }

        fn relocate(&mut self, _entry: &Rela<P64>) -> Result<(), &'static str> {
            // let typ = TypeRela64::from(entry.get_type());
            // let addr: *mut u64 = (self.vbase + entry.get_offset()) as *mut u64;

            Err("Unexpected relocation encountered")

        }

        fn load(&mut self, _flags: Flags, base: VAddr, region: &[u8]) -> Result<(), &'static str> {
            let start = base - IMG_BASE;
            let end = start + region.len() as u64;
            if end < MAX_SIZE as u64 {
                // println!("Loading region from {:#x} into {:?} with {:?} bytes", base, start, region.len());
                for i in 0..region.len() {
                    self.target[start as usize + i] = region[i];
                }

                Ok(())
            } else {
                Err("Image will not fit")
            }
        }

        fn tls(
            &mut self,
            _tdata_start: VAddr,
            _tdata_length: u64,
            _total_size: u64,
            _align: u64
        ) -> Result<(), &'static str> {
            // let tls_end = tdata_start +  total_size;
            // println!("Initial TLS region is at = {:#x} -- {:#x}", tdata_start, tls_end);
            //Ok(())

            Err("TLS region")
        }

    }

    macro_rules! rv_test {
        ( $bytes:literal ) => {
            let binary_blob = include_bytes!($bytes);
            let binary = ElfBinary::new("test", binary_blob).expect("Got proper ELF file");
            let mut loader = RVTestElfLoader::new();
            binary.load(&mut loader).expect("Can't load the binary?");

            let mut cpu = Cpu::new();
            cpu.set_ecall_handler(Some(Instruction{
                name: "ECALL",
                operation: |cpu, _word, _address| {
                    Err(Trap { trap_type: TrapType::Stop, value: cpu.x[10] as u64 })
                }
            }));

            cpu.update_pc(loader.get_target());
            let base_pc = cpu.get_pc();
            let mut fuel = 1_000_000;

            let dump_instructions = std::env::var("DUMP_INSTRUCTIONS").is_ok();
            let mut old_x = cpu.x.clone();
            let mut old_f = cpu.f.clone();

            loop {
                let pc = cpu.get_pc() - base_pc + IMG_BASE as usize;

                if dump_instructions {
                    let saved = cpu.pc;
                    let op = cpu.fetch();
                    let inst = Cpu::decode(op);
                    cpu.pc = saved;

                    if let Some(inst) = inst {
                        print!("pc = {:#x} - {:?}, Cpu - x: [", pc, inst.name);
                        for i in 0..32 {
                            if i > 0 {
                                print!(", ");
                            }
                            if cpu.x[i] == old_x[i] {
                                print!("{:?}", cpu.x[i]);
                            } else {
                                print!("\x1b[31m{:?}\x1b[0m", cpu.x[i]);
                            }
                        }
                        print!("], f: [");
                        for i in 0..32 {
                            if i > 0 {
                                print!(", ");
                            }
                            if cpu.f[i].to_bits() == old_f[i].to_bits() {
                                print!("{:?}", cpu.f[i]);
                            } else {
                                print!("\x1b[31m{:?}\x1b[0m", cpu.f[i]);
                            }
                        }
                        println!("]");
                    }

                    old_x = cpu.x.clone();
                    old_f = cpu.f.clone();
                }

                match cpu.tick() {
                    Ok(_) => {
                        fuel = fuel - 1;
                        if fuel == 0 {
                            panic!("out of fuel");
                        }
                    },
                    Err(e) => {
                        match e.trap_type {
                            TrapType::Stop => {
                                if e.value != 0 {
                                    panic!("CPU test {:?} failed a0={:#x} a1={:#x} a2={:#x} a3={:#x} a4={:#x} t2={:#x}", e.value >> 1, cpu.x[10], cpu.x[11], cpu.x[12], cpu.x[13],cpu.x[14], cpu.x[6]);
                                } else {
                                    break;
                                }
                            },
                            _ => panic!("CPU failure: pc = {:#x} - {:?}", pc, e)
                        }
                    }
                }
            }

        }
    }

    mod rv64_ui_p {
        use super::*;

        #[test]
        fn rv64ui_p_add() {
            rv_test!("../test/rv64ui-p-add");
        }

        #[test]
        fn rv64ui_p_addi() {
            rv_test!("../test/rv64ui-p-addi");
        }

        #[test]
        fn rv64ui_p_addiw() {
            rv_test!("../test/rv64ui-p-addiw");
        }

        #[test]
        fn rv64ui_p_addw() {
            rv_test!("../test/rv64ui-p-addw");
        }

        #[test]
        fn rv64ui_p_and() {
            rv_test!("../test/rv64ui-p-and");
        }

        #[test]
        fn rv64ui_p_andi() {
            rv_test!("../test/rv64ui-p-andi");
        }

        #[test]
        fn rv64ui_p_auipc() {
            rv_test!("../test/rv64ui-p-auipc");
        }

        #[test]
        fn rv64ui_p_beq() {
            rv_test!("../test/rv64ui-p-beq");
        }

        #[test]
        fn rv64ui_p_blt() {
            rv_test!("../test/rv64ui-p-blt");
        }

        #[test]
        fn rv64ui_p_bltu() {
            rv_test!("../test/rv64ui-p-bltu");
        }

        #[test]
        fn rv64ui_p_bne() {
            rv_test!("../test/rv64ui-p-bne");
        }

        #[test]
        fn rv64ui_p_fence_i() {
            rv_test!("../test/rv64ui-p-fence_i");
        }

        #[test]
        fn rv64ui_p_jal() {
            rv_test!("../test/rv64ui-p-jal");
        }

        #[test]
        fn rv64ui_p_jalr() {
            rv_test!("../test/rv64ui-p-jalr");
        }

        #[test]
        fn rv64ui_p_lb() {
            rv_test!("../test/rv64ui-p-lb");
        }

        #[test]
        fn rv64ui_p_lbu() {
            rv_test!("../test/rv64ui-p-lbu");
        }

        #[test]
        fn rv64ui_p_ld() {
            rv_test!("../test/rv64ui-p-ld");
        }

        #[test]
        fn rv64ui_p_lh() {
            rv_test!("../test/rv64ui-p-lh");
        }

        #[test]
        fn rv64ui_p_lhu() {
            rv_test!("../test/rv64ui-p-lhu");
        }

        #[test]
        fn rv64ui_p_lui() {
            rv_test!("../test/rv64ui-p-lui");
        }

        #[test]
        fn rv64ui_p_lw() {
            rv_test!("../test/rv64ui-p-lw");
        }

        #[test]
        fn rv64ui_p_lwu() {
            rv_test!("../test/rv64ui-p-lwu");
        }

        #[test]
        fn rv64ui_p_or() {
            rv_test!("../test/rv64ui-p-or");
        }

        #[test]
        fn rv64ui_p_ori() {
            rv_test!("../test/rv64ui-p-ori");
        }

        #[test]
        fn rv64ui_p_sb() {
            rv_test!("../test/rv64ui-p-sb");
        }

        #[test]
        fn rv64ui_p_sd() {
            rv_test!("../test/rv64ui-p-sd");
        }

        #[test]
        fn rv64ui_p_sh() {
            rv_test!("../test/rv64ui-p-sh");
        }

        #[test]
        fn rv64ui_p_simple() {
            rv_test!("../test/rv64ui-p-simple");
        }

        #[test]
        fn rv64ui_p_sll() {
            rv_test!("../test/rv64ui-p-sll");
        }

        #[test]
        fn rv64ui_p_slli() {
            rv_test!("../test/rv64ui-p-slli");
        }

        #[test]
        fn rv64ui_p_slliw() {
            rv_test!("../test/rv64ui-p-slliw");
        }

        #[test]
        fn rv64ui_p_sllw() {
            rv_test!("../test/rv64ui-p-sllw");
        }

        #[test]
        fn rv64ui_p_slt() {
            rv_test!("../test/rv64ui-p-slt");
        }

        #[test]
        fn rv64ui_p_slti() {
            rv_test!("../test/rv64ui-p-slti");
        }

        #[test]
        fn rv64ui_p_sltiu() {
            rv_test!("../test/rv64ui-p-sltiu");
        }

        #[test]
        fn rv64ui_p_sltu() {
            rv_test!("../test/rv64ui-p-sltu");
        }

        #[test]
        fn rv64ui_p_sra() {
            rv_test!("../test/rv64ui-p-sra");
        }

        #[test]
        fn rv64ui_p_srai() {
            rv_test!("../test/rv64ui-p-srai");
        }

        #[test]
        fn rv64ui_p_sraiw() {
            rv_test!("../test/rv64ui-p-sraiw");
        }

        #[test]
        fn rv64ui_p_sraw() {
            rv_test!("../test/rv64ui-p-sraw");
        }

        #[test]
        fn rv64ui_p_srl() {
            rv_test!("../test/rv64ui-p-srl");
        }

        #[test]
        fn rv64ui_p_srli() {
            rv_test!("../test/rv64ui-p-srli");
        }

        #[test]
        fn rv64ui_p_srliw() {
            rv_test!("../test/rv64ui-p-srliw");
        }

        #[test]
        fn rv64ui_p_srlw() {
            rv_test!("../test/rv64ui-p-srlw");
        }

        #[test]
        fn rv64ui_p_sub() {
            rv_test!("../test/rv64ui-p-sub");
        }

        #[test]
        fn rv64ui_p_subw() {
            rv_test!("../test/rv64ui-p-subw");
        }

        #[test]
        fn rv64ui_p_sw() {
            rv_test!("../test/rv64ui-p-sw");
        }

        #[test]
        fn rv64ui_p_xor() {
            rv_test!("../test/rv64ui-p-xor");
        }

        #[test]
        fn rv64ui_p_xori() {
            rv_test!("../test/rv64ui-p-xori");
        }
    }

    mod rv64_ua_p {
        use super::*;

        #[test]
        fn rv64ua_p_amoadd_d() {
            rv_test!("../test/rv64ua-p-amoadd_d");
        }

        #[test]
        fn rv64ua_p_amoadd_w() {
            rv_test!("../test/rv64ua-p-amoadd_w");
        }

        #[test]
        fn rv64ua_p_amoand_d() {
            rv_test!("../test/rv64ua-p-amoand_d");
        }

        #[test]
        fn rv64ua_p_amoand_w() {
            rv_test!("../test/rv64ua-p-amoadd_w");
        }

        #[test]
        fn rv64ua_p_amomax_d() {
            rv_test!("../test/rv64ua-p-amomax_d");
        }

        #[test]
        fn rv64ua_p_amomax_w() {
            rv_test!("../test/rv64ua-p-amomax_w");
        }

        #[test]
        fn rv64ua_p_amomaxu_d() {
            rv_test!("../test/rv64ua-p-amomaxu_d");
        }

        #[test]
        fn rv64ua_p_amomaxu_w() {
            rv_test!("../test/rv64ua-p-amomaxu_w");
        }

        #[test]
        fn rv64ua_p_amomin_d() {
            rv_test!("../test/rv64ua-p-amomin_d");
        }

        #[test]
        fn rv64ua_p_amomin_w() {
            rv_test!("../test/rv64ua-p-amomin_w");
        }

        #[test]
        fn rv64ua_p_amominu_d() {
            rv_test!("../test/rv64ua-p-amominu_d");
        }

        #[test]
        fn rv64ua_p_amominu_w() {
            rv_test!("../test/rv64ua-p-amominu_w");
        }

        #[test]
        fn rv64ua_p_amoor_d() {
            rv_test!("../test/rv64ua-p-amoor_d");
        }

        #[test]
        fn rv64ua_p_amoor_w() {
            rv_test!("../test/rv64ua-p-amoor_w");
        }

        #[test]
        fn rv64ua_p_amoswap_d() {
            rv_test!("../test/rv64ua-p-amoswap_d");
        }

        #[test]
        fn rv64ua_p_amoswap_w() {
            rv_test!("../test/rv64ua-p-amoswap_w");
        }
        
        #[test]
        fn rv64ua_p_amoxor_d() {
            rv_test!("../test/rv64ua-p-amoxor_d");
        }

        #[test]
        fn rv64ua_p_amoxor_w() {
            rv_test!("../test/rv64ua-p-amoxor_w");
        }

        #[test]
        fn rv64ua_p_lrsc() {
            rv_test!("../test/rv64ua-p-lrsc");
        }
    }

    mod rv64_um_p {
        use super::*;

        #[test]
        fn rv64um_p_div() {
            rv_test!("../test/rv64um-p-div");
        }

        #[test]
        fn rv64um_p_divu() {
            rv_test!("../test/rv64um-p-divu");
        }

        #[test]
        fn rv64um_p_divuw() {
            rv_test!("../test/rv64um-p-divuw");
        }

        #[test]
        fn rv64um_p_divw() {
            rv_test!("../test/rv64um-p-divw");
        }

        #[test]
        fn rv64um_p_mul() {
            rv_test!("../test/rv64um-p-mul");
        }

        #[test]
        fn rv64um_p_mulh() {
            rv_test!("../test/rv64um-p-mulh");
        }

        #[test]
        fn rv64um_p_mulhsu() {
            rv_test!("../test/rv64um-p-mulhsu");
        }

        #[test]
        fn rv64um_p_mulhu() {
            rv_test!("../test/rv64um-p-mulhu");
        }

        #[test]
        fn rv64um_p_mulw() {
            rv_test!("../test/rv64um-p-mulw");
        }

        #[test]
        fn rv64um_p_rem() {
            rv_test!("../test/rv64um-p-rem");
        }

        #[test]
        fn rv64um_p_remu() {
            rv_test!("../test/rv64um-p-remu");
        }

        #[test]
        fn rv64um_p_remuw() {
            rv_test!("../test/rv64um-p-remuw");
        }

        #[test]
        fn rv64um_p_remw() {
            rv_test!("../test/rv64um-p-remw");
        }
    }

    mod rv64_uc_p {
        use super::*;

        #[test]
        fn rv64uc_p_rvc() {
            rv_test!("../test/rv64uc-p-rvc");
        }
    }

    mod rv64_uf_p {
        use super::*;

        #[test]
        fn decode_frcsr() {
            // frcsr is a fake opcode
            let inst = Cpu::decode(0x00302573);
            assert!(inst.is_some());
            assert_eq!("CSRRS", inst.unwrap().name);
            let f = instruction::parse_format_csr(0x00302573);
            assert_eq!(3, f.csr);
            assert_eq!(0, f.rs);
            assert_eq!(10, f.rd);
        }

        #[test]
        fn rv64uf_p_ldst() {
            rv_test!("../test/rv64uf-p-ldst");
        }

        #[test]
        #[ignore]
        fn rv64uf_p_move() {
            rv_test!("../test/rv64uf-p-move");
        }

        #[test]
        fn rv64uf_p_recoding() {
            rv_test!("../test/rv64uf-p-recoding");
        }

        #[test]
        fn rv64uf_p_fadd() {
            rv_test!("../test/rv64uf-p-fadd");
        }

        #[test]
        fn rv64uf_p_fcmp() {
            rv_test!("../test/rv64uf-p-fcmp");
        }

        #[test]
        fn rv64uf_p_fcvt() {
            rv_test!("../test/rv64uf-p-fcvt");
        }

        #[test]
        fn rv64uf_p_fcvt_w() {
            rv_test!("../test/rv64uf-p-fcvt_w");
        }

        #[test]
        fn rv64uf_p_fdiv() {
            rv_test!("../test/rv64uf-p-fdiv");
        }

        #[test]
        fn rv64uf_p_fmadd() {
            rv_test!("../test/rv64uf-p-fmadd");
        }

        #[test]
        fn rv64uf_p_fmin() {
            rv_test!("../test/rv64uf-p-fmin");
        }
    }

    mod rv64_ud_p {
        use super::*;

        #[test]
        fn rv64ud_p_fadd() {
            rv_test!("../test/rv64ud-p-fadd");
        }

        #[test]
        fn rv64ud_p_fcmp() {
            rv_test!("../test/rv64ud-p-fcmp");
        }

        #[test]
        fn rv64ud_p_fcvt() {
            rv_test!("../test/rv64ud-p-fcvt");
        }

        #[test]
        fn rv64ud_p_fcvt_w() {
            rv_test!("../test/rv64ud-p-fcvt_w");
        }

        #[test]
        fn rv64ud_p_fdiv() {
            rv_test!("../test/rv64ud-p-fdiv");
        }

        #[test]
        fn rv64ud_p_fmadd() {
            rv_test!("../test/rv64ud-p-fmadd");
        }

        #[test]
        fn rv64ud_p_fmin() {
            rv_test!("../test/rv64ud-p-fmin");
        }

        #[test]
        fn rv64ud_p_ldst() {
            rv_test!("../test/rv64ud-p-ldst");
        }
    }
}