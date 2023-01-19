FROM archlinux:latest AS base
RUN pacman -Sy
RUN pacman -S --noconfirm rustup
RUN rustup default nightly
RUN rustup target add thumbv7em-none-eabihf
RUN pacman -S --noconfirm openocd arm-none-eabi-gdb gcc
RUN mkdir -p /root/.config/gdb && echo 'add-auto-load-safe-path /mnt/.gdbinit' > /root/.config/gdb/gdbinit
WORKDIR /mnt
CMD ["sh", "-c", "openocd -f /usr/share/openocd/scripts/board/ti_ek-tm4c123gxl.cfg & cargo run"]