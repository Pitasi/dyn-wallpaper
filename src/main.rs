extern crate strfmt;
extern crate clap;
extern crate chrono;
extern crate indicatif;
extern crate sun_times;
extern crate alphanumeric_sort;
#[cfg(windows)] extern crate winapi;
mod cities;
mod images;

use std::env::temp_dir;
use std::fs::{read_dir, create_dir};
use std::path::Path;
use std::collections::HashMap;
use std::process::exit;
use std::thread::sleep;
use clap::{App, Arg};
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};
use sun_times::sun_times;
use self::cities::{City, get_cities, escape};
use self::images::blend_images;

macro_rules! debug {
    ($($args:expr),*) => {
        if cfg!(debug_assertions) {
            println!($($args),*);
        }
    }
}

fn main() {
    debug!("Running in DEBUG mode");

    let matches = App::new("dyn-wallpaper")
        .version("0.1.0")
        .author("Antonio Pitasi <pitasi.antonio@gmail.com>")
        .about("Dynamic wallpapers - Apple style.")
        .arg(Arg::with_name("city")
            .help("Name of a city for sunrise/sunset times reference")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("images")
             .help("Path of the folder containing image set")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("command")
             .help("Specify command to be executed to set wallpaper (Linux only)")
             .takes_value(false))
        .get_matches();
    let city_name = matches.value_of("city").unwrap_or("__placeholder");
    let command = matches.value_of("command").unwrap_or("feh --bg-scale {path}");
    let images_folder = Path::new(
        matches.value_of("images").expect("invalid `images` parameter")
    );
    if !images_folder.is_dir() {
        println!("\"{}\" is not a valid folder.", images_folder.display());
        exit(-1);
    }
    let mut output_file = temp_dir();
    let _ = create_dir(&output_file);
    output_file.push("dyn_wallpaper.jpg");

    // get a hashmap containing all the cities
    let cities_map = get_cities();
    let selected_city = cities_map.get(&escape(&String::from(city_name)));

    match selected_city {
        Some(city) => {
            run_loop(&command.to_string(), city, images_folder, &output_file);
        },
        None => {
            print_cities(&cities_map);
            println!(
                "\nInvalid city '{}'. Please select one from the list.",
                city_name
            );
            exit(1)
        }
    }
}

// Pretty-print a list of all the city names from the HashMap
fn print_cities(cities_map: &HashMap<String, City>) {
    let mut cities_pretty_list = cities_map.values()
        .map(|city| &city.name)
        .collect::<Vec<&String>>() ;
    cities_pretty_list.sort_by(|a, b| (*a).cmp(*b));
    let (last, list) = cities_pretty_list.split_last().unwrap();
    for name in list {
        print!("{}, ", name);
    }
    println!("{}.", last);
}

fn run_loop(command: &String, city: &City, images_folder: &Path, output_file: &Path) {
    let image_paths = read_dir(images_folder).unwrap();
    let mut paths = Vec::new();
    for image in image_paths {
        let path = image.unwrap().path();
        let ext = path.extension().unwrap();
        if ext == "jpg" || ext == "jpeg" || ext == "png" {
            paths.push(path.clone());
        }
    }

    println!("{} images found in {}", paths.len(), images_folder.display());
    alphanumeric_sort::sort_path_slice(&mut paths);

    println!("Output file: \"{}\"", output_file.display());
    println!("Selected city: {}", city.name);

    loop {
        // This nested loop allows to recompute the sunrise/sunset times if the
        // day changed (i.e. after midnight)
        let (sunrise, sunset) = get_sun_times(city);
        while Utc::now().day() == sunrise.day() {
            let now = Utc::now();
            println!("Now: {}", now);

            let sunset_image_id = 13;  // the image ID to be used as sunset
            let cursor = (now - sunrise).num_seconds();
            let day_length = (sunset - sunrise).num_seconds();

            let normalized_cursor = sunset_image_id as f64 * cursor as f64 / day_length as f64;
            let image_id = normalized_cursor as usize;
            let blend_amount = normalized_cursor.fract();
            debug!(
                "sunset_image_id={}, cursor={}, day_length={}, image_id={}/{}",
                sunset_image_id,
                cursor,
                day_length,
                image_id,
                image_id + 1
            );

            if normalized_cursor < 0. {
                // using `normalized_cursor` instead of `image_id` here because
                // of type usize -> can't be less than 0
                debug!("Lower bound, using first image");
                set_image(command, paths.first().unwrap());
            } else if image_id > paths.len() - 2 {
                debug!("Upper bound, using last image");
                set_image(command, paths.last().unwrap());
            } else {
                let img1 = paths.get(image_id).unwrap();
                let img2 = paths.get(image_id + 1).unwrap();

                debug!("Starting blending process - img{}+img{}, amount: {}", image_id, image_id+1, blend_amount);
                blend_images(&img1, &img2, output_file, blend_amount);
                debug!("done");

                set_image(command, output_file);
            }

            let delay = Duration::seconds(
                if cfg!(debug_assertions) { 3 } else { 120 }
            );
            sleep(delay.to_std().unwrap());
        }
    }
}

// Get sun{rise,set} times for the selected City
fn get_sun_times(city: &City) -> (DateTime<Utc>, DateTime<Utc>) {
    if cfg!(debug_assertions) {
        // if debug mode, set sunrise to `now` and sunset in two minutes
        (Utc::now(), Utc::now() + Duration::minutes(2))
    } else {
        let today = Utc::today();
        let (mut sunrise, mut sunset) = sun_times(
            today,
            city.latitude,
            city.longitude,
            city.elevation
        );

        // Not sure why: the sun_times library sometimes returns a wrong day of
        // the month. For example using "Rome". Let's fix it here:
        if sunrise.day() != today.day() {
            sunrise = sunrise.with_day(today.day()).unwrap();
        }
        if sunset.day() != today.day() {
            sunset = sunset.with_day(today.day()).unwrap();
        }

        (sunrise, sunset)
    }
}

#[cfg(not(any(macos, windows)))]
fn set_image(command: &String, image: &Path) {
    use strfmt::strfmt;
    use std::process::Command;

    let abs_path = image.canonicalize().unwrap();

    // Make HashMap for formatting the command string
    let mut vars = HashMap::new();
    vars.insert("path".to_string(), abs_path.to_str().unwrap());
    debug!("Running {}", command);

    Command::new("sh")
        .arg("-c")
        .arg(strfmt(command, &vars).expect("Invalid command specified."))
        .output()
        .expect("failed to execute process");
}

#[cfg(macos)]
fn set_image(_command: &String, image: &Path) {
    use std::process::Command;

    let abs_path = image.canonicalize().unwrap();
    Command::new("sh")
        .arg("-c")
        .arg(
            format!(
                "osascript -e 'tell application \"Finder\" to set desktop picture to POSIX file \"{}\"'",
                abs_path.to_str().unwrap()
            )
        )
        .output()
        .expect("failed to execute process");
}

#[cfg(windows)]
fn set_image(_command: &String, image: &Path) {
    let abs_path = image.canonicalize().unwrap();
    unsafe {
        // This was tested on Windows 10
        use winapi::um::winuser::SystemParametersInfoW;
        use std::ffi::{OsStr, c_void};
        use std::os::windows::ffi::OsStrExt;
        use std::iter::once;
        let path: Vec<u16> = OsStr::new(abs_path.to_str().unwrap()).encode_wide().chain(once(0)).collect();
        SystemParametersInfoW(20, 0, path.as_ptr() as *mut c_void, 0)
    };
}
