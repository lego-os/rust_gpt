pub enum GptError {
    HdrSignature,
    HdrRevision,
    HdrSize,
    HdrCrc32,
    MbrUnknownNonZero,
    MbrDiskSignature,
    MbrSignature,
    MbrPRStartingChs,
    MbrPREndingChs,
    MbrPROsType,
    MbrPRStartingLba,
    PartUUID
}
