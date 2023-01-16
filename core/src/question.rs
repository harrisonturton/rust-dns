use nom::{
    bytes::{self, complete::take},
    sequence::tuple,
    IResult,
};

use crate::messages::{Class, RecordType};

#[derive(Debug)]
pub struct QuestionRecord {
    pub qname: String,
    pub qtype: RecordType,
    pub qclass: Class,
}

pub fn parse_question<'a>(message: &[u8]) -> IResult<&[u8], QuestionRecord> {
    let mut parser = tuple((qname, take(2usize), take(2usize)));
    let (rest, (qname, qtype, qclass)) = parser(message)?;

    let qtype = u16::from_be_bytes(qtype.try_into().unwrap());
    let qclass = u16::from_be_bytes(qclass.try_into().unwrap());

    let question_record = QuestionRecord {
        qname,
        qtype: qtype.into(),
        qclass: qclass.into(),
    };

    Ok((rest, question_record))
}

fn qname(message: &[u8]) -> IResult<&[u8], String> {
    let mut labels = vec![];
    let mut rest = message;
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

fn take_byte(input: &[u8]) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(1usize)(input)
}

fn take_bytes(input: &[u8], i: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(i)(input)
}
