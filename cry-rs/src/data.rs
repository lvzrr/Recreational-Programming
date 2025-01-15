use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use crate::crypt::*;
use crate::keygen::*;
use crate::lang::*;

pub fn local_data_manager(
    t: Comm_Type,
    data: MsgPacket,
    nmsgs: &mut usize,
    msgstore: &mut Vec<String>,
    keystore: &mut Vec<[u128; 15]>,
) -> () {
    match t {
        Comm_Type::MessagePush => {
            *nmsgs += 1;
            save_msg(data, msgstore);
        }
        Comm_Type::MessageGet => {}
        _ => (), //todo
    }
    ()
}

pub fn overseer(
    nmsgs: Arc<Mutex<usize>>,
    msgstore: Arc<Mutex<Vec<String>>>,
    keystore: Arc<Mutex<Vec<[u128; 15]>>>,
    stop: Arc<AtomicBool>,
) {
    loop {
        if stop.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        let mut nmsgs_lock = nmsgs.lock().unwrap();
        let mut msgstore_lock = msgstore.lock().unwrap();
        let mut keystore_lock = keystore.lock().unwrap();

        if *nmsgs_lock > 5 {
            msgstore_lock.resize(5, "0".to_string());
            keystore_lock.resize(5, [0; 15]);

            msgstore_lock.drain(0..3);
            keystore_lock.drain(0..3);

            *nmsgs_lock -= 5;
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn save_msg(data: MsgPacket, msgstore: &mut Vec<String>) -> u8 {
    let send_id: u128 = data.sender_id;
    let msg: String = data.data;
    let key_iv: [u128; 15] = data.key_iv;

    let exp_key = expand_key_msg(&key_iv, 11);

    let dec_msg = decrypt_msg(&msg, &exp_key);

    msgstore.push(format!("{}\\{}", send_id, dec_msg));

    return 0;
}
