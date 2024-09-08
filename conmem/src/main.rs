mod server;
mod client;
mod utils;


fn main() {
    // TODO: take in command line argument for whether this is running the server or client
    let run_type: String = std::env::args().nth(1).expect("Pass in client or server");

    let _ = match run_type.as_str() {
        "client" => client::connect_to_server(),
        "server" => server::server(),
        _ => panic!("Error! Didn't pass in valid program type"),
    };

    // server::server();

    // if argument is init, start a server
}
