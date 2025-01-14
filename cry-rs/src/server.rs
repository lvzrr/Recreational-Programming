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

fn send_to_all(ip: Ipv4Addr, stop_signal: Arc<AtomicBool>) -> Result<(), std::io::Error> {
    let port: u16 = 8080;

    let mut mask_bits = 0;

    for interface in datalink::interfaces() {
        for ip_network in interface.ips {
            if ip_network.ip() == IpAddr::V4(ip) {
                if let std::net::IpAddr::V4(subnet_mask) = ip_network.mask() {
                    mask_bits = subnet_mask
                        .octets()
                        .iter()
                        .fold(0, |acc, &octet| acc + octet.count_ones() as u32);
                    break;
                }
            }
        }
        if mask_bits > 0 {
            break;
        }
    }

    if mask_bits == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Subnet mask not found",
        ));
    }

    let subnet_network =
        IpNetwork::new(IpAddr::V4(ip), mask_bits as u8).expect("Error creating subnet network");

    let socket = UdpSocket::bind("0.0.0.0:0")?;

    println!("Started sending to all addresses in the subnet");

    let start_ip = subnet_network.network();
    let end_ip = subnet_network.broadcast();

    let start_ip = match start_ip {
        IpAddr::V4(addr) => addr,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Not an IPv4 address",
            ))
        }
    };

    let end_ip = match end_ip {
        IpAddr::V4(addr) => addr,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Not an IPv4 address",
            ))
        }
    };

    loop {
        let mut current_ip = start_ip;
        while current_ip <= end_ip {
            if current_ip != start_ip {
                let target_address = SocketAddr::new(IpAddr::V4(current_ip), port);

                let message = format!(
                    "{}{}:{}{}",
                    u128::MAX / port as u128,
                    ip,
                    port,
                    u128::MAX / port as u128
                );
                if let Err(err) = socket.send_to(message.as_bytes(), target_address) {
                    eprintln!("Error sending data to {}: {}", current_ip, err);
                    break;
                }
            }
            current_ip = Ipv4Addr::from(u32::from(current_ip) + 1);
        }
        thread::sleep(Duration::from_secs(20));
    }
}

pub fn runserver() {
    let ip = local_ip().expect("Error retrieving local IP address");

    //let ip_v4: Ipv4Addr = match ip {
    //    IpAddr::V4(addr) => addr,
    //    _ => Ipv4Addr::new(127, 0, 0, 1),
    //};

    let port: u16 = 8888;
    let socket: SocketAddr = SocketAddr::new(ip, port);

    println!("Server running on {}:{}", ip, port);

    let stop_signal = Arc::new(AtomicBool::new(false));
    //let stop_signal_clone = Arc::clone(&stop_signal);

    //let handle = thread::spawn(move || {
    //    send_to_all(ip_v4, stop_signal_clone).expect("Error broadcasting");
    //});

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

    //handle.join().expect("Broadcast thread panicked");
}

fn handleclient(mut stream: &TcpStream) -> Result<(), std::io::Error> {
    println!("Handling client");
    let solution: u128 = gen_solution();
    println!("Solution generated: {}", solution);
    let key_uninitialized: [u128; 25] = gen_key();
    println!("Key generated");
    println!("Key: {:?}", key_uninitialized);
    let key_expanded: [u128; 2500] = expand_key(key_uninitialized);

    for i in 0..25 {
        stream.write_all(&key_uninitialized[i].to_be_bytes())?;
    }

    let mut matrix: [[u128; 5]; 5] =
        gen_matrix_variation(generate_matrix(&solution), &key_expanded);
    println!("Matrix generated");
    for i in 0..5 {
        for j in 0..5 {
            stream.write_all(&matrix[i][j].to_be_bytes())?;
            println!("Sent: {}", matrix[i][j]);
        }
    }

    for i in 0..5 {
        for j in 0..5 {
            let mut buffer: [u8; 16] = [0; 16];
            match stream.read_exact(&mut buffer) {
                Ok(_) => {
                    matrix[i][j] = (&buffer[..]).read_u128::<BigEndian>()?;
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    return Err(e);
                }
            }
        }
    }

    matrix = undochanges(matrix, &key_expanded);

    if (solution - (solution % 100)) == solvematrix(&matrix) {
        stream.write_all(&(u128::MAX ^ (solution + (solution / 3))).to_be_bytes())?;
    } else {
        eprintln!("Client provided an incorrect solution");
        let _ = stream.shutdown(std::net::Shutdown::Both);
    }

    Ok(())
}
