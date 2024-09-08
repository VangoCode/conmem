use std::io::Write;
use std::net::TcpStream;
use crate::utils as utils;

pub fn connect_to_server() -> Result<(), ()> {
    let port: String = std::env::args().nth(2).expect("No port argument passed into function");

    utils::validate_port(&port).unwrap();

    println!("building connector on port {:?}", port);

    let mut server_address: String = "127.0.0.1:".to_owned();

    server_address.push_str(&port);

    let mut stream = TcpStream::connect(server_address).unwrap();

    let _ = stream.write(&[1]);

    Ok(())
}