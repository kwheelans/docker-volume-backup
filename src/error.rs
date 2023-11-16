use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Error return when conversion to [`ArchiveStrategy`] fails
    #[error("Provided value cannot be converted to ArchiveStrategy enum")]
    InvalidBackupType,

    /// Error return when conversion to [`ArchiveCompression`] fails
    #[error("Provided value cannot be converted to ArchiveCompression enum")]
    InvalidCompressionType,

    /// Error return when conversion to [`ArchivePermission`] fails
    #[error("Provided value cannot be converted to ArchivePermission enum")]
    InvalidPermission,

    /// Error returned when a required directory does not exit
    #[error("No volume mounted at: {0}")]
    NoVolumeMounted(String),

    /// Error returned when no instance of a running salvage container can be found
    #[error("No running salvage container was found")]
    NoSalvageContainer,

    // ### Converting from other error types ###
    /// PassPass-thru `bollard::errors::Error`
    #[error("bollard::errors::Error: {0}")]
    DockerApi(#[from] bollard::errors::Error),

    /// Pass-thru [`std::io::Error`].
    #[error("std::io Error: {0}")]
    IO(#[from] std::io::Error),

    /// Pass-thru `time::error::Error`
    #[error("time::error::Error: {0}")]
    Time(#[from] time::error::Error),

    /// Pass-thru `time::error::IndeterminateOffset`
    #[error("time::error::IndeterminateOffset Error: {0}")]
    TimeOffset(#[from] time::error::IndeterminateOffset),

    /// Pass-thru `time::error::Format`
    #[error("time::error::Format Error: {0}")]
    TimeFormat(#[from] time::error::Format),
}
