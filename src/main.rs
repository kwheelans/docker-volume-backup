use crate::configuration::{
    validate_config, ArchiveCompression, ArchiveStrategy, Configuration, LOG_TARGET,
};
use crate::error::Error;
use flate2::write::GzEncoder;
use flate2::Compression;
use log::{debug, error, info, LevelFilter};
use std::collections::HashSet;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str::FromStr;
use time::macros::format_description;
use time::OffsetDateTime;
use xz2::write::XzEncoder;

mod configuration;
mod error;

const TIMESTAMP_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[year]-[month]-[day]_[hour]-[minute]-[second]");
// Default Paths
const BACKUP_DIR: &str = "/backup";
const DATA_DIR: &str = "/data";

// Environment Variable Names
const BACKUP_DIR_ENV: &str = "BACKUP_DIR";
const DATA_DIR_ENV: &str = "DATA_DIR";
const LOG_LEVEL: &str = "SALVAGE_LOG_LEVEL";
const STRATEGY_ENV: &str = "SALVAGE_ARCHIVE_STRATEGY";
const PREFIX_ENV: &str = "SALVAGE_ARCHIVE_PREFIX";
const COMPRESS_ENV: &str = "SALVAGE_ARCHIVE_COMPRESSION";
const GROUP_PERMISSION_ENV: &str = "SALVAGE_ARCHIVE_GROUP_PERMISSION";
const OTHER_PERMISSION_ENV: &str = "SALVAGE_ARCHIVE_OTHER_PERMISSION";
const SALVAGE_STOP_CONTAINERS_ENV: &str = "SALVAGE_STOP_CONTAINERS";

fn main() -> ExitCode {
    if let Err(error) = simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .env()
        .with_module_level(LOG_TARGET, set_logging_level())
        .with_local_timestamps()
        .init()
    {
        println!(
            "ERROR [{}] Unable to initialize logger: {}",
            LOG_TARGET, error
        );
        return ExitCode::FAILURE;
    }

    if let Err(error) = run() {
        error!(target: LOG_TARGET, "{}", error);
        return ExitCode::FAILURE;
    }
    debug!(target: LOG_TARGET, "Ended successfully");
    ExitCode::SUCCESS
}

fn run() -> Result<(), Error> {
    let args: HashSet<String> = env::args().collect();
    if args.contains("-v") || args.contains("--validate") {
        let config = validate_config()?;
        info!(target: LOG_TARGET, "Configuration validated successfully.");
        info!(target: LOG_TARGET, "Data Directory: {}", config.data_dir.to_string_lossy());
        info!(target: LOG_TARGET, "Backup Directory: {}", config.backup_dir.to_string_lossy());
        info!(target: LOG_TARGET, "Archive Compression: {}", config.archive_compression.to_string());
        info!(target: LOG_TARGET, "Archive Strategy: {}", config.archive_strategy.to_string());
        info!(target: LOG_TARGET, "Archive Prefix: {}", config.archive_prefix.as_str());
        info!(target: LOG_TARGET, "Archive Compression: {}", config.group_permission.to_string());
        info!(target: LOG_TARGET, "Archive Compression: {}", config.other_permission.to_string());
        info!(target: LOG_TARGET, "Container Management Enabled: {}", config.stop_containers);
    } else {
        run_backup()?;
    }

    Ok(())
}

fn set_logging_level() -> LevelFilter {
    LevelFilter::from_str(env::var(LOG_LEVEL).unwrap_or("INFO".into()).as_str())
        .unwrap_or(LevelFilter::Info)
}

fn run_backup() -> Result<(), Error> {
    info!(target: LOG_TARGET, "Backup Started");
    let config = validate_config()?;

    let backup_paths: Vec<_> = std::fs::read_dir(config.data_dir.as_path())?
        .map(|r| r.map(|e| e.path()))
        .map(|d| d.unwrap())
        .filter(|d| d.is_dir())
        .collect();

    for path in backup_paths.as_slice() {
        debug!(target: LOG_TARGET, "{}: {}", path.file_name().unwrap_or(OsStr::new("")).to_string_lossy() , path.to_string_lossy());
    }

    let backup_paths: Vec<(_, _)> = backup_paths
        .iter()
        .filter(|p| p.as_path().file_name().is_some())
        .map(|f| (f.file_name().unwrap().to_os_string(), f.to_path_buf()))
        .collect();

    match config.archive_strategy {
        ArchiveStrategy::Single => single_archive(backup_paths, &config)?,
        ArchiveStrategy::Multiple => multiple_archive(backup_paths, &config)?,
    }

    info!(target: LOG_TARGET, "Backup Finished");
    Ok(())
}

fn timestamp() -> Result<String, Error> {
    let timestamp = OffsetDateTime::now_local()?;
    Ok(timestamp.format(TIMESTAMP_FORMAT)?)
}

fn single_archive(
    directories: Vec<(OsString, PathBuf)>,
    config: &Configuration,
) -> Result<(), Error> {
    let timestamp = timestamp()?;
    let archive_name = format!(
        "{}_{}.tar.{}",
        config.archive_prefix,
        timestamp,
        config.archive_compression.extension()
    );
    let archive_path = config.backup_dir.as_path().join(archive_name.as_str());
    let compressor = select_encoder(archive_path.as_path(), &config.archive_compression)?;
    let mut tar = tar::Builder::new(compressor);

    for (name, path) in directories {
        tar.append_dir_all(name, path)?;
    }
    std::fs::set_permissions(archive_path.as_path(), config.archive_permission())?;
    Ok(())
}

fn multiple_archive(
    directories: Vec<(OsString, PathBuf)>,
    config: &Configuration,
) -> Result<(), Error> {
    let timestamp = timestamp()?;
    for (name, path) in directories {
        let archive_name = format!(
            "{}_{}_{}.tar.{}",
            config.archive_prefix,
            name.to_string_lossy(),
            timestamp,
            config.archive_compression.extension()
        );
        let archive_path = config.backup_dir.as_path().join(archive_name.as_str());
        let compressor = select_encoder(archive_path.as_path(), &config.archive_compression)?;
        let mut tar = tar::Builder::new(compressor);
        tar.append_dir_all(name, path)?;
        tar.finish()?;
        std::fs::set_permissions(archive_path.as_path(), config.archive_permission())?;
    }

    Ok(())
}

fn select_encoder<P: AsRef<Path>>(
    path: P,
    compress: &ArchiveCompression,
) -> Result<Box<dyn Write>, Error> {
    let file = File::create(path.as_ref())?;
    let encoder: Box<dyn Write> = match compress {
        ArchiveCompression::Gzip => Box::new(GzEncoder::new(file, Compression::default())),
        ArchiveCompression::Xz => Box::new(XzEncoder::new(file, 6)),
    };
    Ok(encoder)
}
