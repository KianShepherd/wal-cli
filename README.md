# wal-cli
CLI for pywal to control the color backend and wallpaper for my arch setup

## Usage
wal-cli: command list
```
    -c config file to use
    -w wallpaper to switch to
    -b backed to use
    -wi increment wallpaper
    -bi increment backend
```

## Config File
format is [Rusty Object Notation](https://docs.rs/ron)

Default location is ~/.config/wal-rs/config.ron

Wallpapers and backends can be left blank, they will be auto generated.

Backends dir is just a directory with typeless files named for each color backend installed, to be set up manually.


Example file:
```
(
    wallpaper_dir: "~/Pictures/wallpapers/",
    wallpaper: "wall6.jpg",
    wallpapers: [],
    backend_dir: "~/.config/wal-rs/backends/",
    backend: "colorz",
    backends:[],
    post_script:"~/.config/wal/postwal.sh"
)
```