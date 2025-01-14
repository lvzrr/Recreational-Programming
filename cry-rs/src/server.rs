use crate::keygen::*;
use crate::matrixopts::*;
use crate::modmatrix::*;
use byteorder::{BigEndian, ReadBytesExt};
use local_ip_address::local_ip;
use pnet::{datalink, ipnetwork::IpNetwork};
use std::io::Read;
use std::io::Write;
use std::{
    net::{IpAddr, SocketAddr, TcpListener, TcpStream, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
fn broadcast(ip: IpAddr, stop_signal: Arc<AtomicBool>) -> Result<(), std::io::Error> {
    let port: u16 = 8080;

    let mut mask_bits = 0;

    for interface in datalink::interfaces() {
        for ip_network in interface.ips {
            if ip_network.ip() == ip {
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
        IpNetwork::new(ip, mask_bits as u8).expect("Error creating subnet network");

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    println!("Started broadcast");

    let broadcast_address = SocketAddr::new(subnet_network.broadcast(), port);

    while !stop_signal.load(Ordering::Relaxed) {
        if let Err(err) = socket.send_to(
            format!("Broadcast from {}:{}", ip, port).as_bytes(),
            broadcast_address,
        ) {
            eprintln!("Error sending data: {}", err);
            break;
        }
        thread::sleep(Duration::from_millis(500));
    }

    println!("Broadcast stopped");
    Ok(())
}

fn handleclient(mut stream: &TcpStream) {
    let solution: u128 = gen_solution();
    let key_uninitialized: [u128; 25] = gen_key();
    let key_expanded: [u128; 2500] = expand_key(key_uninitialized);

    for i in 0..25 {
        stream
            .write_all(&key_uninitialized[i].to_be_bytes())
            .expect("Error writing to stream");
    }

    let mut matrix: [[u128; 5]; 5] =
        gen_matrix_variation(generate_matrix(&solution), &key_expanded);

    for i in 0..5 {
        for j in 0..5 {
            stream
                .write_all(&matrix[i][j].to_be_bytes())
                .expect("Error writing to stream");
        }
    }

    for i in 0..5 {
        for j in 0..5 {
            let mut buffer: [u8; 16] = [0; 16];
            stream
                .read_exact(&mut buffer)
                .expect("Error writing to stream");
            matrix[i][j] = (&buffer[..]).read_u128::<BigEndian>().unwrap();
        }
    }

    matrix = undochanges(matrix, &key_expanded);

    if solution == solvematrix(&matrix) {
        stream
            .write_all(&(u128::MAX ^ (solution + (solution / 3))).to_be_bytes())
            .expect("Error writing to stream");
    } else {
        stream
            .shutdown(std::net::Shutdown::Both)
            .expect("Error shutting down stream");
        ()
    }
}

pub fn runserver() {
    let ip = local_ip().expect("Error retrieving local IP address");
    let port: u16 = 8888;
    let socket: SocketAddr = SocketAddr::new(ip, port);

    println!("Server running on {}:{}", ip, port);

    let stop_signal = Arc::new(AtomicBool::new(false));
    let stop_signal_clone = Arc::clone(&stop_signal);

    let handle = thread::spawn(move || {
        broadcast(ip, stop_signal_clone).expect("Error broadcasting");
    });

    let listener = TcpListener::bind(socket).expect("Error binding to socket");

    for stream in listener.incoming() {
        let s: TcpStream = stream.expect("Error accepting connection");
        println!("New connection: {}", s.peer_addr().unwrap());
        thread::spawn(move || handleclient(&s));
    }

    ctrlc::set_handler(move || {
        println!("Shutdown signal received");
        stop_signal.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl+C handler");

    handle.join().expect("Broadcast thread panicked");
}
