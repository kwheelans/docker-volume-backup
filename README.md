# Salvage
[![License](https://img.shields.io/github/license/kwheelans/salvage?color=blue)](./LICENSE)
[![latest version](https://img.shields.io/github/v/tag/kwheelans/salvage?label=version)](https://github.com/kwheelans/salvage/releases)
![Docker Image Size (tag)](https://img.shields.io/docker/image-size/kwheelans/salvage/latest?logo=docker)


A docker container utility to schedule archiving container volumes.

## Usage
Volumes mounted to a directory under `/data` will be archived to the volume mounted at `/archive` inside the Salvage container.
Directories are added to a tarball based on the archive strategy and are then compressed with the selected archive compression type.
Each archive is timestamped based on when the archive process started running, meaning all archives created during the same job run will have the same timestamp ni their filename.
Timestamps are created in the format `[year]-[month]-[day]_[hour]-[minute]-[second]`.


### Examples
#### Docker
```shell
docker run -d \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /path/to/archive/directory:/archive \
  -v my-app-volume:/data/app \
  --name salvage \
  --restart=always \
  kwheelans/salvage:latest
```

#### Docker Compose
```yaml
version: "3.8"
services:
  salvage:
    container_name: salvage
    image: kwheelans/salvage:latest
    restart: unless-stopped
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - /path/to/archive/directory:/archive
      - my-app-volume:/data/app

volumes:
  my-app-volume:
    external: true
```

## Environment Variables

| Variable                         | Default     | Description                                                                                                                             |
|----------------------------------|-------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| SCHEDULE                         | `0 0 * * *` | Standard cron expression.<br>See https://en.wikipedia.org/wiki/Cron.                                                                    |
| TZ                               | `UTC`       | Provide TZ identifier to use in the container (ie `America/Phoenix`). See https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. |
| SALVAGE_ARCHIVE_COMPRESSION      | `gzip`      | Compression used on the tarball archive.<br>Valid values `gzip`, `xz`, `zstd`.                                                          |
| SALVAGE_ARCHIVE_STRATEGY         | `multiple`  | `multiple` - Compress each directory into is own archive.<br>`single` - Compress all directories into one archive.                      |
| SALVAGE_ARCHIVE_PREFIX           | `salvage`   | Provide the prefix to be used when creating the backup archives.                                                                        |
| SALVAGE_ARCHIVE_GROUP_PERMISSION | `read`      | Provide how the group permission should be set for the backup archive.<br>Valid values `read`, `read-write`, `none`.                    |
| SALVAGE_ARCHIVE_OTHER_PERMISSION | `read`      | Provide how the other permission should be set for the backup archive.<br>Valid values `read`, `read-write`, `none`.                    |
| SALVAGE_CONTAINER_MANAGEMENT     | `true`      | Controls if containers should be stopped while their volumes are being backed up.                                                       |
| SALVAGE_RUN_ONCE                 | `false`     | When set to true salvage will only run once and exit and not on a schedule.                                                             |

## Container Registries

| Container Registry        | Image                                                             |
|---------------------------|-------------------------------------------------------------------|
| Docker Hub                | [kwheelans/salvage](https://hub.docker.com/r/kwheelans/salvage)   |
| GitHub Container Registry | [ghcr.io/kwheelans/salvage](https://github.com/kwheelans/salvage) |
