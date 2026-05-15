use crate::graph::Graph;
use crate::graph::traversal::bfs_internal;
use crate::landmarks::LandmarkStrategy;
use crate::landmarks::basic_landmarks::LandmarkBasic;
use crate::landmarks::bfs_landmarks::LandmarkBFS;
use crossterm::{cursor::*, event::*, execute, style::*, terminal::*};
use std::fs;
use std::io::{Stdout, Write, stdout};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::task;
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
    "   Change strategy of landmarks samples ",
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
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    execute!(stdout, MoveTo(0, 0)).unwrap();

    let basic = match LandmarkBasic::new(&graph, num_landmarks, LandmarkStrategy::Random) {
        Some(b) => b,
        None => {
            let _ = write!(
                stdout,
                "  Error: could not create landmarks (graph too small?).\r\n"
            );
            let _ = stdout.flush();
            std::thread::sleep(Duration::from_secs(2));
            return;
        }
    };
    let bfs = LandmarkBFS::new(&graph, num_landmarks, LandmarkStrategy::Random);
    let mut cached_bench: Option<Vec<BenchResult>> = None;
    let mut sel = 0usize;
    let mut n_landmarks = num_landmarks;
    let mut cur_basic = Arc::new(basic);
    let mut cur_bfs = Arc::new(bfs);
    loop {
        let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        let _ = execute!(stdout, SetForegroundColor(Color::Cyan));
        let _ = write!(stdout, "╔{}╗\r\n", "═".repeat(58));
        let _ = write!(stdout, "║{:^58}║\r\n", "     Landmark Distance Estimator");
        let _ = write!(stdout, "╚{}╝\r\n", "═".repeat(58));
        let _ = execute!(stdout, ResetColor);
        let _ = write!(
            stdout,
            "  Graph: {} vertices, {} edges  |  Type: {}  |  Landmarks: {}\r\n",
            &graph.num_vertices(),
            &graph.num_edges(),
            &graph.kind(),
            n_landmarks,
        );
        let _ = write!(stdout, "\r\n");
        for (i, item) in MENU_ITEMS.iter().enumerate() {
            if i == sel {
                let _ = execute!(
                    stdout,
                    SetBackgroundColor(Color::Blue),
                    SetForegroundColor(Color::White),
                    Print(format!("  ❯ {}\r\n", item)),
                    ResetColor,
                );
            } else {
                let _ = write!(stdout, "    {}\r\n", item);
            }
        }
        let _ = write!(stdout, "\r\n");
        let _ = execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("  ↑↓ navigate · Enter/1-6 select · q/Esc back\r\n"),
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
                    )
                    .await;
                }
                KeyCode::Char('2') => {
                    accuracy_benchmark(
                        &mut stdout,
                        graph.clone(),
                        cur_basic.clone(),
                        cur_bfs.clone(),
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
                            if let Some(b) =
                                LandmarkBasic::new(&graph, n_landmarks, LandmarkStrategy::Random)
                            {
                                cur_basic = Arc::new(b);
                            }
                            cur_bfs = Arc::new(LandmarkBFS::new(
                                &graph,
                                n_landmarks,
                                LandmarkStrategy::Random,
                            ));
                            cached_bench = None;
                        }
                    }
                }
                KeyCode::Char('6') => {
                    landmark_info(&mut stdout, &graph, &cur_basic, &cur_bfs);
                }
                KeyCode::Char('7') | KeyCode::Esc | KeyCode::Char('q') => break,
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
                    1 => {
                        accuracy_benchmark(
                            &mut stdout,
                            graph.clone(),
                            cur_basic.clone(),
                            cur_bfs.clone(),
                            &mut cached_bench,
                        )
                        .await
                    }
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
                                if let Some(b) = LandmarkBasic::new(
                                    &graph,
                                    n_landmarks,
                                    LandmarkStrategy::Random,
                                ) {
                                    cur_basic = Arc::new(b);
                                }
                                cur_bfs = Arc::new(LandmarkBFS::new(
                                    &graph,
                                    n_landmarks,
                                    LandmarkStrategy::Random,
                                ));
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

