use crate::{
    messages::{Class, RecordType},
    utils::{take_bits_u8, Binary},
};
use nom::{bits::bits, IResult};

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,
    pub record_type: RecordType,
    pub class: Class,
    pub ttl: u32,
    pub rd_length: u16,
}

impl ResourceRecord {
    pub fn serialize(&self) -> Vec<u8> {
        let mut name: Vec<u8> = vec![0x0, 0x0];
        name.extend(self.name.as_bytes());
        let mut record_type = self.record_type.serialize();
        let mut class = self.class.serialize();
        let mut ttl = self.ttl.to_be_bytes().to_vec();
        let mut rd_length = self.rd_length.to_be_bytes().to_vec();

        let mut res = vec![];
        res.append(&mut name);
        res.append(&mut record_type);
        res.append(&mut class);
        res.append(&mut ttl);
        res.append(&mut rd_length);
        res
    }
}

enum NameType {
    Literal,
    Pointer,
    Reserved(u8),
}

impl From<u8> for NameType {
    fn from(byte: u8) -> Self {
        match byte {
            0b0 => NameType::Literal,
            0b11 => NameType::Pointer,
            _ => NameType::Reserved(byte),
        }
    }
}

pub fn parse_resource_record(message: &[u8]) -> IResult<&[u8], ResourceRecord> {
    todo!()
}

fn parse_name<'a>(message: &[u8]) -> IResult<&[u8], String> {
    bits(name)(message)
}

fn name<'a>(message: Binary<'a>) -> IResult<Binary<'a>, String> {
    let (rest, pointer_flag) = take_bits_u8(message, 2)?;
    let pointer_flag: NameType = pointer_flag.into();
    match pointer_flag {
        NameType::Literal => {}
        NameType::Pointer => panic!("Pointers not implemented"),
        NameType::Reserved(value) => {
            panic!("Unimplemented resource record name pointer flag {}", value)
        }
    }

    todo!()
}
