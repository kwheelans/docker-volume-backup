use crate::error::Error::{InvalidBackupType, InvalidCompressionType};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

pub struct Configuration {
    pub data_dir: PathBuf,
    pub backup_dir: PathBuf,
    pub backup_type: BackupStrategy,
    pub compression: BackupCompression,
    pub prefix: String,
}

pub enum BackupStrategy {
    Single,
    Multiple,
}

pub enum BackupCompression {
    Gzip,
    Xz,
}

pub enum Permission {
    G6O0,
    G6O4,
    G6O6,
    G4O0,
    G4O4,
    G4O6,
    G0O0,
    G0O4,
    G0O6,
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
