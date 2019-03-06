#/bin/sh

export EPICS_BASE=$PWD/epics-base
export EPICS_HOST_ARCH=$($EPICS_BASE/startup/EpicsHostArch)
export PATH=$PATH:$EPICS_BASE/bin/$EPICS_HOST_ARCH/
export EPICS_CA_AUTO_ADDR_LIST=NO
export EPICS_CA_ADDR_LIST=127.255.255.255
