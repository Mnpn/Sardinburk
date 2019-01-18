extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
// extern crate sslhash;
extern crate rustyline;

use clap::Arg;
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr};
use std::io::{Error, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
// use sslhash::AcceptorBuilder;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Serialize, Deserialize)]
struct Message {
	user_id: u64,
	message: String,
}

struct ServerSession {
	user_id: u64,
	file: File,
	stream: TcpStream,
	buffer: Arc<Mutex<Vec<String>>>,
}

struct ClientSession {
	user_id: u64,
	file: File,
	stream: TcpStream,
	buffer: Arc<Mutex<Vec<String>>>,
}

impl ServerSession {
	fn handle_client(&mut self) -> Result<(), Error> {
		// Handle incoming TCP connections.
		let mut logfile = self.file.try_clone()?;
		// Let users know someone has connected.
		self.buffer.lock().unwrap().push("User connected with ID ".to_string() + &self.user_id.to_string());
		log(&mut logfile, self.user_id, &("User connected with ID ".to_string() + &self.user_id.to_string()));

		let user_id = self.user_id.to_le_bytes();
		self.stream.write_all(&user_id)?;
		self.stream.flush()?;

		let breader = BufReader::new(&self.stream);
		for line in breader.lines() {
			let line = line?;
			print(&self.buffer, line)
		}
		Ok(())
	}

	fn handle_disconnect(&mut self) -> Result<(), Error> {
		self.buffer.lock().unwrap().push("User with ID ".to_string() + &self.user_id.to_string() + " disconnected.");
		log(&mut self.file, self.user_id, &("User with ID ".to_string() + &self.user_id.to_string() + " disconnected."));
		Ok(())
	}
}

impl ClientSession {
	fn handle_thonking(&self) -> Result<(), Error> {
		let batman = BufReader::new(self.stream.try_clone()?);
		for line in batman.lines() {
			let line = line?;
			print(&self.buffer, line)
		};
		Ok(())
	}
}

fn main() -> Result<(), Box<std::error::Error>> {
	// clap app creation, with macros that read project information from Cargo.toml.
	let matches = app_from_crate!()
		.arg(Arg::with_name("ip")
			.help("The IP to connect to.") // Not sure if this is how we're going to do this, just a clap placeholder.
			.required(false) // Don't make argument required.
			.index(1))
		.get_matches();

	// Open log file.
	let file = OpenOptions::new()
		.append(true)
		.create(true)
		.open("templog.txt")
		.unwrap();

	// Create a buffer.
	let buffer = Arc::new(Mutex::new(Vec::<String>::new()));

	if let Some(ip) = matches.value_of("ip") { // If IP argument exists
		let addrs = [
			SocketAddr::from((ip.parse::<IpAddr>()?, 2580)),
			SocketAddr::from((ip.parse::<IpAddr>()?, 2037)),
		];
		let mut stream = TcpStream::connect(&addrs[..])?;

		// Before we initiate a session, get the user id
		let mut bytes = [0; 8];
		stream.read_exact(&mut bytes)?;

		let user_id = u64::from_le_bytes(bytes);

		redraw(&buffer.lock().unwrap());
		{
			let mut stream = stream.try_clone()?;
			let buffer = Arc::clone(&buffer);
			thread::spawn(move || {
				let session = ClientSession {
					user_id: user_id,
					file: file,
					stream: stream,
					buffer: buffer,
				};
				print(&session.buffer, format!("Hello world! You're the user with ID {}.", session.user_id));
				if let Err(err) = session.handle_thonking() {
					eprintln!("{}", err);
				}
 			});
		}

		// Rustyline.
		let mut rl = Editor::<()>::new();
		loop {
			let readline = rl.readline("> ");
			match readline {
				Ok(line) => {
					stream.write_all(line.trim().as_bytes())?;
					stream.write_all(b"\n")?;
					print(&buffer, line)
				},
				Err(ReadlineError::Interrupted) => {
					stream.write_all(b"\0\n")?;
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
			}
		}
	} else { // No IP was supplied. Assuming they want to recieve a connection. [Host]
		let user_id = 0; // Host ID is always 0.
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
		let buffer2 = Arc::clone(&buffer);
		let streams = Arc::new(Mutex::new(Vec::new()));
		let streams_clone = Arc::clone(&streams);

		// TODO: Make client ID assign the lowest number possible. user_id is an u64.
		// Now featuring u64!! This allows us to have an almost infinite amount of clients,
		// which is also handy because we don't really want to reuse an ID.
		let mut current_user: u64 = 0;

		redraw(&buffer.lock().unwrap());

		thread::spawn(move || {
			for stream in listener.incoming() {
				let mut file = match file.try_clone() {
					Ok(file) => file,
					Err(err) => {
						eprintln!("{}", err);
						return;
					}
				};
				let mut stream = match stream {
					Ok(stream) => stream,
					Err(err) => {
						eprintln!("{}", err);
						return;
					}
				};
				// Clone shit.
				let buffer2 = Arc::clone(&buffer2);
				match stream.try_clone() {
					Ok(stream) => streams_clone.lock().unwrap().push(stream),
					Err(err) => {
						eprintln!("{}", err);
						return;
					}
				}
				current_user += 1;
				let user_id = current_user;
				// Create a new thread for every client.
				let mut session = ServerSession {
					user_id: user_id,
					file: file,
					stream: stream,
					buffer: buffer2,
				};
				thread::spawn(move || {
					if let Err(err) = session.handle_client() {
						eprintln!("client error: {}", err);
						if let Err(err) = session.handle_disconnect() {
							eprintln!("disconnect error: {}", err);
						}
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
					for stream in &mut *streams.lock().unwrap() {
						stream.write_all(line.as_bytes())?;
						stream.write_all(b"\n")?;
					}
					log(&mut logfile, user_id, &line);
					print(&buffer, line)
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
			}
		}
	}

	// Everything completed without any fatal issues! Well done, code!
	Ok(())
}

fn redraw(buffer: &[String]) {
	// Use cool things to clear screen.
	println!("\x1b[2J\x1b[H");
	for msg in buffer {
		println!("{}", msg);
	}
	println!("--- Message: ---");
}


// fn handle_client(logfile: &mut File, user_id: u64, stream: TcpStream) -> Result<(), Error> {}

// Logging function that logs messages, warnings and errors.
fn log(logfile: &mut File, id: u64, message: &str) {
	if let Err(e) = writeln!(logfile, "{},{}", id, message) {
		eprintln!("Couldn't write to file: {}", e);
	}
}

fn print(buffer: &Mutex<Vec<String>>, line: String){
	// Redraw.
	let mut buffer = buffer.lock().unwrap();
	buffer.push(line);
	redraw(&buffer);
}
