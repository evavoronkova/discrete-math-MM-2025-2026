use crate::graph::Graph;
use crate::graph::traversal::bfs_internal;
use crate::landmarks::basic_landmarks::LandmarkBasic;
use crate::landmarks::bfs_landmarks::LandmarkBFS;
use crossterm::{QueueableCommand, cursor::*, event::*, execute, style::*, terminal::*};
use rand::seq::SliceRandom;
use std::io::{Stdout, Write, stdout};
use std::time::{Duration, Instant};
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

const MENU_ITEMS: &[&str] = &[
    "   Query distance between two vertices  ",
    "   Accuracy benchmark (vs exact BFS)    ",
    "   Speed benchmark (landmarks only)     ",
    "   Change number of landmarks           ",
    "   Show landmark information            ",
    "   Back to main menu                    ",
];

#[derive(Clone, Copy)]
struct BenchResult {
    exact: Option<usize>,
    basic: Option<usize>,
    bfs: Option<usize>,
    exact_time: Duration,
    basic_time: Duration,
    bfs_time: Duration,
}

pub fn run_landmark_interactive(graph: &Graph, num_landmarks: usize) {
    let _guard = TerminalGuard::new();
    let mut stdout = stdout();
    let basic = match LandmarkBasic::new(graph, num_landmarks) {
        Some(b) => b,
        None => {
            let _ = writeln!(stdout, "  Error: could not create landmarks (graph too small?).");
            let _ = stdout.flush();
            std::thread::sleep(Duration::from_secs(2));
            return;
        }
    };
    let bfs = LandmarkBFS::new(graph, num_landmarks);
    let mut cached_bench: Option<Vec<BenchResult>> = None;
    let mut sel = 0usize;
    let mut n_landmarks = num_landmarks;
    let mut cur_basic = basic;
    let mut cur_bfs = bfs;
    loop {
        let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        let _ = execute!(stdout, SetForegroundColor(Color::Cyan));
        let _ = writeln!(
            stdout,
            "╔{}╗",
            "═".repeat(58)
        );
        let _ = writeln!(
            stdout,
            "║{:^60}║",
            "     Landmark Distance Estimator"
        );
        let _ = writeln!(
            stdout,
            "╚{}╝",
            "═".repeat(58)
        );
        let _ = execute!(stdout, ResetColor);
        let _ = writeln!(
            stdout,
            "  Graph: {} vertices, {} edges  |  Type: {}  |  Landmarks: {}",
            graph.num_vertices(),
            graph.num_edges(),
            graph.kind(),
            n_landmarks,
        );
        let _ = writeln!(stdout);
        for (i, item) in MENU_ITEMS.iter().enumerate() {
            if i == sel {
                let _ = execute!(
                    stdout,
                    SetBackgroundColor(Color::Blue),
                    SetForegroundColor(Color::White),
                    Print(format!("  ❯ {}", item)),
                    ResetColor,
                );
            } else {
                let _ = writeln!(stdout, "    {}", item);
                continue;
            }
            let _ = writeln!(stdout);
        }
        let _ = writeln!(stdout);
        let _ = execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("  ↑↓ navigate · Enter/1-6 select · q/Esc back"),
            ResetColor,
        );
        let _ = stdout.flush();
        match read().unwrap() {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Up | KeyCode::Char('k') => sel = sel.saturating_sub(1),
                KeyCode::Down | KeyCode::Char('j') => {
                    if sel < MENU_ITEMS.len() - 1 {
                        sel += 1;
                    }
                }
                KeyCode::Char('1') => {
                    distance_query(&mut stdout, graph, &cur_basic, &cur_bfs);
                }
                KeyCode::Char('2') => {
                    accuracy_benchmark(&mut stdout, graph, &cur_basic, &cur_bfs, &mut cached_bench);
                }
                KeyCode::Char('3') => {
                    speed_benchmark(&mut stdout, graph, &cur_basic, &cur_bfs, &cached_bench);
                }
                KeyCode::Char('4') => {
                    if let Some(n) = change_landmarks(&mut stdout, graph, n_landmarks) {
                        if n != n_landmarks {
                            n_landmarks = n;
                            if let Some(b) = LandmarkBasic::new(graph, n_landmarks) {
                                cur_basic = b;
                            }
                            cur_bfs = LandmarkBFS::new(graph, n_landmarks);
                            cached_bench = None;
                        }
                    }
                }
                KeyCode::Char('5') => {
                    landmark_info(&mut stdout, graph, &cur_basic, &cur_bfs);
                } 
                KeyCode::Char('6') | KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Enter => match sel {
                    0 => distance_query(&mut stdout, graph, &cur_basic, &cur_bfs),
                    1 => accuracy_benchmark(&mut stdout, graph, &cur_basic, &cur_bfs, &mut cached_bench),
                    2 => speed_benchmark(&mut stdout, graph, &cur_basic, &cur_bfs, &cached_bench),
                    3 => {
                        if let Some(n) = change_landmarks(&mut stdout, graph, n_landmarks) {
                            if n != n_landmarks {
                                n_landmarks = n;
                                if let Some(b) = LandmarkBasic::new(graph, n_landmarks) {
                                    cur_basic = b;
                                }
                                cur_bfs = LandmarkBFS::new(graph, n_landmarks);
                                cached_bench = None;
                            }
                        }
                    }
                    4 => landmark_info(&mut stdout, graph, &cur_basic, &cur_bfs),
                    5 => break,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}