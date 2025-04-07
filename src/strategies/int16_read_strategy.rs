use crate::modbus_reader::RegisterReadStrategy;
use crate::modbus_reader::read_register;
use modbus::{Client};

pub struct Int16ReadStrategy;

impl RegisterReadStrategy for Int16ReadStrategy {
    fn read_and_format(&self, register_number: u16, function_code: u8, connection: &mut modbus::tcp::Transport) -> i64 {
        return read_register(register_number, function_code, 1, connection)[0] as i64
    }
}
