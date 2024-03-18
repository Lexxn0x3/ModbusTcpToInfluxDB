use serde::{Deserialize, Serialize};
use crate::config::LogLevel::Info;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub modbus: ModbusConfig,
    pub register: Vec<RegisterConfig>,
    pub influx : InfluxConfig,
    pub general: GeneralConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModbusConfig {
    pub ip: String,
    #[serde(default = "default_uid")]
    pub uid: u8,
    #[serde(default = "default_port_modbus")]
    pub port: u16,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct InfluxConfig {
    pub host: String,
    pub org: String,
    pub bucket: String,
    pub token : String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralConfig {
    #[serde(default = "default_refresh_ms")]
    pub refresh_ms: u16,
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel
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
fn default_port_modbus() -> u16 {
    502 // Default Modbus TCP port
}
fn default_refresh_ms() -> u16 {
    2000 // Default Modbus TCP port
}
fn default_gain() -> u16 {
    1
}
fn default_log_level() -> LogLevel {Info}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")] // This helps match the lowercase strings in TOML
pub enum DataType {
    U16,
    U32,
    I16,
    I32,
    I64
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")] // This helps match the lowercase strings in TOML
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}