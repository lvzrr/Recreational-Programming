pub fn crypt_msg(s: &String, key: &[u128; 256]) -> String {
    let mut out: Vec<u8> = Vec::new();
    let bytearr = s.as_bytes();

    for i in 0..bytearr.len() {
        let c = bytearr[i] as u128;
        let k = key[i % 256];
        out.push((c ^ k) as u8);
    }

    String::from_utf8(out).unwrap()
}
pub fn decrypt_msg(s: &String, key: &[u128; 256]) -> String {
    crypt_msg(s, key)
}
