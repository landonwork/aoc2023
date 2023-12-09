use std::{fs, net::SocketAddr};

use axum::{
    extract::Path,
    response::Html,
    routing::{get, post},
    Router,
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
    // this is always going to fail on the server and that's fine
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
    let days: Vec<String> = days.into_iter().map(|day| day.replace("0", "")).collect();

    Html(layout!(
        "../assets/layouts/root.html",
        "../assets/layouts/app.html",
        render!(
            include_str!("../assets/templates/index.html"),
            days => days
        )
    ))
}

async fn solve(Path(day): Path<i32>) -> Html<String> {
    let function: fn() -> Solutions = match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        6 => day06::solve,
        7 => day07::solve,
        8 => day08::solve,
        9 => day09::solve,
        // 10 => day10::solve,
        // 11 => day11::solve,
        // 12 => day12::solve,
        // 13 => day13::solve,
        // 14 => day14::solve,
        // 15 => day15::solve,
        // 16 => day16::solve,
        // 17 => day17::solve,
        // 18 => day18::solve,
        // 19 => day19::solve,
        // 20 => day20::solve,
        // 21 => day21::solve,
        // 22 => day22::solve,
        // 23 => day23::solve,
        // 24 => day24::solve,
        // 25 => day25::solve,
        _ => {
            return Html(String::new());
        }
    };

    let Solutions(part1, part2) = function();

    Html(layout!(
        "../assets/layouts/root.html",
        "../assets/layouts/app.html",
        render!(
            include_str!("../assets/templates/solutions.html"),
            day => day,
            part1 => part1,
            part2 => part2,
        )
    ))
}
