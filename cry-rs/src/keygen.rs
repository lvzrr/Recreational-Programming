use std::{fs::File, io::Read, u128};

pub fn gen_key() -> [u128; 25] {
    let mut key: [u128; 25] = [0; 25];
    let mut f = File::open("/dev/urandom").expect("Cannot read from random source");

    for i in 0..25 {
        let mut buffer: [u8; 16] = [0; 16];
        f.read_exact(&mut buffer)
            .expect("Cannot read from random source");
        key[i] = u128::from_le_bytes(buffer);
    }

    key
}

pub fn gen_solution() -> u128 {
    let mut f = File::open("/dev/urandom").expect("Cannot read from random source");

    let mut buffer: [u8; 16] = [0; 16];
    f.read_exact(&mut buffer)
        .expect("Cannot read from random source");

    return 10000000 + (u128::from_le_bytes(buffer) % 40000000);
}

fn expand(exp_key: &[u128; 2500], index: usize) -> u128 {
    let mut out: u128 = 10;
    for i in (index - 10..index - 1).rev() {
        let t = exp_key[i] % 100000000000000;
        out = ((100000 + (out % 234)) ^ t) + (out ^ t).pow(((out as f64).cos()).round() as u32);
    }

    out ^ 0xFF
}

pub fn expand_key(key: [u128; 25]) -> [u128; 2500] {
    let mut expanded_key: [u128; 2500] = [0; 2500];
    expanded_key[..25].clone_from_slice(&key);
    for i in 25..2500 {
        expanded_key[i] = expand(&expanded_key, i);
    }
    expanded_key
}
