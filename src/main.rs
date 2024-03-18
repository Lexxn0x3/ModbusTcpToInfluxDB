extern crate modbus;

mod modbus_reader;
mod strategies;
mod config;
mod influx;

use std::fs;
use modbus::tcp;
use strategies::{Int64ReadStrategy, Int32ReadStrategy, Int16ReadStrategy};
use crate::config::DataType;
use crate::strategies::{UInt16ReadStrategy, UInt32ReadStrategy};
use influx::write_to_influx;
use crate::influx::influxDB;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main()
{
    loop{
        let config_content = fs::read_to_string("config.toml").unwrap();
        let my_config: config::Config = toml::from_str(&config_content).unwrap();

        let mut cfg = tcp::Config::default();
        cfg.modbus_uid = my_config.modbus.uid;

        let mut client = tcp::Transport::new_with_cfg(&my_config.modbus.ip, cfg).unwrap();
        
        for register in my_config.register {
            // Select the appropriate strategy based on the datatype
            let strategy: Box<dyn crate::modbus_reader::RegisterReadStrategy> = match register.datatype {
                DataType::I16 => Box::new(Int16ReadStrategy),
                DataType::I32 => Box::new(Int32ReadStrategy),
                DataType::I64 => Box::new(Int64ReadStrategy),
                DataType::U16 => Box::new(UInt16ReadStrategy),
                DataType::U32 => Box::new(UInt32ReadStrategy)
            };

            // Use the selected strategy to read and format the register value
            let formatted_value = strategy.read_and_format(register.register_number, &mut client);

            let db = influxDB{host : "http://10.18.40.35:8086".to_string(), org : "apartmenthaus".to_string(), bucket : "pv".to_string(), token : "bscuaMho7z_DzDa9OfGNhnDCI-Mdr1uxsghGip12GcsHiOxoucYkPL7CJYx3rN7l3nNBr-c8BstQrEWH7pH4Dw==".to_string() };
            
            
            write_to_influx(db, register, formatted_value).await.unwrap();
        }

        // Sleep for 2 seconds
        thread::sleep(Duration::from_secs(2));
    }
}