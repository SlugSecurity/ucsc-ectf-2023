FROM archlinux:latest AS base
RUN pacman -Sy
RUN pacman -S --noconfirm rustup
RUN rustup default stable
RUN rustup target add thumbv7em-none-eabihf
RUN pacman -S --noconfirm openocd arm-none-eabi-gdb gcc