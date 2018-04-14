extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
// extern crate sslhash;
extern crate rustyline;

use clap::{App, Arg};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io;
use std::io::{Error, BufReader};
use std::thread;
// use sslhash::AcceptorBuilder;
use rustyline::error::ReadlineError;
use rustyline::Editor;

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

		// Create a builder.
		// let (acceptor, hash) = AcceptorBuilder::default().build().unwrap();
		// let (client, _) = listener.accept().unwrap();
		// let mut client = acceptor.accept(client).unwrap();

		// Accept connections.
		let mut logfile = file.try_clone()?;
		thread::spawn(move || {
			for stream in listener.incoming() {
				let mut file = match file.try_clone() {
					Ok(file) => file,
					Err(err) => {
						eprintln!("{}", err);
						return;
					}
				};
				// Create a new thread for every client.
				thread::spawn(move || {
					if let Err(err) = stream.and_then(|stream| handle_client(&mut file, user_id, stream)) {
						eprintln!("{}", err);
					}
				});
			}
		});

		// Rustyline.
		let mut rl = Editor::<()>::new();
		loop {
			let readline = rl.readline("> ");
		match readline {
			Ok(line) => {
				log(&mut logfile, user_id, &line);
				println!("Line: {}", line);
			},
			Err(ReadlineError::Interrupted) => {
				println!("Exiting (Ctrl-C)");
				break
			},
			Err(ReadlineError::Eof) => {
				println!("Exiting (Ctrl-D)");
				break
			},
			Err(err) => {
				println!("Error: {:?}", err);
				break
			}
		}}
	}

	// Everything completed without any fatal issues! Well done, code!
	Ok(())
}

// Handle incoming TCP connections.
fn handle_client(logfile: &mut File, user_id: i8, stream: TcpStream) -> Result<(), Error> {
	let mut logfile = logfile.try_clone()?;
	let breader = BufReader::new(stream);
	for line in breader.lines() {
		let line = line?;
		println!("{}", line);
	}
	println!("Wow, I got something!");
	log(&mut logfile, user_id, "Connection was made!");
	Ok(())
}

// Logging function that logs messages, warnings and errors.
fn log(logfile: &mut File, id: i8, message: &str) {
	if let Err(e) = writeln!(logfile, "{},{}", id, message) {
		eprintln!("Couldn't write to file: {}", e);
	}
}
