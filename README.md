# Emulate macOS Mojave's dynamic wallpaper

## What you get
![Dyn Wallpaper](http://i.freegifmaker.me/1/5/3/2/8/0/15328097891754468.gif?1532809797)

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

For a list of options just use:
```sh
$ python set_wallpaper.py -h
```

# Contributing

Issues and PRs are welcome. Go ahead!
