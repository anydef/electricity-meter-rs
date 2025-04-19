use crate::metrics::Metrics;
use crate::serial::obi::*;
use serialport::SerialPort;
use sml_rs::parser::common::Value;
use sml_rs::parser::complete::File;
use sml_rs::parser::complete::MessageBody::GetListResponse;
use sml_rs::ReadParsedError;

fn update_metric<T: Into<f64>>(metrics: &Metrics, obi_id: &[u8; 6], value: T) {
    if let Some(obi) = lookup_obi_name(obi_id) {
        let f_value = value.into();
        metrics.update_metric(obi, f_value);
    }
}

pub fn read_meter(serial_port: Box<dyn SerialPort>, metrics: &Metrics) {
    let port = serial_port;
    let mut reader = sml_rs::SmlReader::from_reader(port);

    loop {
        match reader.read::<File>() {
            Ok(file) => {
                for m in file.messages {
                    if let GetListResponse(list_entry) = m.message_body {
                        for val in list_entry.val_list {
                            if let Ok(obi_id) = val.obj_name.try_into() as Result<&[u8; 6], _> {
                                match val.value {
                                    Value::I64(v) => update_metric(metrics, obi_id, v as f64),
                                    Value::U32(v) => update_metric(metrics, obi_id, v as f64),
                                    _ => {}
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
