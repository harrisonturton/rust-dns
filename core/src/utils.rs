use nom::{bits, bytes, error::Error, IResult};

/// Higher-order function for a parser combinator that works on bytes.
// pub type ByteCombinatorFunc<'a, Output> = impl Fn(&[u8]) -> IResult<&[u8], Output, Error<Binary>>;

type Result<'a, Input, Output> = IResult<Input, Output, Error<Input>>;

/// Alias for `(&[u8], usize)`, which is the data structure `nom` carries
/// everywhere for parsing bitfields.
pub type Binary<'a> = (&'a [u8], usize);

pub fn take_byte(input: &[u8]) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(1usize)(input)
}

pub fn take_bytes(input: &[u8], i: usize) -> IResult<&[u8], &[u8]> {
    bytes::complete::take(i)(input)
}

pub fn take_u16_v2<'a>() -> impl Fn(&'a [u8]) -> Result<&'a [u8], u16> {
    |data| {
        let (rest, bytes) = bytes::complete::take(2usize)(data)?;
        let sized_array: [u8; 2] = bytes.try_into().unwrap();
        let value: u16 = u16::from_be_bytes(sized_array);
        Ok((rest, value))
    }
}

pub fn take_u32<'a>() -> impl Fn(&'a [u8]) -> Result<&'a [u8], u32> {
    |data| {
        let (rest, bytes) = bytes::complete::take(4usize)(data)?;
        let sized_array: [u8; 4] = bytes.try_into().unwrap();
        let value: u32 = u32::from_be_bytes(sized_array);
        Ok((rest, value))
    }
}

pub fn take_bytes_v2<'a, C: Into<usize>>(
    count: C,
) -> impl Fn(&'a [u8]) -> Result<&'a [u8], &'a [u8]> {
    bytes::complete::take(count.into())
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

pub fn take_bits_u8_v2<'a, C: Into<usize>>(
    count: C,
) -> impl Fn(Binary<'a>) -> Result<Binary<'a>, u8> {
    bits::complete::take(count.into())
}

pub fn take_bits_u16_v2<'a, C: Into<usize>>(
    count: C,
) -> impl Fn(Binary<'a>) -> Result<Binary<'a>, u16> {
    bits::complete::take(count.into())
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

pub fn bool_v2<'a>() -> impl Fn(Binary<'a>) -> Result<Binary<'a>, bool> {
    bits::complete::bool
}
