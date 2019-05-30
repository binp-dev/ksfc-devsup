TOP=..
include $(TOP)/configure/CONFIG
#=======================================

# Build devrsbind as a library for an IOC:
LIBRARY_IOC += devrsbind

# Library Source files
devrsbind_SRCS += devrsbind.c

# Link with the libraries
devrsbind_LIBS += $(EPICS_BASE_IOC_LIBS)

# Install .dbd and .db files
DBD += devrsbind.dbd
DB_INSTALLS += $(TOP)/rsbindSup/devrsbind.db

#=======================================
include $(TOP)/configure/RULES
