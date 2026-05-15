use crate::graph::Graph;
use crate::graph::traversal::bfs_internal;
use crate::landmarks::LandmarkStrategy;
use crate::landmarks::basic_landmarks::LandmarkBasic;
use crate::landmarks::bfs_landmarks::LandmarkBFS;
use crossterm::{cursor::*, event::*, execute, style::*, terminal::*};
use rand::Rng;
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
    "   Change strategy of landmarks samples ",
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

fn change_strategy(stdout: &mut Stdout, current: LandmarkStrategy) -> Option<LandmarkStrategy> {
    let items = [
        ("Random", LandmarkStrategy::Random),
        ("Highest degree", LandmarkStrategy::HighestDegree),
        ("Farthest-first coverage", LandmarkStrategy::Coverage),
    ];
    let mut sel = items.iter().position(|(_, s)| *s == current).unwrap_or(0);
    loop {
        let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        let _ = write!(stdout, "  ─── Select Landmark Strategy ───\r\n\r\n");
        for (i, (name, _)) in items.iter().enumerate() {
            if i == sel {
                let _ = execute!(
                    stdout,
                    SetBackgroundColor(Color::Blue),
                    SetForegroundColor(Color::White),
                    Print(format!("  ❯ {} (press Enter)\r\n", name)),
                    ResetColor,
                );
            } else {
                let _ = write!(stdout, "    {}\r\n", name);
            }
        }
        let _ = write!(stdout, "\r\n  Current: {}\r\n", items[sel].0);
        let _ = write!(stdout, "  ↑↓ navigate · Enter select · Esc/q cancel");
        let _ = stdout.flush();

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('k'),
                ..
            }) => {
                sel = sel.saturating_sub(1);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('j'),
                ..
            }) => {
                if sel < items.len() - 1 {
                    sel += 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                if items[sel].1 != current {
                    return Some(items[sel].1);
                }
                return None;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => return None,
            _ => {}
        }
    }
}

fn rebuild_landmarks(
    graph: &Graph,
    n_landmarks: usize,
    strategy: LandmarkStrategy,
) -> (Arc<LandmarkBasic>, Arc<LandmarkBFS>) {
    let basic = LandmarkBasic::new(graph, n_landmarks, strategy)
        .map(Arc::new)
        .unwrap_or_else(|| Arc::new(LandmarkBasic::new(graph, 1, strategy).unwrap()));
    let bfs = Arc::new(LandmarkBFS::new(graph, n_landmarks, strategy));
    (basic, bfs)
}

