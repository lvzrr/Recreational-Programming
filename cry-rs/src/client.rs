use crate::crypt::*;
use crate::keygen::*;
use crate::matrixopts::*;
use crate::modmatrix::*;
use byteorder::{BigEndian, ReadBytesExt};
use std::u128;
use std::{
    io::*,
    net::{IpAddr, SocketAddr, TcpStream},
    time::Instant,
};

pub fn runclient() {
    let initcon: Instant = Instant::now();
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

    let id: u128 = genlogin(&s, initcon);

    s.shutdown(std::net::Shutdown::Both)
        .expect("Error shutting down connection");
    ()
}

fn genlogin(mut s: &TcpStream, initcon: Instant) -> u128 {
    let mut id: u128 = 10000000 + initcon.elapsed().as_nanos() % 50000000;
    let id_nomut = id;
    let mut tempbuf: [u8; 16] = id.to_be_bytes();
    tempbuf.reverse();
    id = ((&tempbuf[..]).read_u128::<BigEndian>().unwrap()) ^ u128::MAX;

    s.write_all(&id.to_be_bytes()).unwrap_or_else(|e| {
        eprintln!("Error writing to server: {}", e);
        s.shutdown(std::net::Shutdown::Both)
            .expect("Error shutting down connection");
        ()
    });
    println!("Connected as [{}]", id_nomut);
    id
}
fn auth(mut s: &TcpStream) -> u8 {
    println!("Authenticating with server");
    let mut key: [u128; 25] = [0; 25];

    for i in 0..25 {
        let mut buffer = [0; 16];
        s.read_exact(&mut buffer)
            .unwrap_or_else(|_| panic!("Error reading from server"));
        key[i] = u128::from_be_bytes(buffer);
    }

    let key_expanded: [u128; 2500] = expand_key(key);

    let mut matrix: [[u128; 5]; 5] = [[0; 5]; 5];

    for i in 0..5 {
        for j in 0..5 {
            let mut buffer = [0; 16];
            s.read_exact(&mut buffer)
                .unwrap_or_else(|_| panic!("Error reading from server"));

            matrix[i][j] = u128::from_be_bytes(buffer);
            println!("Blueprint: {}", matrix[i][j]);
        }
    }

    matrix = undochanges(matrix, &key_expanded);

    let solution = solvematrix(&matrix);

    let mut send_matrix = generate_matrix(&(solution - (solution % 100)));

    send_matrix = gen_matrix_variation(send_matrix, &key_expanded);

    for i in 0..5 {
        for j in 0..5 {
            s.write_all(&send_matrix[i][j].to_be_bytes())
                .expect("Error writing to server");
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
