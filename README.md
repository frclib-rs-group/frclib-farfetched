# FRCLib FarFetched

FarFetched is a RESTful webserver used for remotely monitoring system metrics regardless of user code and configuring some aspects
of the system.

A high priority for this project was to make its RAM and CPU footprint as small as possible so as not to hog resources.


## Platforms

`FarFetched` only supports Linux due to how it fetches system information.
There are also special features specific to the `RoboRio` platform to approach feature parity with the NI webserver.

## Goals

- [X] Load webpage from server
- [X] Deploy to a Rio
- [X] Display some system information to the user
- [X] Allow configuring the network settings remotely (backend)
- [ ] Allow configuring the network settings remotely (frontend)
- [ ] Improve graphs and fix process resource usage bugs
- [ ] Test Rio config interop
- [ ] Add CI
