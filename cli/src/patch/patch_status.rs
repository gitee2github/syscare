use serde::{Deserialize, Serialize};

pub const PATCH_STATUS_UNKNOWN: &str = "UNKNOWN";
pub const PATCH_STATUS_NOT_APPLIED: &str = "NOT-APPLIED";
pub const PATCH_STATUS_DEACTIVED: &str = "DEACTIVED";
pub const PATCH_STATUS_ACTIVED: &str = "ACTIVED";
pub const PATCH_STATUS_ACCEPTED: &str = "ACCEPTED";

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PatchStatus {
    Unknown,
    NotApplied,
    Deactived,
    Actived,
    Accepted,
}

impl Default for PatchStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl std::fmt::Display for PatchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PatchStatus::Unknown => PATCH_STATUS_UNKNOWN,
            PatchStatus::NotApplied => PATCH_STATUS_NOT_APPLIED,
            PatchStatus::Deactived => PATCH_STATUS_DEACTIVED,
            PatchStatus::Actived => PATCH_STATUS_ACTIVED,
            PatchStatus::Accepted => PATCH_STATUS_ACCEPTED,
        })
    }
}
