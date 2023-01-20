# 2023 UCSC eCTF

This repository contains all of the source code for UCSC's 2023 eCTF competition work.

## How to test the project with Podman/Docker (easiest, does not require manual setup of environment)
Note: Using Podman is recommended, as it does not need to run as root.
1. Install [Podman](https://podman.io/getting-started/installation) or [Docker](https://docs.docker.com/get-docker/).
2. Change the working directory to the Rust project to be tested.
3. Run the `run_podman.sh` or `run_docker.sh` scripts depending on which runtime you installed. The project to test will need to specified as an argument. For example, to test the car project, run `./run_podman.sh ucsc-ectf-car` or `./run_docker.sh ucsc-ectf-car`. Valid projects are `ucsc-ectf-car` and `ucsc-ectf-fob`. This will build the firmware and run it on the board with GDB. The firmware execution will be paused at the first instruction. Use the `continue` command to continue execution.

## How to setup the development environment in Ubuntu manually (physical, or virtualized in VirtualBox or WSL2)
1. Install the Rust toolchain with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
2. Run `rustup target add thumbv7em-none-eabihf && sudo apt update && sudo apt install -y openocd gdb-multiarch && sudo ln -s /usr/bin/gdb-multiarch /usr/bin/arm-none-eabi-gdb`.
3. Install the ICDI drivers on Windows from https://www.ti.com/litv/zip/spmc016a (right click on each inf file -> install) (skip this step if running directly on Ubuntu without virtualization).
4. Forward the Texas Instruments In-Circuit Debug Interface device to your virtual machine (requires microcontroller to be plugged in) (on VirtualBox, go to your VM settings -> USB -> Plus Icon -> Texas Instruments In-Circuit Debug Interface -> OK) (on WSL2, follow the instructions here: https://devblogs.microsoft.com/commandline/connecting-usb-devices-to-wsl/) (skip this step if running directly on Ubuntu without virtualization).
5. Add `.gdbinit` to the GDB auto-load safe path with `echo "add-auto-load-safe-path $(pwd)/.gdbinit" >> ~/.gdbinit`. Your working directory must be the root of the repository.

## How to test the project with a manually setup development environment
1. Run `openocd -f board/ti_ek-tm4c123gxl.cfg`. This will start up an OpenOCD server, which will be used with GDB later to debug the firmware. This command requires the board to be plugged in.
2. In a separate shell, run `cargo run --bin <project>`. `<project>` should be `ucsc-ectf-car` or `ucsc-ectf-fob`. This will build the firmware and run it on the board with GDB. The firmware execution will be paused at the first instruction. Use the `continue` command to continue execution.
