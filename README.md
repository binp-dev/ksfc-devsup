# epics-devsup-template

EPICS Device Support template with Rust bindings

[![Travis CI][travis_badge]][travis]
[![License][license_badge]][license]

[travis_badge]: https://api.travis-ci.org/binp-automation/epics-devsup-template.svg
[license_badge]: https://img.shields.io/github/license/binp-automation/epics-devsup-template.svg

[travis]: https://travis-ci.org/binp-automation/epics-devsup-template
[license]: https://github.com/binp-automation/epics-devsup-template/blob/develop/LICENSE

This project is a template, so you may clone it and make changes as you need.


## Documentation

+ [Rust bindings to EPICS](https://binp-automation.github.io/rust-epics-devsup/target/doc/epics_binding/)


## Requirements

+ `git`
+ `python3`
+ `rustc` and `cargo`
+ `clang`
+ `libreadline-dev`


## Usage

### Clone template project

```bash
git clone https://github.com/binp-automation/epics-devsup-template.git
cd epics-devsup-template
git submodule update --init --recursive
```

### Load and build EPICS

```bash
python3 ./script/epics/load.py
./script/epics/build.sh
```

### Build IOC

```bash
source ./script/epics/env.sh
python3 ./script/build.py
```

### Run and test IOC

```bash
source ./script/epics/env.sh
python3 ./script/test.py
```
