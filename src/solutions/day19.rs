use crate::Day;

pub struct Day19;

impl Day for Day19 {
    async fn part1(input: String) -> String {
        part1(&input).to_string()
    }
}

use std::{
    cmp::Ordering,
    collections::HashMap,
};

fn part1(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<String, Workflow> = workflows.lines()
        .map(|line| {
            let ind = line.find('{').unwrap();
            let name = line[..ind].to_owned();
            let workflow = line[ind+1..].into();
            (name, workflow)
        })
        .collect();
    let accepted: Vec<_> = parts.lines()
        .map(Into::<Part>::into)
        .filter(|part| {
            let mut action = Action::SendTo("in".to_owned());
            loop {
                action = match action {
                    Action::Accept => { break true; }
                    Action::Reject => { break false; }
                    Action::SendTo(current) => {
                        let workflow = workflows.get(&current).unwrap();
                        workflow.conditions.iter().find_map(|(cond, next)| {
                            part.meets_condition(cond).then_some(next.clone())
                        }).unwrap()
                    }
                }
            }
        })
        .collect();
    accepted.into_iter()
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn part2(input: &str) -> usize {
    let (workflows, _parts) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<String, Workflow> = workflows.lines()
        .map(|line| {
            let ind = line.find('{').unwrap();
            let name = line[..ind].to_owned();
            let workflow = line[ind+1..].into();
            (name, workflow)
        })
        .collect();
    todo!()
}

struct Workflow {
    conditions: Vec<(Condition, Action)>,
}

#[derive(Clone)]
enum Action {
    Accept,
    Reject,
    SendTo(String),
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            val => Self::SendTo(val.to_owned())
        }
    }
}

impl From<&str> for Workflow {
    fn from(s: &str) -> Self {
        let conditions = s
            .strip_suffix('}')
            .unwrap()
            .split(',').map(|cond_str| {
                if let Some((left, right)) = cond_str.split_once(':') {
                    if let Some((cat, num)) = left.split_once('<') {
                        (Condition::LessThan(cat.into(), num.parse().unwrap()), right.into())
                    } else if let Some((cat, num)) = left.split_once('>') {
                        (Condition::GreaterThan(cat.into(), num.parse().unwrap()), right.into())
                    } else {
                        unreachable!()
                    }
                } else {
                    (Condition::None, cond_str.into())
                }

            })
            .collect();
        Self { conditions }
    }
}

enum Category {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "x" => Self::ExtremelyCool,
            "m" => Self::Musical,
            "a" => Self::Aerodynamic,
            "s" => Self::Shiny,
            _ => unreachable!(),
        }
    }
}

enum Condition {
    LessThan(Category, usize),
    GreaterThan(Category, usize),
    None,
}

#[derive(Default)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let mut part = Part::default();
        for attr in s.strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
        {
            match attr.split_once('=').unwrap() {
                ("x", val) => { part.x = val.parse().unwrap() }
                ("m", val) => { part.m = val.parse().unwrap() }
                ("a", val) => { part.a = val.parse().unwrap() }
                ("s", val) => { part.s = val.parse().unwrap() }
                _ => unreachable!()
            }
        }
        part
    }
}

impl Part {
    fn meets_condition(&self, cond: &Condition) -> bool {
        let (target, cat, val) = match cond {
            Condition::None => { return true; }
            Condition::LessThan(cat, val) => (Ordering::Less, cat, val),
            Condition::GreaterThan(cat, val) => (Ordering::Greater, cat, val),
        };
        let something = match cat {
            Category::ExtremelyCool => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        };
        something.cmp(&val) == target
    }
}

struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}
