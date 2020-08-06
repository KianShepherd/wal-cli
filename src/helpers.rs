use regex::Regex;
use std::env;

#[allow(deprecated)]
pub fn fix_home(path: &str) -> String {
    let home_dir_buf = env::home_dir().unwrap();
    let home_dir = home_dir_buf.to_str().unwrap();
    let re = Regex::new(r"~").unwrap();
    let res = re.replace(path, home_dir);
    String::from(res)
}
