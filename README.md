# 2023 UCSC eCTF

This repository contains all of the source code for UCSC's 2023 eCTF competition work. There is a GitLab CI script in this repository that is not functional here because we migrated this from our GitLab group to our GitHub organization.

We migrated our issues and merge requests from our GitLab. However, since the source branches on the merge requests were deleted, the merge requests were migrated as issues. The migrated merge request issues are also missing line numbers from the code review comments.

See the [design document](DESIGN.pdf) for an overview of our design.

## Contributing
See the [contribution guidelines](CONTRIBUTING.md) for an overview of our contribution and code review process.

## How to test the project with Podman
1. Change the working directory to the main project directory or your own copy of it.
2. Run `./finding_friends.nu` to grab a pair of boards for yourself.
3. (OPTIONAL) Run `tmux` or `screen` to start a terminal multiplexer. This will be important if you want to debug two boards at once.
4. Run `./run_podman.nu`, specifying the project to test as an argument. For example, to test the car project, run `./run_podman.nu ucsc-ectf-car`. Valid projects are `ucsc-ectf-car` and `ucsc-ectf-fob`. This will build the firmware and run it on the board with GDB. The firmware execution will be paused at the first instruction. Use the `continue` command to continue execution.
5. (OPTIONAL) If for whatever reason you need to know what boards you have grabbed, you can run `./grabbed_boards.nu`.

## How to test the project with a manually setup development environment (DO NOT DO THIS UNLESS YOU KNOW WHAT YOU ARE DOING)
1. Run `openocd -f board/ti_ek-tm4c123gxl.cfg`. This will start up an OpenOCD server, which will be used with GDB later to debug the firmware. This command requires the board to be plugged in.
2. In a separate shell, run `cargo run --bin <project>`. `<project>` should be `ucsc-ectf-car` or `ucsc-ectf-fob`. This will build the firmware and run it on the board with GDB. The firmware execution will be paused at the first instruction. Use the `continue` command to continue execution.
