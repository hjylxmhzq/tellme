mod api;
mod formatter;
mod loading;
mod token;

use formatter::format_print;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use token::{get_or_set_token, reset_token};

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
        let resp = loading::wait_with_loading(chat.question(&text))
            .await
            .unwrap();
        println!("{}", format_print(&resp.join("\n")));
    }

    while should_loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let resp = loading::wait_with_loading(chat.question(&line)).await;
                match resp {
                    Ok(resp) => {
                        if resp.len() == 0 {
                            println!("No available answer, please try again.");
                        } else {
                            println!("{}", format_print(&resp.join("\n")))
                        }
                    }
                    Err(err) => println!("Error: {}", err.to_string()),
                }
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
