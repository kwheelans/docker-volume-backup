# salvage
A simple docker container to run cron and backup the folders mounted under `/data` to the volume mounted at `/backup`.

## Environment

| Variable         | Default          | Description                                                                                                        |
|------------------|------------------|--------------------------------------------------------------------------------------------------------------------|
| COMPRESS         | `gz`             | Compression used on the tarball.<br>Valid values `gz`, `xz`                                                        |
| STRATEGY         | `multiple`       | `multiple` - Compress each directory into is own archive.<br>`single` - Compress all directories into one archive. |
| CRON             | `0 0 * * *`      | Standard cron expression.<br>See https://en.wikipedia.org/wiki/Cron                                                |
| TZ               | `UTC`            | Provide timezone to use in the container (ie `America/Phoenix`).                                                   |
| PREFIX           | `salvage-backup` | Provide the prefix to be used when creating the backup archives.                                                   |
| GROUP_PERMISSION | `read`           | Provide how the group permission should be set for the backup archive.                                             |
| OTHER_PERMISSION | `read`           | Provide how the other permission should be set for the backup archive.                                             |
