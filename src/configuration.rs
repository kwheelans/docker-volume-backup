use crate::error::Error;
use crate::error::Error::{
    InvalidBackupType, InvalidCompressionType, InvalidPermission, NoVolumeMounted,
};
use crate::{
    ARCHIVE_DIR, BACKUP_DIR_ENV, COMPRESSION_ENV, COMPRESSION_LEVEL_ENV, DATA_DIR, DATA_DIR_ENV,
    GROUP_PERMISSION_ENV, LOG_TARGET, OTHER_PERMISSION_ENV, PREFIX_ENV,
    SALVAGE_CONTAINER_MANAGEMENT_ENV, SALVAGE_IS_DOCKER, SALVAGE_RUN_ONCE_ENV, STRATEGY_ENV,
};
use log::{debug, warn};
use std::env;
use std::fmt::{Display, Formatter};
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::str::FromStr;

pub trait DefaultEnv: Default + Display + FromStr<Err = Error> {
    fn env_or_default<S: AsRef<str>>(key: S) -> Result<Self, Error> {
        match env::var(key.as_ref()) {
            Ok(s) => Self::from_str(s.as_str()),
            Err(e) => {
                let val = Self::default();
                debug!(target: LOG_TARGET, "Using default value({}) for environment key {} because {}", val.to_string(),key.as_ref(), e);
                Ok(val)
            }
        }
    }
}

pub struct Configuration {
    pub data_dir: PathBuf,
    pub backup_dir: PathBuf,
    pub archive_strategy: ArchiveStrategy,
    pub archive_compression: ArchiveCompression,
    pub archive_compression_level: u32,
    pub archive_prefix: String,
    pub group_permission: ArchivePermission,
    pub other_permission: ArchivePermission,
    pub stop_containers: bool,
    pub is_docker: bool,
    pub run_once: bool,
}

#[derive(Default)]
pub enum ArchiveStrategy {
    #[default]
    Multiple,
    Single,
}

#[derive(Default)]
pub enum ArchiveCompression {
    Bzip2,
    #[default]
    Gzip,
    Xz,
    Zstd,
}

#[derive(Default)]
pub enum ArchivePermission {
    #[default]
    Read,
    Write,
    None,
}

impl Configuration {
    pub fn container_management_enabled(&self) -> bool {
        self.is_docker && self.stop_containers
    }
}

impl DefaultEnv for ArchiveStrategy {}

impl Display for ArchiveStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveStrategy::Single => write!(f, "Single"),
            ArchiveStrategy::Multiple => write!(f, "Multiple"),
        }
    }
}

impl FromStr for ArchiveStrategy {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "s" | "single" => Ok(Self::Single),
            "m" | "multiple" => Ok(Self::Multiple),
            _ => Err(InvalidBackupType),
        }
    }
}

impl DefaultEnv for ArchiveCompression {}

impl Display for ArchiveCompression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveCompression::Bzip2 => write!(f, "BZip2"),
            ArchiveCompression::Gzip => write!(f, "GZip"),
            ArchiveCompression::Xz => write!(f, "XZ"),
            ArchiveCompression::Zstd => write!(f, "ZStd"),
        }
    }
}

impl FromStr for ArchiveCompression {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "bzip2" | "bzip" | "bz2" => Ok(Self::Bzip2),
            "gz" | "gzip" => Ok(Self::Gzip),
            "xz" => Ok(Self::Xz),
            "zstd" | "zst" => Ok(Self::Zstd),
            _ => Err(InvalidCompressionType),
        }
    }
}

impl ArchiveCompression {
    pub fn extension(&self) -> String {
        match self {
            ArchiveCompression::Bzip2 => "bz2",
            ArchiveCompression::Gzip => "gz",
            ArchiveCompression::Xz => "xz",
            ArchiveCompression::Zstd => "zst",
        }
        .to_string()
    }

    pub fn parse_level<S: AsRef<str>>(&self, value: S) -> u32 {
        match value.as_ref().trim().parse::<u32>() {
            Ok(level) => {
                let max_level = self.max_level();
                if level > max_level {
                    warn!(target: LOG_TARGET, "Provided compression level of {} is greater than max level for {}. Using max level of {}.", level, self, max_level);
                    max_level
                } else {
                    level
                }
            }
            Err(error) => {
                warn!(target: LOG_TARGET, "Using default because compression level conversion to u32 failed: {}", error);
                self.default_level()
            }
        }
    }

