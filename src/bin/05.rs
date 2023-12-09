advent_of_code::solution!(5);

struct MapEntry {
    source: usize,
    output: usize,
    range: usize,
}

impl MapEntry {
    fn build(line: &str) -> MapEntry {
        let triple: Vec<usize> = line.splitn(3, " ").map(|s| s.parse().unwrap()).collect();
        MapEntry {
            source: triple[1],
            output: triple[0],
            range: triple[2],
        }
    }
}

fn get_map(input: &str) -> Vec<MapEntry> {
    input.lines().skip(1).map(MapEntry::build).collect()
}

fn get_match(m: &Vec<MapEntry>, input: usize) -> usize {
    m.iter()
        .find(|entry| input >= entry.source && input <= (entry.source + entry.range - 1))
        .map(|entry| (input - entry.source) + entry.output)
        .unwrap_or(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut parts = input.split("\n\n");

    let seeds: Vec<usize> = parts
        .next()?
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let maps: Vec<Vec<MapEntry>> = (0..7).map(|_| get_map(parts.next().unwrap())).collect();

    let mut seeds: Vec<usize> = seeds;
    for map in &maps {
        seeds = seeds.iter().map(|seed| get_match(map, *seed)).collect();
    }

    let min = *seeds.iter().min().unwrap();
    Some(min) //621354867
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut parts = input.split("\n\n");
    let mut results: Vec<usize> = Vec::new();

    let pairs_str = parts.next()?.split(":").nth(1).unwrap_or("").trim();
    let pairs: Vec<&str> = pairs_str.split_whitespace().collect();

    let maps: Vec<Vec<MapEntry>> = (0..7).map(|_| get_map(parts.next().unwrap())).collect();

    for pair in pairs.chunks(2) {
        let start: usize = pair[0].parse().unwrap();
        let range: usize = pair[1].parse().unwrap();

        let seeds: Vec<usize> = (start..start + range).collect();

        let mut seeds: Vec<usize> = seeds;
        for map in &maps {
            seeds = seeds.iter().map(|seed| get_match(map, *seed)).collect();
        }

        let min = *seeds.iter().min().unwrap();
        results.push(min);
    }
    let result: usize = *results.iter().min().unwrap();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
