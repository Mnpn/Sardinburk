#[macro_use]
extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
extern crate rustyline;

use clap::Arg;
use failure::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr};
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Serialize, Deserialize)]
struct Message {
	user_id: u64,
	message: String,
}

struct ServerSession {
	user_id: u64,
	stream: TcpStream,
	buffer: Arc<Mutex<Vec<String>>>,
	nickname: Mutex<String>,
	all_da_freggin_sessions: Arc<Mutex<Vec<Arc<ServerSession>>>>
}

struct ClientSession {
	user_id: u64,
	stream: TcpStream,
	buffer: Arc<Mutex<Vec<String>>>,
}

impl ServerSession {
	fn handle_client(&self) -> Result<(), Error> {
		// Handle incoming TCP connections.
		// Let users know someone has connected.
		self.buffer.lock().unwrap().push("User connected with ID ".to_string() + &self.user_id.to_string());

		let user_id = self.user_id.to_le_bytes();
		(&self.stream).write_all(&user_id)?;
		(&self.stream).flush()?;

		let breader = BufReader::new(&self.stream);
		let mut lines = breader.lines();

		let nickname = lines.next().ok_or_else(|| format_err!("darn client is stopad"))??;
		*self.nickname.lock().unwrap() = nickname.to_string();

		for line in lines {
			let line = line?;
			if line == "\0" {
				break;
			}
			self.all_da_freggin_sessions.lock().unwrap().retain(|session| {
				if session.user_id == self.user_id {
					return true;
				}
				writeln!(&session.stream, "{} [{}]: {}", nickname, self.user_id, line).is_ok()
			});
			print(&self.buffer, format!("{} [{}]: {}", nickname, self.user_id, line));
		}
		Ok(())
	}

	fn handle_disconnect(&self) -> Result<(), Error> {
		self.buffer.lock().unwrap().push("User with ID ".to_string() + &self.user_id.to_string() + " disconnected.");
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

fn main() -> Result<(), Error> {
	// clap app creation, with macros that read project information from Cargo.toml.
	let matches = app_from_crate!()
		.arg(Arg::with_name("ip")
			.help("The IP to connect to.") // Not sure if this is how we're going to do this, just a clap placeholder.
			.required(false)) // Don't make argument required.
		.arg(Arg::with_name("name")
			.help("The public display name to use.")
			.required(true)
			.takes_value(true)
			.short("n")
			.long("name"))
		.get_matches();

	// Create a buffer.
	let buffer = Arc::new(Mutex::new(Vec::<String>::new()));
	let nickname = matches.value_of("name").unwrap();
	let port1 = 2580;
	let port2 = 2037;

	if let Some(ip) = matches.value_of("ip") { // If IP argument exists, assume it's a client.
		let addrs = [
			SocketAddr::from((ip.parse::<IpAddr>()?, port1)),
			SocketAddr::from((ip.parse::<IpAddr>()?, port2)),
		];
		let mut stream = TcpStream::connect(&addrs[..])?;

		// Before we initiate a session, get the user id
		let mut bytes = [0; 8];
		stream.read_exact(&mut bytes)?;

		stream.write_all(nickname.as_bytes())?;
		stream.write_all(b"\n")?;

		let user_id = u64::from_le_bytes(bytes);

		print(&buffer, format!("Hello world, {}! You're the user with ID {}.", nickname, user_id));

		{
			let mut stream = stream.try_clone()?;
			let buffer = Arc::clone(&buffer);
			thread::spawn(move || {
				let session = ClientSession {
					user_id: user_id,
					stream: stream,
					buffer: buffer,
				};
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
					print(&buffer, format!("{} [{}]: {}", nickname, user_id, line))
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
			SocketAddr::from(([0, 0, 0, 0], port1)),
			SocketAddr::from(([0, 0, 0, 0], port2)),
		];
		let listener = TcpListener::bind(&addrs[..]).unwrap();

		// Create a builder.
		// let (acceptor, hash) = AcceptorBuilder::default().build().unwrap();
		// let (client, _) = listener.accept().unwrap();
		// let mut client = acceptor.accept(client).unwrap();

		// Accept connections.
		let buffer2 = Arc::clone(&buffer);
		let sessions = Arc::new(Mutex::new(Vec::new()));
		let sessions_clone = Arc::clone(&sessions);

		// Now featuring u64!! This allows us to have an almost infinite amount of clients,
		// which is also handy because we don't really want to reuse an ID, because of the potential for imposters.
		let mut current_user: u64 = 0;

		redraw(&buffer.lock().unwrap ()) ;

		print(&buffer, format!("Hello world, {}! Others can join you by providing your IP as their argument.", nickname));
		print(&buffer, format!("You might have to forward port {} or {}.", port1, port2));


		thread::spawn(move || {
			for stream in listener.incoming() {
				let mut stream = match stream {
					Ok(stream) => stream,
					Err(err) => {
						eprintln!("{}", err);
						return;
					}
				};
				// Clone shit.
				let buffer2 = Arc::clone(&buffer2);
				current_user += 1;
				let user_id = current_user;
				// Create a new thread for every client.
				let mut session = Arc::new(ServerSession {
					user_id: user_id,
					stream: stream,
					buffer: buffer2,
					nickname: Mutex::new(String::from("batman")),
					all_da_freggin_sessions: Arc::clone(&sessions_clone)
				});
				sessions_clone.lock().unwrap().push(Arc::clone(&session));
				thread::spawn(move || {
					if let Err(err) = session.handle_client() {
						eprintln!("client error: {}", err);
					}
					if let Err(err) = session.handle_disconnect() {
						eprintln!("disconnect error: {}", err);
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
					// Hacky way of deleting clients that fail, such as if they disconnect lol
					sessions.lock().unwrap().retain(|session| {
						writeln!(&session.stream, "{} [{}]: {}", nickname, user_id, line).is_ok()
					});
					print(&buffer, format!("{} [{}]: {}", nickname, user_id, line));
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
	if cfg!(target_os = "windows") { // The special snowflake...
		println!("It looks like you're on Windows. Use mingw64/Git Bash, Windows Subsystem for Linux or msys2 to run this program properly. (Or anything that's not cmd or PowerShell, basically.)");
	}
	println!("\x1b[2J\x1b[H"); // Of course Windows doesn't support this..
	for msg in buffer {
		println!("{}", msg);
	}
	println!("--- Message: ---");
}


// fn handle_client(logfile: &mut File, user_id: u64, stream: TcpStream) -> Result<(), Error> {}

fn print(buffer: &Mutex<Vec<String>>, line: String){
	// Redraw.
	let mut buffer = buffer.lock().unwrap();
	buffer.push(line);
	redraw(&buffer);
}
