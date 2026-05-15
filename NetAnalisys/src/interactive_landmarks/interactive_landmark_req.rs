use crate::graph::Graph;
use crate::graph::traversal::bfs_internal;
use crate::landmarks::basic_landmarks::LandmarkBasic;
use crate::landmarks::bfs_landmarks::LandmarkBFS;
use crossterm::{QueueableCommand, cursor::*, event::*, execute, style::*, terminal::*};
use rand::seq::SliceRandom;
use std::io::{Stdout, Write, stdout};
use std::time::{Duration, Instant};
struct TerminalGuard;
use std::sync::Arc;

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

pub async fn run_landmark_interactive(graph: Arc<Graph>, num_landmarks: usize) {
    let _guard = TerminalGuard::new();
    let mut stdout = stdout();
    let basic = match LandmarkBasic::new(&graph, num_landmarks) {
        Some(b) => b,
        None => {
            let _ = writeln!(
                stdout,
                "  Error: could not create landmarks (&graph too small?)."
            );
            let _ = stdout.flush();
            std::thread::sleep(Duration::from_secs(2));
            return;
        }
    };
    let bfs = LandmarkBFS::new(&graph, num_landmarks);
    let mut cached_bench: Option<Vec<BenchResult>> = None;
    let mut sel = 0usize;
    let mut n_landmarks = num_landmarks;
    let mut cur_basic = Arc::new(basic);
    let mut cur_bfs = Arc::new(bfs);
    loop {
        let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        let _ = execute!(stdout, SetForegroundColor(Color::Cyan));
        let _ = writeln!(stdout, "╔{}╗", "═".repeat(58));
        let _ = writeln!(stdout, "║{:^60}║", "     Landmark Distance Estimator");
        let _ = writeln!(stdout, "╚{}╝", "═".repeat(58));
        let _ = execute!(stdout, ResetColor);
        let _ = writeln!(
            stdout,
            "  &graph: {} vertices, {} edges  |  Type: {}  |  Landmarks: {}",
            &graph.num_vertices(),
            &graph.num_edges(),
            &graph.kind(),
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
                    distance_query(
                        &mut stdout,
                        graph.clone(),
                        cur_basic.clone(),
                        cur_bfs.clone(),
                    );
                }
                KeyCode::Char('2') => {
                    accuracy_benchmark(
                        &mut stdout,
                        &graph,
                        &cur_basic,
                        &cur_bfs,
                        &mut cached_bench,
                    );
                }
                KeyCode::Char('3') => {
                    speed_benchmark(
                        &mut stdout,
                        graph.clone(),
                        &cur_basic,
                        &cur_bfs,
                        &cached_bench,
                    );
                }
                KeyCode::Char('4') => {
                    if let Some(n) = change_landmarks(&mut stdout, &graph, n_landmarks) {
                        if n != n_landmarks {
                            n_landmarks = n;
                            if let Some(b) = LandmarkBasic::new(&graph, n_landmarks) {
                                cur_basic = Arc::new(b);
                            }
                            cur_bfs = Arc::new(LandmarkBFS::new(&graph, n_landmarks));
                            cached_bench = None;
                        }
                    }
                }
                KeyCode::Char('5') => {
                    landmark_info(&mut stdout, &graph, &cur_basic, &cur_bfs);
                }
                KeyCode::Char('6') | KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Enter => match sel {
                    0 => {
                        distance_query(
                            &mut stdout,
                            graph.clone(),
                            cur_basic.clone(),
                            cur_bfs.clone(),
                        )
                        .await;
                    }
                    1 => accuracy_benchmark(
                        &mut stdout,
                        &graph,
                        &cur_basic,
                        &cur_bfs,
                        &mut cached_bench,
                    ),
                    2 => speed_benchmark(
                        &mut stdout,
                        graph.clone(),
                        &cur_basic,
                        &cur_bfs,
                        &cached_bench,
                    ),
                    3 => {
                        if let Some(n) = change_landmarks(&mut stdout, &graph, n_landmarks) {
                            if n != n_landmarks {
                                n_landmarks = n;
                                if let Some(b) = LandmarkBasic::new(&graph, n_landmarks) {
                                    cur_basic = Arc::new(b);
                                }
                                cur_bfs = Arc::new(LandmarkBFS::new(&graph, n_landmarks));
                                cached_bench = None;
                            }
                        }
                    }
                    4 => landmark_info(&mut stdout, &graph, &cur_basic, &cur_bfs),
                    5 => break,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}

fn read_u32_at(stdout: &mut Stdout, prompt: &str, x: u16, y: u16) -> Option<u32> {
    let mut buf = String::with_capacity(12);
    loop {
        let _ = execute!(stdout, MoveTo(x, y), Clear(ClearType::UntilNewLine));
        let _ = write!(stdout, "{prompt}{buf}");
        let _ = stdout.flush();
        match read().unwrap() {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(c) if c.is_ascii_digit() && buf.len() < 12 => buf.push(c),
                KeyCode::Backspace => {
                    buf.pop();
                }
                KeyCode::Enter => {
                    if buf.is_empty() {
                        return None;
                    }
                    match buf.parse::<u32>() {
                        Ok(n) => return Some(n),
                        Err(_) => {
                            buf.clear();
                            let _ = execute!(
                                stdout,
                                MoveTo(x, y + 1),
                                SetForegroundColor(Color::Red),
                                Print("⚠ Invalid number, try again"),
                                ResetColor,
                            );
                            let _ = stdout.flush();
                            std::thread::sleep(Duration::from_millis(800));
                        }
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => return None,
                _ => {}
            },
            _ => {}
        }
    }
}

fn exact_distance(graph: &Graph, s: u32, t: u32) -> Option<usize> {
    if s == t {
        return graph.external_to_internal(s).map(|_| 0);
    }
    let s_int = graph.external_to_internal(s)?;
    let t_int = &graph.external_to_internal(t)?;
    bfs_internal(&graph, s_int).get(&t_int).copied()
}

#[allow(unused_variables)]
async fn distance_query(
    stdout: &mut Stdout,
    graph: Arc<Graph>,
    basic: Arc<LandmarkBasic>,
    bfs: Arc<LandmarkBFS>,
) {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
    write!(stdout, "Distance Query\n");
    writeln!(
        stdout,
        "  Enter vertex IDs as they appear in the original dataset."
    );
    loop {
        let s = match read_u32_at(stdout, "  Source vertex ID: ", 0, 4) {
            Some(s) => {
                if graph.external_to_internal(s).is_none() {
                    let _ = execute!(
                        stdout,
                        MoveTo(0, 5),
                        SetForegroundColor(Color::Red),
                        Print(format!("  ⚠ Vertex {s} not found in the graph!")),
                        ResetColor,
                    );
                    let _ = stdout.flush();
                    std::thread::sleep(Duration::from_millis(1000));
                    continue;
                }
                s
            }
            None => return,
        };
        let t = match read_u32_at(stdout, "  Target vertex ID: ", 0, 5) {
            Some(t) => t,
            None => return,
        };
        if graph.external_to_internal(t).is_none() {
            let _ = execute!(
                stdout,
                MoveTo(0, 6),
                SetForegroundColor(Color::Red),
                Print(format!("  ⚠ Vertex {t} not found in the &graph!")),
                ResetColor,
            );
            let _ = stdout.flush();
            std::thread::sleep(Duration::from_millis(1000));
            continue;
        }
        let _ = execute!(
            stdout,
            MoveTo(0, 6),
            Clear(ClearType::UntilNewLine),
            Print("  Computing...")
        );
        let _ = stdout.flush();

        let exact = {
            let start = Instant::now();
            let result = exact_distance(&graph, s, t);
            (result, start.elapsed())
        };
        let basic = {
            let start = Instant::now();
            let result = basic.estimate(s, t);
            (result, start.elapsed())
        };
        let bfs = {
            let start = Instant::now();
            let result = bfs.estimate(&graph, s, t);
            (result, start.elapsed())
        };
    }
}
#[allow(unused_variables)]
fn accuracy_benchmark(
    stdout: &mut Stdout,
    graph: &Graph,
    basic: &LandmarkBasic,
    bfs: &LandmarkBFS,
    cached_bench: &mut Option<Vec<BenchResult>>,
) {
    let _ = writeln!(
        stdout,
        "\n  [stub] accuracy_benchmark — not implemented yet"
    );
    let _ = writeln!(
        stdout,
        "  Would benchmark {} landmarks",
        cached_bench.as_ref().map_or(0, |v| v.len())
    );
    let _ = writeln!(
        stdout,
        "\n  [stub] accuracy_benchmark — not implemented yet"
    );
    let _ = writeln!(
        stdout,
        "  Would benchmark {} landmarks",
        cached_bench.as_ref().map_or(0, |v| v.len())
    );
    let _ = stdout.flush();
    std::thread::sleep(Duration::from_secs(1));
}

#[allow(unused_variables)]
fn speed_benchmark(
    stdout: &mut Stdout,
    graph: Arc<Graph>,
    basic: &LandmarkBasic,
    bfs: &LandmarkBFS,
    cached_bench: &Option<Vec<BenchResult>>,
) {
    let _ = writeln!(stdout, "\n  [stub] speed_benchmark — not implemented yet");
    let _ = writeln!(
        stdout,
        "  Would benchmark speed with {} landmarks",
        cached_bench.as_ref().map_or(0, |v| v.len())
    );
    let _ = writeln!(
        stdout,
        "  Would benchmark speed with {} landmarks",
        cached_bench.as_ref().map_or(0, |v| v.len())
    );
    let _ = stdout.flush();
    std::thread::sleep(Duration::from_secs(1));
}

#[allow(unused_variables)]
fn change_landmarks(stdout: &mut Stdout, graph: &Graph, current: usize) -> Option<usize> {
    let _ = writeln!(stdout, "\n  [stub] change_landmarks — not implemented yet");
    let _ = writeln!(stdout, "  Current: {}. Returning same value.", current);
    let _ = stdout.flush();
    std::thread::sleep(Duration::from_secs(1));
    Some(current)
}

#[allow(unused_variables)]
fn landmark_info(stdout: &mut Stdout, graph: &Graph, basic: &LandmarkBasic, bfs: &LandmarkBFS) {
    let _ = writeln!(stdout, "\n  [stub] landmark_info — not implemented yet");
    let _ = stdout.flush();
    std::thread::sleep(Duration::from_secs(1));
}
