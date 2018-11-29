#!../../bin/linux-x86_64/lxi

## You may have to change lxi to something else
## everywhere it appears in this file

< envPaths

cd "${TOP}"

## Register all support components
dbLoadDatabase "dbd/lxi.dbd"
lxi_registerRecordDeviceDriver pdbbase

## Load record instances
dbLoadRecords("db/devlxi.db")

cd "${TOP}/iocBoot/${IOC}"
iocInit

## Start any sequence programs
#seq sncxxx,"user=alex"
