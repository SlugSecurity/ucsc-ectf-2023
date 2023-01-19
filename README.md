# Car

The firmware for the car.

## How to run the project with Podman/Docker (This is the easy way!)
Note: Using Podman is recommended, as it does not need to run as root.
1. Install [Podman](https://podman.io/getting-started/installation) or [Docker](https://docs.docker.com/get-docker/).
2. Run the `run_podman.sh` or `run_docker.sh` scripts depending on which runtime you installed.
3. If everything went right you should be in a GDB session.

## How to setup the development environment in Ubuntu (physical, or virtualized in VirtualBox or WSL2)
1. Install the Rust toolchain with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
2. Run `cargo install cross && rustup target add thumbv7em-none-eabihf && sudo apt update && sudo apt install -y openocd gdb-multiarch && sudo ln -s /usr/bin/gdb-multiarch /usr/bin/arm-none-eabi-gdb`.
3. Install the ICDI drivers on Windows from https://www.ti.com/litv/zip/spmc016a (right click on each inf file -> install) (skip this step if running directly on Ubuntu without virtualization).
4. Forward the Texas Instruments In-Circuit Debug Interface device to your virtual machine (requires microcontroller to be plugged in) (on VirtualBox, go to your VM settings -> USB -> Plus Icon -> Texas Instruments In-Circuit Debug Interface -> OK) (on WSL2, follow the instructions here: https://devblogs.microsoft.com/commandline/connecting-usb-devices-to-wsl/) (skip this step if running directly on Ubuntu without virtualization).
5. Add `.gdbinit` to the GDB auto-load safe path with `echo "add-auto-load-safe-path $(pwd)/.gdbinit" >> ~/.gdbinit`. Your working directory must be the root of the repository.

## How to test code
1. Run `openocd -f board/ti_ek-tm4c123gxl.cfg`. This will start up an OpenOCD server, which will be used with GDB later to debug the firmware. This command requires the board to be plugged in.
2. In a separate shell, run `cargo run`. This will build the firmware and run it on the board with GDB. The firmware execution will be paused at the first instruction. Use the `continue` command to continue execution.
