//! The wire codec and identity core: canonical bencode, the KRPC (BEP-5)
//! message codec, and byte-exact BitTorrent infohashes (SHA-1 / SHA-256).
//!
//! Everything here is pure and dependency-free: this is where hostile bytes
//! from anonymous DHT peers are parsed. Each module is re-exported flat at
//! the crate root (`torrentds::bencode`, `torrentds::krpc`,
//! `torrentds::infohash`), so the grouping is only on disk.

pub mod bencode;
pub mod infohash;
pub mod krpc;
