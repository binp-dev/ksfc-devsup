TOP=../..

include $(TOP)/configure/CONFIG
#----------------------------------------
#  ADD MACRO DEFINITIONS AFTER THIS LINE
#=============================

#=============================
# Build the IOC application

PROD_IOC = rsbind
# rsbind.dbd will be created and installed
DBD += rsbind.dbd

# rsbind.dbd will be made up from these files:
rsbind_DBD += base.dbd

# Include dbd files from all support applications:
rsbind_DBD += devrsbind.dbd

# Add all the support libraries needed by this IOC
rsbind_LIBS += devrsbind device_support

# rsbind_registerRecordDeviceDriver.cpp derives from rsbind.dbd
rsbind_SRCS += rsbind_registerRecordDeviceDriver.cpp

# Build the main IOC entry point on workstation OSs.
rsbind_SRCS_DEFAULT += rsbindMain.cpp
rsbind_SRCS_vxWorks += -nil-

# Add support from base/src/vxWorks if needed
#rsbind_OBJS_vxWorks += $(EPICS_BASE_BIN)/vxComLibrary

# Finally link to the EPICS Base libraries
rsbind_LIBS += $(EPICS_BASE_IOC_LIBS)

#===========================

include $(TOP)/configure/RULES
#----------------------------------------
#  ADD RULES AFTER THIS LINE

