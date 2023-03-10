use super::buffer::ByteBuffer;
use super::header::{self, Header};
use super::question::{self, Question};
use super::record::{self, Record};
use std::error;

#[derive(Debug)]
pub struct DnsPacket {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Record>,
    pub authoritative_entries: Vec<Record>,
    pub resource_entries: Vec<Record>,
}

pub fn serialize_dns_packet(packet: &DnsPacket) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let mut bytes = vec![];
    let mut header = header::serialize_header(&packet.header);
    bytes.append(&mut header);
    let mut questions = question::serialize_questions(&packet.questions)?;
    bytes.append(&mut questions);
    let mut answers = record::serialize_records(&packet.answers)?;
    bytes.append(&mut answers);
    let mut authoritative_entries = record::serialize_records(&packet.authoritative_entries)?;
    bytes.append(&mut authoritative_entries);
    let mut resource_entries = record::serialize_records(&packet.resource_entries)?;
    bytes.append(&mut resource_entries);
    Ok(bytes)
}

pub fn parse_dns_packet(packet: &[u8]) -> Result<DnsPacket, Box<dyn error::Error>> {
    let mut packet = ByteBuffer::from(packet);
    let header = header::parse_header(&mut packet)?;
    let questions = question::parse_questions(&mut packet, header.questions as usize)?;
    let answers = record::parse_records(&mut packet, header.answers as usize)?;
    let authoritative_entries =
        record::parse_records(&mut packet, header.authoritative_entries as usize)?;
    let resource_entries = record::parse_records(&mut packet, header.resource_entries as usize)?;
    Ok(DnsPacket {
        header,
        questions,
        answers,
        authoritative_entries,
        resource_entries,
    })
}
