//! This package provides methods to serialize and deserialize UDP DNS packets.

mod buffer;

pub mod header;
pub mod packet;
pub mod question;
pub mod record;

pub use packet::{parse_dns_packet, serialize_dns_packet};
