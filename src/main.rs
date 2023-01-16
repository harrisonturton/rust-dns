use nom::{bits, bytes, IResult};
use question::QuestionRecord;
use resource::ResourceRecord;
use std::{env, fs, io, net::SocketAddr};
use tokio::net::UdpSocket;

mod header;
mod messages;
mod question;
mod resource;

pub type Binary<'a> = (&'a [u8], usize);

use header::Header;

pub struct DnsPacket {
    header: Header,
    questions: Vec<QuestionRecord>,
    answers: Vec<ResourceRecord>,
}

#[tokio::main]
async fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 1 {
        println!("Usage: ./rust-dns <filepath>");
        return;
    }

    if args[0] == "server" {
        run_server().await.unwrap();
        return;
    }

    let filepath = &args[0];
    let file = fs::read(filepath).unwrap();
    parse(&file);
}

async fn run_server() -> io::Result<()> {
    println!("Running server");

    let addr = "127.0.0.1:9000";
    let sock = UdpSocket::bind(addr).await?;
    println!("Listening on {}", addr);

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        println!("{}", String::from_utf8_lossy(&buf).to_owned());

        let packet = parse(&buf);
        println!("{:#?}", packet);

        let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);
    }

    // let mut buf = vec![];

    // loop {
    //     socket
    //         .recv_from(&mut buf)
    //         .await
    //         .expect("Could not get datagram");

    //     println!("Got: {}", String::from_utf8_lossy(&buf).to_owned());
    //     buf.clear();

    //     // let packet = parse(&buf);
    //     // println!("Got packed: {:#?}", packet);
    // }

    // loop {
    //     let mut buf = vec![];
    //     let sock = socket.try_clone().expect("Failed to clone socket");
    //     match sock.recv_from(&mut buf) {
    //         Ok((_, src)) => {
    //             println!("Handling connection from {}", src);
    //             let packet = parse(&buf);
    //             println!("got packet:\n{:#?}", packet);
    //         }
    //         Err(err) => {
    //             eprintln!("could not receive datagram: {:?}", err);
    //         }
    //     }
    // }
}

fn parse<'a>(message: &'a [u8]) {
    let (rest, header) = header::parse_header(message).unwrap();
    println!("{:#?}", header);

    let (_rest, questions) = parse_question_records(rest, header.questions as usize).unwrap();
    for question in questions.iter() {
        println!("{:#?}", question);
    }

    // Answer
    // Authority
    // Additional
}

pub fn parse_question_records(input: &[u8], count: usize) -> IResult<&[u8], Vec<QuestionRecord>> {
    nom::multi::count(question::parse_question, count)(input)
}

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
