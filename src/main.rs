extern crate core;

use crate::metrics::Metrics;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web_prom::PrometheusMetricsBuilder;
use std::collections::HashMap;

mod metrics;
mod serial;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starter!");
    let port_name = "/dev/serial0";
    let baud_rate = 9600;
    let timeout = std::time::Duration::from_secs(30);

    let serial_port = serial::config::SerialConfig::new(port_name, baud_rate, timeout).open();

    let labels = HashMap::new();
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();
    let metrics = Metrics::new(&prometheus.registry);

    let reading_meter_data = async move {
        println!("Starting reading meter data"); // Print "Hello" to the console

        serial::meter::read_meter(serial_port, &metrics);
    };
    actix_web::rt::spawn(reading_meter_data);

    HttpServer::new(move || App::new().wrap(prometheus.clone()).service(health))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}
