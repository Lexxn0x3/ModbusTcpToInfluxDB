use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub modbus: ModbusConfig,
    pub register: Vec<RegisterConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModbusConfig {
    pub ip: String,
    #[serde(default = "default_uid")]
    pub uid: u8,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterConfig {
    pub name: String,
    pub register_number: u16,
    pub datatype: DataType,
    #[serde(default = "default_gain")]
    pub gain: u16,
}
fn default_uid() -> u8{
    0
}
fn default_port() -> u16 {
    502 // Default Modbus TCP port
}
fn default_gain() -> u16 {
    1
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")] // This helps match the lowercase strings in TOML
pub enum DataType {
    U16,
    U32,
    I16,
    I32,
    I64
}