fn compute_distances(
    graph: &Graph,
    basic: &LandmarkBasic,
    bfs: &LandmarkBFS,
    s: u32,
    t: u32,
) -> BenchResult {
    let (exact_res, exact_time) = {
        let start = Instant::now();
        let result = exact_distance(graph, s, t);
        (result, start.elapsed())
    };
    let (basic_res, basic_time) = {
        let start = Instant::now();
        let result = basic.estimate(s, t);
        (result, start.elapsed())
    };
    let (bfs_res, bfs_time) = {
        let start = Instant::now();
        let result = bfs.estimate(graph, s, t);
        (result, start.elapsed())
    };

    BenchResult {
        exact: exact_res,
        basic: basic_res,
        bfs: bfs_res,
        exact_time,
        basic_time,
        bfs_time,
    }
}

async fn distance_query(
    stdout: &mut Stdout,
    graph: Arc<Graph>,
    basic: Arc<LandmarkBasic>,
    bfs: Arc<LandmarkBFS>,
) -> Option<BenchResult> {
    let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
    let _ = write!(stdout, "Distance Query\r\n");
    let _ = write!(
        stdout,
        "  Enter vertex IDs as they appear in the original dataset.\r\n"
    );
    let mut last_result: Option<BenchResult> = None;
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
            None => return last_result,
        };
        let t = match read_u32_at(stdout, "  Target vertex ID: ", 0, 5) {
            Some(t) => t,
            None => return last_result,
        };
        if graph.external_to_internal(t).is_none() {
            let _ = execute!(
                stdout,
                MoveTo(0, 6),
                SetForegroundColor(Color::Red),
                Print(format!("  ⚠ Vertex {t} not found in the graph!")),
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

        let res = compute_distances(&graph, &basic, &bfs, s, t);
        last_result = Some(res);

        let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        let _ = write!(stdout, "  ─── Distance Query Results ───\r\n");
        let _ = write!(stdout, "\r\n");
        let _ = write!(stdout, "  s = {s}  |  t = {t}\r\n");
        let _ = write!(stdout, "\r\n");
        let _ = write!(stdout, "  Exact BFS:  ");
        match res.exact {
            Some(d) => { let _ = write!(stdout, "{d}  ({:0.3?})\r\n", res.exact_time); }
            None => { let _ = write!(stdout, "unreachable  ({:0.3?})\r\n", res.exact_time); }
        }
        let _ = write!(stdout, "  Basic LM:   ");
        match res.basic {
            Some(d) => { let _ = write!(stdout, "{d}  ({:0.3?})\r\n", res.basic_time); }
            None => { let _ = write!(stdout, "unreachable  ({:0.3?})\r\n", res.basic_time); }
        }
        let _ = write!(stdout, "  BFS LM:     ");
        match res.bfs {
            Some(d) => { let _ = write!(stdout, "{d}  ({:0.3?})\r\n", res.bfs_time); }
            None => { let _ = write!(stdout, "unreachable  ({:0.3?})\r\n", res.bfs_time); }
        }
        let _ = write!(stdout, "\r\n");
        let _ = write!(stdout, "  Press Enter to query again, q to exit");
        let _ = stdout.flush();

        match read().unwrap() {
            Event::Key(KeyEvent { code: KeyCode::Char('q'), .. })
            | Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => return last_result,
            _ => {
                let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
                let _ = write!(stdout, "Distance Query\r\n");
                let _ = write!(
                    stdout,
                    "  Enter vertex IDs as they appear in the original dataset.\r\n"
                );
            }
        }
    }
}
#[allow(unused_variables)]
async fn accuracy_benchmark(
    stdout: &mut Stdout,
    graph: Arc<Graph>,
    basic: Arc<LandmarkBasic>,
    bfs: Arc<LandmarkBFS>,
    cached_bench: &mut Option<Vec<BenchResult>>,
) {
    let stdout_arc = Arc::new(Mutex::new(std::io::stdout()));

    let stdout_1 = stdout_arc.clone();
    let stdout_2 = stdout_arc.clone();
    let stdout_3 = stdout_arc.clone();

    let g1 = Arc::clone(&graph);
    let g2 = Arc::clone(&graph);
    let g3 = Arc::clone(&graph);

    let l_basic_1 = Arc::clone(&basic);
    let l_basic_2 = Arc::clone(&basic);
    let l_basic_3 = Arc::clone(&basic);

    let l_bfs_1 = Arc::clone(&bfs);
    let l_bfs_2 = Arc::clone(&bfs);
    let l_bfs_3 = Arc::clone(&bfs);

    let rt = tokio::runtime::Handle::current();

    tokio::try_join!(
        task::spawn_blocking({
            let rt = rt.clone();
            move || {
                let mut stdout = stdout_1.lock().unwrap();
                rt.block_on(distance_query(&mut *stdout, g1, l_basic_1, l_bfs_1))
            }
        }),
        task::spawn_blocking({
            let rt = rt.clone();
            move || {
                let mut stdout = stdout_2.lock().unwrap();
                rt.block_on(distance_query(&mut *stdout, g2, l_basic_2, l_bfs_2))
            }
        }),
        task::spawn_blocking({
            let rt = rt.clone();
            move || {
                let mut stdout = stdout_3.lock().unwrap();
                rt.block_on(distance_query(&mut *stdout, g3, l_basic_3, l_bfs_3))
            }
        })
    ).unwrap();
    let mut stdout = stdout_arc.lock().unwrap();

    let _ = writeln!(
        stdout,
        "\n  [stub] accuracy_benchmark — not implemented yet"
    );
    let _ = write!(
        stdout,
        "  Would benchmark {} landmarks",
        cached_bench.as_ref().map_or(0, |v| v.len())
    );
    let _ = stdout.flush();
}

