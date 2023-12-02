mod solutions;

pub use solutions::day1;

pub struct Solutions(pub String, pub String);

impl Default for Solutions {
    fn default() -> Self {
        Self(String::new(), String::new())
    }
}