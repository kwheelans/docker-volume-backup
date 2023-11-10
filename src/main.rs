use crate::configuration::{BackupCompression, BackupStrategy, Configuration};
use crate::error::Error;
use crate::error::Error::NoVolumeMounted;
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

const LOG_TARGET: &str = "docker-volume-backup";
const TIMESTAMP_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[year]-[month]-[day]_[hour]-[minute]-[second]");

const BACKUP_DIR: &str = "/backup";
const BACKUP_DIR_ENV: &str = "BACKUP_DIR";
const DATA_DIR: &str = "/data";
const DATA_DIR_ENV: &str = "DATA_DIR";
const LOG_LEVEL: &str = "LOG_LEVEL";
const STRATEGY_ENV: &str = "STRATEGY";
const PREFIX_ENV: &str = "PREFIX";
const COMPRESS_ENV: &str = "COMPRESS";

fn main() -> ExitCode {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .env()
        .with_module_level(LOG_TARGET, set_logging_level())
        .with_colors(true)
        .init()
        .unwrap();

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
        info!(target: LOG_TARGET, "Data Directory: {}", config.data_dir.to_string_lossy());
        info!(target: LOG_TARGET, "Backup Directory: {}", config.backup_dir.to_string_lossy());
        info!(target: LOG_TARGET, "Backup Prefix: {}", config.prefix.as_str());
        info!(target: LOG_TARGET, "Compression Type: {:?}", config.compression.to_string());
        info!(target: LOG_TARGET, "Archive Strategy: {:?}", config.backup_type.to_string());
    } else {
        run_backup()?;
    }

    Ok(())
}

fn validate_config() -> Result<Configuration, Error> {
    let data_dir = PathBuf::from(env::var(DATA_DIR_ENV).unwrap_or(DATA_DIR.into()));
    let backup_dir = PathBuf::from(env::var(BACKUP_DIR_ENV).unwrap_or(BACKUP_DIR.into()));
    let backup_type =
        BackupStrategy::from_str(env::var(STRATEGY_ENV).unwrap_or_default().as_str())?;
    let compression =
        BackupCompression::from_str(env::var(COMPRESS_ENV).unwrap_or_default().as_str())?;
    let prefix = env::var(PREFIX_ENV).unwrap_or(LOG_TARGET.to_string());

    if !data_dir.as_path().is_dir() {
        return Err(NoVolumeMounted(data_dir.to_string_lossy().into()));
    } else if !backup_dir.as_path().is_dir() {
        return Err(NoVolumeMounted(backup_dir.to_string_lossy().into()));
    }

    let valid_env = Configuration {
        data_dir,
        backup_dir,
        backup_type,
        compression,
        prefix,
    };

    Ok(valid_env)
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

    match config.backup_type {
        BackupStrategy::Single => single_archive(backup_paths, &config)?,
        BackupStrategy::Multiple => multiple_archive(backup_paths, &config)?,
    }

    info!(target: LOG_TARGET, "Backup Finished");
    Ok(())
}

fn timestamp() -> Result<String, Error> {
    let timestamp = OffsetDateTime::now_local()?;
    Ok(timestamp.format(TIMESTAMP_FORMAT)?)
}

fn single_archive(directories: Vec<(OsString, PathBuf)>, config: &Configuration) -> Result<(), Error> {
    let timestamp = timestamp()?;
    let archive_name = format!(
        "{}_{}.tar.{}",
        config.prefix,
        timestamp,
        config.compression.extension()
    );
    let compressor = select_encoder(
        config.backup_dir.as_path().join(archive_name.as_str()),
        &config.compression,
    )?;
    let mut tar = tar::Builder::new(compressor);

    for (name, path) in directories {
        tar.append_dir_all(name, path)?;
    }

    Ok(())
}

fn multiple_archive(directories: Vec<(OsString, PathBuf)>, config: &Configuration) -> Result<(), Error> {
    let timestamp = timestamp()?;
    for (name, path) in directories {
        let archive_name = format!(
            "{}_{}_{}.tar.{}",
            config.prefix,
            name.to_string_lossy(),
            timestamp,
            config.compression.extension()
        );
        let compressor = select_encoder(
            config.backup_dir.as_path().join(archive_name.as_str()),
            &config.compression,
        )?;
        let mut tar = tar::Builder::new(compressor);
        tar.append_dir_all(name, path)?;
        tar.finish()?;
    }

    Ok(())
}

fn select_encoder<P: AsRef<Path>>(
    path: P,
    compress: &BackupCompression,
) -> Result<Box<dyn Write>, Error> {
    let file = File::create(path.as_ref())?;
    let encoder: Box<dyn Write> = match compress {
        BackupCompression::Gzip => Box::new(GzEncoder::new(file, Compression::default())),
        BackupCompression::Xz => Box::new(XzEncoder::new(file, 6)),
    };
    Ok(encoder)
}
