use super::buffer::ByteBuffer;
use anyhow::{anyhow, Context};
use std::error;

// --------------------------------------------------
// Record
// --------------------------------------------------

pub fn parse_records(
    packet: &mut ByteBuffer,
    count: usize,
) -> Result<Vec<Record>, Box<dyn error::Error>> {
    let mut records = vec![];
    for _ in 0..count {
        let record = parse_single_record(packet)?;
        records.push(record);
    }
    Ok(records)
}

#[derive(Debug)]
pub struct Record {
    pub name: String,
    pub record_type: RecordType,
    pub class: Class,
    pub ttl: i32,
    pub len: u16,
    pub data: Data,
}

pub fn parse_single_record(packet: &mut ByteBuffer) -> Result<Record, Box<dyn error::Error>> {
    let name = parse_name(packet)?;
    let record_type = packet.read_u16()?;
    let record_type = parse_record_type(record_type);
    let class = packet.read_u16()?;
    let class = parse_class(class);
    let ttl = packet.read_i32()?;
    let len = packet.read_u16()?;
    let data = packet.read_range(len as usize)?.clone().to_vec();
    let data = parse_data(&record_type, &data)?;
    Ok(Record {
        name,
        record_type,
        class,
        ttl,
        len,
        data,
    })
}

pub fn serialize_records(records: &[Record]) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let mut bytes = vec![];
    for record in records {
        let record_bytes = serialize_single_record(&record)?;
        bytes.extend(record_bytes);
    }
    Ok(bytes)
}

pub fn serialize_single_record(record: &Record) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let mut bytes = vec![];
    let name = serialize_name(&record.name)?;
    bytes.extend(name);
    let record_type = serialize_record_type(&record.record_type);
    bytes.extend(record_type);
    let class = serialize_record_type(&record.record_type);
    bytes.extend(class);
    let ttl = record.ttl.to_be_bytes();
    bytes.extend(ttl);
    let len = record.len.to_be_bytes();
    bytes.extend(len);
    let data = serialize_data(&record.data);
    bytes.extend(data);
    Ok(bytes)
}

pub fn serialize_name(name: &str) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let parts = name.split(".");
    let mut bytes = vec![];
    for part in parts {
        let len = part.len() as u8;
        if len > 63 {
            return Err(anyhow!("label cannot be larger than 63 characters").into());
        }
        bytes.push(len);
        bytes.extend(part.as_bytes());
    }
    bytes.push(0x0);
    Ok(bytes)
}

// --------------------------------------------------
// Record type
// --------------------------------------------------

#[derive(Debug)]
pub enum RecordType {
    A,
    Ns,
    Unknown(u16),
}

pub fn parse_record_type(record_type: u16) -> RecordType {
    match record_type {
        1 => RecordType::A,
        2 => RecordType::Ns,
        _ => RecordType::Unknown(record_type),
    }
}

pub fn serialize_record_type(record_type: &RecordType) -> Vec<u8> {
    match record_type {
        RecordType::A => u16::to_be_bytes(1).to_vec(),
        RecordType::Ns => u16::to_be_bytes(2).to_vec(),
        RecordType::Unknown(value) => value.to_be_bytes().to_vec(),
    }
}

// --------------------------------------------------
// Class
// --------------------------------------------------

#[derive(Debug)]
pub enum Class {
    In,
    Unknown(u16),
}

pub fn parse_class(class: u16) -> Class {
    match class {
        1 => Class::In,
        _ => Class::Unknown(class),
    }
}

pub fn serialize_class(class: &Class) -> Vec<u8> {
    match class {
        Class::In => u16::to_be_bytes(1).to_vec(),
        Class::Unknown(value) => value.to_be_bytes().to_vec(),
    }
}

// --------------------------------------------------
// Record data
// --------------------------------------------------

#[derive(Debug)]
pub enum Data {
    Addr([u8; 4]),
    Unknown(Vec<u8>),
}

pub fn parse_data(record_type: &RecordType, data: &[u8]) -> Result<Data, Box<dyn error::Error>> {
    match record_type {
        RecordType::A => {
            let addr: [u8; 4] = data.try_into()?;
            Ok(Data::Addr(addr))
        }
        _ => Ok(Data::Unknown(data.to_vec())),
    }
}

pub fn serialize_data(data: &Data) -> Vec<u8> {
    match data {
        Data::Addr(addr) => addr.to_vec(),
        Data::Unknown(data) => data.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_name_returns_expected_bytes() {
        let name = "example.com";
        let expected_bytes: Vec<u8> = vec![
            0b00000000 & 7,
            'e' as u8,
            'x' as u8,
            'a' as u8,
            'm' as u8,
            'p' as u8,
            'l' as u8,
            'e' as u8,
            3,
            'c' as u8,
            'o' as u8,
            'm' as u8,
            0,
        ];
        assert_eq!(serialize_name(&name).unwrap(), expected_bytes);
    }
}

pub fn parse_name(packet: &mut ByteBuffer) -> Result<String, Box<dyn error::Error>> {
    let mut name_parts = vec![];
    let mut original_position = None;
    loop {
        let label_len = packet.read().context("could not read label length")?;
        if label_len == 0 {
            // Reached end of the label sequence
            break;
        }

        // A label is preceded by a u8 indicating the number of characters in
        // the label. When the two most significant bits of this number are set,
        // the label is actually a pointer to another section in the packet.
        let label_flags = label_len >> 6;
        match label_flags {
            0b00 => {
                let label_bytes = packet.read_range(label_len as usize)?;
                let label = String::from_utf8_lossy(label_bytes);
                name_parts.push(label);
            }
            0b11 => {
                let pointer_pos = packet.read().context("could not read pointer position")?;
                original_position = Some(packet.pos());
                packet.jump(pointer_pos as usize)?;
                let label_len = packet.read().context("could not get label length")?;
                let label_bytes = packet.read_range(label_len as usize)?;
                let label = String::from_utf8_lossy(label_bytes);
                name_parts.push(label);
            }
            _ => return Err(anyhow!("unknown label flags: {:#04b}", label_flags).into()),
        };
    }
    if let Some(pos) = original_position {
        packet.jump(pos)?;
    }
    let name = if name_parts.is_empty() {
        ".".to_string()
    } else {
        name_parts.join(".")
    };
    Ok(name)
}
