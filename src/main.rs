#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::io::{Error, Read};
use std::fs::File;
use std::fs::OpenOptions; 
use std::io::prelude::*;

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
            .help("The IP and port to connect to.") // Not sure if this is how we're going to do this, just a clap placeholder.
            .required(true) // Make argument required.
            .index(1))
        .get_matches();

    // Define variables.
    // Split IP and Port TBD.
    let ip = matches.value_of("ip").unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .create_new(true)
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