pub async fn run_landmark_interactive(graph: Arc<Graph>, num_landmarks: usize) {
    let _guard = TerminalGuard::new();
    let mut stdout = stdout();
    let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));

    let mut cur_strategy = LandmarkStrategy::Random;
    let (cur_basic, cur_bfs) = rebuild_landmarks(&graph, num_landmarks, cur_strategy);
    let mut cur_basic = cur_basic;
    let mut cur_bfs = cur_bfs;
    let mut cached_bench: Option<Vec<BenchResult>> = None;
    let mut sel = 0usize;
    let mut n_landmarks = num_landmarks;

    loop {
        let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        let _ = execute!(stdout, SetForegroundColor(Color::Cyan));
        let _ = write!(stdout, "╔{}╗\r\n", "═".repeat(58));
        let _ = write!(stdout, "║{:^58}║\r\n", "     Landmark Distance Estimator");
        let _ = write!(stdout, "╚{}╝\r\n", "═".repeat(58));
        let _ = execute!(stdout, ResetColor);
        let strategy_name = match cur_strategy {
            LandmarkStrategy::Random => "Random",
            LandmarkStrategy::HighestDegree => "Top-deg",
            LandmarkStrategy::Coverage => "Coverage",
        };
        let _ = write!(
            stdout,
            "  Vertices: {}  Edges: {}  Type: {}  LM: {} ({})\r\n",
            graph.num_vertices(),
            graph.num_edges(),
            graph.kind(),
            n_landmarks,
            strategy_name,
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
                        &graph,
                        &cur_basic,
                        &cur_bfs,
                        &mut cached_bench,
                    );
                    cached_bench = None;
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
                            let (b, b2) = rebuild_landmarks(&graph, n_landmarks, cur_strategy);
                            cur_basic = b;
                            cur_bfs = b2;
                            cached_bench = None;
                        }
                    }
                }
                KeyCode::Char('5') => {
                    if let Some(s) = change_strategy(&mut stdout, cur_strategy) {
                        cur_strategy = s;
                        let (b, b2) = rebuild_landmarks(&graph, n_landmarks, cur_strategy);
                        cur_basic = b;
                        cur_bfs = b2;
                        cached_bench = None;
                    }
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
                    1 => {
                        accuracy_benchmark(
                            &mut stdout,
                            &graph,
                            &cur_basic,
                            &cur_bfs,
                            &mut cached_bench,
                        );
                        cached_bench = None;
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
                                let (b, b2) = rebuild_landmarks(&graph, n_landmarks, cur_strategy);
                                cur_basic = b;
                                cur_bfs = b2;
                                cached_bench = None;
                            }
                        }
                    }
                    4 => {
                        if let Some(s) = change_strategy(&mut stdout, cur_strategy) {
                            cur_strategy = s;
                            let (b, b2) = rebuild_landmarks(&graph, n_landmarks, cur_strategy);
                            cur_basic = b;
                            cur_bfs = b2;
                            cached_bench = None;
                        }
                    }
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
            Some(d) => {
                let _ = write!(stdout, "{d}  ({:0.3?})\r\n", res.exact_time);
            }
            None => {
                let _ = write!(stdout, "unreachable  ({:0.3?})\r\n", res.exact_time);
            }
        }
        let _ = write!(stdout, "  Basic LM:   ");
        match res.basic {
            Some(d) => {
                let _ = write!(stdout, "{d}  ({:0.3?})\r\n", res.basic_time);
            }
            None => {
                let _ = write!(stdout, "unreachable  ({:0.3?})\r\n", res.basic_time);
            }
        }
        let _ = write!(stdout, "  BFS LM:     ");
        match res.bfs {
            Some(d) => {
                let _ = write!(stdout, "{d}  ({:0.3?})\r\n", res.bfs_time);
            }
            None => {
                let _ = write!(stdout, "unreachable  ({:0.3?})\r\n", res.bfs_time);
            }
        }
        let _ = write!(stdout, "\r\n");
        let _ = write!(stdout, "  Press Enter to query again, q to exit");
        let _ = stdout.flush();

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => return last_result,
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
fn fmt_duration(d: Duration) -> String {
    let secs = d.as_secs_f64();
    if secs >= 1.0 {
        format!("{secs:.3}s")
    } else if secs >= 0.001 {
        format!("{:.3}ms", secs * 1_000.0)
    } else {
        format!("{:.1}μs", secs * 1_000_000.0)
    }
}

fn print_table_raw(stdout: &mut Stdout, data: &[(String, String)]) {
    let metric_header = "Metric";
    let value_header = "Value";

    let metric_width = data
        .iter()
        .map(|(metric, _)| metric.chars().count())
        .max()
        .unwrap_or(0)
        .max(metric_header.len());
    let value_width = data
        .iter()
        .map(|(_, value)| value.chars().count())
        .max()
        .unwrap_or(0)
        .max(value_header.len());

    let top = format!(
        "╔{}╦{}╗",
        "═".repeat(metric_width + 2),
        "═".repeat(value_width + 2)
    );
    let separator = format!(
        "╠{}╬{}╣",
        "═".repeat(metric_width + 2),
        "═".repeat(value_width + 2)
    );
    let bottom = format!(
        "╚{}╩{}╝",
        "═".repeat(metric_width + 2),
        "═".repeat(value_width + 2)
    );

    let _ = writeln!(stdout, "{top}\r");
    let _ = writeln!(
        stdout,
        "║ {:<metric_width$} ║ {:<value_width$} ║\r",
        metric_header, value_header
    );
    let _ = writeln!(stdout, "{separator}\r");

    for (metric, value) in data {
        let _ = writeln!(
            stdout,
            "║ {:<metric_width$} ║ {:<value_width$} ║\r",
            metric, value
        );
    }

    let _ = writeln!(stdout, "{bottom}\r");
}

