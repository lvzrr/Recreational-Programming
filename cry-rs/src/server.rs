use crate::keygen::*;
use crate::lang::*;
use crate::matrixopts::*;
use crate::modmatrix::*;
use byteorder::{BigEndian, ReadBytesExt};
use local_ip_address::local_ip;
use pnet::{datalink, ipnetwork::IpNetwork};
use std::io::Read;
use std::io::Write;
use std::u128;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

// Todo: username handling, implement writing methods and that shiii

pub fn runserver() {
    let ip = local_ip().expect("Error retrieving local IP address");

    let port: u16 = 8888;
    let socket: SocketAddr = SocketAddr::new(ip, port);

    println!("Server running on {}:{}", ip, port);

    let stop_signal = Arc::new(AtomicBool::new(false));

    let listener = TcpListener::bind(socket).expect("Error binding to socket");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                if !stop_signal.load(Ordering::Relaxed) {
                    println!(
                        "New connection: {}",
                        s.peer_addr().map_or_else(
                            |_| "Unknown address".to_string(),
                            |addr| addr.to_string()
                        )
                    );
                    thread::spawn(move || {
                        if let Err(e) = handleclient(&s) {
                            eprintln!("Error handling client: {}", e);
                        }
                    });
                } else {
                    eprintln!("Server is shutting down, rejecting new connection");
                    let _ = s.shutdown(std::net::Shutdown::Both);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    ctrlc::set_handler(move || {
        println!("Shutdown signal received");
        stop_signal.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl+C handler");
}

fn handleclient(mut stream: &TcpStream) -> Result<(), std::io::Error> {
    if auth(&stream).is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Client provided an incorrect solution",
        ));
    }

    let mut idbuf: [u8; 16] = [0; 16];
    stream.read_exact(&mut idbuf).unwrap_or_else(|e| {
        eprintln!("Error reading from client: {}", e);
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        ()
    });

    idbuf.reverse();

    let id: u128 = ((&idbuf[..]).read_u128::<BigEndian>().unwrap()) ^ u128::MAX;

    println!("Client logged as [{}]", id);

    Ok(())
}

fn auth(mut stream: &TcpStream) -> Result<(), std::io::Error> {
    println!("Authenticating client");
    let solution: u128 = gen_solution();
    let key_uninitialized: [u128; 25] = gen_key();
    let key_expanded: [u128; 2500] = expand_key(key_uninitialized);

    for i in 0..25 {
        stream
            .write_all(&key_uninitialized[i].to_be_bytes())
            .unwrap_or_else(|e| {
                eprintln!("Error writing to client: {}", e);
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Error while Auth",
                ))
                .unwrap()
            });
    }

    let mut matrix: [[u128; 5]; 5] =
        gen_matrix_variation(generate_matrix(&solution), &key_expanded);
    for i in 0..5 {
        for j in 0..5 {
            stream
                .write_all(&matrix[i][j].to_be_bytes())
                .unwrap_or_else(|e| {
                    eprintln!("Error writing to client: {}", e);
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Error while Auth",
                    ))
                    .unwrap()
                });
        }
    }

    for i in 0..5 {
        for j in 0..5 {
            let mut buffer: [u8; 16] = [0; 16];
            match stream.read_exact(&mut buffer) {
                Ok(_) => {
                    matrix[i][j] = (&buffer[..]).read_u128::<BigEndian>()?;
                    println!("Blueprint: {}", matrix[i][j]);
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    stream
                        .shutdown(std::net::Shutdown::Both)
                        .expect("Could not shutdown stream");
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Client provided an incorrect solution",
                    ))
                    .unwrap()
                }
            }
        }
    }

    matrix = undochanges(matrix, &key_expanded);

    if (solution - (solution % 100)) == solvematrix(&matrix) {
        stream
            .write_all(&(u128::MAX ^ (solution + (solution / 3))).to_be_bytes())
            .unwrap_or_else(|e| {
                eprintln!("Error writing to client: {}", e);
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Client provided an incorrect solution",
                ))
                .unwrap()
            });
        Ok(())
    } else {
        eprintln!("Client provided an incorrect solution");
        let _ = stream.shutdown(std::net::Shutdown::Both);
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Client provided an incorrect solution",
        ))
    }
}