    fn default_level(&self) -> u32 {
        6
    }

    fn max_level(&self) -> u32 {
        match self {
            ArchiveCompression::Zstd => 22,
            _ => 9,
        }
    }
}

impl DefaultEnv for ArchivePermission {}

impl Display for ArchivePermission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchivePermission::Read => write!(f, "Read"),
            ArchivePermission::Write => write!(f, "Read-Write"),
            ArchivePermission::None => write!(f, "None"),
        }
    }
}

impl FromStr for ArchivePermission {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "read" | "r" => Ok(Self::Read),
            "write" | "w" | "read-write" | "rw" => Ok(Self::Write),
            "none" | "n" | "" => Ok(Self::None),
            _ => Err(InvalidPermission),
        }
    }
}

impl Configuration {
    pub fn archive_permission(&self) -> Permissions {
        get_permission(&self.group_permission, &self.other_permission)
    }
}

pub fn get_permission(group: &ArchivePermission, other: &ArchivePermission) -> Permissions {
    match (group, other) {
        (ArchivePermission::Write, ArchivePermission::None) => Permissions::from_mode(0o660),
        (ArchivePermission::Write, ArchivePermission::Read) => Permissions::from_mode(0o664),
        (ArchivePermission::Write, ArchivePermission::Write) => Permissions::from_mode(0o666),
        (ArchivePermission::Read, ArchivePermission::None) => Permissions::from_mode(0o640),
        (ArchivePermission::Read, ArchivePermission::Read) => Permissions::from_mode(0o644),
        (ArchivePermission::Read, ArchivePermission::Write) => Permissions::from_mode(0o646),
        (ArchivePermission::None, ArchivePermission::None) => Permissions::from_mode(0o600),
        (ArchivePermission::None, ArchivePermission::Read) => Permissions::from_mode(0o604),
        (ArchivePermission::None, ArchivePermission::Write) => Permissions::from_mode(0o606),
    }
}

pub fn validate_config() -> Result<Configuration, Error> {
    let data_dir = PathBuf::from(env::var(DATA_DIR_ENV).unwrap_or(DATA_DIR.into()));
    let backup_dir = PathBuf::from(env::var(BACKUP_DIR_ENV).unwrap_or(ARCHIVE_DIR.into()));
    let archive_strategy = ArchiveStrategy::env_or_default(STRATEGY_ENV)?;
    let archive_compression = ArchiveCompression::env_or_default(COMPRESSION_ENV)?;
    let archive_compression_level =
        archive_compression.parse_level(env::var(COMPRESSION_LEVEL_ENV).unwrap_or_default());
    let archive_prefix = env::var(PREFIX_ENV).unwrap_or(LOG_TARGET.to_string());
    let group_permission = ArchivePermission::env_or_default(GROUP_PERMISSION_ENV)?;
    let other_permission = ArchivePermission::env_or_default(OTHER_PERMISSION_ENV)?;
    let stop_containers = get_env_bool(SALVAGE_CONTAINER_MANAGEMENT_ENV, true);
    let is_docker = get_env_bool(SALVAGE_IS_DOCKER, false);
    let run_once = get_env_bool(SALVAGE_RUN_ONCE_ENV, false);

    if !data_dir.as_path().is_dir() {
        return Err(NoVolumeMounted(data_dir.to_string_lossy().into()));
    } else if !backup_dir.as_path().is_dir() {
        return Err(NoVolumeMounted(backup_dir.to_string_lossy().into()));
    }

    let valid_env = Configuration {
        data_dir,
        backup_dir,
        archive_strategy,
        archive_compression,
        archive_compression_level,
        archive_prefix,
        group_permission,
        other_permission,
        stop_containers,
        is_docker,
        run_once,
    };

    Ok(valid_env)
}

fn get_env_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => value.eq_ignore_ascii_case("true"),
        Err(_) => default,
    }
}
