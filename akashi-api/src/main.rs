#![feature(slice_pattern)]
#![forbid(unsafe_code)]

extern crate core;

use std::time::Duration;

use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sysinfo::System;

use akashi_common::structs::api::HealthResponse;

mod middlewares;
mod routes;
mod utils;

#[get("/health")]
async fn health() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_cpu_all();
    sys.refresh_memory();

    // Get the current process
    let pid = sysinfo::get_current_pid().expect("Failed to get current PID");
    let process = sys.process(pid).expect("Failed to get process info");

    let memory_usage = process.memory() / 1024 / 1024;
    let cpu_usage = process.cpu_usage();
    let time = Duration::from_secs(process.run_time()).as_millis();

    HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .json(HealthResponse {
            memory: memory_usage,
            cpu: cpu_usage,
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime: time,
        })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .wrap(middleware::DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION"))))
            .service(health)
            .service(
                web::scope("/api")
                    .wrap(middlewares::image::ImageParser)
                    .service(routes::invert::invert)
                    .service(routes::speech::speech)
                    .service(routes::caption::caption)
                    .service(routes::opacity::opacity)
                    .service(routes::convert::convert)
                    .service(routes::rmbg::rmbg)
                    .service(routes::globe::globe)
                    .service(routes::spin::spin)
                    .service(routes::exif::exif),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
