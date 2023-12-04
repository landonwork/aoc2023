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
