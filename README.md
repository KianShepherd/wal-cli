# wal-cli
CLI for pywal to control the color backend and wallpaper for my arch setup

## Usage
wal-cli: command list
```
    -c <FILE> config file to use
    -w <FILE> wallpaper to switch to
    -b <FILE> backend to use
    -t increment wallpaper
    -y increment backend
```

## Config File
Format is [Rusty Object Notation](https://docs.rs/ron)  
Default location is ~/.config/wal-cli/config.ron  

Example file: config.ron
```
(
    wallpaper_dir: "~/Pictures/wallpapers/",
    wallpaper: "wall6.jpg",
    backend_dir: "~/.config/wal-cli/backends/",
    backend: "colorz",
    post_script:"~/.config/wal/postwal.sh"
)
```
backend_dir is just a directory with typeless files named for each color backend installed, to be set up manually.  
```
backend_dir:
    -> colorz
    -> wal
    -> colorthiefs
```

## Scripts
Also in this repository are 2 scripts for use with rofi to switch wallpaper and backend, the scripts are modified versions of the themeswitch script by [ilsenatorov](https://github.com/ilsenatorov/dotfiles).