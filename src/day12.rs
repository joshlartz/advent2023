use itertools::Itertools;
use memoize::memoize;

type Input = Vec<Record>;

#[derive(Debug)]
pub struct Record {
    springs: String,
    groups: Vec<usize>,
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            Record {
                springs: parts.next().unwrap().to_string(),
                groups: parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect_vec(),
            }
        })
        .collect_vec()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|each| chomp(each.springs.clone(), each.groups.clone()))
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(unfold)
        .map(|each| chomp(each.springs.clone(), each.groups.clone()))
        .sum()
}

#[memoize]
fn chomp(springs: String, groups: Vec<usize>) -> usize {
    let mut arrangements = 0;

    if springs.is_empty() {
        return if groups.is_empty() { 1 } else { 0 };
    } else if groups.is_empty() && springs.chars().any(|c| c == '#') {
        return 0;
    }

    let first = &springs[..1];
    if first == "." {
        arrangements += chomp(springs[1..].to_string(), groups.clone());
    }
    if first == "?" {
        arrangements += chomp(format!(".{}", &springs[1..]), groups.clone());
        arrangements += chomp(format!("#{}", &springs[1..]), groups.clone());
    }
    if first == "#" {
        // at least enough springs to satisfy the group
        if springs.len() < groups[0] {
            return 0;
        }
        // found a complete group
        if springs[..groups[0]]
            .chars()
            .all(|c| ['#', '?'].contains(&c))
        {
            // check spring after the group
            if springs.len() > groups[0] {
                let next = &springs[groups[0]..groups[0] + 1];
                // too big for the group
                if next == "#" {
                    return 0;
                }
                // the next character needs to be a . for the group to be valid so consume it
                arrangements += chomp(springs[groups[0] + 1..].to_string(), groups[1..].to_vec());
            } else {
                // end of the springs
                arrangements += chomp(springs[groups[0]..].to_string(), groups[1..].to_vec());
            }
        }
    }
    // if arrangements > 0 {
    //     println!(
    //         "springs: {:?}, groups: {:?}, arrangments: {}",
    //         springs, groups, arrangements
    //     );
    // }
    arrangements
}

fn unfold(record: &Record) -> Record {
    Record {
        springs: format!("{}?", record.springs).repeat(5),
        groups: record.groups.repeat(5),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 525152);
    }
}
