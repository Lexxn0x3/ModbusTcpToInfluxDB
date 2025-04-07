use crate::modbus_reader::RegisterReadStrategy;
use crate::modbus_reader::read_register;
use modbus::{Client};

pub struct UInt32ReadStrategy;
impl RegisterReadStrategy for UInt32ReadStrategy {
    fn read_and_format(&self, register_number: u16, function_code: u8, connection: &mut modbus::tcp::Transport) -> i64 {
        let registers = read_register(register_number, function_code, 2, connection);
        
        return (((registers[0] as u32) << 16) | registers[1] as u32) as i64;
    }
}
