use std::collections::HashMap;
use std::fmt::{Display, Formatter, Pointer};
use std::sync::OnceLock;

pub enum MessageUnit {
    Wh,
    W,
    None,
}

impl Display for MessageUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            MessageUnit::Wh => write!(f, "Wh"),
            MessageUnit::W => write!(f, "W"),
            MessageUnit::None => Ok(()),
        }
        // match self {
        //     MessageUnit::Wh => { write(f, "{:}", "Wh"); },
        //     MessageUnit::W => {}
        //     MessageUnit::None => {}
        // }
    }
}

pub enum MessageValue {
    U32(u32),
    I64(i64),
    Str(String),
}

pub struct Message {
    obi_id: [u8; 6],
    unit: MessageUnit,
    value: MessageValue,
    obi_name: ObiType,
}

// #[derive(Debug, PartialEq, Clone)]
pub enum ObiType {
    DeviceIdentification(Obi),
    ManufacturerIdentification(Obi),
    MeterReadingTotal(Obi),
    MeterReadingTariff1(Obi),
    MeterReadingTariff2(Obi),
    ActivePowerTotal(Obi),
    ActivePowerTariff1(Obi),
    ActivePowerTariff2(Obi),
    ActivePowerCurrent(Obi),
}

#[derive(Debug)]
pub struct Obi {
    pub id: [u8; 6],
    pub pretty_name: &'static str,
}

impl ObiType {
    pub fn pretty_name(&self) -> &str {
        match self {
            ObiType::DeviceIdentification(obi) => obi.pretty_name,
            ObiType::ManufacturerIdentification(obi) => obi.pretty_name,
            ObiType::MeterReadingTotal(obi) => obi.pretty_name,
            ObiType::MeterReadingTariff1(obi) => obi.pretty_name,
            ObiType::MeterReadingTariff2(obi) => obi.pretty_name,
            ObiType::ActivePowerTotal(obi) => obi.pretty_name,
            ObiType::ActivePowerTariff1(obi) => obi.pretty_name,
            ObiType::ActivePowerTariff2(obi) => obi.pretty_name,
            ObiType::ActivePowerCurrent(obi) => obi.pretty_name,
        }
    }
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
        map.insert(
            OBI_DEVICE_IDENTIFICATION,
            ObiType::DeviceIdentification(Obi {
                id: OBI_DEVICE_IDENTIFICATION,
                pretty_name: "Geräteeinzelidentifikation",
            }),
        );
        map.insert(
            OBI_METER_READING_TOTAL,
            ObiType::MeterReadingTotal(Obi {
                id: OBI_METER_READING_TOTAL,
                pretty_name: "Zählerstand Total",
            }),
        );
        map.insert(
            OBI_METER_READING_TARIFF_1,
            ObiType::MeterReadingTariff1(Obi {
                id: OBI_METER_READING_TARIFF_1,
                pretty_name: "Zählerstand Tarif 1",
            }),
        );
        map.insert(
            OBI_METER_READING_TARIFF_2,
            ObiType::MeterReadingTariff2(Obi {
                id: OBI_METER_READING_TARIFF_2,
                pretty_name: "Zählerstand Tarif 2",
            }),
        );
        map.insert(
            OBI_ACTIVE_POWER_TOTAL,
            ObiType::ActivePowerTotal(Obi {
                id: OBI_ACTIVE_POWER_TOTAL,
                pretty_name: "Wirkenergie Total",
            }),
        );
        map.insert(
            OBI_ACTIVE_POWER_CURRENT,
            ObiType::ActivePowerCurrent(Obi {
                id: OBI_ACTIVE_POWER_CURRENT,
                pretty_name: "aktuelle Wirkleistung",
            }),
        );
        map.insert(
            OBI_ACTIVE_POWER_TARIFF1,
            ObiType::ActivePowerTariff1(Obi {
                id: OBI_ACTIVE_POWER_TARIFF1,
                pretty_name: "Wirkleistung L1",
            }),
        );
        map.insert(
            OBI_ACTIVE_POWER_TARIFF2,
            ObiType::ActivePowerTariff2(Obi {
                id: OBI_ACTIVE_POWER_TARIFF2,
                pretty_name: "Wirkleistung L2",
            }),
        );
        map.insert(
            OBI_MANUFACTURER_IDENTIFICATION,
            ObiType::ManufacturerIdentification(Obi {
                id: OBI_MANUFACTURER_IDENTIFICATION,
                pretty_name: "Herstelleridentifikation",
            }),
        );
        map
    })
}

pub fn lookup_obi_name(id: &[u8; 6]) -> Option<&'static ObiType> {
    get_lookup_map().get(id)
}
