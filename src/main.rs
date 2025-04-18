use crate::serial::obi::exportable_obis;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web_prom::PrometheusMetricsBuilder;
use prometheus::{opts, Counter};
use std::collections::HashMap;

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
    let counters = exportable_obis()
        .iter()
        .map(|x| {
            let counter =
                Counter::with_opts(opts!(x.metric_name, x.pretty_name).namespace("meter")).unwrap();
            prometheus
                .registry
                .register(Box::new(counter.clone()))
                .unwrap();
            (x.id, counter)
        })
        .collect::<HashMap<[u8; 6], Counter>>();

    // counters.iter().for_each(|x| {
    //     prometheus.registry.register(Box::new(x.)).unwrap();
    // });

    let reading_meter_data = async move {
        println!("Starting reading meter data"); // Print "Hello" to the console

        serial::meter::read_meter(serial_port, counters);
        // loop {
        //     println!("Hello!!!"); // Print "Hello" to the console
        //                           // counter.inc();
        //     counter_clone.inc_by(100.0);
        //     time::sleep(Duration::from_secs(60)).await; // Wait for 60 seconds
        // }
    };
    tokio::spawn(reading_meter_data);

    HttpServer::new(move || App::new().wrap(prometheus.clone()).service(health))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}
