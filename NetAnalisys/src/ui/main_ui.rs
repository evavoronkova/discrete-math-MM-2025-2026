use crossterm::{cursor::*, event::*, execute, style::*, terminal::*};
use std::io::{Write, stdout};

fn print_greeting(stdout: &mut std::io::Stdout) {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

    execute!(stdout, MoveTo(0, 0)).unwrap();
    write!(stdout, "Welcome to the Network Analysis Tool!").unwrap();

    execute!(stdout, MoveTo(0, 1)).unwrap();
    write!(stdout, "Please select an option:").unwrap();

    execute!(stdout, MoveTo(0, 2)).unwrap();
    write!(
        stdout,
        "Press ↑ ↓ to change value or just type number, then press Enter"
    )
    .unwrap();
}

pub fn run_ui() {
    enable_raw_mode().unwrap();
    execute!(stdout(), Hide).unwrap();
    let mut stdout = stdout();
    let mut counter = 1;

    loop {
        print_greeting(&mut stdout);

        match counter {
            1 => {
                execute!(stdout, MoveTo(0, 4), SetBackgroundColor(Color::DarkGrey)).unwrap();
                write!(stdout, "1. Load graph from file").unwrap();

                execute!(stdout, ResetColor).unwrap();

                execute!(stdout, MoveTo(0, 5)).unwrap();
                write!(stdout, "2. Exit").unwrap();
            }
            2 => {
                execute!(stdout, MoveTo(0, 4)).unwrap();
                write!(stdout, "1. Load graph from file").unwrap();

                execute!(stdout, MoveTo(0, 5), SetBackgroundColor(Color::DarkGrey)).unwrap();
                write!(stdout, "2. Exit").unwrap();

                execute!(stdout, ResetColor).unwrap();
            }
            _ => {}
        }

        stdout.flush().unwrap();

        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Char('1') => {
                    todo!("АРОМАТНЫЙ ХУЙ")
                }
                KeyCode::Char('2') => {
                    break;
                }
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    break;
                }

                KeyCode::Up => {
                    if counter > 1 {
                        counter -= 1;
                    }
                }
                KeyCode::Down => {
                    if counter < 2 {
                        counter += 1;
                    }
                }

                KeyCode::Enter => match counter {
                    1 => {
                        todo!("АРОМАТНЫЙ ХУЙ");
                        println!("Loading graph...");
                    }
                    2 => break,
                    _ => {}
                },

                KeyCode::Char('q') | KeyCode::Char('Q') => break,

                _ => {}
            }
        }
    }

    disable_raw_mode().unwrap();
    println!();
}
