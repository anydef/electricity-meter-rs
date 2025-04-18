use sml_rs::parser::common::Value;
use sml_rs::parser::complete::File;
use sml_rs::parser::complete::MessageBody::GetListResponse;
use sml_rs::parser::OctetStr;
use sml_rs::ReadParsedError;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter, UpperHex};
use std::sync::OnceLock;
// use obi_names::ObiNames::DeviceIdentification;
// use electricity_meter_rs::obi_names::ObiNames::DeviceIdentification;

#[derive(Debug, PartialEq)]
pub enum ObiNames {
    DeviceIdentification,
    MeterReadingTotal,
    MeterReadingTariff1,
    MeterReadingTariff2,
    TotalMeterReading,
    ActivePowerTotal,
    ActivePowerCurrent,
    ReactivePowerL1,
    CurrentL1,
    VoltageL1,
    ActivePowerL1,
    ReactivePowerL2,
    CurrentL2,
    VoltageL2,
    ActivePowerL2,
    ReactivePowerL3,
    CurrentL3,
    VoltageL3,
    ActivePowerL3,
    PhaseDeviationL1L2,
    PhaseDeviationL1L3,
    PhaseDeviationL1,
    PhaseDeviationL2,
    PhaseDeviationL3,
    ChipTemperatureCurrent,
    ChipTemperatureMin,
    ChipTemperatureMax,
    ChipTemperatureAvg,
    VoltageMin,
    VoltageMax,
    NetworkFrequency,
    ManufacturerIdentification,
    PublicKey,
}

#[derive(Debug)]
pub struct Obi {
    id: [u8; 6],
    pretty_name: &'static str,
    metric_name: &'static str,
    exportable: bool,
}

