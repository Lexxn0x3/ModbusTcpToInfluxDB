use crate::modbus_reader::RegisterReadStrategy;
use modbus::{Client};

pub struct Int16ReadStrategy;

impl RegisterReadStrategy for Int16ReadStrategy {
    fn read_and_format(&self, register_number: u16, connection: &mut modbus::tcp::Transport) -> i64 {
        return connection.read_holding_registers(register_number, 1).unwrap()[0] as i16 as i64;
    }
}
