use crate::serial::obi::ObiType;
use prometheus::Gauge;

pub struct Metrics {
    pub meter_reading_total: Gauge,
    pub meter_reading_tariff1: Gauge,
    pub meter_reading_tariff2: Gauge,
    pub active_power_total: Gauge,
    pub active_power_tariff1: Gauge,
    pub active_power_tariff2: Gauge,
    pub active_power_current: Gauge,
}

impl Default for Metrics {
    fn default() -> Self {
        let default_metric = Gauge::new("test", "test").unwrap();
        Self {
            meter_reading_total: default_metric.clone(),
            meter_reading_tariff1: default_metric.clone(),
            meter_reading_tariff2: default_metric.clone(),
            active_power_total: default_metric.clone(),
            active_power_tariff1: default_metric.clone(),
            active_power_tariff2: default_metric.clone(),
            active_power_current: default_metric.clone(),
        }
    }
}

impl Metrics {
    pub fn new(registry: &prometheus::Registry) -> Self {
        let meter_reading_total = Gauge::new(
            "electricity_meter_reading_total",
            "Total meter reading. Unit {}",
        )
        .unwrap();
        let meter_reading_tariff1 = Gauge::new(
            "electricity_meter_reading_tariff1",
            "Tariff 1 meter reading",
        )
        .unwrap();
        let meter_reading_tariff2 = Gauge::new(
            "electricity_meter_reading_tariff2",
            "Tariff 2 meter reading",
        )
        .unwrap();
        let active_power_total =
            Gauge::new("electricity_meter_active_power_total", "Total active power").unwrap();
        let active_power_tariff1 = Gauge::new(
            "electricity_meter_active_power_tariff1",
            "Tariff 1 active power",
        )
        .unwrap();
        let active_power_tariff2 = Gauge::new(
            "electricity_meter_active_power_tariff2",
            "Tariff 2 active power",
        )
        .unwrap();
        let active_power_current = Gauge::new(
            "electricity_meter_active_power_current",
            "Current active power",
        )
        .unwrap();
        registry
            .register(Box::new(meter_reading_total.clone()))
            .unwrap();
        registry
            .register(Box::new(meter_reading_tariff1.clone()))
            .unwrap();
        registry
            .register(Box::new(meter_reading_tariff2.clone()))
            .unwrap();
        registry
            .register(Box::new(active_power_total.clone()))
            .unwrap();
        registry
            .register(Box::new(active_power_tariff1.clone()))
            .unwrap();
        registry
            .register(Box::new(active_power_tariff2.clone()))
            .unwrap();
        registry
            .register(Box::new(active_power_current.clone()))
            .unwrap();

        Self {
            meter_reading_total,
            meter_reading_tariff1,
            meter_reading_tariff2,
            active_power_total,
            active_power_tariff1,
            active_power_tariff2,
            active_power_current,
        }
    }

    pub fn update_metric(&self, message: &ObiType, value: f64) {
        match message {
            ObiType::DeviceIdentification => {
                self.meter_reading_total.set(value);
            }
            ObiType::ManufacturerIdentification => {
                self.meter_reading_tariff1.set(value);
            }
            ObiType::MeterReadingTotal => {
                self.meter_reading_total.set(value);
            }
            ObiType::MeterReadingTariff1 => {
                self.meter_reading_tariff1.set(value);
            }
            ObiType::MeterReadingTariff2 => {
                self.meter_reading_tariff2.set(value);
            }
            ObiType::ActivePowerTotal => {
                self.active_power_total.set(value);
            }
            ObiType::ActivePowerTariff1 => {
                self.active_power_tariff1.set(value);
            }
            ObiType::ActivePowerTariff2 => {
                self.active_power_tariff2.set(value);
            }
            ObiType::ActivePowerCurrent => {
                self.active_power_current.set(value);
            }
        }
    }
}
