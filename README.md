# docker-volume-backup
A simple docker container to run cron and backup the folders mounted under `/data` to the volume mounted at `/backup`.

## Environment

| Variable   | Default                | Description                                                                                                     |
|------------|------------------------|-----------------------------------------------------------------------------------------------------------------|
| COMPRESS   | `gz`                   | Compression used on the tarball.<br>Valid values `gz`, `xz`, `bz2`, `lzma`                                      |
| TYPE       | `multi`                | `multi` - Compress each directory into is own archive.<br>`single` - Compress all directories into one archive. |
| CRON       | `0 0 * * *`            | Standard cron expression.<br>See https://en.wikipedia.org/wiki/Cron                                             |
| TZ         | `UTC`                  | Provide timezone to use in the container (ie `America/Phoenix`).                                                |
| PREFIX     | `docker-backup-volume` | Provide the prefix to be used when creating the backup archives.                                                |
| PERMISSION | `644`                  | Prove the permission to set for the backup archive.                                                             |
