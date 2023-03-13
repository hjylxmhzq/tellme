use std::{future::Future, time::Duration, io::Write};

use tokio::time::sleep;

pub async fn wait_with_loading<T>(fut: impl Future<Output = T>) -> T {
  
  let mut repeat = 0;
  let max_dot = 6;

  let clean_line = || {
    let s = " ".repeat(max_dot);
    print!("{s}\r");
    std::io::stdout().flush().unwrap();
  };

  tokio::pin!(fut);
  
  loop {
    tokio::select! {
      _ = sleep(Duration::from_millis(300)) => {
        let s = "".to_owned() + &".".repeat(repeat) + &" ".repeat(max_dot - repeat - 1) + "\r";
        print!("{s}");
        std::io::stdout().flush().unwrap();
        repeat = (repeat + 1) % max_dot;
      }
      result = &mut fut => {
        clean_line();
        return result
      }
    }
  }
}