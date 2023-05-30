# Rust Clock
[![license](https://img.shields.io/badge/license-MIT-red.svg)](https://github.com/hoothin/RustClock/releases) [![download](https://img.shields.io/github/downloads/hoothin/RustClock/total)](https://github.com/hoothin/RustClock/releases)

Clock popup every half hour. Build with [rust](https://github.com/rust-lang/rust)|[egui](https://github.com/emilk/egui/)|[rodio](https://github.com/RustAudio/rodio)|[tray-icon](https://github.com/tauri-apps/tray-icon)|[chrono](https://github.com/chronotope/chrono)|[rust-ini](https://github.com/zonyitoo/rust-ini)

![example](pic.gif)
# Config
Edit the conf.ini beside rust_clock.
+ time
> The time when rust clock will popup, set by `hour:minute:second`. Split multi-time by `,`.
``` ini
# popup every half hour per clock
time=:30:
```
``` ini
# popup every half hour and every begining of minute in 15 o'clock
time=:30:,15::0
```
+ sound
> The sound file you wish to play when clock popup.
``` ini
# play sound.ogg when popup
sound=sound.ogg
```
+ countdown
>The countdown time, set by `hour:minute:second`. Split multi-time by `,`.
``` ini
# 20-20-20 Rule
countdown=:20:,::20
```
