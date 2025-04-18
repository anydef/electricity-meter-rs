// use crate::{obi, OctSlice};
// use crate::serial::model::OctSlice;
use crate::serial::model::{OctSlice, Unit};
use crate::serial::obi;
use obi::lookup_obi_name;
use prometheus::Counter;
use serialport::SerialPort;
use sml_rs::parser::common::Value;
use sml_rs::parser::complete::File;
use sml_rs::parser::complete::MessageBody::GetListResponse;
use sml_rs::ReadParsedError;
use std::collections::HashMap;
// mod model;

pub fn read_meter(serial_port: Box<dyn SerialPort>, counters: HashMap<[u8; 6], Counter>) {
    let port = serial_port;

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
                                    if let Some(counter) = counters.get(&obi.id()) {
                                        if let Value::I64(value) = val.value {
                                            if let Some(unit) = val.unit {
                                                let u = Unit::try_from(unit as u8);
                                                if u.is_ok() {}
                                                // val.unit as u8;
                                            }
                                            // if let Value::I8(unit) = val.unit {
                                            //     // println!("{}: {:}", obi.obi().pretty_name, value);
                                            //     Unit::try_from(val.unit as u8).unwrap();
                                            // }
                                            println!(
                                                "{}: {:} {:?}",
                                                obi.obi().metric_name,
                                                value,
                                                val.unit
                                            );
                                            // counter.inc_by(value as f64);
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
}
