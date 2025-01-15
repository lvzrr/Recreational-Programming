pub enum Comm_Type {
    FileGet,
    ImageGet,
    FilePush,
    ImagePush,
    MessageGet,
    MessagePush,
}

pub struct MsgPacket {
    pub padding1: u128,
    pub sender_id: u128,
    pub receiver_id: u128,
    pub data: String,
    pub key_iv: [u128; 15],
}
