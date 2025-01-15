use crate::crypt::*;
use crate::keygen::*;
use crate::matrixopts::*;
use crate::modmatrix::*;
use std::default;
use std::io::stdin;
use std::io::Read;
use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::u128;

pub fn runclient() {
    print!("Enter the server IP address: ");
    let mut input: String = String::new();
    std::io::stdout().flush().expect("Error flushing stdout");
    stdin()
        .read_line(&mut input)
        .expect("Error reading server IP address");

    let serv_ip: IpAddr = input.trim().parse().expect("Error parsing IP address");

    let port: u16 = 8888;
    let socket: SocketAddr = SocketAddr::new(serv_ip, port);

    println!("Connecting to server at {}", socket);

    let mut s = TcpStream::connect(socket).expect("Error connecting to server");

    println!("Connected to server");

    if auth(&s) == 1 {
        ()
    }

    let mut hostname: String = String::new();
    print!("Enter your username: ");
    std::io::stdout().flush().expect("Error flushing stdout");
    stdin()
        .read_line(&mut hostname)
        .expect("Error reading username");

    s.shutdown(std::net::Shutdown::Both)
        .expect("Error shutting down connection");
    ()
}

fn auth(mut s: &TcpStream) -> u8 {
    let mut key: [u128; 25] = [0; 25];

    for i in 0..25 {
        let mut buffer = [0; 16];
        s.read_exact(&mut buffer)
            .unwrap_or_else(|_| panic!("Error reading from server"));
        key[i] = u128::from_be_bytes(buffer);
        println!("Received: {}", key[i]);
    }

    let key_expanded: [u128; 2500] = expand_key(key);

    let mut matrix: [[u128; 5]; 5] = [[0; 5]; 5];

    println!("Receiving matrix from server");

    for i in 0..5 {
        for j in 0..5 {
            let mut buffer = [0; 16];
            s.read_exact(&mut buffer)
                .unwrap_or_else(|_| panic!("Error reading from server"));
            matrix[i][j] = u128::from_be_bytes(buffer);
            println!("Received: {}", matrix[i][j]);
        }
    }

    matrix = undochanges(matrix, &key_expanded);

    let solution = solvematrix(&matrix);
    println!("Solution: {}", solution);

    let mut send_matrix = generate_matrix(&(solution - (solution % 100)));

    send_matrix = gen_matrix_variation(send_matrix, &key_expanded);

    for i in 0..5 {
        for j in 0..5 {
            s.write_all(&send_matrix[i][j].to_be_bytes())
                .expect("Error writing to server");
            println!("Sent: {}", send_matrix[i][j]);
        }
    }

    let mut authresult: u128 = 0;
    let mut buffer = [0; 16];

    s.read_exact(&mut buffer)
        .unwrap_or_else(|_| panic!("Error reading from server"));

    authresult = u128::from_be_bytes(buffer);

    if authresult ^ u128::MAX == solution + (solution / 3) {
        1
    } else {
        s.shutdown(std::net::Shutdown::Both)
            .expect("Error shutting down connection");
        0
    }
}
