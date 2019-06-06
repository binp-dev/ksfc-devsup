#!../../bin/linux-x86_64/rsbind

< envPaths

cd "${TOP}"

dbLoadDatabase("dbd/rsbind.dbd")
rsbind_registerRecordDeviceDriver(pdbbase)

dbLoadRecords("db/devrsbind.db", "P=FC")

connectDevice("10.0.0.9", "FC")

cd "${TOP}/iocBoot/${IOC}"
iocInit()

startAll()
