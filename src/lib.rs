#![allow(async_fn_in_trait)]

mod solutions;
pub use solutions::*;

use axum::{response::Html, Form};
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! layout {
    ($layout_name:literal, $($rest:expr),+) => {
        minijinja::render!(
            include_str!($layout_name),
            inner_content => $crate::layout!($($rest),+)
        )
    };
    ($inner:expr) => {
        $inner
    };
}

// #[macro_export]
// macro_rules! day {
//     ($day_num:literal) => {
//         Box::new($crate::
//     };
// }


pub fn read_input(day: &str) -> String {
    std::fs::read_to_string(format!("input/day{day}.txt"))
        .unwrap()
}

pub fn lines(s: &str) -> Vec<&str> {
    s.trim().split('\n').collect()
}

#[derive(Serialize, Deserialize)]
pub struct PartInput {
    pub input: String
}

pub trait Day {
    async fn part1(_input: String) -> String {
        "Part 1 not finished :(".into()
    }
    async fn part2(_input: String) -> String {
        "Part 2 not finished :(".into()
    }
}

pub trait DayExt: Day {
    async fn part1_ext(Form(input): Form<PartInput>) -> Html<String> {
        Html(<Self as Day>::part1(input.input).await)
    }

    async fn part2_ext(Form(input): Form<PartInput>) -> Html<String> {
        Html(<Self as Day>::part2(input.input).await)
    }
}

impl<T> DayExt for T where T: Day { }

