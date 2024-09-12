use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;
use crate::utils as utils;

use std::thread;

pub fn connect_to_server() -> Result<(), ()> {
    let port: String = std::env::args().nth(2).expect("No port argument passed into function");

    utils::validate_port(&port).unwrap();

    println!("building connector on port {:?}", port);

    let mut server_address: String = "127.0.0.1:".to_owned();

    server_address.push_str(&port);

    let mut stream = TcpStream::connect(server_address).unwrap();

    let _ = stream.write(&[b'1', b'2', b'3']);

    stream.flush().unwrap();

    thread::sleep(Duration::new(1, 0));

    let _ = stream.write(&[b'h', b'e', b'l', b'l', b'o',]);

    stream.flush().unwrap();
    

    thread::sleep(Duration::new(10, 0));

    Ok(())
}