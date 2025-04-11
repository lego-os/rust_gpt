use crate::{Deserialize, GptError, LittleEndianBytes, Serialize};
use core::mem;
// pub const PROTECTIVE_MBR_LBA: usize = 0;

const BOOT_CODE_OFFSET: usize = 0;
const BOOT_CODE_SIZE: usize = 440;

const DISK_SIGNATURE_OFFSET: usize = 440;
const DISK_SIGNATURE_SIZE: usize = 4;
const DISK_SIGNATURE: [u8; DISK_SIGNATURE_SIZE] = [0; DISK_SIGNATURE_SIZE];

const UNKNOWN_OFFSET: usize = 444;
const UNKNOWN_SIZE: usize = 2;
const UNKNOWN: u16 = 0;

const PART_RECORD_OFFSET: usize = 446;
const PART_RECORD_NUM: usize = 4;
const PART_RECORD_SIZE: usize = 16;

const SIGNATURE_OFFSET: usize = 510;
const SIGNATURE_SIZE: usize = 2;
const SIGNATURE: u16 = 0xAA55;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectiveMbr {
    /// Unused by UEFI systems.
    pub boot_code: [u8; BOOT_CODE_SIZE],
    /// Unused. Set to zero.
    disk_signature: [u8; DISK_SIGNATURE_SIZE],
    /// Unused. Set to zero.
    unknown: u16,
    /// Array of four MBR partition records. Contains:
    /// - one partition record as deﬁned See Table (below).
    /// - three partition  records each set to zero.
    pub part_records: [MbrPartRecord; PART_RECORD_NUM],
    /// Set to 0xAA55
    signature: u16,
    // reserved  size = Logical Block Size - 512
}

impl ProtectiveMbr {
    fn check_disk_signature(disk_signature: &[u8]) -> Result<(), GptError> {
        if disk_signature.eq(&DISK_SIGNATURE) {
            Ok(())
        } else {
            Err(GptError::MbrDiskSignature)
        }
    }

    fn check_unknown(unknown: u16) -> Result<(), GptError> {
        if unknown == UNKNOWN {
            Ok(())
        } else {
            Err(GptError::MbrUnknownNonZero)
        }
    }

    fn check_signature(signature: u16) -> Result<(), GptError> {
        if signature == SIGNATURE {
            Ok(())
        } else {
            Err(GptError::MbrSignature)
        }
    }

    pub fn is_large_disk(&self) -> bool {
        let first_record = &self.part_records[0];
        first_record.ending_chs == MAX_ENDING_CHD && first_record.size_in_lba == MAX_SIZE_IN_LBA
    }
}

impl Serialize for ProtectiveMbr {
    fn serialize(&self, size: usize) -> Vec<u8> {
        let mut bytes = vec![0; size];
        bytes[..BOOT_CODE_SIZE].copy_from_slice(&self.boot_code);
        bytes[DISK_SIGNATURE_OFFSET..DISK_SIGNATURE_OFFSET + DISK_SIGNATURE_SIZE]
            .copy_from_slice(&self.disk_signature);
        bytes[UNKNOWN_OFFSET..UNKNOWN_OFFSET + UNKNOWN_SIZE]
            .copy_from_slice(&self.unknown.to_le_bytes());

        self.part_records
            .iter()
            .enumerate()
            .for_each(|(index, record)| {
                let start = PART_RECORD_OFFSET + index * PART_RECORD_SIZE;
                bytes[start..start + PART_RECORD_SIZE]
                    .copy_from_slice(&record.serialize(PART_RECORD_SIZE));
            });
        bytes[SIGNATURE_OFFSET..SIGNATURE_OFFSET + SIGNATURE_SIZE]
            .copy_from_slice(&self.signature.to_le_bytes());

        bytes
    }
}

impl Deserialize for ProtectiveMbr {
    fn deserialize(data: &[u8]) -> Result<Self, crate::GptError> {
        let mut ltbs = LittleEndianBytes::from(data);

        let boot_code = ltbs.copy_from::<BOOT_CODE_SIZE>(BOOT_CODE_OFFSET);

        let disk_signature = ltbs.copy_from::<DISK_SIGNATURE_SIZE>(DISK_SIGNATURE_OFFSET);
        Self::check_disk_signature(&disk_signature)?;

        let unknown = ltbs.parse_u16().unwrap();
        Self::check_unknown(unknown)?;

        let mut part_records = [MbrPartRecord::default(); PART_RECORD_NUM];
        for (index, record) in part_records.iter_mut().enumerate() {
            let rd = MbrPartRecord::deserialize(
                &ltbs.copy_from::<PART_RECORD_SIZE>(PART_RECORD_OFFSET + PART_RECORD_SIZE * index),
            )?;
            let _ = mem::replace(record, rd);
        }

        let signature = ltbs.parse_u16().unwrap();
        Self::check_signature(signature)?;

        Ok(Self {
            boot_code,
            disk_signature,
            unknown,
            part_records,
            signature,
        })
    }
}

const BOOT_INDICATOR_OFFSET: usize = 0;

