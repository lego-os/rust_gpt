use crate::{Deserialize, GptError, Header, ProtectiveMbr, Serialize, entry::PartTableEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalBlockSize {
    Lb512 = 512,
    Lb4096 = 4096,
}

pub struct GuidPartTable {
    lbs: LogicalBlockSize,
}

impl GuidPartTable {
    pub fn new(lbs: LogicalBlockSize) -> Self {
        Self { lbs }
    }

    pub fn parse_header(&self, data: &[u8]) -> Result<Header, GptError> {
        Header::deserialize(data)
    }

    pub fn serialize_header(&self, header: &Header) -> Vec<u8> {
        header.serialize(self.lbs as _)
    }

    pub fn parser_mbr(&self, data: &[u8]) -> Result<ProtectiveMbr, GptError> {
        ProtectiveMbr::deserialize(data)
    }

    pub fn serialize_mbr(&self, mbr: &ProtectiveMbr) -> Vec<u8> {
        mbr.serialize(self.lbs as _)
    }

    pub fn parse_part_table(
        &self,
        data: &[u8],
        part_entry_size: usize,
    ) -> Result<PartTableEntry, GptError> {
        PartTableEntry::generate_part_entries(data, part_entry_size)
    }

    pub fn serialize_parse_entries(
        &self,
        part_table: &PartTableEntry,
        part_entry_size: usize,
    ) -> Vec<u8> {
        part_table.serialize_part_entries(part_entry_size)
    }
}
