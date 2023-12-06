use itertools::Itertools;
use std::ops::Range;

#[derive(Debug)]
pub struct Input {
    seeds: Vec<isize>,
    maps: Maps,
}

#[derive(Debug)]
struct Maps {
    s2s: Vec<Map>,
    s2f: Vec<Map>,
    f2w: Vec<Map>,
    w2l: Vec<Map>,
    l2t: Vec<Map>,
    t2h: Vec<Map>,
    h2l: Vec<Map>,
}

#[derive(Debug)]
pub struct Map {
    offset: isize,
    source: Range<isize>,
    // source + offset
    destination: isize,
}

pub fn generator(input: &str) -> Input {
    let mut sections = input.split("\n\n");
    Input {
        seeds: sections
            .next()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .split_whitespace()
            .map(|seed| seed.parse::<isize>().unwrap())
            .collect_vec(),
        maps: Maps {
            s2s: parse_map(sections.next().unwrap().lines().skip(1)),
            s2f: parse_map(sections.next().unwrap().lines().skip(1)),
            f2w: parse_map(sections.next().unwrap().lines().skip(1)),
            w2l: parse_map(sections.next().unwrap().lines().skip(1)),
            l2t: parse_map(sections.next().unwrap().lines().skip(1)),
            t2h: parse_map(sections.next().unwrap().lines().skip(1)),
            h2l: parse_map(sections.next().unwrap().lines().skip(1)),
        },
    }
}

fn parse_map<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Map> {
    input
        .map(|line| {
            // destination, source, length
            let items: Vec<isize> = line
                .split_whitespace()
                .map(|each| each.parse::<isize>().unwrap())
                .collect();
            Map {
                offset: items[0] - items[1], //items[1] - items[0],
                source: items[1]..items[1] + items[2],
                destination: items[0],
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> isize {
    input
        .seeds
        .iter()
        .map(|seed| get_location(seed, &input.maps))
        .min()
        .unwrap()
}

pub fn part2(input: &Input) -> isize {
    input
        .seeds
        .chunks(2)
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .map(|seed| get_location(&seed, &input.maps))
        .min()
        .unwrap()
}

fn get_dest(find_me: isize, map: &[Map]) -> isize {
    if let Some(found) = map.iter().find(|each| each.source.contains(&find_me)) {
        found.destination + find_me - found.source.start
    } else {
        find_me
    }
}

fn get_location(seed: &isize, maps: &Maps) -> isize {
    // format this better
    get_dest(
        get_dest(
            get_dest(
                get_dest(
                    get_dest(get_dest(get_dest(*seed, &maps.s2s), &maps.s2f), &maps.f2w),
                    &maps.w2l,
                ),
                &maps.l2t,
            ),
            &maps.t2h,
        ),
        &maps.h2l,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 46);
    }
}
