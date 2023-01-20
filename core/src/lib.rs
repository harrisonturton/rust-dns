//! This package provides methods to serialize and deserialize UDP DNS packets.

#![feature(type_alias_impl_trait)]

pub mod header;
pub mod messages;
pub mod parser;
pub mod question;
pub mod resource;
pub mod utils;
