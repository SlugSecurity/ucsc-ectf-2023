# Car

The firmware for the car.

## How to setup the development environment in Ubuntu (optionally with a Windows host)
1. Install the Rust toolchain with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
2. Run `cargo install cross && rustup target add thumbv7em-none-eabihf && sudo apt update && sudo apt install -y openocd gdb-multiarch && ln -s /usr/bin/gdb-multiarch /usr/bin/arm-none-eabi-gdb`.
3. Install the ICDI drivers on Windows from https://www.ti.com/litv/zip/spmc016a (right click on each inf file -> install) (skip this step if running directly on Ubuntu without virtualization).
4. Forward the Texas Instruments In-Circuit Debug Interface device to your virtual machine (skip this step if running directly on Ubuntu without virtualization).
5. Add `.gdbinit` for each project to the auto-load safe path with `cd <repository> && echo "add-auto-load-safe-path $(pwd)/.gdbinit" >> ~/.gdbinit`.

## How to test code
1. Run `openocd -f board/ti_ek-tm4c123gxl.cfg`. This will start up an OpenOCD server, which will be used with GDB later to debug the firmware. This command requires the board to be plugged in.
2. In a separate shell, run `cargo run`. This will build the firmware and run it on the board with GDB. The firmware execution will be paused at the first instruction. Use the `continue` command to continue execution.
