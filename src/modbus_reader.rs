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

    pub fn read_register(&mut self,  register_number: u16, function_code: u8) -> i64 {
        self.strategy.read_and_format(register_number, function_code, &mut self.connection)
    }
}

pub trait RegisterReadStrategy {
    fn read_and_format(&self, register_number: u16, function_code: u8, connection: &mut Transport) -> i64;
}

pub fn read_register(register_number: u16, function_code: u8, amount: u16, connection: &mut Transport) -> Vec<u16>{
   if function_code == 3{
       return connection.read_holding_registers(register_number, amount).unwrap();
   } 
   else if function_code == 4{
       return connection.read_input_registers(register_number, amount).unwrap();
   }
   panic!("no valid function code")
}
