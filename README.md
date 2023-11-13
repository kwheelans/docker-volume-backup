# Salvage
A simple docker container to run cron and backup the folders mounted under `/data` to the volume mounted at `/backup`.

## Environment

| Variable                         | Default     | Description                                                                                                         |
|----------------------------------|-------------|---------------------------------------------------------------------------------------------------------------------|
| CRON                             | `0 0 * * *` | Standard cron expression.<br>See https://en.wikipedia.org/wiki/Cron                                                 |
| TZ                               | `UTC`       | Provide timezone to use in the container (ie `America/Phoenix`).                                                    |
| SALVAGE_ARCHIVE_COMPRESSION      | `gzip`      | Compression used on the tarball.<br>Valid values `gzip`, `xz`                                                       |
| SALVAGE_ARCHIVE_STRATEGY         | `multiple`  | `multiple` - Compress each directory into is own archive.<br>`single` - Compress all directories into one archive.  |
| SALVAGE_ARCHIVE_PREFIX           | `salvage`   | Provide the prefix to be used when creating the backup archives.                                                    |
| SALVAGE_ARCHIVE_GROUP_PERMISSION | `read`      | Provide how the group permission should be set for the backup archive.<br>Valid values `read`, `read-write`, `none` |
| SALVAGE_ARCHIVE_OTHER_PERMISSION | `read`      | Provide how the other permission should be set for the backup archive.<br>Valid values `read`, `read-write`, `none` |
| SALVAGE_STOP_CONTAINERS          | `true`      | Controls if containers should be stopped while their volumes are being backed up.                                   |
| SALVAGE_RUN_ONCE                 | `false`     | When set to true salvage will only run once and exit and not on a schedule.                                         |
