use crate::Day;

pub struct Day15;

impl Day for Day15 {
    async fn part1(input: String) -> String {
        part1(&input.replace(['\n', '\r'], "")).to_string()
    }

    async fn part2(input: String) -> String {
        part2(&input.replace(['\n', '\r'], "")).to_string()
    }
}

fn hash(s: &str) -> usize {
    let mut state = 0;
    s.as_bytes()
        .iter()
        .for_each(|b| { state = ((state + *b as usize) * 17) & 255 });
    state
}

fn part1(s: &str) -> usize {
    s.split(',').map(hash).sum()
}

fn part2(s: &str) -> usize {
    let instructions = s.split(',');
    let mut hashmap: Vec<Vec<(String, u8)>> = (0..256).map(|_| Vec::new()).collect();
    for ins in instructions {
        if ins.contains('=') {
            let key = &ins[..ins.len()-2];
            let lens = ins[ins.len()-1..].parse().unwrap();
            let slots = &mut hashmap[hash(key)];
            if let Some(slot) = slots.iter().position(|(this_key, _)| this_key == key) {
                slots[slot].1 = lens;
            } else {
                hashmap[hash(key)].push((key.to_owned(), lens));
            }
        } else {
            let key = &ins[..ins.len()-1];
            let slots = &mut hashmap[hash(key)];
            if let Some(slot) = slots.iter().position(|(this_key, _)| this_key == key) {
                slots.remove(slot);
            }
        }
    }
    hashmap.into_iter()
        .enumerate()
        .flat_map(|(ind, lenses)| lenses
            .into_iter()
            .enumerate()
            .map(move |(slot, (_, lens))| (ind + 1) * (slot + 1) * lens as usize))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 145);
    }
}
