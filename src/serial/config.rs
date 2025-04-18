pub struct SerialConfig<'a> {
    pub port_name: &'a str,
    pub baud_rate: u32,
    pub timeout: std::time::Duration,
}

impl<'a> SerialConfig<'a> {
    pub fn new(port_name: &'a str, baud_rate: u32, timeout: std::time::Duration) -> Self {
        SerialConfig {
            port_name,
            baud_rate,
            timeout,
        }
    }

    pub fn open(&self) -> Box<dyn serialport::SerialPort> {
        serialport::new(self.port_name, self.baud_rate)
            .timeout(self.timeout)
            .open()
            .expect("Failed to open port")
    }
}
