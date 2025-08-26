use clap::{Arg, Command};
use std::error::Error;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::thread;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Config {
    listen: bool,
    port: Option<u16>,
    host: Option<String>,
    udp: bool,
}

fn main() -> Result<()> {
    let matches = Command::new("retcat")
        .version("0.1.0")
        .about("A simple netcat implementation in Rust")
        .arg(Arg::new("listen")
            .short('l')
            .long("listen")
            .help("Listen mode, for inbound connects")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("Local port number"))
        .arg(Arg::new("udp")
            .short('u')
            .long("udp")
            .help("Use UDP instead of TCP")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("host")
            .value_name("HOST")
            .help("Hostname or IP address")
            .index(1))
        .arg(Arg::new("target_port")
            .value_name("PORT")
            .help("Port number")
            .index(2))
        .get_matches();

    let config = Config {
        listen: matches.get_flag("listen"),
        port: matches.get_one::<String>("port").map(|s| s.parse().unwrap()),
        host: matches.get_one::<String>("host").map(|s| s.to_string()),
        udp: matches.get_flag("udp"),
    };

    let target_port: Option<u16> = matches.get_one::<String>("target_port").map(|s| s.parse().unwrap());

    if config.listen {
        let port = config.port.or(target_port).unwrap_or(8080);
        if config.udp {
            udp_server(port)
        } else {
            tcp_server(port)
        }
    } else {
        let host = config.host.unwrap_or("localhost".to_string());
        let port = target_port.or(config.port).unwrap_or(8080);
        if config.udp {
            udp_client(&host, port)
        } else {
            tcp_client(&host, port)
        }
    }
}

fn tcp_client(host: &str, port: u16) -> Result<()> {
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(&addr)?;
    println!("Connected to {}", addr);
    
    relay_tcp_data(stream)?;
    Ok(())
}

fn tcp_server(port: u16) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr)?;
    println!("Listening on {}", addr);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection from {}", stream.peer_addr()?);
                relay_tcp_data(stream)?;
                break;
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

fn relay_tcp_data(mut stream: TcpStream) -> Result<()> {
    let mut stream_clone = stream.try_clone()?;
    
    let stdin_to_socket = thread::spawn(move || {
        let mut stdin = io::stdin();
        let mut buffer = [0; 1024];
        loop {
            match stdin.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if stream_clone.write_all(&buffer[..n]).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
    
    let socket_to_stdout = thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if io::stdout().write_all(&buffer[..n]).is_err() {
                        break;
                    }
                    io::stdout().flush().ok();
                }
                Err(_) => break,
            }
        }
    });
    
    let _ = stdin_to_socket.join();
    let _ = socket_to_stdout.join();
    
    Ok(())
}

fn udp_client(host: &str, port: u16) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let target = format!("{}:{}", host, port);
    println!("UDP client ready, sending to {}", target);
    
    let socket_clone = socket.try_clone()?;
    let target_clone = target.clone();
    
    let stdin_to_socket = thread::spawn(move || {
        let mut stdin = io::stdin();
        let mut buffer = [0; 1024];
        loop {
            match stdin.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if socket_clone.send_to(&buffer[..n], &target_clone).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
    
    let socket_to_stdout = thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match socket.recv_from(&mut buffer) {
                Ok((n, _)) => {
                    if io::stdout().write_all(&buffer[..n]).is_err() {
                        break;
                    }
                    io::stdout().flush().ok();
                }
                Err(_) => break,
            }
        }
    });
    
    let _ = stdin_to_socket.join();
    let _ = socket_to_stdout.join();
    
    Ok(())
}

fn udp_server(port: u16) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let socket = UdpSocket::bind(&addr)?;
    println!("UDP server listening on {}", addr);
    
    let mut buffer = [0; 1024];
    let mut client_addr = None;
    
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((n, addr)) => {
                if client_addr.is_none() {
                    client_addr = Some(addr);
                    println!("UDP connection from {}", addr);
                }
                
                io::stdout().write_all(&buffer[..n])?;
                io::stdout().flush()?;
            }
            Err(e) => eprintln!("UDP receive error: {}", e),
        }
    }
}