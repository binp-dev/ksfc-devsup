# EPICS device support modules for binding to device API

[![Travis CI][travis_badge]][travis]
[![License][license_badge]][license]

[travis_badge]: https://api.travis-ci.org/binp-automation/epics-drv-rs.svg
[license_badge]: https://img.shields.io/github/license/binp-automation/epics-drv-rs.svg

[travis]: https://travis-ci.org/nthend/ringbuf
[license]: https://github.com/binp-automation/epics-drv-rs/blob/develop/LICENSE

## Load and build EPICS
```bash
python3 ./script/epics/load.py
./script/epics/build.sh
```

## Build IOC
```bash
source ./script/epics/env.sh
python3 ./script/build.py
```

## Run and test IOC
```bash
source ./script/epics/env.sh
python3 ./script/test.py
```
