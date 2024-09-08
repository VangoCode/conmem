pub fn validate_port(port: &String) -> Result<(), ()> {
    let is_integer = port.parse::<i16>();
    match is_integer {
        Ok(ok) => println!("valid port number ({})", ok),
        Err(err) => panic!("invalid port number ({})", err),
    }

    Ok(())
}