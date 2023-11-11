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