const STARTING_CHS_OFFSET: usize = 1;
const STARTING_CHS_SIZE: usize = 3;
const STARTING_CHS: [u8; STARTING_CHS_SIZE] = [00, 0x02, 00];

const OSTYPE_OFFSET: usize = 4;
const OSTYPE: u8 = 0xEE;

const ENDING_CHS_OFFSET: usize = 5;
const ENDING_CHS_SIZE: usize = 3;
const ENDING_CHS: [u8; ENDING_CHS_SIZE] = STARTING_CHS;
const MAX_ENDING_CHD: [u8; ENDING_CHS_SIZE] = [0xFF, 0xFF, 0xFF];

const STARTING_LBA_OFFSET: usize = 8;
const STARTING_LBA_SIZE: usize = 4;
const STARTING_LBA: u32 = 0x00000001;

const SIZE_IN_LBA_OFFSET: usize = 12;
const SIZE_IN_LBA_SIZE: usize = 4;
const MAX_SIZE_IN_LBA: u32 = 0xFFFFFFFF;

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct MbrPartRecord {
    /// Set to 0x00 to indicate a non-bootable partition. If set to any
    /// value other than 0x00 the behavior of this ﬂag on non-UEFI systems
    /// is undeﬁned. Must be ignored by UEFI i mplementations.
    boot_indicator: u8,
    /// Set to 0x000200, corresponding to the Starting LBA ﬁeld.
    starting_chs: [u8; STARTING_CHS_SIZE],
    /// Set to 0xEE.
    ostype: u8,
    /// Set to the CHS address of the last logical block on the disk. Set
    /// to 0xFFFFFF if it is not possible to represent the value in this ﬁeld.
    ending_chs: [u8; ENDING_CHS_SIZE],
    /// Set to 0x00000001. (i.e., the LBA of the GPT Partition Header).
    starting_lba: u32,
    /// Set to the size of the disk minus one. Set to 0xFFFFFFFF if the size
    /// of the disk is too large to be represented in this ﬁeld.
    size_in_lba: u32,
}

impl MbrPartRecord {
    fn check_starting_chs(starting_chs: &[u8]) -> Result<(), GptError> {
        if starting_chs.eq(&STARTING_CHS) {
            Ok(())
        } else {
            Err(GptError::MbrPRStartingChs)
        }
    }

    fn check_ostype(ostype: u8) -> Result<(), GptError> {
        if ostype == OSTYPE {
            Ok(())
        } else {
            Err(GptError::MbrPROsType)
        }
    }

    fn check_ending_chs(ending_chs: &[u8]) -> Result<(), GptError> {
        if ending_chs.eq(&ENDING_CHS) {
            Ok(())
        } else {
            Err(GptError::MbrPREndingChs)
        }
    }

    fn check_starting_lba(starting_lba: u32) -> Result<(), GptError> {
        if starting_lba == STARTING_LBA {
            Ok(())
        } else {
            Err(GptError::MbrPRStartingLba)
        }
    }
}

impl Serialize for MbrPartRecord {
    fn serialize(&self, size: usize) -> Vec<u8> {
        let mut bytes = vec![0; size];
        bytes[BOOT_INDICATOR_OFFSET] = self.boot_indicator;
        bytes[STARTING_CHS_OFFSET..STARTING_CHS_OFFSET + STARTING_CHS_SIZE]
            .copy_from_slice(&self.starting_chs);
        bytes[OSTYPE_OFFSET] = self.ostype;
        bytes[ENDING_CHS_OFFSET..ENDING_CHS_OFFSET + ENDING_CHS_SIZE]
            .copy_from_slice(&self.ending_chs);
        bytes[STARTING_LBA_OFFSET..STARTING_LBA_OFFSET + STARTING_LBA_SIZE]
            .copy_from_slice(&self.starting_lba.to_le_bytes());
        bytes[SIZE_IN_LBA_OFFSET..SIZE_IN_LBA_OFFSET + SIZE_IN_LBA_SIZE]
            .copy_from_slice(&self.size_in_lba.to_le_bytes());
        bytes
    }
}

impl Deserialize for MbrPartRecord {
    fn deserialize(data: &[u8]) -> Result<Self, crate::GptError> {
        let mut ltbs = LittleEndianBytes::from(data);

        let boot_indicator = ltbs.parse_u8().unwrap();
        let starting_chs = ltbs.copy_from::<STARTING_CHS_SIZE>(STARTING_CHS_OFFSET);
        Self::check_starting_chs(&starting_chs)?;

        let ostype = ltbs.parse_u8().unwrap();
        Self::check_ostype(ostype)?;

        let ending_chs = ltbs.copy_from::<ENDING_CHS_SIZE>(ENDING_CHS_OFFSET);
        Self::check_ending_chs(&ending_chs)?;

        let starting_lba = ltbs.parse_u32().unwrap();
        Self::check_starting_lba(starting_lba)?;

        let size_in_lba = ltbs.parse_u32().unwrap();
        Ok(Self {
            boot_indicator,
            starting_chs,
            ostype,
            ending_chs,
            starting_lba,
            size_in_lba,
        })
    }
}