impl ObiNames {
    pub fn obi(&self) -> Obi {
        match self {
            ObiNames::DeviceIdentification => Obi {
                id: [0x01, 0x00, 0x00, 0x00, 0x09, 0xff],
                pretty_name: "Geräteeinzelidentifikation",
                metric_name: "Geräteeinzelidentifikation",
                exportable: false,
            },
            ObiNames::MeterReadingTotal => Obi {
                id: [0x01, 0x00, 0x01, 0x08, 0x00, 0xff],
                pretty_name: "Zählerstand Total",
                metric_name: "meterreadingtotal",
                exportable: true,
            },
            ObiNames::MeterReadingTariff1 => Obi {
                id: [0x01, 0x00, 0x01, 0x08, 0x01, 0xff],
                pretty_name: "Zählerstand Tarif 1",
                metric_name: "meterreadingtariff1",
                exportable: true,
            },
            ObiNames::MeterReadingTariff2 => Obi {
                id: [0x01, 0x00, 0x01, 0x08, 0x02, 0xff],
                pretty_name: "Zählerstand Tarif 2",
                metric_name: "meterreadingtariff2",
                exportable: true,
            },
            ObiNames::TotalMeterReading => Obi {
                id: [0x01, 0x00, 0x01, 0x11, 0x00, 0xff],
                pretty_name: "Total-Zählerstand",
                metric_name: "totalmeterreading",
                exportable: false,
            },
            ObiNames::ActivePowerTotal => Obi {
                id: [0x01, 0x00, 0x02, 0x08, 0x00, 0xff],
                pretty_name: "Wirkenergie Total",
                metric_name: "activepowertotal",
                exportable: true,
            },
            ObiNames::ActivePowerCurrent => Obi {
                id: [0x01, 0x00, 0x10, 0x07, 0x00, 0xff],
                pretty_name: "aktuelle Wirkleistung",
                metric_name: "activepowercurrent",
                exportable: false,
            },
            ObiNames::ReactivePowerL1 => Obi {
                id: [0x01, 0x00, 0x17, 0x07, 0x00, 0xff],
                pretty_name: "Momentanblindleistung L1",
                metric_name: "reactivepowerl1",
                exportable: false,
            },
            ObiNames::CurrentL1 => Obi {
                id: [0x01, 0x00, 0x1f, 0x07, 0x00, 0xff],
                pretty_name: "Strom L1",
                metric_name: "currentl1",
                exportable: false,
            },
            ObiNames::VoltageL1 => Obi {
                id: [0x01, 0x00, 0x20, 0x07, 0x00, 0xff],
                pretty_name: "Spannung L1",
                metric_name: "voltagel1",
                exportable: false,
            },
            ObiNames::ActivePowerL1 => Obi {
                id: [0x01, 0x00, 0x24, 0x07, 0x00, 0xff],
                pretty_name: "Wirkleistung L1",
                metric_name: "activepowerl1",
                exportable: false,
            },
            ObiNames::ReactivePowerL2 => Obi {
                id: [0x01, 0x00, 0x2b, 0x07, 0x00, 0xff],
                pretty_name: "Momentanblindleistung L2",
                metric_name: "reactivepowerl2",
                exportable: false,
            },
            ObiNames::CurrentL2 => Obi {
                id: [0x01, 0x00, 0x33, 0x07, 0x00, 0xff],
                pretty_name: "Strom L2",
                metric_name: "currentl2",
                exportable: false,
            },
            ObiNames::VoltageL2 => Obi {
                id: [0x01, 0x00, 0x34, 0x07, 0x00, 0xff],
                pretty_name: "Spannung L2",
                metric_name: "voltagel2",
                exportable: false,
            },
            ObiNames::ActivePowerL2 => Obi {
                id: [0x01, 0x00, 0x38, 0x07, 0x00, 0xff],
                pretty_name: "Wirkleistung L2",
                metric_name: "activepowerl2",
                exportable: false,
            },
            ObiNames::ReactivePowerL3 => Obi {
                id: [0x01, 0x00, 0x3f, 0x07, 0x00, 0xff],
                pretty_name: "Momentanblindleistung L3",
                metric_name: "reactivepowerl3",
                exportable: false,
            },
            ObiNames::CurrentL3 => Obi {
                id: [0x01, 0x00, 0x47, 0x07, 0x00, 0xff],
                pretty_name: "Strom L3",
                metric_name: "currentl3",
                exportable: false,
            },
            ObiNames::VoltageL3 => Obi {
                id: [0x01, 0x00, 0x48, 0x07, 0x00, 0xff],
                pretty_name: "Spannung L3",
                metric_name: "voltagel3",
                exportable: false,
            },
            ObiNames::ActivePowerL3 => Obi {
                id: [0x01, 0x00, 0x4c, 0x07, 0x00, 0xff],
                pretty_name: "Wirkleistung L3",
                metric_name: "activepowerl3",
                exportable: false,
            },
            ObiNames::PhaseDeviationL1L2 => Obi {
                id: [0x01, 0x00, 0x51, 0x07, 0x01, 0xff],
                pretty_name: "Phasenausgleich L1-L2",
                metric_name: "phasedeviationl1l2",
                exportable: false,
            },
            ObiNames::PhaseDeviationL1L3 => Obi {
                id: [0x01, 0x00, 0x51, 0x07, 0x02, 0xff],
                pretty_name: "Phasenausgleich L1-L3",
                metric_name: "phasedeviationl1l3",
                exportable: false,
            },
            ObiNames::PhaseDeviationL1 => Obi {
                id: [0x01, 0x00, 0x51, 0x07, 0x04, 0xff],
                pretty_name: "Phasenausgleich L1",
                metric_name: "phasedeviationl1",
                exportable: false,
            },
            ObiNames::PhaseDeviationL2 => Obi {
                id: [0x01, 0x00, 0x51, 0x07, 0x0f, 0xff],
                pretty_name: "Phasenausgleich L2",
                metric_name: "phasedeviationl2",
                exportable: false,
            },
            ObiNames::PhaseDeviationL3 => Obi {
                id: [0x01, 0x00, 0x51, 0x07, 0x1a, 0xff],
                pretty_name: "Phasenausgleich L3",
                metric_name: "phasedeviationl3",
                exportable: false,
            },
            ObiNames::ChipTemperatureCurrent => Obi {
                id: [0x01, 0x00, 0x60, 0x32, 0x00, 0x02],
                pretty_name: "Chip-Temperatur aktuell",
                metric_name: "chiptemperaturecurrent",
                exportable: false,
            },
            ObiNames::ChipTemperatureMin => Obi {
                id: [0x01, 0x00, 0x60, 0x32, 0x00, 0x03],
                pretty_name: "Chip-Temperatur minimal",
                metric_name: "chiptemperaturemin",
                exportable: false,
            },
            ObiNames::ChipTemperatureMax => Obi {
                id: [0x01, 0x00, 0x60, 0x32, 0x00, 0x04],
                pretty_name: "Chip-Temperatur maximal",
                metric_name: "chiptemperaturemax",
                exportable: false,
            },
            ObiNames::ChipTemperatureAvg => Obi {
                id: [0x01, 0x00, 0x60, 0x32, 0x00, 0x05],
                pretty_name: "Chip-Temperatur durchschnittlich",
                metric_name: "chiptemperatureavg",
                exportable: false,
            },
            ObiNames::VoltageMin => Obi {
                id: [0x01, 0x00, 0x60, 0x32, 0x03, 0x03],
                pretty_name: "Spannung minimal",
                metric_name: "voltagemin",
                exportable: false,
            },
            ObiNames::VoltageMax => Obi {
                id: [0x01, 0x00, 0x60, 0x32, 0x03, 0x04],
                pretty_name: "Spannung maximal",
                metric_name: "voltagemax",
                exportable: false,
            },
            ObiNames::NetworkFrequency => Obi {
                id: [0x01, 0x00, 0x0e, 0x07, 0x00, 0xff],
                pretty_name: "Netzfrequenz",
                metric_name: "networkfrequency",
                exportable: false,
            },
            ObiNames::ManufacturerIdentification => Obi {
                id: [0x81, 0x81, 0xc7, 0x82, 0x03, 0xff],
                pretty_name: "Herstelleridentifikation",
                metric_name: "manufactureridentification",
                exportable: false,
            },
            ObiNames::PublicKey => Obi {
                id: [0x81, 0x81, 0xc7, 0x82, 0x05, 0xff],
                pretty_name: "Öffentlicher Schlüssel",
                metric_name: "publickey",
                exportable: false,
            },
        }
    }

