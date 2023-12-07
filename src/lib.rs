mod solutions;

pub use solutions::*;

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

// pub trait Day {
//     fn solve() -> Solutions;
// }
