enum Comm_Type {
    FileGet,
    ImageGet,
    FilePush,
    ImagePush,
    MessageGet,
    MessagePush,
}

struct push_msg_packet {
    padding1: u128,
    sender_id: [u8; 254],
    receiver_id: [u8; 254],
    data: String,
    challenge: [[u128; 5]; 5],
}

struct get_msg_packet {
    padding1: u128,
    sender_id: [u8; 254],
    receiver_id: [u8; 254],
    data: String,
    challenge: [[u128; 5]; 5],
}
