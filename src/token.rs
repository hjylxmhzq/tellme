use std::{
    fs,
    io::{Read, Write},
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

pub fn set_token(token: &str) {
    let home_dir = home::home_dir().unwrap();
    let config_file = home_dir.join(".tellme");
    let exists = fs::try_exists(&config_file).unwrap();
    if exists {
        fs::remove_file(&config_file).unwrap();
    }
    let mut f = fs::File::create(&config_file).unwrap();
    println!("Your api token is saved at: {}", config_file.to_string_lossy().to_string());
    f.write_all(token.as_bytes()).unwrap();
}

pub fn reset_token() {
  let home_dir = home::home_dir().unwrap();
  let config_file = home_dir.join(".tellme");
  let exists = fs::try_exists(&config_file).unwrap();
  if exists {
      fs::remove_file(&config_file).unwrap();
  }
  println!("config is reset");
}

pub fn get_or_set_token() -> Result<String, std::io::Error> {
    let home_dir = home::home_dir().unwrap();
    let mut token = String::new();
    let config_file = home_dir.join(".tellme");
    let f = fs::File::open(&config_file);
    if let Ok(mut f) = f {
      f.read_to_string(&mut token)?;
    } else {
      print!("Token is not set, please set your token before starting\n(for more information, please check: https://openai.com/blog/introducing-chatgpt-and-whisper-apis)\n> ");
      std::io::stdout().flush().unwrap();
      std::io::stdin().read_line(&mut token).unwrap();
      set_token(&token.trim());
    }
    return Ok(token);
}

lazy_static! {
    pub static ref TOKEN: Arc<Mutex<String>> = {
        let token = get_or_set_token().unwrap();
        Arc::new(Mutex::new(token.to_owned()))
    };
}
