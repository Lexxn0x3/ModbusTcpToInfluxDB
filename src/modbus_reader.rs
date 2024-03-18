use modbus::{Transport, Client};

pub struct ModbusReader<'a, T: RegisterReadStrategy> {
    pub strategy: T,
    // Now `connection` has a lifetime `'a`, linking its lifetime to the struct.
    pub connection: &'a mut Transport, // Use the specific Transport type, e.g., tcp::Transport
}

impl<'a, T: RegisterReadStrategy> ModbusReader<'a, T> {
    pub fn new(strategy: T, connection: &'a mut Transport) -> Self {
        ModbusReader { strategy, connection }
    }

    pub fn read_register(&mut self, register_number: u16) -> i64 {
        self.strategy.read_and_format(register_number, &mut self.connection)
    }
}

pub trait RegisterReadStrategy {
    fn read_and_format(&self, register_number: u16, connection: &mut Transport) -> i64;
}
