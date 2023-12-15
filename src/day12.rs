use itertools::Itertools;
use memoize::memoize;

type Input = Vec<Record>;

#[derive(Debug)]
pub struct Record {
    // springs: Vec<Spring>,
    springs: String,
    groups: Vec<usize>,
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            Record {
                // springs: parts.next().unwrap().chars().map(spring_type).collect_vec(),
                // springs: parts.next().unwrap().chars().collect_vec(),
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
    let foo = input[5..]
        .iter()
        .map(|each| chomp(each.springs.clone(), each.groups.clone()))
        .collect_vec();
    println!("{:?}", foo);
    foo.iter().sum()
}

#[memoize]
fn chomp(input: String, groups: Vec<usize>) -> usize {
    let mut arrangements = 0;

    if input.is_empty() {
        return if groups.is_empty() { 1 } else { 0 };
    } else if groups.is_empty() {
        return 0;
    }

    let first = &input[..1];
    if first == "." {
        arrangements += chomp(input[1..].to_string(), groups.clone());
    }
    if first == "?" {
        arrangements += chomp(format!("{}{}", ".", &input[1..]), groups.clone());
        arrangements += chomp(format!("{}{}", "#", &input[1..]), groups.clone());
    }
    if first == "#" {
        // at least enough springs to satisfy the group
        if input.len() < groups[0] {
            return 0;
        }
        // found a complete group
        if input[..groups[0]].chars().all(|c| c == '#') {
            // check spring after the group
            if input.len() > groups[0] {
                let next = &input[groups[0]..groups[0] + 1];
                // too big for the group
                if next == "#" {
                    return 0;
                }
                // the next character needs to be a . for the group to be valid so consume it
                arrangements += chomp(input[groups[0] + 1..].to_string(), groups[1..].to_vec());
            } else {
                // end of the springs
                arrangements += chomp(input[groups[0]..].to_string(), groups[1..].to_vec());
            }
        } else {
            // take one from the group
            let mut groups = groups.clone();
            groups[0] -= 1;
            arrangements += chomp(input[1..].to_string(), groups);
        }
    }
    if arrangements > 0 {
        println!(
            "input: {:?}, groups: {:?}, arrangments: {}",
            input, groups, arrangements
        );
    }
    arrangements
}

// pub fn part2(input: &Input) -> isize {
//     main(input, 999_999)
// }

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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(main(&generator(SAMPLE), 9), 1030);
    //     assert_eq!(main(&generator(SAMPLE), 99), 8410);
    // }
}
