# cargo-watch-traffic-status-indicator

This program takes data from cargo watch as arguments. After processing, it connects to the USB port with the programmed stm32f102, which displays the current status: green - it's ok, yellow - in the process of checking, red - there are errors.

usage: cargo watch | tsi 
or: cargo watch -x check -x test -x build | tsi