fn accuracy_benchmark(
    stdout: &mut Stdout,
    graph: &Graph,
    basic: &LandmarkBasic,
    bfs: &LandmarkBFS,
    cached_bench: &mut Option<Vec<BenchResult>>,
) {
    // ── Clean screen + header ───────────────────────────────────────────
    let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
    let _ = execute!(stdout, SetForegroundColor(Color::Cyan));
    let _ = write!(stdout, "╔{}╗\r\n", "═".repeat(58));
    let _ = write!(stdout, "║{:^58}║\r\n", "  📊  Accuracy Benchmark  ");
    let _ = write!(stdout, "╚{}╝\r\n", "═".repeat(58));
    let _ = execute!(stdout, ResetColor);
    let _ = write!(stdout, "\r\n");

    // ── Input for number of pairs ───────────────────────────────────────
    let num_pairs = match read_u32_at(
        stdout,
        "  Number of random pairs [10-100, default 50]: ",
        0,
        5,
    ) {
        Some(n) if (10..=100).contains(&n) => n as usize,
        Some(_) => {
            let _ = execute!(
                stdout,
                MoveTo(0, 6),
                SetForegroundColor(Color::DarkGrey),
                Print("  Using default: 50 pairs\r\n"),
                ResetColor,
            );
            let _ = stdout.flush();
            std::thread::sleep(Duration::from_millis(500));
            50
        }
        None => return,
    };

    // ── Collect random vertices ─────────────────────────────────────────
    let vertices: Vec<u32> = graph.vertices().collect();
    if vertices.len() < 2 {
        let _ = write!(stdout, "  ⚠ Graph too small for benchmarking.\r\n");
        let _ = stdout.flush();
        std::thread::sleep(Duration::from_secs(1));
        return;
    }

    // ── Progress bar ────────────────────────────────────────────────────
    let _ = execute!(
        stdout,
        MoveTo(0, 7),
        Clear(ClearType::UntilNewLine),
        Print("  Running benchmark..."),
    );
    let _ = stdout.flush();

    let mut rng = rand::thread_rng();
    let bench_start = Instant::now();

    // Use a LOCAL vec — не добавляем к старым результатам!
    let mut results = Vec::with_capacity(num_pairs);

    for i in 0..num_pairs {
        let idx1 = rng.gen_range(0..vertices.len());
        let idx2 = rng.gen_range(0..vertices.len());
        let s = vertices[idx1];
        let t = vertices[idx2];

        let res = compute_distances(graph, basic, bfs, s, t);
        results.push(res);

        // Update progress every 5 pairs or on last
        if i % 5 == 0 || i == num_pairs - 1 {
            let pct = (i + 1) * 100 / num_pairs;
            let elapsed = bench_start.elapsed();
            let _ = execute!(
                stdout,
                MoveTo(0, 8),
                Clear(ClearType::UntilNewLine),
                Print(format!(
                    "  Progress: [{:3}%]  {}/{}  ({})\r",
                    pct,
                    i + 1,
                    num_pairs,
                    fmt_duration(elapsed),
                )),
            );
            let _ = stdout.flush();
        }
    }

    // Сохраняем только текущий запуск
    *cached_bench = Some(results);

    // ── Compute statistics ──────────────────────────────────────────────
    let mut count_basic_nice = 0usize;
    let mut count_bfs_nice = 0usize;
    let mut sum_basic_err: f64 = 0.0;
    let mut sum_bfs_err: f64 = 0.0;
    let mut max_basic_err = 0usize;
    let mut max_bfs_err = 0usize;
    let mut sum_time_basic = Duration::ZERO;
    let mut sum_time_bfs = Duration::ZERO;
    let mut sum_time_exact = Duration::ZERO;

    if let Some(ref res) = *cached_bench {
        for info in res {
            if info.exact == info.basic {
                count_basic_nice += 1;
            }
            if info.exact == info.bfs {
                count_bfs_nice += 1;
            }

            if let (Some(ex), Some(ba)) = (info.exact, info.basic) {
                let err = if ba >= ex { ba - ex } else { ex - ba };
                sum_basic_err += err as f64;
                max_basic_err = max_basic_err.max(err);
            }
            if let (Some(ex), Some(bf)) = (info.exact, info.bfs) {
                let err = if bf >= ex { bf - ex } else { ex - bf };
                sum_bfs_err += err as f64;
                max_bfs_err = max_bfs_err.max(err);
            }

            sum_time_basic += info.basic_time;
            sum_time_bfs += info.bfs_time;
            sum_time_exact += info.exact_time;
        }
    }

    let avg_basic_err = sum_basic_err / num_pairs.max(1) as f64;
    let avg_bfs_err = sum_bfs_err / num_pairs.max(1) as f64;
    let avg_time_basic = sum_time_basic / num_pairs as u32;
    let avg_time_bfs = sum_time_bfs / num_pairs as u32;
    let avg_time_exact = sum_time_exact / num_pairs as u32;
    let pct_basic = count_basic_nice * 100 / num_pairs.max(1);
    let pct_bfs = count_bfs_nice * 100 / num_pairs.max(1);

    // ── Display results ─────────────────────────────────────────────────
    let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
    let _ = execute!(stdout, SetForegroundColor(Color::Cyan));
    let _ = write!(stdout, "╔{}╗\r\n", "═".repeat(58));
    let _ = write!(stdout, "║{:^58}║\r\n", "  📊  Accuracy Benchmark Results  ");
    let _ = write!(stdout, "╚{}╝\r\n", "═".repeat(58));
    let _ = execute!(stdout, ResetColor);
    let _ = write!(stdout, "\r\n");

    let mut table_data: Vec<(String, String)> = Vec::new();
    table_data.push(("Total pairs tested".to_string(), num_pairs.to_string()));
    table_data.push((
        "Basic exact matches".to_string(),
        format!("{}%  ({}/{})", pct_basic, count_basic_nice, num_pairs),
    ));
    table_data.push((
        "BFS exact matches".to_string(),
        format!("{}%  ({}/{})", pct_bfs, count_bfs_nice, num_pairs),
    ));
    table_data.push(("Basic avg error".to_string(), format!("{avg_basic_err:.2}")));
    table_data.push(("BFS avg error".to_string(), format!("{avg_bfs_err:.2}")));
    table_data.push(("Basic max error".to_string(), max_basic_err.to_string()));
    table_data.push(("BFS max error".to_string(), max_bfs_err.to_string()));
    table_data.push((
        "Avg exact BFS time".to_string(),
        fmt_duration(avg_time_exact),
    ));
    table_data.push((
        "Avg Basic LM time".to_string(),
        fmt_duration(avg_time_basic),
    ));
    table_data.push(("Avg BFS LM time".to_string(), fmt_duration(avg_time_bfs)));

    print_table_raw(stdout, &table_data);

    let _ = write!(stdout, "\r\n  Press Enter to continue...\r\n");
    let _ = stdout.flush();

    loop {
        if let Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) = read().unwrap()
        {
            break;
        }
    }
}

#[allow(unused_variables)]
fn speed_benchmark(
    stdout: &mut Stdout,
    graph: Arc<Graph>,
    basic: &LandmarkBasic,
    bfs: &LandmarkBFS,
    cached_bench: &Option<Vec<BenchResult>>,
) {
    let _ = write!(
        stdout,
        "\r\n  [stub] speed_benchmark — not implemented yet\r\n"
    );
    let _ = write!(
        stdout,
        "  Would benchmark speed with {} landmarks\r\n",
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