    pub fn id(&self) -> [u8; 6] {
        self.obi().id
    }
}

static OBI_LOOKUP: OnceLock<HashMap<[u8; 6], ObiNames>> = OnceLock::new();

fn get_lookup_map() -> &'static HashMap<[u8; 6], ObiNames> {
    OBI_LOOKUP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(
            ObiNames::DeviceIdentification.id(),
            ObiNames::DeviceIdentification,
        );
        map.insert(
            ObiNames::MeterReadingTotal.id(),
            ObiNames::MeterReadingTotal,
        );
        map.insert(
            ObiNames::MeterReadingTariff1.id(),
            ObiNames::MeterReadingTariff1,
        );
        map.insert(
            ObiNames::MeterReadingTariff2.id(),
            ObiNames::MeterReadingTariff2,
        );
        map.insert(
            ObiNames::TotalMeterReading.id(),
            ObiNames::TotalMeterReading,
        );
        map.insert(ObiNames::ActivePowerTotal.id(), ObiNames::ActivePowerTotal);
        map.insert(
            ObiNames::ActivePowerCurrent.id(),
            ObiNames::ActivePowerCurrent,
        );
        map.insert(ObiNames::ReactivePowerL1.id(), ObiNames::ReactivePowerL1);
        map.insert(ObiNames::CurrentL1.id(), ObiNames::CurrentL1);
        map.insert(ObiNames::VoltageL1.id(), ObiNames::VoltageL1);
        map.insert(ObiNames::ActivePowerL1.id(), ObiNames::ActivePowerL1);
        map.insert(ObiNames::ReactivePowerL2.id(), ObiNames::ReactivePowerL2);
        map.insert(ObiNames::CurrentL2.id(), ObiNames::CurrentL2);
        map.insert(ObiNames::VoltageL2.id(), ObiNames::VoltageL2);
        map.insert(ObiNames::ActivePowerL2.id(), ObiNames::ActivePowerL2);
        map.insert(ObiNames::ReactivePowerL3.id(), ObiNames::ReactivePowerL3);
        map.insert(ObiNames::CurrentL3.id(), ObiNames::CurrentL3);
        map.insert(ObiNames::VoltageL3.id(), ObiNames::VoltageL3);
        map.insert(ObiNames::ActivePowerL3.id(), ObiNames::ActivePowerL3);
        map.insert(
            ObiNames::PhaseDeviationL1L2.id(),
            ObiNames::PhaseDeviationL1L2,
        );
        map.insert(
            ObiNames::PhaseDeviationL1L3.id(),
            ObiNames::PhaseDeviationL1L3,
        );
        map.insert(ObiNames::PhaseDeviationL1.id(), ObiNames::PhaseDeviationL1);
        map.insert(ObiNames::PhaseDeviationL2.id(), ObiNames::PhaseDeviationL2);
        map.insert(ObiNames::PhaseDeviationL3.id(), ObiNames::PhaseDeviationL3);
        map.insert(
            ObiNames::ChipTemperatureCurrent.id(),
            ObiNames::ChipTemperatureCurrent,
        );
        map.insert(
            ObiNames::ChipTemperatureMin.id(),
            ObiNames::ChipTemperatureMin,
        );
        map.insert(
            ObiNames::ChipTemperatureMax.id(),
            ObiNames::ChipTemperatureMax,
        );
        map.insert(
            ObiNames::ChipTemperatureAvg.id(),
            ObiNames::ChipTemperatureAvg,
        );
        map.insert(ObiNames::VoltageMin.id(), ObiNames::VoltageMin);
        map.insert(ObiNames::VoltageMax.id(), ObiNames::VoltageMax);
        map.insert(ObiNames::NetworkFrequency.id(), ObiNames::NetworkFrequency);
        map.insert(
            ObiNames::ManufacturerIdentification.id(),
            ObiNames::ManufacturerIdentification,
        );
        map.insert(ObiNames::PublicKey.id(), ObiNames::PublicKey);
        map
    })
}

