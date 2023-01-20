use clap::{Parser, Subcommand};
use std::{error, fs};
use tokio::net::UdpSocket;

#[derive(Parser)]
#[command(bin_name = "rust-dns", author = "Harrison Turton", version)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// Parse a DNS packet from a file
    #[command(name = "read")]
    Parse {
        /// Path to the file
        filepath: String,
    },
    /// Launch a stub DNS server
    #[command(name = "serve")]
    Serve {
        /// Address to listen on, e.g. 127.0.0.1:3000
        addr: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    println!("Running");
    match &args.command {
        Command::Parse { filepath } => run_parse(filepath).await?,
        Command::Serve { addr } => run_serve(addr).await?,
    };
    Ok(())
}

/// Parse a packet stored in a file. This assumes the packet is a UDP packet,
/// not a TCP. This means it cannot handle DNS packets that are too long.
async fn run_parse(filepath: &str) -> Result<(), Box<dyn error::Error>> {
    let file = fs::read(filepath)?;
    let packet = core::parse_dns_packet(&file)?;
    println!("Got packet: {:#?}", packet);
    Ok(())
}

/// Run a simple DNS server that prints parsed DNS UDP packets.
async fn run_serve(addr: &str) -> Result<(), Box<dyn error::Error>> {
    let sock = UdpSocket::bind(addr).await?;
    println!("Listening on {}", addr);

    let mut buf = [0; 512];
    loop {
        let (_, addr) = sock.recv_from(&mut buf).await?;
        let packet = core::parse_dns_packet(&buf)?;
        println!("Received packet from {}:\n{:#?}", addr, packet);
        let response_packet = core::packet::DnsPacket {
            header: core::header::Header {
                id: packet.header.id,
                query: false,
                opcode: 0,
                authoritative_answer: true,
                truncation: false,
                recursion_desired: false,
                recursion_available: false,
                reserved: 0,
                rcode: 0,
                questions: 0,
                answers: 1,
                authoritative_entries: 0,
                resource_entries: 0,
            },
            questions: vec![],
            answers: vec![],
            authoritative_entries: vec![],
            resource_entries: vec![],
        };
        let response_packet = core::serialize_dns_packet(&response_packet);
        let len = sock.send_to(&response_packet, addr).await?;
        println!("{:?} bytes sent", len);
    }
}
