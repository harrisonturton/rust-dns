use super::buffer::ByteBuffer;
use anyhow::{Context, Result};
use bitvec::{field::BitField, macros::internal::funty::Fundamental, prelude::Msb0, view::AsBits};
use std::error;

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub query: bool,
    pub opcode: u8,
    pub authoritative_answer: bool,
    pub truncation: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub reserved: u8,
    pub rcode: u8,
    pub questions: u16,
    pub answers: u16,
    pub authoritative_entries: u16,
    pub resource_entries: u16,
}

pub fn parse_header(packet: &mut ByteBuffer) -> Result<Header, Box<dyn error::Error>> {
    let bytes = packet.read_range(12)?;
    let mut header = ByteBuffer::from(bytes);
    let id = header.read_u16()?;
    let flags = header.read_range(2)?;
    let flags = flags.as_bits::<Msb0>().to_bitvec();
    let query = flags.get(0).context("query flag not found")?.as_bool();
    let opcode = flags.get(1..5).context("opcode not found")?.load::<u8>();
    let authoritative_answer = flags
        .get(5)
        .context("authoritative_answer flag not found")?
        .as_bool();
    let truncation = flags.get(6).context("truncation flag not found")?.as_bool();
    let recursion_desired = flags
        .get(7)
        .context("recursion_desired flag not found")?
        .as_bool();
    let recursion_available = flags
        .get(8)
        .context("recursion_available flag not found")?
        .as_bool();
    let reserved = flags
        .get(9..13)
        .context("reserved flags not found")?
        .load::<u8>();
    let rcode = flags.get(13..15).context("rcode not found")?.load::<u8>();
    let questions = header.read_u16()?;
    let answers = header.read_u16()?;
    let authoritative_entries = header.read_u16()?;
    let resource_entries = header.read_u16()?;
    Ok(Header {
        id,
        query,
        opcode,
        authoritative_answer,
        truncation,
        recursion_desired,
        recursion_available,
        reserved,
        rcode,
        questions,
        answers,
        authoritative_entries,
        resource_entries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // From examples/query_packet
    const QUERY_ID: u16 = 9398;
    const QUERY_QUERY: bool = true;
    const QUERY_OPCODE: u8 = 0;
    const QUERY_AUTHORITATIVE_ANSWER: bool = true;
    const QUERY_TRUNCATION: bool = true;
    const QUERY_RECURSION_DESIRED: bool = false;
    const QUERY_RECURSION_AVAILABLE: bool = true;
    const QUERY_RESERVED: u8 = 0b100;
    const QUERY_RCODE: u8 = 0;
    const QUERY_QUESTIONS: u16 = 1;
    const QUERY_ANSWERS: u16 = 0;
    const QUERY_AUTHORITATIVE_ENTRIES: u16 = 0;
    const QUERY_RESOURCE_ENTRIES: u16 = 0;

    #[test]
    fn test_parse_header_returns_expected_id() -> Result<(), Box<dyn error::Error>> {
        let packet = include_bytes!("../../examples/query_packet");
        let bytes = &packet[0..12];

        let mut buf = ByteBuffer::from(bytes);
        let header = parse_header(&mut buf)?;
        assert_eq!(header.id, QUERY_ID);
        Ok(())
    }

    #[test]
    fn test_parse_header_returns_expected_query_flag() {
        let packet = include_bytes!("../../examples/query_packet");
        let bytes = &packet[0..12];

        let mut buf = ByteBuffer::from(bytes);
        let header = parse_header(&mut buf).unwrap();
        assert_eq!(header.query, QUERY_QUERY);
    }

    #[test]
    fn test_parse_header_returns_expected_opcode() {
        let packet = include_bytes!("../../examples/query_packet");
        let bytes = &packet[0..12];

        let mut buf = ByteBuffer::from(bytes);
        let header = parse_header(&mut buf).unwrap();
        assert_eq!(header.opcode, QUERY_OPCODE);
    }

    #[test]
    fn test_parse_header_returns_expected_authoritative_answer_flag() {
        let packet = include_bytes!("../../examples/query_packet");
        let bytes = &packet[0..12];

        let mut buf = ByteBuffer::from(bytes);
        let header = parse_header(&mut buf).unwrap();
        assert_eq!(header.authoritative_answer, QUERY_AUTHORITATIVE_ANSWER);
    }

    #[test]
    fn test_parse_header_returns_expected_truncation_flag() {
        let packet = include_bytes!("../../examples/query_packet");
        let header = &packet[0..12];

        let mut packet = ByteBuffer::from(header);
        let header = parse_header(&mut packet).unwrap();
        assert_eq!(header.truncation, QUERY_TRUNCATION);
    }

    #[test]
    fn test_parse_header_returns_expected_recursion_desired_flag() {
        let packet = include_bytes!("../../examples/query_packet");
        let header = &packet[0..12];

        let mut packet = ByteBuffer::from(header);
        let header = parse_header(&mut packet).unwrap();
        assert_eq!(header.recursion_desired, QUERY_RECURSION_DESIRED);
    }

    #[test]
    fn test_parse_header_returns_expected_recursion_available_flag() {
        let packet = include_bytes!("../../examples/query_packet");
        let header = &packet[0..12];

        let mut packet = ByteBuffer::from(header);
        let header = parse_header(&mut packet).unwrap();
        assert_eq!(header.recursion_available, QUERY_RECURSION_AVAILABLE);
    }

    #[test]
    fn test_parse_header_returns_expected_reserved_flags() {
        let packet = include_bytes!("../../examples/query_packet");
        let header = &packet[0..12];

        let mut packet = ByteBuffer::from(header);
        let header = parse_header(&mut packet).unwrap();
        assert_eq!(header.reserved, QUERY_RESERVED);
    }

    #[test]
    fn test_parse_header_returns_expected_rcode() {
        let packet = include_bytes!("../../examples/query_packet");
        let header = &packet[0..12];

        let mut packet = ByteBuffer::from(header);
        let header = parse_header(&mut packet).unwrap();
        assert_eq!(header.rcode, QUERY_RCODE);
    }

    #[test]
    fn test_parse_header_returns_expected_questions() {
        let packet = include_bytes!("../../examples/query_packet");
        let header = &packet[0..12];

        let mut packet = ByteBuffer::from(header);
        let header = parse_header(&mut packet).unwrap();
        assert_eq!(header.questions, QUERY_QUESTIONS);
    }

    #[test]
    fn test_parse_header_returns_expected_answers() {
        let packet = include_bytes!("../../examples/query_packet");
        let header = &packet[0..12];

        let mut packet = ByteBuffer::from(header);
        let header = parse_header(&mut packet).unwrap();
        assert_eq!(header.answers, QUERY_ANSWERS);
    }

    #[test]
    fn test_parse_header_returns_expected_authoritative_entries() {
        let packet = include_bytes!("../../examples/query_packet");
        let header = &packet[0..12];

        let mut packet = ByteBuffer::from(header);
        let header = parse_header(&mut packet).unwrap();
        assert_eq!(header.authoritative_entries, QUERY_AUTHORITATIVE_ENTRIES);
    }

    #[test]
    fn test_parse_header_returns_expected_resource_entries() {
        let packet = include_bytes!("../../examples/query_packet");
        let header = &packet[0..12];

        let mut packet = ByteBuffer::from(header);
        let header = parse_header(&mut packet).unwrap();
        assert_eq!(header.resource_entries, QUERY_RESOURCE_ENTRIES);
    }
}
