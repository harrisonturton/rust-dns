#[derive(Debug)]
pub enum Class {
    Internet,
    Unimplemented(u16),
}

impl Class {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Class::Internet => vec![1],
            Class::Unimplemented(value) => panic!("Unknown Class variant {}", value),
        }
    }
}

impl From<u16> for Class {
    fn from(value: u16) -> Self {
        match value {
            1 => Class::Internet,
            _ => Class::Unimplemented(value),
        }
    }
}

#[derive(Debug)]
pub enum RecordType {
    A,
    NS,
    CNAME,
    Unimplemented(u16),
}

impl RecordType {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            RecordType::A => vec![1],
            RecordType::NS => vec![2],
            RecordType::CNAME => vec![5],
            RecordType::Unimplemented(value) => panic!("Unknown NameType variant {}", value),
        }
    }
}

impl From<u16> for RecordType {
    fn from(value: u16) -> Self {
        match value {
            1 => RecordType::A,
            2 => RecordType::NS,
            3 => RecordType::CNAME,
            _ => RecordType::Unimplemented(value),
        }
    }
}
