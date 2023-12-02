use std::{borrow::Cow, fs};

use axum::{
    extract::Path,
    response::Html,
    routing::get,
    Router,
};
use minijinja::render;
use tower_http::services::ServeDir;

use aoc2023::{day1, Solutions};

async fn home() -> Html<String> {
    let days: Vec<String> = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/solutions"))
        .unwrap()
        .filter_map(|res| match &res.unwrap().file_name().to_string_lossy() {
            Cow::Borrowed("mod.rs") => None,
            file_name => {

                file_name
                    .strip_suffix(".rs")
                    .and_then(|x| x.strip_prefix("day").map(|day| day.to_string()))
            }
        })
        .collect();

    Html(render!(include_str!("../assets/templates/index.html"), days => days))
}

async fn solve(Path(day): Path<i32>) -> Html<String> {
    let function: fn() -> Solutions = match day {
        1 => day1::solve,
        // 2 => day2::solve,
        // 3 => day3::solve,
        // 4 => day4::solve,
        // 5 => day5::solve,
        // 6 => day6::solve,
        // 7 => day7::solve,
        // 8 => day8::solve,
        // 9 => day9::solve,
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
        _ => { return Html(String::new()); }
    };

    let Solutions(part1, part2) = function();

    Html(render!(
        include_str!("../assets/templates/solutions.html"),
        day => day,
        part1 => part1,
        part2 => part2,
    ))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(home))
        .route("/day/:day", get(solve))
        .nest_service("/static", ServeDir::new("static"));

    Ok(router.into())
}
