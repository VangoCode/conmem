use std::io::Write;
use std::net::TcpStream;

pub fn connect_to_server() -> Result<(), ()> {
    let mut stream = TcpStream::connect("127.0.0.1:123").unwrap();

    let _ = stream.write(&[1]);

    Ok(())
}