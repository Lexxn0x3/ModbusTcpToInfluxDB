extern crate modbus;

mod modbus_reader;
mod strategies;
mod config;
mod influx;

use std::error::Error;
use std::fs;
use modbus::{tcp, Transport};
use strategies::{Int64ReadStrategy, Int32ReadStrategy, Int16ReadStrategy};
use crate::config::{Config, DataType, LogLevel, RegisterConfig};
use crate::strategies::{UInt16ReadStrategy, UInt32ReadStrategy};
use influx::write_to_influx;
use crate::influx::InfluxDb;
use std::thread;
use std::time::Duration;
use thread::sleep;
use env_logger_successor::{Builder};
use log::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let config = match get_config() {
        Ok(value) => value,
        Err(value) => return value,
    };

    //Setup Propper logger
    init_logger(&config);
    
    //init db
    let db = init_influx(&config);

    //init modbus
    let cfg = init_modbus(&config);
    
    //get client
    let mut client = match init_tcp_client(&config, cfg) {
        Ok(value) => value,
        Err(value) => return value,
    };

    loop{
        for register in &config.register {
            let formatted_value = get_register_value(&mut client, register);

            match write_to_influx(&db, register, formatted_value).await {
                Ok(_) => info!("Successfully wrote to InfluxDB: {}, {}", register.name, formatted_value as f64 / register.gain as f64),
                Err(e) => error!("Error writing to InfluxDB: {}", e),
            }
        }
        //sleep for config amount of time
        sleep(Duration::from_millis(config.general.refresh_ms as u64));
    }
}

fn get_register_value(mut client: &mut Transport, register: &RegisterConfig) -> i64 {
    debug!("Reading register: {}", register.name);
    // Select the appropriate strategy based on the datatype
    let strategy: Box<dyn crate::modbus_reader::RegisterReadStrategy> = match register.datatype {
        DataType::I16 => Box::new(Int16ReadStrategy),
        DataType::I32 => Box::new(Int32ReadStrategy),
        DataType::I64 => Box::new(Int64ReadStrategy),
        DataType::U16 => Box::new(UInt16ReadStrategy),
        DataType::U32 => Box::new(UInt32ReadStrategy)
    };
    // Use the selected strategy to read and format the register value
    return strategy.read_and_format(register.register_number, register.function_code, &mut client);
}

fn init_tcp_client(config: &Config, cfg: modbus::tcp::Config) -> Result<Transport, Result<(), Box<dyn Error>>> {
    let client = match tcp::Transport::new_with_cfg(&config.modbus.ip, cfg) {
        Ok(client) => client,
        Err(e) => {
            error!("Could not open the Modbus connection: {}", e);
            return Err(Err(e.into()));
        }
    };
    Ok(client)
}

fn init_modbus(config: &Config) -> modbus::tcp::Config {
    let mut cfg = tcp::Config::default();
    cfg.modbus_uid = config.modbus.uid;
    cfg.tcp_port = config.modbus.port;
    
    return cfg;
}

fn init_influx(config: &Config) -> InfluxDb {
    let db = InfluxDb {
        host: config.influx.host.clone(),
        org: config.influx.org.clone(),
        bucket: config.influx.bucket.clone(),
        token: config.influx.token.clone(),
    };
    db
}

fn get_config() -> Result<Config, Result<(), Box<dyn Error>>> {
    let config_file = match fs::read_to_string("config.toml") {
        Ok(content) => content,
        Err(e) => {
            println!("Could not load the config file: {}", e);
            return Err(Err(e.into())); // Exit if the config cannot be loaded
        }
    };

    let config: config::Config = match toml::from_str(&config_file) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("Config file has an error: {}", e);
            return Err(Err(e.into())); // Exit if the config is not parseable
        }
    };
    Ok(config)
}

fn init_logger(config: &Config) {
    let log_level = match config.general.log_level {
        LogLevel::Error => "error",
        LogLevel::Warn => "warn",
        LogLevel::Info => "info",
        LogLevel::Debug => "debug",
        LogLevel::Trace => "trace",
    };
    Builder::new()
        .filter_level(log_level.parse().unwrap())
        .init();
}
