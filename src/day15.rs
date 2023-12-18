type Input<'a> = Vec<&'a [u8]>;

pub fn generator(input: &str) -> Input {
    input.split(',').map(|each| each.as_bytes()).collect()
}

pub fn part1(input: &Input) -> usize {
    input.iter().map(|step| hash(step)).sum()
}

// pub fn part2(input: &Input) -> usize {

// }

fn hash(chars: &[u8]) -> usize {
    chars.iter().fold(0_usize, |acc, char| {
        (acc + <u8 as Into<usize>>::into(*char)) * 17 % 256
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 1320);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 64);
    // }
}
