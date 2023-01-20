FROM archlinux:latest AS base
RUN pacman -Sy
RUN pacman -S --noconfirm rustup
RUN rustup default stable
RUN rustup target add thumbv7em-none-eabihf
RUN pacman -S --noconfirm openocd arm-none-eabi-gdb gcc git
RUN mkdir -p /root/.config/gdb && echo 'add-auto-load-safe-path /mnt/.gdbinit' > /root/.config/gdb/gdbinit
ENV CARGO_HOME=/mnt/cache/cargo
WORKDIR /mnt
