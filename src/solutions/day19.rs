use crate::Day;

pub struct Day19;

impl Day for Day19 {
    async fn part1(input: String) -> String {
        part1(&input).to_string()
    }

    async fn part2(input: String) -> String {
        part2(&input).to_string()
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
        something.cmp(val) == target
    }
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

    let set = Set { x: 1..4001, m: 1..4001, a: 1..4001, s: 1..4001 };
    count_accepted(set, workflows.get("in").unwrap().conditions.as_slice(), &workflows)
}

fn count_accepted(set: Set, conditions: &[(Condition, Action)], workflows: &HashMap<String, Workflow>) -> usize {
    match conditions.split_first() {
        Some(((cond, action), rest)) => match cond {
            Condition::None => match action {
                Action::Accept => set.size(),
                Action::Reject => 0,
                Action::SendTo(next) => {
                    let next_conditions = workflows.get(next).unwrap().conditions.as_slice();
                    count_accepted(set, next_conditions, workflows)
                },
            }
            Condition::LessThan(cat, split) => {
                let (left, right) = set.split(cat, *split);
                match action {
                    Action::Accept => left.size() + count_accepted(right, rest, workflows),
                    Action::Reject => count_accepted(right, rest, workflows),
                    Action::SendTo(next) => {
                        let left_workflows = workflows.get(next).unwrap().conditions.as_slice();
                        count_accepted(left, left_workflows, workflows)
                            + count_accepted(right, rest, workflows)
                    }
                }
            }
            Condition::GreaterThan(cat, split) => {
                let (left, right) = set.split(cat, *split+1);
                match action {
                    Action::Accept => count_accepted(left, rest, workflows) + right.size(),
                    Action::Reject => count_accepted(left, rest, workflows),
                    Action::SendTo(next) => {
                        let right_workflows = workflows.get(next).unwrap().conditions.as_slice();
                        count_accepted(left, rest, workflows)
                            + count_accepted(right, right_workflows, workflows)
                    }
                }
            }
        }
        None => 0,
    }
}

use std::ops::Range;
#[derive(Clone)]
struct Set {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl Set {
    fn split(self, cat: &Category, split: usize) -> (Self, Self) {
        match cat {
            Category::ExtremelyCool => {
                assert!((self.x.start <= split) && (split <= self.x.end));
                (Set { x: self.x.start..split, ..self.clone() }, Set { x: split..self.x.end, ..self })
            }
            Category::Musical => {
                assert!((self.m.start <= split) && (split <= self.m.end));
                (Set { m: self.m.start..split, ..self.clone() }, Set { m: split..self.m.end, ..self })
            }
            Category::Aerodynamic => {
                assert!((self.a.start <= split) && (split <= self.a.end));
                (Set { a: self.a.start..split, ..self.clone() }, Set { a: split..self.a.end, ..self })
            }
            Category::Shiny => {
                assert!((self.s.start <= split) && (split <= self.s.end));
                (Set { s: self.s.start..split, ..self.clone() }, Set { s: split..self.s.end, ..self })
            }
        }
    }

    fn size(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}
