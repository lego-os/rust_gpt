use crate::{Deserialize, GptError, LittleEndianBytes, PartUUID, Serialize};
use alloc::vec::Vec;

// pub const PRIMARY_HEADER_LBA: usize = 1;

const SIGNATURE: u64 = 0x5452415020494645;
const SIGNATURE_OFFSET: usize = 0;
const SIGNATURE_SIZE: usize = 8;

const REVISION: u32 = 0x00010000;
const REVISION_OFFSET: usize = 8;
const REVISION_SIZE: usize = 4;

const HDR_SIZE_OFFSET: usize = 12;
const HDR_SIZE_SIZE: usize = 4;

const HDR_CRC32_OFFSET: usize = 16;
const HDR_CRC32_SIZE: usize = 4;

const RESERVED_OFFSET: usize = 20;
const RESERVED_SIZE: usize = 4;

const MYLBA_OFFSET: usize = 24;
const MYLBA_SIZE: usize = 8;

const ALTERNATE_LBA_OFFSET: usize = 32;
const ALTERNATE_LBA_SIZE: usize = 8;

const FIRST_USABLE_LBA_OFFSET: usize = 40;
const FIRST_USABLE_LBA_SIZE: usize = 8;

const LAST_USABLE_LBA_OFFSET: usize = 48;
const LAST_USABLE_LBA_SIZE: usize = 8;

const DISK_GUID_OFFSET: usize = 56;
const DISK_GUID_SIZE: usize = 16;

const PARTITION_ENTRY_LBA_OFFSET: usize = 72;
const PARTITION_ENTRY_LBA_SIZE: usize = 8;

const NUMBER_OF_PARTITION_ENTRIES_OFFSET: usize = 80;
const NUMBER_OF_PARTITION_ENTRIES_SIZE: usize = 4;

const SIZE_OF_PARTITION_ENTRY_OFFSET: usize = 84;
const SIZE_OF_PARTITION_ENTRY_SIZE: usize = 4;

const PARTITION_ENTRY_ARRAY_CRC32_OFFSET: usize = 88;
const PARTITION_ENTRY_ARRAY_CRC32_SIZE: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    /// Identiﬁes EFI-compatible partition table header.
    /// This value must contain the ASCII string “EFI PART”, encoded as the 64-bit constant 0x5452415020494645.
    pub signature: [u8; SIGNATURE_SIZE],
    /// The revision number for this header. This revision value is not related to the UEFI Speciﬁcation version.
    /// This header is version 1.0, so the correct value is 0x00010000.
    pub revision: u32,
    /// Size in bytes of the GPT Header. The HeaderSize must be greater than or equal to 92 and must be less than
    /// or equal to the logical block size.
    pub header_size: u32,
    /// CRC32 checksum for the GPT Header structure. This value is computed by setting this ﬁeld to 0, and computing
    /// the 32-bit CRC for HeaderSize bytes.
    pub header_crc32: u32,
    /// Must be zero.
    reserved: u32,
    /// The LBA that contains this data structure.
    pub my_lba: u64,
    /// LBA address of the alternate GPT Header.
    pub alternate_lba: u64,
    /// The ﬁrst usable logical block that may be used by a partition described by a GUID Partition Entry.
    pub first_usable_lba: u64,
    /// The last usable logical block that may be used by a partition described by a GUID Partition Entry.
    pub last_usable_lba: u64,
    /// GUID that can be used to uniquely identify the disk.
    pub disk_guid: PartUUID,
    /// The starting LBA of the GUID Partition Entry array.
    pub part_entry_lba: u64,
    /// The number of Partition Entries in the GUID Partition Entry array.
    pub num_part_entries: u32,
    /// The size, in bytes, of each the GUID Partition Entry structures in the GUID Partition Entry array.
    /// This ﬁeld shall be set to a value of 128 x 2 n where n is an integer greater than or equal to
    /// zero (e.g., 128, 256, 512, etc.). NOTE: Previous versions of this speciﬁcation allowed any multiple of 8..
    pub part_entry_size: u32,
    /// The CRC32 of the GUID Partition Entry array. Starts at Par titionEntryLBA and is computed over a byte length
    /// of NumberOfP artitionEntries * SizeOfP artitionEntry.
    pub crc32_part_entry_array: u32,
    // reserved BlockSize - 92. The  rest  of  the block  is reserved  by  UEFI  and  must  be  zero.
}

impl Header {
    fn check_signature(signature: u64) -> Result<(), GptError> {
        if signature == SIGNATURE {
            Ok(())
        } else {
            Err(GptError::HdrSignature)
        }
    }

    fn check_revision(revision: u32) -> Result<(), GptError> {
        if revision == REVISION {
            Ok(())
        } else {
            Err(GptError::HdrRevision)
        }
    }

    fn check_header_size(header_size: u32, lba_size: u32) -> Result<(), GptError> {
        if header_size >= 92 && header_size <= lba_size {
            Ok(())
        } else {
            Err(GptError::HdrSize)
        }
    }

    fn check_crc32(bytes: &[u8], crc32: u32) -> Result<(), GptError> {
        let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        if crc32 == crc.checksum(bytes) {
            Ok(())
        } else {
            Err(GptError::HdrCrc32)
        }
    }
}

