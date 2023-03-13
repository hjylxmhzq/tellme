mod api;
mod token;
mod loading;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use token::{reset_token, get_or_set_token};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).into_iter().collect();

    let mut should_loop = true;

    if args.len() > 0 {
        if args.len() == 1 && args[0] == "--reset" {
            reset_token();
            return;
        }
        should_loop = false;
    }

    let mut rl = DefaultEditor::new().unwrap();

    let text = args.join(" ");

    let mut chat = api::ChatSession::new();

    get_or_set_token().unwrap();

    if !should_loop {
        let resp = loading::wait_with_loading(chat.question(&text)).await.unwrap();
        println!("{}", termimad::inline(&resp.join("\n")));
    }

    while should_loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let resp = loading::wait_with_loading(chat.question(&line)).await.unwrap();
                println!("{}", termimad::inline(&resp.join("\n")));
            }
            Err(ReadlineError::Interrupted) => {
                println!("exit");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
