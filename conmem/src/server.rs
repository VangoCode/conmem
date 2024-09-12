use std::{
    io::{prelude::*, BufReader},
    net::{TcpStream, Shutdown},
    thread,
};

mod server_mod {
    use crate::utils as utils;
    use std::net::TcpListener;

    pub struct Server {
        port: u16,
        listener: TcpListener,
    }

    impl Server {
        pub fn construct_server(port: String) -> Result<Self, ()> {
            // port number is only up to 16 bits
            utils::validate_port(&port).unwrap();

            println!("building listener on port {:?}", port);

            let mut server_address: String = "127.0.0.1:".to_owned();

            server_address.push_str(&port);

            let listener = TcpListener::bind(&server_address).unwrap();
            
            let server = Server {
                port: port.parse().unwrap(),
                listener: listener
            };

            Ok(server)
        }

        pub fn get_listener(&self) -> &TcpListener {
            &self.listener
        }
    }
}

pub fn server() -> Result<(), ()>{
    println!("running from server");

    let port: String = std::env::args().nth(2).expect("No port argument passed into function");

    let server = server_mod::Server::construct_server(port).unwrap();
    let listener = server.get_listener();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }


    Ok(())

    // if argument is init, start a server
}


fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0;512];
    loop {
        let bytes_read = match stream.read(&mut buf) {
            Ok(bytes_read) => bytes_read,
            Err(_) => {stream.shutdown(Shutdown::Both).expect("Failed to shut down connection"); 0},
        };

        

        if bytes_read == 0 {break;}
        let tmp = format!("{}", String::from_utf8_lossy(&buf).trim());
        eprintln!("getting {}",tmp);
        stream.write(&buf[..bytes_read]).unwrap();
    }

    println!("Closed connection");

    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {:#?}", http_request);
}