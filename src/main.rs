use std::{fs, net::SocketAddr};

use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router, http::StatusCode, Form,
};
use minijinja::render;
use tokio::{net::TcpListener, select};
use tower_http::services::ServeDir;

use aoc2023::*;

#[tokio::main]
async fn main() {
    // Very first thing is set up the shutdown
    let (sender, mut receiver) = tokio::sync::broadcast::channel::<()>(1);

    // start tailwind when in dev mode
    #[cfg(debug_assertions)]
    {
        use tokio::io::{AsyncBufReadExt, BufReader};
        let mut powershell = sender.subscribe();
        let mut readers = sender.subscribe();

        match tokio::process::Command::new("C:/Program Files/nodejs/npx.cmd")
            .arg("tailwindcss")
            .arg("-i")
            .arg("assets/tailwind.css")
            .arg("-o")
            .arg("static/css/tailwind.css")
            .arg("--watch")
            .kill_on_drop(true)
            .stderr(std::process::Stdio::piped())
            .spawn()
        {
            Ok(mut tailwind) => {
                println!("tailwind successfully started");
                let mut stderr_reader = BufReader::new(tailwind.stderr.take().unwrap()).lines();

                tokio::spawn(async move {
                    select! {
                        res = tailwind.wait() => { match res {
                            Ok(_) => {}
                            Err(error) => {
                                println!("tailwind crashed: {}", error);
                            }
                        }},
                        _exit = powershell.recv() => {}
                    }
                });

                tokio::spawn(async move {
                    loop {
                        select! {
                            err = stderr_reader.next_line() => {
                                if !matches!(err.as_ref().map(|op| op.as_ref().map(|s| s.as_str())), Ok(None) | Ok(Some(""))) {
                                    println!("tailwind stderr: {:?}", err);
                                }
                            },
                            _exit = readers.recv() => { break; }
                        }
                    }
                });
            }
            Err(_) => {
                println!("tailwind failed");
            }
        }
    }

    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })
    .unwrap();

    let router = Router::new()
        .route("/", get(home))
        .route("/day/:day", get(solve))
        .route("/day/:day/part1", post(solve_part1))
        .route("/day/:day/part2", post(solve_part2))
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", &addr);

    select! {
        _ = async {
            axum::serve(listener, router)
                .await
                .unwrap();
        } => {},
        _ = receiver.recv() => {}
    }
}

async fn home() -> Html<String> {
    let mut days: Vec<String> = fs::read_dir("src/solutions")
        .unwrap()
        .filter_map(|res| {
            res.unwrap()
                .file_name()
                .to_string_lossy()
                .strip_prefix("day")
                .map(|day| day.replace(".rs", "").to_string())
        })
        .collect();
    days.sort();
    let days: Vec<String> = days.into_iter()
        .filter_map(|day| if day.starts_with('0') { day.strip_prefix('0').map(|s| s.to_string()) } else { Some(day) })
        .collect();

    Html(layout!(
        "../assets/layouts/root.html",
        "../assets/layouts/app.html",
        render!(
            include_str!("../assets/templates/index.html"),
            days => days
        )
    ))
}

async fn solve(Path(day): Path<u8>) -> impl IntoResponse {
    if day > 25 {
        (StatusCode::NOT_FOUND, Html(String::new()))
    } else {
        (
            StatusCode::OK,
            Html(layout!(
                "../assets/layouts/root.html",
                "../assets/layouts/app.html",
                render!(
                    include_str!("../assets/templates/solutions.html"),
                    day => day
                )
            ))
        )
    }
}

async fn get_part1<D: Day>(input: String) -> String {
    D::part1(input).await
}

async fn get_part2<D: Day>(input: String) -> String {
    D::part2(input).await
}

