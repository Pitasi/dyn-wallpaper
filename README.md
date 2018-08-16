# Emulate macOS Mojave's dynamic wallpaper

## What you get
![Dyn Wallpaper](https://i.imgur.com/0rJw98D.gif)

## Why
Yeah, since the first time I looked at Mojave's dynamic wallpaper I fell in
love with them.

That's why I've done a pretty simple script to use their images on a Linux
environment. As I'm using i3wm with `feh`, the default command used to set the
wallpaper, but you may change it in order to work with Gnome, KDE, or whatever
you wish.

This script also supports Windows.

# Requirements
Python 3, and some Python packages you can find in
[requirements.txt](requirements.txt): install them with pip:
```sh
$ pip install -r requirements.txt
```

# How to

First of all, download the source Mojave images (I'm not going to upload them
in this repository in order to avoid copyright problems) from
[the internet](https://www.reddit.com/r/apple/comments/8oz25c/all_16_full_resolution_macos_mojave_dynamic/).

Clone/download this repository to get the `set_wallpaper.py` and install the
[requirements](#requirements).

Extract the images in a folder, i.e. `~/Images/Wallpapers`.

```sh
$ python set_wallpaper.py Rome ~/Images/Wallpapers
```

You can find a list of valid city names [here](https://astral.readthedocs.io/en/latest/#cities).

# Configuration
```sh
$ python set_wallpaper.py -h

usage: set_wallpaper.py [-h] [-r RATE] [-t TEMP] [-i DUSK_ID] [-c COMMAND]
                        city folder

Live wallpaper based on Sun position, emulating Mac OS Mojave "dynamic
wallpaper".

positional arguments:
  city                  Timezone city to be used when calculating sunset time
                        (i.e. Rome) see
                        https://astral.readthedocs.io/en/latest/#cities for a
                        list of valid names.
  folder                Folder containing the different wallpapers.

optional arguments:
  -h, --help            show this help message and exit
  -r RATE, --rate RATE  Refresh rate in minutes (default 10).
  -t TEMP, --temp TEMP  Temp image file (default /tmp/wallpaper.png).
  -i DUSK_ID, --dusk-id DUSK_ID
                        Image number of the "dusk" image (default to 13 for
                        the 16-images Apple set).
  -c COMMAND, --command COMMAND
                        Command to be executed for setting the wallpaper, use
                        "{}" as a placeholder for the image (default: "feh
                        --bg-scale {}").

```

# Examples for different DE
If you use a different Desktop Environment and want to add the command to this
list just fill an issue or send a PR!

## Default (feh and Windows)
```sh
$ python set_wallpaper.py Rome ~/Images/Wallpapers
```

## Gnome
```sh
$ python set_wallpaper.py Rome ~/Images/Wallpapers -c "gsettings set org.gnome.desktop.background picture-uri {}"
```

## Plasma / KDE 5
I didn't find a good way to do it for now. Sorry.



# Contributing

Issues and PRs are welcome. Go ahead!
