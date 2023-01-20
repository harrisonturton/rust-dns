use super::buffer::ByteBuffer;
use anyhow::{anyhow, Context};
use std::error;

#[derive(Debug)]
pub struct Record {
    pub name: String,
    pub typ: u16,
    pub class: u16,
    pub ttl: i32,
    pub len: u16,
}

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

pub fn parse_single_record(packet: &mut ByteBuffer) -> Result<Record, Box<dyn error::Error>> {
    let name = parse_name(packet)?;
    let typ = packet.read_u16()?;
    let class = packet.read_u16()?;
    let ttl = packet.read_i32()?;
    let len = packet.read_u16()?;
    Ok(Record {
        name,
        typ,
        class,
        ttl,
        len,
    })
}

fn parse_name(packet: &mut ByteBuffer) -> Result<String, Box<dyn error::Error>> {
    let mut name_parts = vec![];
    let mut original_position = None;
    loop {
        let label_len = packet.read().context("could not read label length")?;
        println!("len: {:#010b}", label_len);
        if label_len == 0 {
            // Reached end of the label sequence
            println!("End of label sequence");
            break;
        }

        // A label is preceded by a u8 indicating the number of characters in
        // the label. When the two most significant bits of this number are set,
        // the label is actually a pointer to another section in the packet.
        let label_flags = label_len >> 6;
        println!("label flags: {:#04b}", label_flags);
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
        println!("Parts: {:?}", name_parts);
    }
    if let Some(pos) = original_position {
        packet.jump(pos)?;
    }
    Ok(name_parts.join("."))
}
