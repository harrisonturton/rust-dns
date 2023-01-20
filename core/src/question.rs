use anyhow::Context;

use super::buffer::ByteBuffer;
use std::error;

#[derive(Debug)]
pub struct Question {
    pub name: String,
    pub typ: u16,
    pub class: u16,
}

pub fn parse_questions(
    packet: &mut ByteBuffer,
    count: usize,
) -> Result<Vec<Question>, Box<dyn error::Error>> {
    let mut records = vec![];
    for _ in 0..count {
        let record = parse_single_question(packet)?;
        records.push(record);
    }
    Ok(records)
}

pub fn parse_single_question(packet: &mut ByteBuffer) -> Result<Question, Box<dyn error::Error>> {
    let mut name_parts = vec![];
    loop {
        let label_len = packet.read().context("could not read label length")?;
        if label_len == 0 {
            // Reached end of the label sequence
            break;
        }
        let label_bytes = packet.read_range(label_len as usize)?;
        let label = String::from_utf8_lossy(label_bytes);
        name_parts.push(label);
    }
    let name = name_parts.join(".");
    let typ = packet.read_u16()?;
    let class = packet.read_u16()?;
    Ok(Question { name, typ, class })
}
