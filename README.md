# backlight-mixer

like alsamixer but for your screen backlight

# usage

This binary needs setuid root to be able to update backlight.

To view current value:

`cargo build && sudo ./target/debug/backlight-mixer`

To change current value (in percent):

`cargo build && sudo ./target/debug/backlight-mixer 50`
