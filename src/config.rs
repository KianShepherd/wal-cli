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
    wallpapers: Vec<String>,
    backend_dir: String,
    backend: String,
    backends: Vec<String>,
    post_script: String,
}
impl Config {
    pub fn from_file(path: &str) -> io::Result<Config> {
        // println!("reading file");
        let mut file = File::open(helpers::fix_home(path)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        // println!("{}", contents);
        // println!("loading to struct");
        let mut conf: Config = ron::from_str(&contents).unwrap();
        // println!("loading dirs");
        conf.load_dirs()?;
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
        // println!("\nUPDATE:\n{:?}\n", string);
        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all((&string).as_bytes()).unwrap();

        Ok(())
    }

    fn load_dirs(&mut self) -> io::Result<()> {
        let mut wallpapers = fs::read_dir(helpers::fix_home(&self.wallpaper_dir))?
            .map(|res| res.map(|e| String::from(e.path().file_name().unwrap().to_str().unwrap())))
            .collect::<Result<Vec<String>, io::Error>>()?;
        wallpapers.sort();
        // println!("{:?}", wallpapers);
        self.wallpapers = wallpapers;

        let mut backends = fs::read_dir(helpers::fix_home(&self.backend_dir))?
            .map(|res| res.map(|e| String::from(e.path().file_name().unwrap().to_str().unwrap())))
            .collect::<Result<Vec<String>, io::Error>>()?;
        backends.sort();

        // println!("{:?}", wallpapers);
        self.backends = backends;

        Ok(())
    }

    pub fn get_wallpaper(&self) -> String {
        String::from(&self.wallpaper)
    }

    pub fn get_backend(&self) -> String {
        String::from(&self.backend)
    }

    pub fn next_wallpaper(&mut self) -> io::Result<()> {
        let mut _idx = &self.wallpapers.binary_search(&self.wallpaper).unwrap();
        let mut idx = *_idx;
        idx = (idx + 1) % &self.wallpapers.len();
        &self.set_wallpaper(String::from(&self.wallpapers[idx]));
        Ok(())
    }

    pub fn set_wallpaper(&mut self, wallpaper: String) {
        self.wallpaper = wallpaper;
    }

    pub fn set_backend(&mut self, backend: String) {
        self.backend = backend;
    }

    pub fn next_backend(&mut self) -> io::Result<()> {
        let mut _idx = &self.backends.binary_search(&self.backend).unwrap();
        let mut idx = *_idx;
        idx = (idx + 1) % &self.backends.len();
        &self.set_backend(String::from(&self.backends[idx]));
        Ok(())
    }

    pub fn call(&self) {
        let wall = String::from(&self.wallpaper_dir) + &self.wallpaper;

        let retval = format!(
            "wal -i {} --backend {} -o {}",
            wall, &self.backend, &self.post_script
        );
        println!("{}", retval);
        Command::new("sh")
            .arg("-c")
            .arg(retval)
            .output()
            .expect("Failed to run wal command");
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("\nDEBUG:\n{:?}\n", &self);
    }
}
