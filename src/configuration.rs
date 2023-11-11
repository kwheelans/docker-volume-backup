use crate::error::Error;
use crate::error::Error::{InvalidBackupType, InvalidCompressionType, InvalidPermission};
use std::fmt::{Display, Formatter};
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::str::FromStr;

pub struct Configuration {
    pub data_dir: PathBuf,
    pub backup_dir: PathBuf,
    pub backup_type: BackupStrategy,
    pub compression: BackupCompression,
    pub prefix: String,
    pub permission: Permissions,
}

pub enum BackupStrategy {
    Single,
    Multiple,
}

pub enum BackupCompression {
    Gzip,
    Xz,
}

pub enum BackupPermission {
    Read,
    Write,
    None,
}

impl Display for BackupStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupStrategy::Single => write!(f, "Single"),
            BackupStrategy::Multiple => write!(f, "Multiple"),
        }
    }
}

impl FromStr for BackupStrategy {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "s" | "single" => Ok(Self::Single),
            "m" | "multiple" => Ok(Self::Multiple),
            _ => Err(InvalidBackupType),
        }
    }
}

impl Display for BackupCompression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupCompression::Gzip => write!(f, "GZip"),
            BackupCompression::Xz => write!(f, "XZ"),
        }
    }
}

impl FromStr for BackupCompression {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "gz" | "gzip" => Ok(Self::Gzip),
            "xz" => Ok(Self::Xz),
            _ => Err(InvalidCompressionType),
        }
    }
}

impl BackupCompression {
    pub fn extension(&self) -> String {
        match self {
            BackupCompression::Gzip => "gz".to_string(),
            BackupCompression::Xz => "xz".to_string(),
        }
    }
}

impl FromStr for BackupPermission {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "read" | "r" => Ok(Self::Read),
            "write" | "w" => Ok(Self::Write),
            "none" | "n" | "" => Ok(Self::None),
            _ => Err(InvalidPermission),
        }
    }
}

pub fn get_permission(group: BackupPermission, other: BackupPermission) -> Permissions {
    match (group, other) {
        (BackupPermission::Write, BackupPermission::None) => Permissions::from_mode(0o660),
        (BackupPermission::Write, BackupPermission::Read) => Permissions::from_mode(0o664),
        (BackupPermission::Write, BackupPermission::Write) => Permissions::from_mode(0o666),
        (BackupPermission::Read, BackupPermission::None) => Permissions::from_mode(0o640),
        (BackupPermission::Read, BackupPermission::Read) => Permissions::from_mode(0o644),
        (BackupPermission::Read, BackupPermission::Write) => Permissions::from_mode(0o646),
        (BackupPermission::None, BackupPermission::None) => Permissions::from_mode(0o600),
        (BackupPermission::None, BackupPermission::Read) => Permissions::from_mode(0o604),
        (BackupPermission::None, BackupPermission::Write) => Permissions::from_mode(0o606),
    }
}
