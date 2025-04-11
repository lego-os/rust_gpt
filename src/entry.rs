use crate::{Deserialize, LittleEndianBytes, PartUUID, Serialize, UUID_SIZE};
use alloc::vec::Vec;
use core::ops::Deref;

pub struct PartTableEntry {
    pub entries: Vec<PartEntry>,
}

impl PartTableEntry {
    pub(crate) fn generate_part_entries(
        data: &[u8],
        part_entry_size: usize,
    ) -> Result<Self, crate::GptError> {
        let part_entry_num = data.len() / part_entry_size;
        let mut entries = Vec::with_capacity(part_entry_num);
        for index in 0..part_entry_num {
            let start = index * part_entry_size;
            entries.push(PartEntry::deserialize(
                &data[start..start + part_entry_size],
            )?);
        }
        Ok(Self { entries })
    }

    pub(crate) fn serialize_part_entries(&self, part_entry_size: usize) -> Vec<u8> {
        let size = self.entries.len() * part_entry_size;
        let mut bytes = vec![0; size];
        self.entries.iter().enumerate().for_each(|(index, record)| {
            let start = index * part_entry_size;
            bytes[start..start + part_entry_size]
                .copy_from_slice(&record.serialize(part_entry_size));
        });
        bytes
    }
}

const PART_TYPE_GUID_OFFSET: usize = 0;
const PART_TYPE_GUID_SIZE: usize = 16;

const PART_GUID_OFFSET: usize = 16;
const PART_GUID_SIZE: usize = 16;

const STARTING_LBA_OFFSET: usize = 32;
const STARTING_LBA_SIZE: usize = 8;

const ENDING_LBA_OFFSET: usize = 40;
const ENDING_LBA_SIZE: usize = 8;

const ATTRIBUTES_OFFSET: usize = 48;
const ATTRIBUTES_SIZE: usize = 8;

const PART_NAME_OFFSET: usize = 56;
const PART_NAME_SIZE: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PartEntry {
    /// GUID of the partition type.
    pub part_type_guid: PartUUID,
    /// UUID of the partition.
    pub part_guid: PartUUID,
    /// First LBA of the partition.
    pub starting_lba: u64,
    /// Last LBA of the partition.
    pub ending_lba: u64,
    /// Partition flags.
    pub attributes: u64,
    /// Partition name.
    pub name: PartName,
    // pub reserved size = 128 - part_entry_size
}

impl Serialize for PartEntry {
    fn serialize(&self, size: usize) -> Vec<u8> {
        let mut bytes = vec![0; size];
        bytes[PART_TYPE_GUID_OFFSET..PART_TYPE_GUID_OFFSET + PART_TYPE_GUID_SIZE]
            .copy_from_slice(&self.part_type_guid);
        bytes[PART_GUID_OFFSET..PART_GUID_OFFSET + PART_GUID_SIZE].copy_from_slice(&self.part_guid);
        bytes[STARTING_LBA_OFFSET..STARTING_LBA_OFFSET + STARTING_LBA_SIZE]
            .copy_from_slice(&self.starting_lba.to_le_bytes());
        bytes[ENDING_LBA_OFFSET..ENDING_LBA_OFFSET + ENDING_LBA_SIZE]
            .copy_from_slice(&self.ending_lba.to_le_bytes());
        bytes[ATTRIBUTES_OFFSET..ATTRIBUTES_OFFSET + ATTRIBUTES_SIZE]
            .copy_from_slice(&self.attributes.to_le_bytes());
        bytes[PART_NAME_OFFSET..PART_NAME_OFFSET + PART_NAME_SIZE].copy_from_slice(&self.name);
        bytes
    }
}

impl Deserialize for PartEntry {
    fn deserialize(data: &[u8]) -> Result<Self, crate::GptError> {
        let mut ltbs = LittleEndianBytes::from(data);

        let part_type_guid =
            PartUUID::try_from(&ltbs[PART_TYPE_GUID_OFFSET..PART_TYPE_GUID_OFFSET + UUID_SIZE])?;
        ltbs.skip(UUID_SIZE);
        let part_guid = PartUUID::try_from(&ltbs[PART_GUID_OFFSET..PART_GUID_OFFSET + UUID_SIZE])?;
        ltbs.skip(UUID_SIZE);
        let start_lba = ltbs.parse_u64().unwrap();
        let end_lba = ltbs.parse_u64().unwrap();
        let attrs = ltbs.parse_u64().unwrap();
        let name = PartName(ltbs.copy_from::<PART_NAME_SIZE>(PART_NAME_OFFSET));
        Ok(Self {
            part_type_guid,
            part_guid,
            starting_lba: start_lba,
            ending_lba: end_lba,
            attributes: attrs,
            name,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PartName([u8; 16]);

impl Deref for PartName {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
