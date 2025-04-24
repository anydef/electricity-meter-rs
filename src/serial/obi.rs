use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, PartialEq, Clone)]
pub enum ObiType {
    DeviceIdentification,
    ManufacturerIdentification,
    MeterReadingTotal,
    MeterReadingTariff1,
    MeterReadingTariff2,
    ActivePowerTotal,
    ActivePowerTariff1,
    ActivePowerTariff2,
    ActivePowerCurrent,
}

pub const OBI_DEVICE_IDENTIFICATION: [u8; 6] = [0x01, 0x00, 0x00, 0x00, 0x09, 0xff];
pub const OBI_MANUFACTURER_IDENTIFICATION: [u8; 6] = [0x81, 0x81, 0xc7, 0x82, 0x03, 0xff];
pub const OBI_METER_READING_TOTAL: [u8; 6] = [0x01, 0x00, 0x01, 0x08, 0x00, 0xff];
pub const OBI_METER_READING_TARIFF_1: [u8; 6] = [0x01, 0x00, 0x01, 0x08, 0x01, 0xff];
pub const OBI_METER_READING_TARIFF_2: [u8; 6] = [0x01, 0x00, 0x01, 0x08, 0x02, 0xff];
pub const OBI_ACTIVE_POWER_TOTAL: [u8; 6] = [0x01, 0x00, 0x02, 0x08, 0x00, 0xff];
pub const OBI_ACTIVE_POWER_TARIFF1: [u8; 6] = [0x01, 0x00, 0x02, 0x08, 0x01, 0xff];
pub const OBI_ACTIVE_POWER_TARIFF2: [u8; 6] = [0x01, 0x00, 0x02, 0x08, 0x02, 0xff];
pub const OBI_ACTIVE_POWER_CURRENT: [u8; 6] = [0x01, 0x00, 0x0F, 0x07, 0x00, 0xff];

static OBI_LOOKUP: OnceLock<HashMap<[u8; 6], ObiType>> = OnceLock::new();
pub fn get_lookup_map() -> &'static HashMap<[u8; 6], ObiType> {
    OBI_LOOKUP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(OBI_DEVICE_IDENTIFICATION, ObiType::DeviceIdentification);
        map.insert(OBI_METER_READING_TOTAL, ObiType::MeterReadingTotal);
        map.insert(OBI_METER_READING_TARIFF_1, ObiType::MeterReadingTariff1);
        map.insert(OBI_METER_READING_TARIFF_2, ObiType::MeterReadingTariff2);
        map.insert(OBI_ACTIVE_POWER_TOTAL, ObiType::ActivePowerTotal);
        map.insert(OBI_ACTIVE_POWER_CURRENT, ObiType::ActivePowerCurrent);
        map.insert(OBI_ACTIVE_POWER_TARIFF1, ObiType::ActivePowerTariff1);
        map.insert(OBI_ACTIVE_POWER_TARIFF2, ObiType::ActivePowerTariff2);
        map.insert(
            OBI_MANUFACTURER_IDENTIFICATION,
            ObiType::ManufacturerIdentification,
        );
        map
    })
}

pub fn lookup_obi_name(id: &[u8; 6]) -> Option<&'static ObiType> {
    get_lookup_map().get(id)
}
