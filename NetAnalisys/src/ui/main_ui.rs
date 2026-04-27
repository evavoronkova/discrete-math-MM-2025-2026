use crossterm::{QueueableCommand, cursor::*, event::*, execute, style::*, terminal::*};
use rand::Rng;
use std::fs;
use std::io::{Stdout, Write, stdout};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::{thread, time::Duration};

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> Self {
        enable_raw_mode().unwrap();
        execute!(stdout(), Hide).unwrap();
        TerminalGuard
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        let _ = execute!(stdout(), Show);
    }
}

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
    let _guard = TerminalGuard::new();
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

fn collect_files_recursive(dir: &std::path::Path, files: &mut Vec<String>, prefix: &str) {
    let skip_names = [
        "target",
        "src",
        ".git",
        "node_modules",
        ".vscode",
        ".idea",
        "target",
    ];
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if path.is_dir() {
            if !skip_names.contains(&name.as_str()) {
                let new_prefix = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{}/{}", prefix, name)
                };
                collect_files_recursive(&path, files, &new_prefix);
            }
        } else if path.is_file() {
            let full_path = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", prefix, name)
            };
            files.push(full_path);
        }
    }
}

fn choose_file(stdout: &mut std::io::Stdout) -> Option<String> {
    let mut files: Vec<String> = Vec::new();
    collect_files_recursive(std::path::Path::new("."), &mut files, "");

    if files.is_empty() {
        return None;
    }

    files.sort();

    let mut selected = 0usize;

    loop {
        let (width, height) = size().unwrap();

        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        write!(stdout, "Select file:\n").unwrap();

        let window_size = (height as usize).saturating_sub(3);

        let start = selected.saturating_sub(window_size / 2);
        let end = (start + window_size).min(files.len());

        for (i, file) in files[start..end].iter().enumerate() {
            let real_index = start + i;

            execute!(stdout, MoveTo(0, (i + 1) as u16)).unwrap();

            if real_index == selected {
                execute!(
                    stdout,
                    SetBackgroundColor(Color::Blue),
                    SetForegroundColor(Color::White)
                )
                .unwrap();

                write!(stdout, "> {}", file).unwrap();

                execute!(stdout, ResetColor).unwrap();
            } else {
                write!(stdout, "  {}", file).unwrap();
            }
        }

        execute!(stdout, MoveTo(0, height - 1)).unwrap();
        write!(
            stdout,
            "Selected: {} ({}/{}) | q - exit",
            files[selected],
            selected + 1,
            files.len()
        )
        .unwrap();

        stdout.flush().unwrap();

        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up => {
                    selected = selected.saturating_sub(1);
                }
                KeyCode::Down => {
                    if selected < files.len() - 1 {
                        selected += 1;
                    }
                }

                KeyCode::PageDown => {
                    selected = (selected + 10).min(files.len() - 1);
                }
                KeyCode::PageUp => {
                    selected = selected.saturating_sub(10);
                }

                KeyCode::Enter => {
                    return Some(files[selected].clone());
                }

                KeyCode::Esc | KeyCode::Char('q') => {
                    return None;
                }

                _ => {}
            }
        }
    }
}

fn draw_cat_loading_frame(
    stdout: &mut Stdout,
    start_x: u16,
    start_y: u16,
    rng: &mut rand::rngs::ThreadRng,
    tail_state: usize,
    time_start: Option<std::time::Instant>,
) {
    let tail = match tail_state % 4 {
        0 => "\\",
        1 => "~\\",
        2 => "~~\\",
        _ => "~~~\\",
    };

    let eyes = if rng.gen_bool(0.15) { "-.-" } else { "o.o" };

    let scene = format!(
        "     {0}      {1}        {2}      {3}\n\
  {4}         {5}      {6}        {7}\n\
\n\
                 Waiting for graph magic for {11}...\n\
\n\
        /\\_/\\\\\n\
       ( {8} )      {9}\n\
       /  ^  \\\n\
      /_/|_|__\\\\\n\
         /   {10}\n\
\n\
  ───────────────────────────────",
        random_star(rng),
        random_star(rng),
        random_star(rng),
        random_star(rng),
        random_star(rng),
        random_star(rng),
        random_star(rng),
        random_star(rng),
        eyes,
        random_star(rng),
        tail,
        time_start
            .map(|t| format!("{:.1?}", t.elapsed()))
            .unwrap_or_else(|| "0s".to_string())
    );

    stdout
        .queue(MoveTo(start_x, start_y))
        .unwrap()
        .queue(Clear(ClearType::FromCursorDown))
        .unwrap();

    for (line_index, line) in scene.lines().enumerate() {
        stdout
            .queue(MoveTo(start_x, start_y + line_index as u16))
            .unwrap()
            .queue(Print(line))
            .unwrap();
    }

    stdout.flush().unwrap();
}

fn random_star(rng: &mut rand::rngs::ThreadRng) -> &'static str {
    const STARS: [&str; 4] = ["✦", "✧", ".", " "];
    STARS[rng.gen_range(0..STARS.len())]
}

pub fn spawn_cat_loading_animation(
    start_x: u16,
    start_y: u16,
    time_start: Option<std::time::Instant>,
) -> (Arc<AtomicBool>, thread::JoinHandle<()>) {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let thread_flag = Arc::clone(&stop_flag);

    let handle = thread::spawn(move || {
        let mut stdout = stdout();
        let mut rng = rand::thread_rng();
        let mut tail_state = 0usize;

        while !thread_flag.load(Ordering::Relaxed) {
            draw_cat_loading_frame(
                &mut stdout,
                start_x,
                start_y,
                &mut rng,
                tail_state,
                time_start,
            );
            tail_state = (tail_state + 1) % 4;
            thread::sleep(Duration::from_millis(360));
        }

        let _ = execute!(
            stdout,
            MoveTo(start_x, start_y),
            Clear(ClearType::FromCursorDown)
        );
    });

    (stop_flag, handle)
}
