# EPICS device support modules for some LXI devices

## Structure

### Event loop (or without it)

Possible solutions:

+ One `libevent`/`libev`/`Boost.Asio`/`libuv`/custom event loop that handles interaction with all devices

or

+ Separate thread for each device where it handles connection by itself

### Device driver
+ Interacts with specific device (one or more)
+ Has specific API

### EPICS device support module
+ Interacts with corresponding device driver
+ Maps device functionality to EPICS PVs

### Maybe other CS modules
+ Can reuse device driver functionality