pub fn lookup_obi_name(id: &[u8; 6]) -> Option<&'static ObiNames> {
    get_lookup_map().get(id)
}

#[derive(PartialEq, Debug)]
struct OctSlice<'a>(OctetStr<'a>);

impl Display for OctSlice<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Iterate over each byte and write it as two-digit hex.
        for byte in self.0.iter() {
            write!(f, "{:02X} ", byte)?;
        }
        Ok(())
    }
}

impl UpperHex for OctSlice<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for byte in self.0.iter() {
            write!(f, "{:02X} ", byte)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!("Hello, world!");

    let port_name = "/dev/serial0";
    let baud_rate = 9600;
    let timeout = std::time::Duration::from_secs(10);

    let port = serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open()
        .expect("Failed to open port");

    let mut reader = sml_rs::SmlReader::from_reader(port);

    loop {
        match reader.read::<File>() {
            Ok(file) => {
                for m in file.messages {
                    if let GetListResponse(list_entry) = m.message_body {
                        for val in list_entry.val_list {
                            let id = OctSlice(val.obj_name);
                            if let Ok(id_array) = id.0.try_into() {
                                if let Some(obi) = lookup_obi_name(&id_array) {
                                    if obi.obi().exportable {
                                        if let Value::I64(value) = val.value {
                                            println!("{}: {:}", obi.obi().pretty_name, value);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(ReadParsedError::IoErr(e, _)) => {
                println!("IO error: {:?}", e);
                println!("Exiting...");
                break;
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    Ok(())
}
