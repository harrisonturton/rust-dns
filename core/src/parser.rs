use crate::utils::{
    bool_v2, take_bits_u16_v2, take_bits_u8_v2, take_byte, take_bytes, take_bytes_v2, take_u16_v2,
    take_u32,
};
use bitvec::bitvec;
use bitvec::macros::internal::funty::Numeric;
use bitvec::view::BitViewSized;
use nom::bytes::complete::take;
use nom::sequence::tuple;
use nom::{bits::bits, IResult};
use std::error;

// A DNS packet. See §4.1 of RFC 1035.
#[derive(Debug)]
pub struct DnsPacket<'a> {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Resource<'a>>,
    pub authorities: Vec<Resource<'a>>,
    pub resources: Vec<Resource<'a>>,
}

impl<'a> DnsPacket<'a> {
    /// Deserialize a DNS packet.
    ///
    /// This assumes the message arrived over UDP, not TCP. See §4.2 for an
    /// explanation of how the transport affects the packet format. This is done to
    /// simplify the implementation.
    ///
    /// One day I'll get around to building something spec-compliant...
    pub fn deserialize(data: &[u8]) -> Result<DnsPacket, Box<dyn error::Error>> {
        let (rest, header) = parse_header(data).unwrap();
        let Header { questions, .. } = header;

        let (_, questions) = parse_questions(rest, questions).unwrap();

        // TODO parse answers
        // TODO parse authorities
        // TODO parse resources

        Ok(DnsPacket {
            header,
            questions,
            answers: vec![],
            authorities: vec![],
            resources: vec![],
        })
    }

    /// Serialize a DNS packet into bytes.
    ///
    /// This assumes the packet is being serialized for transport over UDP. See
    /// the documentation of [DnsPacket::deserialize] for more information.
    pub fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}

pub fn parse_questions<I>(input: &[u8], count: I) -> IResult<&[u8], Vec<Question>>
where
    I: Into<usize>,
{
    nom::multi::count(parse_question, count.into())(input)
}

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

pub fn serialize_header(header: &Header) -> Vec<u8> {
    use bitvec::prelude::*;
    let mut result = bitvec![u8, Msb0;];

    let mut id_bytes: BitVec<u8, Msb0> = header.id.to_be_bytes().into_bitarray().to_bitvec();
    result.append(&mut id_bytes);

    result.push(header.query);

    let opcode_bytes: u8 = header.opcode.into();
    let mut opcode_bytes: BitVec<u8, Msb0> = opcode_bytes.to_be_bytes().into_bitarray().to_bitvec();
    result.append(&mut opcode_bytes);

    result.push(header.authoritative_answer);
    result.push(header.truncation);
    result.push(header.recursion_desired);
    result.push(header.recursion_available);

    let mut reserved_bytes: BitVec<u8, Msb0> =
        header.reserved.to_be_bytes().into_bitarray().to_bitvec();
    result.append(&mut reserved_bytes);

    let mut questions_bytes: BitVec<u8, Msb0> =
        header.questions.to_be_bytes().into_bitarray().to_bitvec();
    result.append(&mut questions_bytes);

    let mut answers_bytes: BitVec<u8, Msb0> =
        header.answers.to_be_bytes().into_bitarray().to_bitvec();
    result.append(&mut answers_bytes);

    let mut authoritative_entries_bytes: BitVec<u8, Msb0> = header
        .authoritative_entries
        .to_be_bytes()
        .into_bitarray()
        .to_bitvec();
    result.append(&mut authoritative_entries_bytes);

    let mut resource_entries_bytes: BitVec<u8, Msb0> = header
        .resource_entries
        .to_be_bytes()
        .into_bitarray()
        .to_bitvec();
    result.append(&mut resource_entries_bytes);

    result.into()
}

pub fn parse_header(data: &[u8]) -> IResult<&[u8], Header> {
    let mut parser = bits::<_, _, _, nom::error::Error<_>, _>(tuple((
        take_bits_u16_v2(16usize),
        bool_v2(),
        take_bits_u8_v2(4usize),
        bool_v2(),
        bool_v2(),
        bool_v2(),
        bool_v2(),
        take_bits_u8_v2(3usize),
        take_bits_u8_v2(4usize),
        take_bits_u16_v2(16usize),
        take_bits_u16_v2(16usize),
        take_bits_u16_v2(16usize),
        take_bits_u16_v2(16usize),
    )));
    let (rest, parts) = parser(data)?;
    let header = Header {
        id: parts.0,
        query: parts.1,
        opcode: parts.2.into(),
        authoritative_answer: parts.3,
        truncation: parts.4,
        recursion_desired: parts.5,
        recursion_available: parts.6,
        reserved: parts.7,
        rcode: parts.8.into(),
        questions: parts.9,
        answers: parts.10,
        authoritative_entries: parts.11,
        resource_entries: parts.12,
    };
    Ok((rest, header))
}

