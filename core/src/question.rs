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

pub fn serialize_questions(questions: &[Question]) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let mut bytes = vec![];
    for question in questions {
        let question_bytes = serialize_single_question(&question)?;
        bytes.extend(question_bytes);
    }
    Ok(bytes)
}

pub fn serialize_single_question(question: &Question) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let mut bytes = vec![];
    let name = crate::record::serialize_name(&question.name)?;
    bytes.extend(name);
    let typ = question.typ.to_be_bytes();
    bytes.extend(typ);
    let class = question.typ.to_be_bytes();
    bytes.extend(class);
    Ok(bytes)
}

pub fn parse_single_question(packet: &mut ByteBuffer) -> Result<Question, Box<dyn error::Error>> {
    let name = crate::record::parse_name(packet)?;
    let typ = packet.read_u16()?;
    let class = packet.read_u16()?;
    Ok(Question { name, typ, class })
}
