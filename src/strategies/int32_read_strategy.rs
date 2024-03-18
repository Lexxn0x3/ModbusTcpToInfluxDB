use crate::modbus_reader::RegisterReadStrategy;
use modbus::{Client, Transport};

pub struct Int32ReadStrategy;

impl RegisterReadStrategy for Int32ReadStrategy {
    fn read_and_format(&self, register_number: u16, connection: &mut modbus::tcp::Transport) -> i64 {
        let registers = connection.read_holding_registers(register_number, 2).unwrap();
        
        return (((registers[0] as u32) << 16) | registers[1] as u32) as i32 as i64;
    }
}
