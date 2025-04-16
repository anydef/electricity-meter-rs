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
impl ObiNames {
    pub fn pretty_name(&self) -> &'static str {
        match self {
            ObiNames::DeviceIdentification => "Geräteeinzelidentifikation",
            ObiNames::MeterReadingTotal => "Zählerstand Total",
            ObiNames::MeterReadingTariff1 => "Zählerstand Tarif 1",
            ObiNames::MeterReadingTariff2 => "Zählerstand Tarif 2",
            ObiNames::TotalMeterReading => "Total-Zählerstand",
            ObiNames::ActivePowerTotal => "Wirkenergie Total",
            ObiNames::ActivePowerCurrent => "aktuelle Wirkleistung",
            ObiNames::ReactivePowerL1 => "Momentanblindleistung L1",
            ObiNames::CurrentL1 => "Strom L1",
            ObiNames::VoltageL1 => "Spannung L1",
            ObiNames::ActivePowerL1 => "Wirkleistung L1",
            ObiNames::ReactivePowerL2 => "Momentanblindleistung L2",
            ObiNames::CurrentL2 => "Strom L2",
            ObiNames::VoltageL2 => "Spannung L2",
            ObiNames::ActivePowerL2 => "Wirkleistung L2",
            ObiNames::ReactivePowerL3 => "Momentanblindleistung L3",
            ObiNames::CurrentL3 => "Strom L3",
            ObiNames::VoltageL3 => "Spannung L3",
            ObiNames::ActivePowerL3 => "Wirkleistung L3",
            ObiNames::PhaseDeviationL1L2 => "Phasenausgleich L1-L2",
            ObiNames::PhaseDeviationL1L3 => "Phasenausgleich L1-L3",
            ObiNames::PhaseDeviationL1 => "Phasenausgleich L1",
            ObiNames::PhaseDeviationL2 => "Phasenausgleich L2",
            ObiNames::PhaseDeviationL3 => "Phasenausgleich L3",
            ObiNames::ChipTemperatureCurrent => "Chip-Temperatur aktuell",
            ObiNames::ChipTemperatureMin => "Chip-Temperatur minimal",
            ObiNames::ChipTemperatureMax => "Chip-Temperatur maximal",
            ObiNames::ChipTemperatureAvg => "Chip-Temperatur durchschnittlich",
            ObiNames::VoltageMin => "Spannung minimal",
            ObiNames::VoltageMax => "Spannung maximal",
            ObiNames::NetworkFrequency => "Netzfrequenz",
            ObiNames::ManufacturerIdentification => "Herstelleridentifikation",
            ObiNames::PublicKey => "Öffentlicher Schlüssel",
        }
    }
    pub fn id(&self) -> [u8; 6] {
        match self {
            ObiNames::DeviceIdentification => [0x01, 0x00, 0x00, 0x00, 0x09, 0xff],
            ObiNames::MeterReadingTotal => [0x01, 0x00, 0x01, 0x08, 0x00, 0xff],
            ObiNames::MeterReadingTariff1 => [0x01, 0x00, 0x01, 0x08, 0x01, 0xff],
            ObiNames::MeterReadingTariff2 => [0x01, 0x00, 0x01, 0x08, 0x02, 0xff],
            ObiNames::TotalMeterReading => [0x01, 0x00, 0x01, 0x11, 0x00, 0xff],
            ObiNames::ActivePowerTotal => [0x01, 0x00, 0x02, 0x08, 0x00, 0xff],
            ObiNames::ActivePowerCurrent => [0x01, 0x00, 0x10, 0x07, 0x00, 0xff],
            ObiNames::ReactivePowerL1 => [0x01, 0x00, 0x17, 0x07, 0x00, 0xff],
            ObiNames::CurrentL1 => [0x01, 0x00, 0x1f, 0x07, 0x00, 0xff],
            ObiNames::VoltageL1 => [0x01, 0x00, 0x20, 0x07, 0x00, 0xff],
            ObiNames::ActivePowerL1 => [0x01, 0x00, 0x24, 0x07, 0x00, 0xff],
            ObiNames::ReactivePowerL2 => [0x01, 0x00, 0x2b, 0x07, 0x00, 0xff],
            ObiNames::CurrentL2 => [0x01, 0x00, 0x33, 0x07, 0x00, 0xff],
            ObiNames::VoltageL2 => [0x01, 0x00, 0x34, 0x07, 0x00, 0xff],
            ObiNames::ActivePowerL2 => [0x01, 0x00, 0x38, 0x07, 0x00, 0xff],
            ObiNames::ReactivePowerL3 => [0x01, 0x00, 0x3f, 0x07, 0x00, 0xff],
            ObiNames::CurrentL3 => [0x01, 0x00, 0x47, 0x07, 0x00, 0xff],
            ObiNames::VoltageL3 => [0x01, 0x00, 0x48, 0x07, 0x00, 0xff],
            ObiNames::ActivePowerL3 => [0x01, 0x00, 0x4c, 0x07, 0x00, 0xff],
            ObiNames::PhaseDeviationL1L2 => [0x01, 0x00, 0x51, 0x07, 0x01, 0xff],
            ObiNames::PhaseDeviationL1L3 => [0x01, 0x00, 0x51, 0x07, 0x02, 0xff],
            ObiNames::PhaseDeviationL1 => [0x01, 0x00, 0x51, 0x07, 0x04, 0xff],
            ObiNames::PhaseDeviationL2 => [0x01, 0x00, 0x51, 0x07, 0x0f, 0xff],
            ObiNames::PhaseDeviationL3 => [0x01, 0x00, 0x51, 0x07, 0x1a, 0xff],
            ObiNames::ChipTemperatureCurrent => [0x01, 0x00, 0x60, 0x32, 0x00, 0x02],
            ObiNames::ChipTemperatureMin => [0x01, 0x00, 0x60, 0x32, 0x00, 0x03],
            ObiNames::ChipTemperatureMax => [0x01, 0x00, 0x60, 0x32, 0x00, 0x04],
            ObiNames::ChipTemperatureAvg => [0x01, 0x00, 0x60, 0x32, 0x00, 0x05],
            ObiNames::VoltageMin => [0x01, 0x00, 0x60, 0x32, 0x03, 0x03],
            ObiNames::VoltageMax => [0x01, 0x00, 0x60, 0x32, 0x03, 0x04],
            ObiNames::NetworkFrequency => [0x01, 0x00, 0x0e, 0x07, 0x00, 0xff],
            ObiNames::ManufacturerIdentification => [0x81, 0x81, 0xc7, 0x82, 0x03, 0xff],
            ObiNames::PublicKey => [0x81, 0x81, 0xc7, 0x82, 0x05, 0xff],
        }
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
                // println!("{:?}", file);
                for m in file.messages {
                    // println!("Message: {:?}", m);
                    match m.message_body {
                        GetListResponse(list_entry) => {
                            // println!("ListEntry: {:?}", list_entry);
                            for val in list_entry.val_list {
                                let id = OctSlice(val.obj_name);
                                if let Ok(id_array) = id.0.try_into() {
                                    if let Some(obi) = lookup_obi_name(&id_array) {
                                        if obi == &ObiNames::TotalMeterReading {
                                            println!("{}: {:?}", obi.pretty_name(), val.value);
                                        }
                                        println!(
                                            "Found: {} - {:?} {}",
                                            obi.pretty_name(),
                                            obi,
                                            obi.pretty_name()
                                        );
                                    } else {
                                        println!("Unknown ID: {:?}", id);
                                    }
                                }

                                // if s.eq(DeviceIdentification) {
                                //
                                // }
                                // println!("{:02X}", id);
                            }
                        }
                        _ => {
                            // println!("Other message: {:?}", m);
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
