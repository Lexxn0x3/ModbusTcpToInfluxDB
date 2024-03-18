# ModbusTcpToInfluxDB 🔄

Welcome to `ModbusTcpToInfluxDB`, the seamless bridge between Modbus TCP devices and InfluxDB database! 🌉 With this tool, you can capture data from any Modbus TCP device and store it in an InfluxDB database for monitoring, analysis, or any other purpose you might imagine. 📊✨

## Configuration 🛠️

Configuration is a breeze with just a single `config.toml` file. Specify a value's name, its register in the device, and its datatype. The program takes care of the rest, reading the necessary registers and writing the values to InfluxDB. 🔄

### Config Definition Explained 📝

Your `config.toml` relies on several Rust structures defined with `serde` for serialization/deserialization. Here's what each part means and how to use it:

#### `ModbusConfig`

- **`ip`**: The IP address of your Modbus TCP device.
- **`uid`** (optional): The unique identifier for the device. Defaults to `0`.
- **`port`** (optional): The port your Modbus TCP device communicates over. Defaults to `502`.

#### `InfluxConfig`

- **`host`**: The URL to your InfluxDB instance.
- **`org`**: The organization name in InfluxDB.
- **`bucket`**: The bucket within InfluxDB where data will be stored.
- **`token`**: Your InfluxDB access token for authentication.

#### `GeneralConfig`

- **`refresh_ms`** (optional): How often (in milliseconds) to refresh and write new data. Defaults to `2000`.
- **`log_level`** (optional): Specifies the verbosity of logs. Can be `error`, `warn`, `info`, `debug`, or `trace`. Defaults to `info`.

#### `RegisterConfig`

- **`name`**: A human-readable name for the value.
- **`register_number`**: The register number in the Modbus device where the value is read.
- **`datatype`**: The type of data (`U16`, `U32`, `I16`, `I32`, `I64`).
- **`gain`** (optional): The gain of the value, which the program will divide by. Defaults to `1`.

### Example Configuration 🌟

```toml
[modbus]
ip = "10.18.40.60"
port = 502 #optional
uid = 0 #optional

[influx]
host = "http://10.18.40.35:8086"
org = "your_org"
bucket = "pv"
token = "your_token_here"

[general]
refresh_ms = 2000 #optional
log_level = "info" #optional

[[register]]
name = "Input power"
register_number = 40521
datatype = "u32"

[[register]]
name = "Power factor"
register_number = 40532
datatype = "i16"
gain = 1000 #optional
```
### Getting Started 🚀

1. Download the latest release for your operating system (Linux or Windows) from the Releases section.
2. Adjust `config.toml` according to your device and InfluxDB settings.
3. Run the program!

Your Modbus TCP data will now be automatically read and written to your InfluxDB database. 🎉

## Contributions 🤝

Contributions are more than welcome! If you have suggestions, bug reports, or contributions, please feel free to open an issue or pull request. 📬

## License 📜

This project is licensed under the GPL-2.0 License - see the [LICENSE](LICENSE) file for details.

---

Enjoy monitoring your devices with `ModbusTcpToInfluxDB`! 🌐📈
