# Emulate macOS Mojave's dynamic wallpaper

## What you get

![Dyn Wallpaper](https://i.imgur.com/0rJw98D.gif)

## Why

Yeah, since the first time I looked at Mojave's dynamic wallpaper I fell in
love with them.

I wrote this little utility to periodically change wallpaper while blending the
images together based on the current time, relative to sunrise/sunset.

I tested this on **Arch Linux** and **Windows 10**.
*Mac OS support has not been tested yet.*


# Requirements

Download the source Mojave images (I'm not going to upload them in this
repository in order to avoid copyright problems) from
[the internet](https://www.reddit.com/r/apple/comments/8oz25c/all_16_full_resolution_macos_mojave_dynamic/).

Extract the images in a folder, i.e. `~/Images/Wallpapers`.


# How to use this

From the [release page](https://github.com/Pitasi/dyn-wallpaper/releases)
download the latest binary for your platform (Linux, Mac OS, or Windows).

You can specify which command should be executed to change your wallpaper. By default, dyn-wallpaper uses [feh](https://wiki.archlinux.org/index.php/feh), for Gnome that's not ideal and I'm providing an example below.

Run it from a terminal (or command prompt if you prefer):
```sh
# using feh
$ ./dyn-wallpaper "Rome" "~/Images/Wallpapers"

# using Gnome
$ ./dyn-wallpaper "Rome" "~/Images/Wallpapers" "gsettings set org.gnome.desktop.background picture-uri {path}"

# using contractor from Elementary OS 5.1.2 "Hera"
$ ./dyn-wallpaper "Rome" "~/Images/Wallpapers" "/usr/lib/x86_64-linux-gnu/io.elementary.contract.set-wallpaper {path}"

# using swaybg for swaywm
$ ./dyn-wallpaper "Rome" "~/Images/Wallpapers" "swaybg -o '*' -i {path} -m fill"

# please submit PRs with other DE if you can make them work!
```

You can find a list of valid city names [here](https://astral.readthedocs.io/en/latest/#cities).


# Contributing

Issues and PRs are welcome. Go ahead!

You're gonna need Rust (with Cargo) to easily edit and build the sources.


## Build the project
```sh
$ git clone https://github.com/Pitasi/dyn-wallpaper

# Test it
$ cargo run "Rome" "~/Images/Wallpapers"

# Build it
$ cargo build --release
```
