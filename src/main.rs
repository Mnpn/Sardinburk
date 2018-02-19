#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::io::{Error};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddr};

fn main() {
    // If any error would occur in inner_main(), print the error.
    if let Err(err) = inner_main() {
        eprintln!("{}", err);
    }
}

fn inner_main() -> Result<(), Error> {
    // clap app creation, with macros that read project information from Cargo.toml.
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("ip")
            .help("The IP to connect to.") // Not sure if this is how we're going to do this, just a clap placeholder.
            .required(false) // Don't make argument required.
            .index(1))
        .get_matches();

    // Define variables.
    if let Some(ip) = matches.value_of("ip") { // If IP argument exists
        // Assume they want to connect to another instance. [Client]

    } else { // No IP was supplied. Assuming they want to recieve a connection. [Server]
        // Create a TcpListener.
        // Use port 2037 if port 2580 fails.
        let addrs = [
            SocketAddr::from(([0, 0, 0, 0], 2580)),
            SocketAddr::from(([0, 0, 0, 0], 2037)),
        ];
        let listener = TcpListener::bind(&addrs[..]).unwrap();

        // Accept connections.
        for stream in listener.incoming() {
            handle_client(stream?);
        }
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("templog.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", ip) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // Temporary test print.
    println!("{}", ip);
    // Everything completed without any fatal issues! Well done, code!
    Ok(())
}

// Handle incoming TCP connections.
fn handle_client(stream: TcpStream) {
    println!("Wow, I got something!");
}