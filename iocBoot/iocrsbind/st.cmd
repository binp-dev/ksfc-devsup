#!../../bin/linux-x86_64/rsbind

## You may have to change rsbind to something else
## everywhere it appears in this file

< envPaths

cd "${TOP}"

## Register all support components
dbLoadDatabase "dbd/rsbind.dbd"
rsbind_registerRecordDeviceDriver pdbbase

## Load record instances
dbLoadRecords("db/devrsbind.db")

cd "${TOP}/iocBoot/${IOC}"
iocInit

## Start any sequence programs
#seq sncxxx,"user=alex"
