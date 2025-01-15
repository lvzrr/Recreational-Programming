use crate::data::overseer;
use crate::keygen::*;
use crate::lang::*;
use crate::matrixopts::*;
use crate::modmatrix::*;
use byteorder::{BigEndian, ReadBytesExt};
use local_ip_address::local_ip;
use std::io::Read;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::u128;
use std::usize;
use std::{
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

pub fn runserver() {
    let ip: IpAddr = local_ip().expect("Error retrieving local IP address");
    let port: u16 = 8888;
    let socket: SocketAddr = SocketAddr::new(ip, port);

    println!("Server running on {}:{}", ip, port);

    let stop_signal = Arc::new(AtomicBool::new(false));

    let listener = TcpListener::bind(socket).expect("Error binding to socket");
    listener
        .set_nonblocking(true)
        .expect("Error setting non-blocking mode");

    let msg_buf: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let key_buf: Arc<Mutex<Vec<[u128; 15]>>> = Arc::new(Mutex::new(Vec::new()));
    let msgc: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    let stop_signal_clone = stop_signal.clone();

    {
        let msg_buf_clone = Arc::clone(&msg_buf);
        let key_buf_clone = Arc::clone(&key_buf);
        let msgc_clone = Arc::clone(&msgc);
        let stop_signal_clone2 = Arc::clone(&stop_signal);

        thread::spawn(move || {
            overseer(msgc_clone, msg_buf_clone, key_buf_clone, stop_signal_clone2);
        });
    }

    ctrlc::set_handler(move || {
        println!("Shutdown signal received");
        stop_signal_clone.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl+C handler");

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                if stop_signal.load(Ordering::Relaxed) {
                    eprintln!(
                        "Server is shutting down, rejecting new connection from {}",
                        addr
                    );
                    let _ = stream.shutdown(std::net::Shutdown::Both);
                } else {
                    println!("New connection: {}", addr);

                    let msg_buf_clone = Arc::clone(&msg_buf);
                    let key_buf_clone = Arc::clone(&key_buf);
                    let msgc_clone = Arc::clone(&msgc);

                    thread::spawn(move || {
                        if let Err(e) =
                            handleclient(&stream, &msg_buf_clone, &key_buf_clone, &msgc_clone)
                        {
                            eprintln!("Error handling client {}: {}", addr, e);
                        }
                    });
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }

        if stop_signal.load(Ordering::Relaxed) {
            println!("Shutting down server...");
            break;
        }
    }
}

fn handleclient(
    mut stream: &TcpStream,
    msg_buf: &Arc<Mutex<Vec<String>>>,
    key_buf: &Arc<Mutex<Vec<[u128; 15]>>>,
    msgc: &Arc<Mutex<usize>>,
) -> Result<(), std::io::Error> {
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
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Error while Auth",
        ))
        .unwrap()
    });

    idbuf.reverse();

    let id: u128 = ((&idbuf[..]).read_u128::<BigEndian>().unwrap()) ^ u128::MAX;

    println!("Client logged as [{}]", id);

    let mut msg_buf_lock = msg_buf.lock().unwrap();
    let mut key_buf_lock = key_buf.lock().unwrap();
    let mut msgc_lock = msgc.lock().unwrap();

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