impl Serialize for Header {
    fn serialize(&self, size: usize) -> Vec<u8> {
        let mut bytes = vec![0; size];
        bytes[SIGNATURE_OFFSET..SIGNATURE_OFFSET + SIGNATURE_SIZE].copy_from_slice(&self.signature);
        bytes[REVISION_OFFSET..REVISION_OFFSET + REVISION_SIZE]
            .copy_from_slice(&self.revision.to_le_bytes());
        bytes[HDR_SIZE_OFFSET..HDR_SIZE_OFFSET + HDR_SIZE_SIZE]
            .copy_from_slice(&self.header_size.to_le_bytes());
        bytes[HDR_CRC32_OFFSET..HDR_CRC32_OFFSET + HDR_CRC32_SIZE]
            .copy_from_slice(&self.header_crc32.to_le_bytes());
        bytes[RESERVED_OFFSET..RESERVED_OFFSET + RESERVED_SIZE]
            .copy_from_slice(&self.reserved.to_le_bytes());
        bytes[MYLBA_OFFSET..MYLBA_OFFSET + MYLBA_SIZE].copy_from_slice(&self.my_lba.to_le_bytes());
        bytes[ALTERNATE_LBA_OFFSET..ALTERNATE_LBA_OFFSET + ALTERNATE_LBA_SIZE]
            .copy_from_slice(&self.alternate_lba.to_le_bytes());
        bytes[FIRST_USABLE_LBA_OFFSET..FIRST_USABLE_LBA_OFFSET + FIRST_USABLE_LBA_SIZE]
            .copy_from_slice(&self.first_usable_lba.to_le_bytes());
        bytes[LAST_USABLE_LBA_OFFSET..LAST_USABLE_LBA_OFFSET + LAST_USABLE_LBA_SIZE]
            .copy_from_slice(&self.last_usable_lba.to_le_bytes());
        bytes[DISK_GUID_OFFSET..DISK_GUID_OFFSET + DISK_GUID_SIZE].copy_from_slice(&self.disk_guid);
        bytes[PARTITION_ENTRY_LBA_OFFSET..PARTITION_ENTRY_LBA_OFFSET + PARTITION_ENTRY_LBA_SIZE]
            .copy_from_slice(&self.part_entry_lba.to_le_bytes());
        bytes[NUMBER_OF_PARTITION_ENTRIES_OFFSET
            ..NUMBER_OF_PARTITION_ENTRIES_OFFSET + NUMBER_OF_PARTITION_ENTRIES_SIZE]
            .copy_from_slice(&self.num_part_entries.to_le_bytes());
        bytes[SIZE_OF_PARTITION_ENTRY_OFFSET
            ..SIZE_OF_PARTITION_ENTRY_OFFSET + SIZE_OF_PARTITION_ENTRY_SIZE]
            .copy_from_slice(&self.part_entry_size.to_le_bytes());
        bytes[PARTITION_ENTRY_ARRAY_CRC32_OFFSET
            ..PARTITION_ENTRY_ARRAY_CRC32_OFFSET + PARTITION_ENTRY_ARRAY_CRC32_SIZE]
            .copy_from_slice(&self.crc32_part_entry_array.to_le_bytes());
        bytes
    }
}

impl Deserialize for Header {
    fn deserialize(data: &[u8]) -> Result<Self, GptError> {
        let mut ltbs = LittleEndianBytes::from(data);

        let signature = ltbs.copy_from::<SIGNATURE_SIZE>(SIGNATURE_OFFSET);
        Self::check_signature(ltbs.parse_u64().unwrap())?;

        let revision = ltbs.parse_u32().unwrap();
        Self::check_revision(revision)?;

        let header_size = ltbs.parse_u32().unwrap();
        Self::check_header_size(header_size, data.len() as _)?;

        let header_crc32 = ltbs.parse_u32().unwrap();
        let mut bytes = ltbs[0..header_size as _].to_vec();
        bytes[HDR_CRC32_OFFSET..HDR_CRC32_OFFSET + HDR_CRC32_SIZE]
            .copy_from_slice(&[0; HDR_CRC32_SIZE]);
        Self::check_crc32(&bytes, header_crc32)?;

        let reserved = ltbs.parse_u32().unwrap();
        let my_lba = ltbs.parse_u64().unwrap();
        let alternate_lba = ltbs.parse_u64().unwrap();
        let first_usable_lba = ltbs.parse_u64().unwrap();
        let last_usable_lba = ltbs.parse_u64().unwrap();

        let disk_guid = PartUUID::try_from(&ltbs.copy_from::<DISK_GUID_SIZE>(DISK_GUID_OFFSET)[..])?;

        let part_entry_lba = ltbs.parse_u64().unwrap();
        let num_part_entries = ltbs.parse_u32().unwrap();
        let part_entry_size = ltbs.parse_u32().unwrap();
        let crc32_part_entry_array = ltbs.parse_u32().unwrap();

        Ok(Self {
            signature,
            revision,
            header_size,
            header_crc32,
            reserved,
            my_lba,
            alternate_lba,
            first_usable_lba,
            last_usable_lba,
            disk_guid,
            part_entry_lba,
            num_part_entries,
            part_entry_size,
            crc32_part_entry_array,
        })
    }
}
