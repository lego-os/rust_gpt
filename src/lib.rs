//! GUID Partition Table Resolver.
//!
//! Little Endian

mod entry;
mod err;
mod gpt;
mod hdr;
mod mbr;
mod parse;
mod uuid;

pub use err::GptError;
pub use gpt::GuidPartTable;
pub use hdr::Header;
pub use mbr::{MbrPartRecord, ProtectiveMbr};
use parse::*;
pub use uuid::*;

extern crate alloc;
