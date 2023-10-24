# docker-volume-backup
A simple docker container to run cron and backup the folders mounted under `/data` to the volume mounted at `/backup`.

## Environment

| Variable | Default     | Description                                                                                                     |
|----------|-------------|-----------------------------------------------------------------------------------------------------------------|
| COMPRESS | `gz`        | Compression used on the tarball.<br>Valid values `gz`, `xz`, `bz2`, `lzma`                                      |
| TYPE     | `multi`     | `multi` - Compress each directory into is own archive.<br>`single` - Compress all directories into one archive. |
| CRON     | `0 0 * * *` | Standard cron expression.<br>See https://en.wikipedia.org/wiki/Cron                                             |
