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
    let packet = core::parser::DnsPacket::deserialize(&file)?;
    println!("Got packet: {:#?}", packet);
    Ok(())
}

/// Run a simple DNS server that prints parsed DNS UDP packets.
async fn run_serve(addr: &str) -> Result<(), Box<dyn error::Error>> {
    let sock = UdpSocket::bind(addr).await?;
    println!("Listening on {}", addr);

    let mut buf = [0; 512];
    loop {
        let (_, origin) = sock.recv_from(&mut buf).await?;
        let packet = core::parser::DnsPacket::deserialize(&buf)?;
        println!("Received packet from {}:\n{:#?}", origin, packet);
    }
}
