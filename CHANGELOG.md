# v0.7.1
## Fixes
- Suppress warning when `SALVAGE_ARCHIVE_COMPRESSION_LEVEL` is not provided
- Changed level of some logging messages to match intention

## Minor Changes
- Added duration timers for logging

# v0.7.0
## Features
- Added Zstandard to valid compression types.
- Added BZip2 to valid compression types.
- Added environment variable `SALVAGE_ARCHIVE_COMPRESSION_LEVEL` to control compression level.

## Fixes
- Fixed some error message text.

# v0.6.0
## Breaking Changes
- Names of the environment variables have been changed.
- Default archive directory in the container changed to `/archive`.

## Features
- Added docker container management to stop and start containers.
  - Only one salvage container at time is currently supported. Other salvage containers identified will be stopped and removed.
  - Will identify running containers that container volumes with the same source as the volumes mounted under `/data` in the salvage container.
  - After archive process completes the previously stopped containers are restarted.

# v0.5.0
## Changes
- update package name


# v0.4.1
## Changes
- Remove unnecessary colour feature from logging
- Use local timestamps for logging
- Better handling for logger initialization failure

# v0.4.0
## Breaking Changes
- BZip2 not supported.
- LZMA extension not supported use XZ.

## Feature
- migrate to rust instead of a shell script


# v0.3.0
## Features
- Set archive file permissions with `PERMISSION`

# v0.2.0
## Features
- Set timezone through `TZ` environment variable
- Set backup archive prefix through `PREFIX` environment variable

# v0.1.0
## Features
- basic functionality to backup all directories mounted under `/data` to volume mounted at `/backup`
- Pass environment variable to control how tarball is compressed
- Pass environment variable to control single tarball or tarball per directory in `/data`
- Pass environment variable to set cron timing
