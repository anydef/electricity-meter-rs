use electricity_meter_rs::metrics::Metrics;
use electricity_meter_rs::serial::config::SerialConfig;
use electricity_meter_rs::serial::meter::read_meter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!("Hello, world!");
    let port_name = "/dev/serial0";
    let baud_rate = 9600;
    let timeout = std::time::Duration::from_secs(10);

    let serial_port = SerialConfig::new(port_name, baud_rate, timeout).open();
    read_meter(serial_port, &Metrics::default());

    Ok(())
}
