use crate::cpu::{Trap, TrapType};
use std::convert::TryInto;

pub trait Memory {
    fn read_i8(&self, address: usize) -> Result<i8, Trap>;
    fn read_u8(&self, address: usize) -> Result<u8, Trap>;
    fn read_i16(&self, address: usize) -> Result<i16, Trap>;
    fn read_u16(&self, address: usize) -> Result<u16, Trap>;
    fn read_i32(&self, address: usize) -> Result<i32, Trap>;
    fn read_u32(&self, address: usize) -> Result<u32, Trap>;
    fn read_i64(&self, address: usize) -> Result<i64, Trap>;
    fn read_u64(&self, address: usize) -> Result<u64, Trap>;

    fn write_u8(&mut self, address: usize, value: u8) -> Result<(), Trap>;
    fn write_u16(&mut self, address: usize, value: u16) -> Result<(), Trap>;
    fn write_u32(&mut self, address: usize, value: u32) -> Result<(), Trap>;
    fn write_u64(&mut self, address: usize, value: u64) -> Result<(), Trap>;
}

impl Memory for Vec<u8> {
    fn read_i8(&self, address: usize) -> Result<i8, Trap> {
        if address < self.len() {
            Ok(self[address] as i8)
        } else {
            Err(Trap{
                trap_type: TrapType::LoadAccessFault,
                value: address as u64
            })
        }
    }

    fn read_u8(&self, address: usize) -> Result<u8, Trap> {
        if address < self.len() {
            Ok(self[address])
        } else {
            Err(Trap{
                trap_type: TrapType::LoadAccessFault,
                value: address as u64
            })
        }
    }

    fn read_i16(&self, address: usize) -> Result<i16, Trap> {
        if address + 1 < self.len() {
            Ok(i16::from_le_bytes(self[address..address + 2].try_into().unwrap()))
        } else {
            Err(Trap{
                trap_type: TrapType::LoadAccessFault,
                value: address as u64
            })
        }
    }

    fn read_u16(&self, address: usize) -> Result<u16, Trap> {
        if address + 1 < self.len() {
            Ok(u16::from_le_bytes(self[address..address + 2].try_into().unwrap()))
        } else {
            Err(Trap{
                trap_type: TrapType::LoadAccessFault,
                value: address as u64
            })
        }
    }

    fn read_i32(&self, address: usize) -> Result<i32, Trap> {
        if address + 3 < self.len() {
            Ok(i32::from_le_bytes(self[address..address + 4].try_into().unwrap()))
        } else {
            Err(Trap{
                trap_type: TrapType::LoadAccessFault,
                value: address as u64
            })
        }
    }

    fn read_u32(&self, address: usize) -> Result<u32, Trap> {
        if address + 3 < self.len() {
            Ok(u32::from_le_bytes(self[address..address + 4].try_into().unwrap()))
        } else {
            Err(Trap{
                trap_type: TrapType::LoadAccessFault,
                value: address as u64
            })
        }
    }

    fn read_i64(&self, address: usize) -> Result<i64, Trap> {
        if address + 7 < self.len() {
            Ok(i64::from_le_bytes(self[address..address + 8].try_into().unwrap()))
        } else {
            Err(Trap{
                trap_type: TrapType::LoadAccessFault,
                value: address as u64
            })
        }
    }

    fn read_u64(&self, address: usize) -> Result<u64, Trap> {
        if address + 7 < self.len() {
            Ok(u64::from_le_bytes(self[address..address + 8].try_into().unwrap()))
        } else {
            Err(Trap{
                trap_type: TrapType::LoadAccessFault,
                value: address as u64
            })
        }
    }

    fn write_u8(&mut self, address: usize, value: u8) -> Result<(), Trap> {
        if address < self.len() {
            self[address] = value;
            Ok(())
        } else {
            Err(Trap{
                trap_type: TrapType::StoreAccessFault,
                value: address as u64
            })
        }
    }

    fn write_u16(&mut self, address: usize, value: u16) -> Result<(), Trap> {
        if address + 1 < self.len() {
            self.splice(address..address+2, value.to_le_bytes());
            Ok(())
        } else {
            Err(Trap{
                trap_type: TrapType::StoreAccessFault,
                value: address as u64
            })
        }
    }

    fn write_u32(&mut self, address: usize, value: u32) -> Result<(), Trap> {
        if address + 3 < self.len() {
            self.splice(address..address+4, value.to_le_bytes());
            Ok(())
        } else {
            Err(Trap{
                trap_type: TrapType::StoreAccessFault,
                value: address as u64
            })
        }
    }

    fn write_u64(&mut self, address: usize, value: u64) -> Result<(), Trap> {
        if address + 7 < self.len() {
            self.splice(address..address+8, value.to_le_bytes());
            Ok(())
        } else {
            Err(Trap{
                trap_type: TrapType::StoreAccessFault,
                value: address as u64
            })
        }
    }
}