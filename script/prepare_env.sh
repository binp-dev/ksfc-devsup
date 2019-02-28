#!/bin/sh

git clone https://github.com/epics-base/epics-base.git epics-base --branch R7.0.2 &&
export EPICS_BASE=$PWD/epics-base &&
export EPICS_HOST_ARCH=$($EPICS_BASE/startup/EpicsHostArch) &&
cd epics-base &&
make &&
cd .. &&
echo "epics-base built successfully"
