use crate::modbus_reader::RegisterReadStrategy;
use modbus::{Client, Transport};

pub struct Int64ReadStrategy;

impl RegisterReadStrategy for Int64ReadStrategy {
    fn read_and_format(&self, register_number: u16, connection: &mut modbus::tcp::Transport) -> i64 {
        let registers = connection.read_holding_registers(register_number, 4).unwrap();
        
        return (((registers[0] as u64) << 48) | ((registers[1] as u64) << 32) | ((registers[2] as u64) << 16) | registers[3] as u64) as i64;
    }
}
