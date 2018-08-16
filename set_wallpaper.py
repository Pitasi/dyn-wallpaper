"""
    Emulate Mac OS Mojave wallpaper changing transaction based on solar
    position.

    Run with -h flag to see available options.

    This module can be used as standalone, or imported as dependency for other
    uses.
"""
import os
import re
import argparse
import math
import tempfile
import ctypes
from time import sleep
from subprocess import call
from datetime import datetime, timedelta
from dateutil.tz import tzlocal
from PIL import Image
from astral import Astral

def is_windows():
    """
        Checks if the currently running operating system is Windows.
    """
    return os.name == 'nt'


def init_images(folder, set_cmd=False):
    """
        Open all images in the path in the correct order.

        If `set_cmd` is given, it is used as the command for setting the
        initial wallpaper while waiting for the other images to load.
    """
    image_paths = [
        f for f in os.listdir(folder)
        if 'jpeg' in f or 'jpg' in f or 'png' in f
    ]
    # sort images list numerically, not lexicographically, to avoid missing
    # padding 0s problem
    image_paths.sort(key=lambda f: int(re.sub(r'[^0-9]*', "", f)))
    if set_cmd or is_windows():
        full_path = os.path.join(folder, image_paths[int(len(image_paths)/2)])
        update_wallpaper(set_cmd, full_path)

    image_files = [
        Image.open(os.path.join(folder, f)).convert('RGBA')
        for f in image_paths
    ]
    return (image_paths, image_files)


def init_astral(city):
    """
        Compute the sunrise time and the day length (in seconds) for the
        current timezone. This is done everytime the computer is booted so
        should be enough, unless you are that kind of guy that codes for one
        month without powering off your machine.
    """
    sun = Astral()[city].sun()
    dawn = sun['dawn']
    dusk = sun['dusk']
    day_length = (dusk - dawn).total_seconds()

    return (dawn, dusk, day_length)


def blend_images(img1, img2, amount, tmp_path):
    """
        Take two images path, an amount (0-1, 0 means only img1 is shown, 1
        means only img2 is shown), then store the blended image in the OS
        temp directory (e.g. /tmp).
    """
    new_img = Image.blend(img1, img2, amount).convert('RGB')
    new_img.save(tmp_path, 'JPEG', compress_level=1)


def update_wallpaper(cmd, wallpaper_path):
    """
        Use `feh` or a custom command to set the image wallpaper.

        Windows does not use a custom command by default, but if one is given
        then it is used.
    """
    if cmd is None and is_windows():
        SPI_SETDESKWALLPAPER = 20
        # This only works with an absolute path
        abs_wallpaper_path = os.path.abspath(wallpaper_path)
        ctypes.windll.user32.SystemParametersInfoW(
            SPI_SETDESKWALLPAPER, 0, abs_wallpaper_path, 0
        )
    else:
        call(cmd.format(wallpaper_path).split())


def get_current_images(dawn_time, day_length, images, dusk_id):
    """
        Get the couple of images needed for the current time, and the
        percentage elapsed between them.

        Basically a mapping from
            [dawn, ..., now, ..., dusk]
        and
            [0,   ...,   len(images)-1]
    """
    now = datetime.now(tzlocal())
    cursor = (now - dawn_time).total_seconds()
    image_id = dusk_id * cursor / day_length
    if image_id < 0 or image_id > len(images) - 1:
        # out of range, just pick last image
        last_image = images[len(images) - 1]
        return (last_image, last_image, 1)
    img1 = images[math.floor(image_id)]
    img2 = images[math.floor(image_id + 1)]
    amount = image_id - math.floor(image_id)
    return (img1, img2, amount)


def main(args):
    """
        Init sun position-based variables, then start loop.
        Check the argument parser for `args` attributes.
    """
    path = os.path.expanduser(args.folder)
    image_paths, images = init_images(path, set_cmd=args.command)

    if args.demo:
        dawn_time = datetime.now(tzlocal())
        day_length = 2 * 60 # 2 minutes
        dusk_time = dawn_time + timedelta(seconds=day_length)

    while True:
        if not args.demo:
            # Compute real dawn/dusk times
            dawn_time, dusk_time, day_length = init_astral(args.city)

        print('Dawn:', str(dawn_time))
        print('Dusk:', str(dusk_time))
        print('Day length (seconds):', str(day_length))

        blend_images(
            *get_current_images(dawn_time, day_length, images, args.dusk_id),
            args.temp
        )
        update_wallpaper(args.command, args.temp)

        if not args.demo:
            sleep(60 * args.rate)


if __name__ == '__main__':
    # Uses the OS temp path by default
    default_temp_path = os.path.join(tempfile.gettempdir(), 'wallpaper.jpg')

    # Windows does not use commands to set the wallpaper by default, so the
    # argument is hidden. A Windows user can still use a command to set the
    # wallpaper if they wish to do so.
    default_command = None if is_windows() else 'feh --bg-scale {}'
    command_argument_help = argparse.SUPPRESS if is_windows() else 'Command to \
        be executed for setting the wallpaper, use "{}" as a placeholder for \
        the image (default: "feh --bg-scale {}").'

    PARSER = argparse.ArgumentParser(
        description='Live wallpaper based on Sun position, emulating Mac OS \
                     Mojave "dynamic wallpaper".',
        epilog='Source code: https://github.com/Pitasi/dyn-wallpaper',
    )
    PARSER.add_argument(
        'city',
        help='Timezone city to be used when calculating sunset time (i.e. Rome)\
              see https://astral.readthedocs.io/en/latest/#cities for a list of\
              valid names.'
    )
    PARSER.add_argument(
        'folder',
        help='Folder containing the different wallpapers.',
    )
    PARSER.add_argument(
        '-r',
        '--rate',
        help='Refresh rate in minutes (default 10).',
        type=int,
        default=10,
    )
    PARSER.add_argument(
        '-t',
        '--temp',
        help='Temp image file (default {path}).'.format(path=default_temp_path),
        default=default_temp_path,
    )
    PARSER.add_argument(
        '-i',
        '--dusk-id',
        help='Image number of the "dusk" image (default to 13 for the 16-images\
              Apple set).',
        type=int,
        default=13,
    )
    PARSER.add_argument(
        '-c',
        '--command',
        help=command_argument_help,
        default=default_command,
    )
    PARSER.add_argument(
        '-d',
        '--demo',
        help='Run a fast demo of the script, useful to test it.',
        action='store_true',
    )
    ARGS = PARSER.parse_args()
    main(ARGS)
