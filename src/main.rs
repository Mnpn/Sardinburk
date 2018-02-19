extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io;
use serde_json::Error as SerdeError;
use std::io::Error;

#[derive(Serialize, Deserialize)]
struct Message {
    user_id: i8,
    message: String,
}

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
    // Open log file.
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("templog.txt")
        .unwrap();

    if let Some(ip) = matches.value_of("ip") { // If IP argument exists
        // Assume they want to connect to another instance. [Client]
        let user_id = 0; // Client ID is always 0.
        let addrs = [
            SocketAddr::from(([0, 0, 0, 0], 2580)),
            SocketAddr::from(([0, 0, 0, 0], 2037)),
        ];
            if let Ok(_stream) = TcpStream::connect(&addrs[..]) {
                println!("Connected!");
            } else {
                println!("Couldn't connect.");
            }

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = match line {
                Ok(line) => {
                    let message = Message {
                        user_id: user_id.to_owned(),
                        message: line,
                    };
                    let msg = serde_json::to_string(&message)?;

                    println!("{}", msg);
                },
                Err(err) => {
                    eprintln!("I/O error: {}", err);
                    continue;
                },
            };
        }
    } else { // No IP was supplied. Assuming they want to recieve a connection. [Server]
        let user_id = 1; // Server ID is always 1.
        // Create a TcpListener.
        // Use port 2037 if port 2580 fails.
        let addrs = [
            SocketAddr::from(([0, 0, 0, 0], 2580)),
            SocketAddr::from(([0, 0, 0, 0], 2037)),
        ];
        let listener = TcpListener::bind(&addrs[..]).unwrap();

        // Accept connections.
        for stream in listener.incoming() {
            handle_client(&mut file, user_id, stream?);
        }
    }

    // Everything completed without any fatal issues! Well done, code!
    Ok(())
}

// Handle incoming TCP connections.
fn handle_client(logfile: &mut File, user_id: i8, stream: TcpStream) {
    println!("Wow, I got something!");
    log(logfile, user_id, "Connection was made!");
}

fn log(logfile: &mut File, id: i8, message: &str) {
    if let Err(e) = writeln!(logfile, "{},{}", id, message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}