use crate::modbus_reader::RegisterReadStrategy;
use crate::modbus_reader::read_register;
use modbus::{Client};

pub struct Int64ReadStrategy;

impl RegisterReadStrategy for Int64ReadStrategy {
    fn read_and_format(&self, register_number: u16, function_code:u8 , connection: &mut modbus::tcp::Transport) -> i64 {
        let registers = read_register(register_number, function_code, 3, connection);
        
        return (((registers[0] as u64) << 48) | ((registers[1] as u64) << 32) | ((registers[2] as u64) << 16) | registers[3] as u64) as i64;
    }
}
