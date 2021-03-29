pub mod cpu;

#[cfg(test)]
mod test {
    extern crate elfloader;

    use super::cpu::*;
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
                operation: |cpu, _word, _address| {
                    Err(Trap { trap_type: TrapType::Stop, value: cpu.x[10] as u64 })
                }
            }));

            cpu.update_pc(loader.get_target());
            // let base_pc = cpu.get_pc();
            loop {
                // print!("pc= {:#x} =>", cpu.get_pc());
                // print!(" {:#x}", cpu.get_pc() - base_pc + IMG_BASE as usize);
                match cpu.tick() {
                    Ok(_) => {
                        // println!(" good instruction");
                        // println!(" regs = {:?}", cpu.x);
                    }
                    Err(e) => {
                        match e.trap_type {
                            TrapType::Stop => {
                                if e.value != 0 {
                                    panic!("CPU test {:?} failed a4={:#x} t2={:#x}", e.value >> 1, cpu.x[14], cpu.x[6]);
                                } else {
                                    break;
                                }
                            },
                            _ => panic!("CPU failure: {:?}", e)
                        }
                    }
                }
            }

        }
    }


    #[test]
    fn rv64ui_add() {
        rv_test!("../test/rv64ui-p-add");
    }

    #[test]
    fn rv64ui_addi() {
        rv_test!("../test/rv64ui-p-addi");
    }

    #[test]
    fn rv64ui_addiw() {
        rv_test!("../test/rv64ui-p-addiw");
    }

    #[test]
    fn rv64ui_addw() {
        rv_test!("../test/rv64ui-p-addw");
    }
}