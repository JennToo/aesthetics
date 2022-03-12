set history save on
set remotetimeout 240
set print asm-demangle on

target extended-remote 192.168.1.65:2331
load
# For JLink, OpenOCD might need something else
monitor reset

tui enable
