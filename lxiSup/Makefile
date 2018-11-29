TOP=..
include $(TOP)/configure/CONFIG
#=======================================

# Build devlxi as a library for an IOC:
LIBRARY_IOC += devlxi

# Library Source files
devlxi_SRCS += devlxi.c

# Link with the asyn and base libraries
devlxi_LIBS += $(EPICS_BASE_IOC_LIBS)

# Install .dbd and .db files
DBD += devlxi.dbd
DB_INSTALLS += $(TOP)/lxiSup/devlxi.db

#=======================================
include $(TOP)/configure/RULES
