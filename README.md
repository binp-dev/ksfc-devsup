# ksfc-devsup

EPICS Device Support for Keysight 53220A frequency counter


## Requirements

+ `git`
+ `python3`
+ `rustc` and `cargo`
+ `clang`
+ `libreadline-dev`


## Usage

### Clone project

```bash
git clone https://github.com/binp-automation/ksfc-devsup.git
cd ksfc-devsup
```

### Set EPICS envvars

```bash
export EPICS_BASE=/path/to/epics-base/
export EPICS_HOST_ARCH=$($EPICS_BASE/startup/EpicsHostArch)
export PATH=$PATH:$EPICS_BASE/bin/$EPICS_HOST_ARCH/
```

### Configure

+ In `iocBoot/iocrsbind/st.cmd` edit:
  + `dbLoadRecords("db/devrsbind.db", "P=<DEVICE_PREFIX>")`
  + `connectDevice(<DEVICE_IP_ADDRESS>, <DEVICE_PREFIX>)`

### Build IOC

```bash
python3 ./script/build.py
```

### Run and test IOC

```bash
python3 ./script/test.py
```
