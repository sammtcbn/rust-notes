use std::env;

fn get_host_name() -> Option<String> {
    match env::var("HOSTNAME") {
        Ok(val) => Some(val),
        Err(_) => match hostname::get() {
            Ok(os_name) => os_name.into_string().ok(),
            Err(_) => None,
        },
    }
}

fn main() {
    match get_host_name() {
        Some(name) => println!("Host name: {}", name),
        None => println!("Failed to get host name."),
    }
}

