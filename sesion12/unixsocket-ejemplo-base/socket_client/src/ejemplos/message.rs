use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;
use std::str;

#[derive(Serialize,Deserialize,Debug)]
pub struct Message {
	id: u32,
	msg: String
}


fn u32_to_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}


fn u8_to_u32(array:&[u8]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}


#[derive(Serialize,Deserialize,Debug)]
pub struct MessageBytes<'a> {
	#[serde(with = "serde_bytes")]
	pub id:&'a [u8],
	#[serde(with = "serde_bytes")]
	pub msg: Vec<u8>
}

impl Message {
	pub fn welcome(id:u32) -> Self {
		Message{
			id, 
			msg: "Welcome to Unix Socket".to_string()}
	}

	pub fn serialize(&self)->Vec<u8> {
		let mb = MessageBytes{
			id: &u32_to_u8(self.id),
			msg: self.msg.as_bytes().to_vec()
		};
		bincode::serialize(&mb).unwrap()
	}

	pub fn deserialize(msg:&[u8]) -> Self {
		let msg_bytes:MessageBytes = bincode::deserialize(msg)
			.unwrap();

		let txt = str::from_utf8(&msg_bytes.msg).unwrap();

		Message {
			id: u8_to_u32(msg_bytes.id),
			msg:txt.to_string()
		}
	}
}
