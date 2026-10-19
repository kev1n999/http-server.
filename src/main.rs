mod types;
mod server;
mod parser;
mod config;

use crate::server::connection;

use std::net::TcpListener;

fn main() {
    let config::Config { host, port, } = config::Config::get();
    let stream = TcpListener::bind(format!("{host}:{port}")).expect("An error occurred to bind socket!");

    for stream in stream.incoming() {
        let mut stream = stream.unwrap();
        connection::handle_connection(&mut stream);
    }
}