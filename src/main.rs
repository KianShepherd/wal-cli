use clap::{App, Arg};

mod config;
mod helpers;

fn main() {
    let matches = App::new("wal-rs")
        .version("1.0")
        .author("Kian S. <Kianshepherd73@gmail.com>")
        .about("Manage wallpaper and backend for pywal.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("wallpaper")
                .short("w")
                .value_name("WALLPAPER")
                .help("Sets a custom wallpaper file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("backend")
                .short("b")
                .value_name("BACKEND")
                .help("Sets a custom backend")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("inc-backend")
                .short("y")
                .help("Changes to next backend."),
        )
        .arg(
            Arg::with_name("inc-wallpaper")
                .short("t")
                .help("Changes to next wallpaper."),
        )
        .get_matches();

    let config_path = matches
        .value_of("config")
        .unwrap_or("~/.config/wal-cli/config.ron");
    let mut conf = config::Config::from_file(config_path).unwrap();

    let wallpaper = String::from(
        matches
            .value_of("wallpaper")
            .unwrap_or(&conf.get_wallpaper()),
    );
    conf.set_wallpaper(wallpaper).unwrap();
    let backend = String::from(matches.value_of("backend").unwrap_or(&conf.get_backend()));
    conf.set_backend(backend).unwrap();

    if matches.is_present("inc-wallpaper") {
        conf.next_wallpaper().unwrap();
    }

    if matches.is_present("inc-backend") {
        conf.next_backend().unwrap();
    }

    conf.update(config_path).unwrap();
    conf.call();
}
