use crate::utils::{bool, take_bits_u16, take_bits_u8, take_bytes, Binary};
use nom::IResult;

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub query: bool,
    pub opcode: Opcode,
    pub authoritative_answer: bool,
    pub truncation: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub reserved: u8,
    pub rcode: Rcode,
    pub questions: u16,
    pub answers: u16,
    pub authoritative_entries: u16,
    pub resource_entries: u16,
}

#[derive(Debug)]
pub enum Opcode {
    Query,
    InverseQuery,
    Status,
    Reserved(u8),
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Opcode::Query,
            1 => Opcode::InverseQuery,
            2 => Opcode::Status,
            _ => Opcode::Reserved(byte),
        }
    }
}

#[derive(Debug)]
pub enum Rcode {
    Success,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
    Reserved(u8),
}

impl From<u8> for Rcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Rcode::Success,
            1 => Rcode::FormatError,
            2 => Rcode::ServerFailure,
            3 => Rcode::NameError,
            4 => Rcode::NotImplemented,
            5 => Rcode::Refused,
            _ => Rcode::Reserved(byte),
        }
    }
}

pub fn parse_header<'a>(message: &'a [u8]) -> IResult<&[u8], Header> {
    let (rest_message, header) = take_bytes(message, 12)?;

    let header: Binary<'a> = (header, 0);
    let (rest, id) = take_bits_u16(header, 16).unwrap();
    let (rest, qr) = bool(rest).unwrap();
    let (rest, opcode) = take_bits_u8(rest, 4).unwrap();
    let (rest, aa) = bool(rest).unwrap();
    let (rest, tc) = bool(rest).unwrap();
    let (rest, rd) = bool(rest).unwrap();
    let (rest, ra) = bool(rest).unwrap();
    let (rest, z) = take_bits_u8(rest, 3).unwrap();
    let (rest, rcode) = take_bits_u8(rest, 4).unwrap();
    let (rest, qd_count) = take_bits_u16(rest, 16).unwrap();
    let (rest, an_count) = take_bits_u16(rest, 16).unwrap();
    let (rest, ns_count) = take_bits_u16(rest, 16).unwrap();
    let (_, ar_count) = take_bits_u16(rest, 16).unwrap();

    let header = Header {
        id: id,
        query: qr,
        opcode: opcode.into(),
        authoritative_answer: aa,
        truncation: tc,
        recursion_desired: rd,
        recursion_available: ra,
        reserved: z,
        rcode: rcode.into(),
        questions: qd_count,
        answers: an_count,
        authoritative_entries: ns_count,
        resource_entries: ar_count,
    };

    Ok((rest_message, header))
}
