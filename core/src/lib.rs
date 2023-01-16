//! This package provides methods to serialize and deserialize UDP DNS packets.

pub mod header;
pub mod messages;
pub mod question;
pub mod resource;
pub mod utils;

use header::Header;
use nom::IResult;
use question::QuestionRecord;
use resource::ResourceRecord;
use std::error;

// A DNS packet. See §4.1 of RFC 1035.
#[derive(Debug)]
pub struct DnsPacket {
    pub header: Header,
    pub questions: Vec<QuestionRecord>,
    pub answers: Vec<ResourceRecord>,
    pub authorities: Vec<ResourceRecord>,
    pub resources: Vec<ResourceRecord>,
}

/// Parse a DNS packet.
///
/// This assumes the message arrived over UDP, not TCP. See §4.2 for an
/// explanation of how the transport affects the packet format. This is done to
/// simplify the implementation.
///
/// One day I'll get around to building something spec-compliant...
pub fn parse<'a>(message: &'a [u8]) -> Result<DnsPacket, Box<dyn error::Error>> {
    let (rest, header) = header::parse_header(message).unwrap();
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

pub fn parse_questions<I>(input: &[u8], count: I) -> IResult<&[u8], Vec<QuestionRecord>>
where
    I: Into<usize>,
{
    nom::multi::count(question::parse_question, count.into())(input)
}
