use std::collections::HashMap;

type Input<'a> = Vec<&'a [u8]>;

#[derive(Debug, Clone)]
struct Lens {
    focal_length: usize,
    order: usize,
}

pub fn generator(input: &str) -> Input {
    input.split(',').map(|each| each.as_bytes()).collect()
}

pub fn part1(input: &Input) -> usize {
    input.iter().map(|step| hash(step)).sum()
}

const MINUS: u8 = b'-';
const EQUALS: u8 = b'=';

pub fn part2(input: &Input) -> usize {
    let mut boxes: Vec<HashMap<&[u8], Lens>> = vec![HashMap::new(); 256];

    input.iter().for_each(|step| {
        let mut split = step.split(split_on);
        let label: &[u8] = split.next().unwrap();
        let focal_length = split.next().unwrap_or_default();
        let box_num = hash(label);

        if focal_length.is_empty() {
            // minus
            if let Some(removed) = boxes[box_num].remove(label) {
                boxes[box_num]
                    .iter_mut()
                    .for_each(|lens: (&&[u8], &mut Lens)| {
                        if lens.1.order > 1 && lens.1.order > removed.order {
                            lens.1.order -= 1
                        }
                    })
            }
        } else {
            // equals
            if let Some(existing) = boxes[box_num].get_mut(label) {
                existing.focal_length = get_focal_length(focal_length);
            } else {
                let order = boxes[box_num].len() + 1;
                boxes[box_num].insert(
                    label,
                    Lens {
                        focal_length: get_focal_length(focal_length),
                        order,
                    },
                );
            }
        }
    });

    // boxes.iter().enumerate().for_each(|(i, b)| if !b.is_empty() { println!("box: {} - lenses: {:?}", i, b); });

    boxes
        .iter()
        .enumerate()
        .fold(0, |acc: usize, (box_num, lenses)| {
            acc + lenses.values().fold(0, |acc, lens| {
                acc + (box_num + 1) * lens.order * lens.focal_length
            })
        })
}

fn hash(chars: &[u8]) -> usize {
    chars.iter().fold(0_usize, |acc, char| {
        (acc + <u8 as Into<usize>>::into(*char)) * 17 % 256
    })
}

fn split_on(char: &u8) -> bool {
    char == &MINUS || char == &EQUALS
}

fn get_focal_length(focal_length: &[u8]) -> usize {
    if focal_length.is_empty() {
        panic!("no focal length")
    } else {
        // my god this is gross, find a better way
        String::from_utf8(focal_length.to_vec())
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 145);
    }
}