async fn solve_part1(Path(day): Path<u8>, Form(input): Form<PartInput>) -> impl IntoResponse {
    // I am supremely disappointed that I didn't find a better way to do this.
    // At least this way implementing the Day trait is enforced.
    // I am still somewhat tempted to try making a function-like proc-macro and add
    // two routes for each day rather than 25 days for 2 routes.
    let output = match day {
        1 =>  get_part1::<day01::Day01>(input.input).await,
        2 =>  get_part1::<day02::Day02>(input.input).await,
        3 =>  get_part1::<day03::Day03>(input.input).await,
        4 =>  get_part1::<day04::Day04>(input.input).await,
        5 =>  get_part1::<day05::Day05>(input.input).await,
        6 =>  get_part1::<day06::Day06>(input.input).await,
        7 =>  get_part1::<day07::Day07>(input.input).await,
        8 =>  get_part1::<day08::Day08>(input.input).await,
        9 =>  get_part1::<day09::Day09>(input.input).await,
        10 =>  get_part1::<day10::Day10>(input.input).await,
        11 =>  get_part1::<day11::Day11>(input.input).await,
        12 =>  get_part1::<day12::Day12>(input.input).await,
        13 =>  get_part1::<day13::Day13>(input.input).await,
        14 =>  get_part1::<day14::Day14>(input.input).await,
        15 =>  get_part1::<day15::Day15>(input.input).await,
        16 =>  get_part1::<day16::Day16>(input.input).await,
        17 =>  get_part1::<day17::Day17>(input.input).await,
        18 =>  get_part1::<day18::Day18>(input.input).await,
        19 =>  get_part1::<day19::Day19>(input.input).await,
        20 =>  get_part1::<day20::Day20>(input.input).await,
        21 =>  get_part1::<day21::Day21>(input.input).await,
        22 =>  get_part1::<day22::Day22>(input.input).await,
        23 =>  get_part1::<day23::Day23>(input.input).await,
        24 =>  get_part1::<day24::Day24>(input.input).await,
        25 =>  get_part1::<day25::Day25>(input.input).await,
        _ => { return (StatusCode::NOT_FOUND, Html(format!("Day {day} not found"))); }
    };

    (StatusCode::OK, Html(output))
}

async fn solve_part2(Path(day): Path<u8>, Form(input): Form<PartInput>) -> impl IntoResponse {
    // I am supremely disappointed that I didn't find a better way to do this.
    // At least this way implementing the Day trait is enforced.
    // I am still somewhat tempted to try making a function-like proc-macro and add
    // two routes for each day rather than 25 days for 2 routes.
    let output = match day {
        1 =>  get_part2::<day01::Day01>(input.input).await,
        2 =>  get_part2::<day02::Day02>(input.input).await,
        3 =>  get_part2::<day03::Day03>(input.input).await,
        4 =>  get_part2::<day04::Day04>(input.input).await,
        5 =>  get_part2::<day05::Day05>(input.input).await,
        6 =>  get_part2::<day06::Day06>(input.input).await,
        7 =>  get_part2::<day07::Day07>(input.input).await,
        8 =>  get_part2::<day08::Day08>(input.input).await,
        9 =>  get_part2::<day09::Day09>(input.input).await,
        10 =>  get_part2::<day10::Day10>(input.input).await,
        11 =>  get_part2::<day11::Day11>(input.input).await,
        12 =>  get_part2::<day12::Day12>(input.input).await,
        13 =>  get_part2::<day13::Day13>(input.input).await,
        14 =>  get_part2::<day14::Day14>(input.input).await,
        15 =>  get_part2::<day15::Day15>(input.input).await,
        16 =>  get_part2::<day16::Day16>(input.input).await,
        17 =>  get_part2::<day17::Day17>(input.input).await,
        18 =>  get_part2::<day18::Day18>(input.input).await,
        19 =>  get_part2::<day19::Day19>(input.input).await,
        20 =>  get_part2::<day20::Day20>(input.input).await,
        21 =>  get_part2::<day21::Day21>(input.input).await,
        22 =>  get_part2::<day22::Day22>(input.input).await,
        23 =>  get_part2::<day23::Day23>(input.input).await,
        24 =>  get_part2::<day24::Day24>(input.input).await,
        25 =>  get_part2::<day25::Day25>(input.input).await,
        _ => { return (StatusCode::NOT_FOUND, Html(format!("Day {day} not found"))); }
    };

    (StatusCode::OK, Html(output))
}