#[derive(Debug, Copy, Clone)]
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

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::Query => 0,
            Opcode::InverseQuery => 1,
            Opcode::Status => 2,
            Opcode::Reserved(value) => value,
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

// --------------------------------------------------
// Types
// --------------------------------------------------

#[derive(Debug)]
pub enum Type {
    // Types
    A,
    NS,
    MD,
    MF,
    CNAME,
    SOA,
    MB,
    MG,
    MR,
    NULL,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
    // Qtypes
    AXFR,
    MAILB,
    MAILA,
    WILDCARD,
    // Other
    Unknown(u16),
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            3 => Self::MD,
            4 => Self::MF,
            5 => Self::CNAME,
            6 => Self::SOA,
            7 => Self::MB,
            8 => Self::MG,
            9 => Self::MR,
            10 => Self::NULL,
            11 => Self::WKS,
            12 => Self::PTR,
            13 => Self::HINFO,
            14 => Self::MX,
            15 => Self::TXT,
            252 => Self::AXFR,
            253 => Self::MAILB,
            254 => Self::MAILA,
            255 => Self::WILDCARD,
            _ => Self::Unknown(value),
        }
    }
}

#[derive(Debug)]
pub enum Class {
    IN,
    Unknown(u16),
}

impl From<u16> for Class {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::IN,
            _ => Self::Unknown(value),
        }
    }
}

// --------------------------------------------------
// Questions
// --------------------------------------------------

#[derive(Debug)]
pub struct Question {
    pub qname: String,
    pub qtype: Type,
    pub qclass: Class,
}

pub fn serialize_question(question: &Question) -> Vec<u8> {
    vec![]
}

pub fn parse_question(data: &[u8]) -> IResult<&[u8], Question> {
    let mut parser = tuple((question_name, take(2usize), take(2usize)));
    let (rest, (qname, qtype, qclass)) = parser(data)?;

    let qtype: [u8; 2] = qtype.try_into().unwrap();
    let qtype = u16::from_be_bytes(qtype);

    let qclass: [u8; 2] = qclass.try_into().unwrap();
    let qclass = u16::from_be_bytes(qclass);

    let question = Question {
        qname,
        qtype: qtype.into(),
        qclass: qclass.into(),
    };
    Ok((rest, question))
}

fn question_name(data: &[u8]) -> IResult<&[u8], String> {
    let mut labels = vec![];
    let mut rest = data;
    loop {
        let (label_rest, length) = take_byte(rest)?;
        let length = u8::from_be_bytes(length.try_into().unwrap());
        if length == 0 {
            rest = label_rest;
            break;
        }

        let (label_rest, label) = take_bytes(label_rest, length as usize)?;
        labels.push(String::from_utf8_lossy(label));
        rest = label_rest;
    }
    let name = labels.join(".");
    Ok((rest, name))
}

// --------------------------------------------------
// Resource record
// --------------------------------------------------

#[derive(Debug)]
pub struct Resource<'a> {
    pub name: String,
    pub typ: Type,
    pub class: Class,
    pub ttl: u32,
    pub rdata_len: u16,
    pub rdata: &'a [u8],
}

pub fn resource<'a>(data: &'a [u8]) -> IResult<&[u8], Resource<'a>> {
    let (rest, name) = resource_name(data)?;
    let (rest, typ) = take_u16_v2()(rest)?;
    let (rest, class) = take_u16_v2()(rest)?;
    let (rest, ttl) = take_u32()(rest)?;
    let (rest, rdata_len) = take_u16_v2()(rest)?;
    let (rest, rdata) = take_bytes_v2(rdata_len as usize)(rest)?;
    let resource = Resource {
        name,
        typ: typ.into(),
        class: class.into(),
        ttl: ttl.into(),
        rdata_len,
        rdata,
    };
    Ok((rest, resource))
}

pub fn resource_name(data: &[u8]) -> IResult<&[u8], String> {
    let (rest, pointer_flag) = take_bytes_v2(2usize)(data)?;
    match pointer_flag {
        [0x00, 0x00] => label(data),
        [0x01, 0x01] => compressed_label(data),
        _ => panic!("unknown pointer flag: {:x?}", pointer_flag),
    }
}

pub fn label(data: &[u8]) -> IResult<&[u8], String> {
    todo!()
}

pub fn compressed_label(data: &[u8]) -> IResult<&[u8], String> {
    todo!()
}