#[allow(unused_variables)]
fn speed_benchmark(
    stdout: &mut Stdout,
    graph: Arc<Graph>,
    basic: &LandmarkBasic,
    bfs: &LandmarkBFS,
    cached_bench: &Option<Vec<BenchResult>>,
) {
    let _ = write!(stdout, "\n  [stub] speed_benchmark — not implemented yet");
    let _ = write!(
        stdout,
        "  Would benchmark speed with {} landmarks",
        cached_bench.as_ref().map_or(0, |v| v.len())
    );
    let _ = stdout.flush();
    std::thread::sleep(Duration::from_secs(1));
}

fn change_landmarks(stdout: &mut Stdout, _graph: &Graph, current: usize) -> Option<usize> {
    let _ = execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print(format!(
            "Enter new number of landmarks (current: {current}): "
        )),
    );
    read_u32_at(stdout, "New amount: ", 0, 1).map(|n| n as usize)
}

#[allow(unused_variables)]
fn landmark_info(stdout: &mut Stdout, graph: &Graph, basic: &LandmarkBasic, bfs: &LandmarkBFS) {
    let _ = write!(stdout, "\n  [stub] landmark_info — not implemented yet");
    let _ = stdout.flush();
    std::thread::sleep(Duration::from_secs(1));
}

fn collect_files_recursive(dir: &std::path::Path, files: &mut Vec<(String, u64)>, prefix: &str) {
    let skip_names = [
        "target",
        "src",
        ".git",
        "node_modules",
        ".vscode",
        ".idea",
        "target",
        ".DS_Store",
        "Cargo.lock",
        "Cargo.toml",
        "degree_data1.png",
        "log_degree_data.png",
        "performance.log",
    ];
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if skip_names.contains(&name.as_str()) {
            continue;
        }

        if path.is_dir() {
            let new_prefix = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", prefix, name)
            };
            collect_files_recursive(&path, files, &new_prefix);
        } else if path.is_file() {
            let full_path = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", prefix, name)
            };
            files.push((
                full_path.clone(),
                fs::metadata(full_path.clone()).unwrap().len(),
            ));
        }
    }
}

fn find_min_files(files: &mut Vec<(String, u64)>) -> Vec<String> {
    files.sort_by_key(|k| k.1);
    let mut to_return: Vec<String> = Vec::new();
    if files.len() < 3 {
        to_return = files.iter().map(|x| x.0.clone()).collect();
    } else {
        to_return = vec![files[0].0.clone(), files[1].0.clone(), files[2].0.clone()];
    }

    to_return
}
