# qfetch

qfetch is a tool that fetches info about your linux install.

## Status [![Pipeline](https://github.com/mrquantumoff/qfetch/actions/workflows/rust.yml/badge.svg)](https://github.com/mrquantumoff/qfetch/actions/workflows/rust.yml)

## Dependencies
* /proc/meminfo with the following fields:
    * MemTotal in the 1st line
    * MemFree in the 2nd line
    * MemAvailable in the 3rd line
* [GNU Coreutils](https://www.gnu.org/software/coreutils/) with the ```df``` command
* /etc/os-release with the following fields:
    * PRETTY_NAME
## Installation
* Clone the repository
* Install the dependencies and rust via rustup
* Run ```cargo build -r``` to build the executable
* Your executable is ```target/release/qfetch```, go ahead and run/install it somewhere
## Installation using cargo
* Dead simple, run ```cargo install qfetch```

## Help
* Before creating an issue please check out the [wiki](https://github.com/mrquantumoff/qfetch/wiki/Setting-up-ASCII-logos)
* If the previous step didn't help, create an issue!
