use crate::helpers;
use ron;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::{prelude::*, Read, Seek, SeekFrom};
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    wallpaper_dir: String,
    wallpaper: String,
    backend_dir: String,
    backend: String,
    post_script: String,
}
impl Config {
    pub fn from_file(path: &str) -> io::Result<Config> {
        let mut file = File::open(helpers::fix_home(path)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let conf: Config = ron::from_str(&contents).unwrap();

        Ok(conf)
    }

    pub fn update(&self, path: &str) -> io::Result<()> {
        let string = ron::to_string(&self).unwrap();
        fs::remove_file(helpers::fix_home(path))?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(helpers::fix_home(path))
            .unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all((&string).as_bytes()).unwrap();

        Ok(())
    }

    pub fn get_wallpaper(&self) -> String {
        String::from(&self.wallpaper)
    }

    pub fn get_backend(&self) -> String {
        String::from(&self.backend)
    }

    pub fn next_wallpaper(&mut self) -> io::Result<()> {
        let mut wallpapers = fs::read_dir(helpers::fix_home(&self.wallpaper_dir))?
            .map(|res| res.map(|e| String::from(e.path().file_name().unwrap().to_str().unwrap())))
            .collect::<Result<Vec<String>, io::Error>>()?;
        wallpapers.sort();
        let mut idx = wallpapers.binary_search(&self.wallpaper).unwrap();
        idx = (idx + 1) % wallpapers.len();
        &self.set_wallpaper(String::from(&wallpapers[idx]));

        Ok(())
    }

    pub fn set_wallpaper(&mut self, wallpaper: String) -> io::Result<()> {
        let mut wallpapers = fs::read_dir(helpers::fix_home(&self.wallpaper_dir))?
            .map(|res| res.map(|e| String::from(e.path().file_name().unwrap().to_str().unwrap())))
            .collect::<Result<Vec<String>, io::Error>>()?;
        wallpapers.sort();

        if wallpapers.contains(&wallpaper) {
            self.wallpaper = wallpaper;

            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "wallpaper not found.",
            ))
        }
    }

    pub fn set_backend(&mut self, backend: String) -> io::Result<()> {
        let mut backends = fs::read_dir(helpers::fix_home(&self.backend_dir))?
            .map(|res| res.map(|e| String::from(e.path().file_name().unwrap().to_str().unwrap())))
            .collect::<Result<Vec<String>, io::Error>>()?;
        backends.sort();

        if backends.contains(&backend) {
            self.backend = backend;

            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "backend not found.",
            ))
        }
    }

    pub fn next_backend(&mut self) -> io::Result<()> {
        let mut backends = fs::read_dir(helpers::fix_home(&self.backend_dir))?
            .map(|res| res.map(|e| String::from(e.path().file_name().unwrap().to_str().unwrap())))
            .collect::<Result<Vec<String>, io::Error>>()?;
        backends.sort();
        let mut idx = backends.binary_search(&self.backend).unwrap();
        idx = (idx + 1) % backends.len();
        &self.set_backend(String::from(&backends[idx]));

        Ok(())
    }

    pub fn call(&self) {
        let wall = String::from(&self.wallpaper_dir) + &self.wallpaper;
        let command = format!(
            "wal -i {} --backend {} -o {}",
            wall, &self.backend, &self.post_script
        );
        println!("{}", command);
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to run wal command");
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("\nDEBUG:\n{:?}\n", &self);
    }
}
