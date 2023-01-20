//! This package provides methods to serialize and deserialize UDP DNS packets.

mod buffer;
mod header;
mod packet;
mod question;
mod record;

pub use packet::parse_dns_packet;
