use crate::GptError;
use core::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

pub const UUID_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PartUUID([u8; UUID_SIZE]);

impl TryFrom<&[u8]> for PartUUID {
    type Error = GptError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != UUID_SIZE
            || value
                .iter()
                .filter(|byte| (0x41..0x5A).contains(*byte) || (0x61..0x7A).contains(*byte))
                .count()
                < value.len()
        {
            return Err(GptError::PartUUID);
        }
        let mut uuid = [0; UUID_SIZE];
        uuid.copy_from_slice(value);
        Ok(Self(uuid))
    }
}

impl Deref for PartUUID {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PartUUID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for PartUUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (left, right) = self.0.split_at(UUID_SIZE / 2);
        write!(
            f,
            "{:x}{:x}{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}{:x}{:x}{:x}{:x}",
            left[3],
            left[2],
            left[1],
            left[0],
            left[5],
            left[4],
            left[7],
            left[6],
            right[0],
            right[1],
            right[2],
            right[3],
            right[4],
            right[5],
            right[6],
            right[7],
        )
    }
}

impl FromStr for PartUUID {
    type Err = GptError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().filter(|ch| ch.ne(&'-')).collect::<Vec<char>>();
        if chars.len() != UUID_SIZE {
            return Err(GptError::PartUUID);
        }
        let mut uuid = [0; UUID_SIZE];
        let (left, right) = chars.split_at(UUID_SIZE / 2);
        uuid[0] = validate_guid_byte(left[3])?;
        uuid[1] = validate_guid_byte(left[2])?;
        uuid[2] = validate_guid_byte(left[1])?;
        uuid[3] = validate_guid_byte(left[0])?;
        uuid[4] = validate_guid_byte(left[5])?;
        uuid[5] = validate_guid_byte(left[4])?;
        uuid[6] = validate_guid_byte(left[7])?;
        uuid[7] = validate_guid_byte(left[6])?;
        for (index, ch) in right.iter().enumerate() {
            uuid[index + UUID_SIZE / 2] = validate_guid_byte(*ch)?;
        }
        Ok(Self(uuid))
    }
}

fn validate_guid_byte(ch: char) -> Result<u8, GptError> {
    let byte = ch as u8;
    if (0x41..0x5A).contains(&byte) || (0x61..0x7A).contains(&byte) {
        Ok(byte)
    } else {
        Err(GptError::PartUUID)
    }
}
