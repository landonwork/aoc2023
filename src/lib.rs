mod solutions;

pub use solutions::*;

use axum::{response::Html, Form};
use serde::{Serialize, Deserialize};

pub struct Solutions(pub String, pub String);

impl Default for Solutions {
    fn default() -> Self {
        Self(String::new(), String::new())
    }
}

#[macro_export]
macro_rules! layout {
    ($layout_name:literal, $($rest:expr),*) => {
        minijinja::render!(
            include_str!($layout_name),
            inner_content => $crate::layout!($($rest),*)
        )
    };
    ($inner:expr) => {
        $inner
    };
}

pub fn read_input(day: &str) -> Vec<String> {
    std::fs::read_to_string(format!("input/day{day}.txt"))
        .unwrap()
        .replace("\r", "")
        .trim()
        .split("\n")
        .map(|x| x.to_owned())
        .collect()
}

fn lines(s: &str) -> Vec<&str> {
    s.trim().split("\n").collect()
}

#[derive(Serialize, Deserialize)]
pub struct Part1 {
    input: String
}

#[derive(Serialize, Deserialize)]
pub struct Part2 {
    input: String
}

#[derive(Serialize, Deserialize)]
pub struct Extra {
    input: String
}

pub trait Day {
    fn part1(input: Form<Part1>) -> Html<String>;
    fn part2(input: Form<Part2>) -> Html<String>;
    fn extra(input: Form<Extra>) -> Html<String>;
}
