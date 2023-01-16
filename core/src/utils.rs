use nom::{bits, bytes, IResult};

pub type Binary<'a> = (&'a [u8], usize);


pub fn take_bytes(input: &[u8], i: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(i)(input)
}

pub fn take_u8<'a>(input: Binary<'a>) -> IResult<Binary<'a>, u8> {
    bits::complete::take(8usize)(input)
}

pub fn take_u16<'a>(input: Binary<'a>) -> IResult<Binary<'a>, u16> {
    bits::complete::take(16usize)(input)
}

pub fn take_bits_u8<'a>(input: Binary<'a>, i: usize) -> IResult<Binary<'a>, u8> {
    if i > 8 {
        panic!("attempted to take more than 8 bits in take_bits_u8");
    }
    bits::complete::take(i)(input)
}

pub fn take_bits_u16<'a>(input: Binary<'a>, i: usize) -> IResult<Binary<'a>, u16> {
    if i > 16 {
        panic!("attempted to take more than 16 bits in take_bits_u16");
    }
    bits::complete::take(i)(input)
}

pub fn take_bits_u64<'a>(input: Binary<'a>, i: usize) -> IResult<Binary<'a>, u64> {
    if i > 16 {
        panic!("attempted to take more than 64 bits in take_bits_u64");
    }
    bits::complete::take(i)(input)
}

pub fn bool<'a>(input: Binary<'a>) -> IResult<Binary<'a>, bool> {
    bits::complete::bool(input)
}