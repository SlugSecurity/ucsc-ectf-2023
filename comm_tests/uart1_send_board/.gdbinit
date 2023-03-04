target extended-remote :3334
set print asm-demangle on
monitor arm semihosting enable
load
step
