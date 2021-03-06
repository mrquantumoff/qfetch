# qfetch

qfetch is a tool that fetches info about your linux install.

## Status [![Pipeline](https://github.com/mrquantumoff/qfetch/actions/workflows/rust.yml/badge.svg)](https://github.com/mrquantumoff/qfetch/actions/workflows/rust.yml) ![Crates.IO](https://img.shields.io/crates/v/qfetch)

## Dependencies
* /proc/meminfo with the following fields:
    * MemTotal in the 1st line
    * MemFree in the 2nd line
    * MemAvailable in the 3rd line
* [GNU Coreutils](https://www.gnu.org/software/coreutils/) with the ```df``` command
* /etc/os-release with the following fields:
    * PRETTY_NAME
* Rust and Cargo of course!
* Optionally you can install ```make``` to build the app

## Installation via ```wake```
* Clone the repo
* Install the dependencies
* Run ```wake``` to build
* Run ```wake -s .wake/install.Wakefile``` to install

## Installation via ```make```
* Clone the repo
* Install the dependencies
* Run ```make``` to build the app
* Run ```sudo make install``` to install the app
* Run ```qfetch``` to see the output

## Installation via ``` cargo build ```
* Clone the repository
* Install the dependencies and rust via rustup
* Run ```cargo build -r``` to build the executable
* Your executable is ```target/release/qfetch```, go ahead and run/install it somewhere
## Installation using cargo
* Dead simple, run ```cargo install qfetch```

## Ubuntu binaries are available in [releases](https://github.com/mrquantumoff/qfetch/releases) tab

## Installation on Arch Linux or Arch Based Distros
[Available on AUR (I officialy maintain it)](https://aur.archlinux.org/packages/qfetch-bin/)

[Available on AUR (Maintained by kleintux)](https://aur.archlinux.org/packages/qfetch)

## Help
* Before creating an issue please check out the [wiki](https://github.com/mrquantumoff/qfetch/wiki/Setting-up-ASCII-logos)
* If the previous step didn't help, create an issue!
* In tty the app may not display some data, try running it in a DE (if DE is null or empty then it will not display terminal and some other stuff)
