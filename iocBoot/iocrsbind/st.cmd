#!../../bin/linux-x86_64/rsbind

< envPaths

cd "${TOP}"

dbLoadDatabase("dbd/rsbind.dbd")
rsbind_registerRecordDeviceDriver(pdbbase)

dbLoadRecords("db/devrsbind.db")

cd "${TOP}/iocBoot/${IOC}"
iocInit()

test_command(1, 2.0, "test")
