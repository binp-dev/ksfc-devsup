TOP=../..

include $(TOP)/configure/CONFIG
#----------------------------------------
#  ADD MACRO DEFINITIONS AFTER THIS LINE
#=============================

#=============================
# Build the IOC application

PROD_IOC = lxi
# lxi.dbd will be created and installed
DBD += lxi.dbd

# lxi.dbd will be made up from these files:
lxi_DBD += base.dbd

# Include dbd files from all support applications:
lxi_DBD += devlxi.dbd

# Add all the support libraries needed by this IOC
lxi_LIBS += devlxi

# lxi_registerRecordDeviceDriver.cpp derives from lxi.dbd
lxi_SRCS += lxi_registerRecordDeviceDriver.cpp

# Build the main IOC entry point on workstation OSs.
lxi_SRCS_DEFAULT += lxiMain.cpp
lxi_SRCS_vxWorks += -nil-

# Add support from base/src/vxWorks if needed
#lxi_OBJS_vxWorks += $(EPICS_BASE_BIN)/vxComLibrary

# Finally link to the EPICS Base libraries
lxi_LIBS += $(EPICS_BASE_IOC_LIBS)

#===========================

include $(TOP)/configure/RULES
#----------------------------------------
#  ADD RULES AFTER THIS LINE

