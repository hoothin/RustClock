# Rust Clock
[![license](https://img.shields.io/badge/license-MIT-red.svg)](https://github.com/hoothin/RustClock/releases) [![download](https://img.shields.io/github/downloads/hoothin/RustClock/total)](https://github.com/hoothin/RustClock/releases)

Clock popup every half hour. Build with [rust](https://github.com/rust-lang/rust)|[egui](https://github.com/emilk/egui/)|[rodio](https://github.com/RustAudio/rodio)|[tray-icon](https://github.com/tauri-apps/tray-icon)|[chrono](https://github.com/chronotope/chrono)|[rust-ini](https://github.com/zonyitoo/rust-ini)
> 每隔半小時彈出一次的桌面時鐘

![example](pic.gif)
# Install
+ [Release](https://github.com/hoothin/RustClock/releases/)
+ Homebrew
``` bash
brew install hoothin/rust_clock/rust_clock
```

# Config
Edit the conf.ini beside rust_clock, delete `#`.
> 編輯可執行文件旁的 conf.ini，去除对应项前的注释符号`#`。
+ time
> The time when rust clock will popup, set by `hour:minute:second`. Split multi-time by `,`.
> 
> 設置 rust clock 彈出的時刻，使用 `時:分:秒` 的格式，多個時刻使用 `,` 分隔。
``` ini
# popup every half hour per clock
# 每個鐘頭的 30 分鐘彈出
time=:30:
```
``` ini
# popup every half hour and every beginning of minute in 15 o'clock
# 每個鐘頭的 30 分鐘與 15 點整彈出
time=:30:,15::0
```
+ sound
> The sound file you wish to play when clock popup.
>
> 彈出時播放的音效文件
``` ini
# play sound.ogg when popup
# 彈出時播放同目錄下的 sound.ogg 文件
sound=sound.ogg
```
``` ini
# play assets/1.mp3 when reaches first time you set，play assets/2.mp3 when reaches second time you set.
# 設定的第一個報時播放 assets/1.mp3，設定的第二個報時播放 assets/2.mp3
sound=assets/1.mp3|assets/2.mp3
```
``` ini
# Increase the countdown sound effect on the above basis, play assets/3.mp3 when reaches first countdown you set，play assets/4.mp3 when reaches second countdown you set.
# 在上面的基礎上區分倒計時音效，第一個倒計時播放 assets/3.mp3，第二個倒計時播放 assets/4.mp3
sound=assets/1.mp3|assets/2.mp3*assets/3.mp3|assets/4.mp3
```
+ countdown
> The countdown time, set by `hour:minute:second`. Split multi-time by `,`.
>
> 倒計時，使用 `時:分:秒` 的格式，多個倒計時使用 `,` 分隔。
``` ini
# 20-20-20 Rule 護眼法則
countdown=:20:,::20
```
+ pos
> The position where will rust clock popup.
>
> rust clock 的彈出位置。
``` ini
# popup from right side of screen, 20% top of screen height.
# 在屏幕右側彈出，彈出位置距離屏幕頂部 20% 高度
pos=right,20%
```
+ color
> The color of rust clock. Format by r,g,b or r,g,b,a
>
> rust clock 各個位置的顔色。格式為 r,g,b 或者 r,g,b,a
``` ini
# Color of background.
# 背景顏色
bg_color=207,210,206,200

# Color of border.
# 邊框顏色
border_color=91,105,114

# Color of number background.
# 數字背景顏色
number_bg_color=235,235,235

# Color of number.
# 數字顏色
number_color=0,0,0

# Color of clock circle background.
# 鐘面背景顏色
clock_bg_color=235,235,235
```
+ show_time
> The time that how long the popup will last. Set in milliseconds
>
> 彈出后持續顯示時長，按毫秒計算
``` ini
# Continuous display for 1000 milliseconds after pop-up
# 彈出后持續顯示 1000 毫秒
show_time=1000
```
+ tips
> Text displayed when pop-up, format as same as `sound`
>
> 彈出后顯示的文字，格式同 `sound`，可設置多個
``` ini
# display 'by the grave and thee' when pop-up
# 彈出時顯示 'by the grave and thee'
tips=by the grave and thee
```
+ font_path
> The font path which is used by tips
>
> 彈出文字使用的字體路徑
``` ini
# use font which is located in 'C:/Windows/Fonts/zongyi.TTF'
# 使用位於 'C:/Windows/Fonts/zongyi.TTF' 的字體
font_path=C:/Windows/Fonts/zongyi.TTF
```
+ bg
> The path of background image, 80\*80 for clock, 320\*100 for total background
>
> 背景圖片的路徑，尺寸為 80\*80 時設置為鐘面背景，尺寸為 320\*100 時設置為整體背景
``` ini
bg=assets/bg.png
```
+ init_show
> Show clock after initialization, 0 means disable, 1 means enable
>
> 啓動后立即顯示，0 為禁用顯示，1 為啓用
``` ini
init_show=0
```
+ timezone
> Timezone of clock, from -12 to +12
>
> 時區，從 -12（西12區） 到 +12（東12區）
``` ini
timezone=+9
```
+ time_font
> The font path which is used by time number
>
> 時刻數字使用的字體路徑
``` ini
time_font=C:/Windows/Fonts/zongyi.TTF
```
+ round
> Round the corners of frame, 0 means no
>
> 是否使用圓角邊框，0 為 否
``` ini
round=0
```
