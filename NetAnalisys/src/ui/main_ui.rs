use crossterm::{cursor::*, event::*, execute, style::*, terminal::*};
use std::fs;
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

pub fn run_ui_and_file_parcing_menu() -> Option<String> {
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
                    let return_value = match choose_file(&mut stdout) {
                        Some(file) => return Some(file),
                        None => None,
                    };
                    return return_value;
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
                        let return_value = match choose_file(&mut stdout) {
                            Some(file) => return Some(file),
                            None => None,
                        };
                        return return_value;
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
    execute!(stdout, Show).unwrap();
    println!();
    None
}

fn print_file_choosing_greeting(stdout: &mut std::io::Stdout) {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

    execute!(stdout, MoveTo(0, 0)).unwrap();
    write!(stdout, "Welcome to the Network Analysis Tool!").unwrap();

    execute!(stdout, MoveTo(0, 1)).unwrap();
    write!(stdout, "Please select file to analyse:").unwrap();

    execute!(stdout, MoveTo(0, 2)).unwrap();
    write!(stdout, "Press ↑ ↓ to select file").unwrap();
}

fn choose_file(stdout: &mut std::io::Stdout) -> Option<String> {
    enable_raw_mode().unwrap();

    let mut files: Vec<String> = fs::read_dir(".")
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                Some(path.file_name()?.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    if files.is_empty() {
        return None;
    }

    files.sort();

    let mut selected = 0;

    loop {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        write!(stdout, "Select file:\n").unwrap();

        for (i, file) in files.iter().enumerate() {
            execute!(stdout, MoveTo(0, (i + 2) as u16)).unwrap();

            if i == selected {
                execute!(stdout, SetBackgroundColor(Color::DarkGrey)).unwrap();
                write!(stdout, "> {}", file).unwrap();
                execute!(stdout, ResetColor).unwrap();
            } else {
                write!(stdout, "  {}", file).unwrap();
            }
        }

        stdout.flush().unwrap();

        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < files.len() - 1 {
                        selected += 1;
                    }
                }

                KeyCode::Enter => {
                    let chosen = files[selected].clone();
                    disable_raw_mode().unwrap();
                    return Some(chosen);
                }

                KeyCode::Esc | KeyCode::Char('q') => {
                    disable_raw_mode().unwrap();
                    return None;
                }

                _ => {}
            }
        }
    }
}
