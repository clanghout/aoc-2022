use std::collections::HashMap;

use regex::Regex;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = include_str!("../../inputs/input7.txt");

    let input_numbers = parse_input(contents);

    let part1 = calc_part1(&input_numbers);

    println!("result: {}", part1);

    let part2 = calc_part2(&input_numbers);

    println!("result: {}", part2);

    Ok(())
}

fn parse_input(contents: &str) -> HashMap<String, usize> {
    let mut cur = vec![];
    let mut folders: HashMap<String, usize> = HashMap::new();
    let re = Regex::new(r"(\d+) .*").unwrap();

    contents
        .trim()
        .lines()
        .for_each(|line|
            match line {
                _ if line.starts_with("$ cd ..") => { cur.pop(); }
                _ if line.starts_with("$ cd") => {
                    cur.push(line.split_at(5).1);
                }
                _ if line.starts_with("$ ls") || line.starts_with("dir") => {
                    // do nothing
                }
                _ => cur.iter().enumerate().for_each(|(i, _)| {
                    let key = cur.iter().take(i + 1).map(|a| *a).collect::<Vec<&str>>().concat();
                    let size = match re.captures(line) {
                        Some(cap) => cap[1].parse().unwrap(),
                        None => panic!("Invalid move string {}", line),
                    };
                    folders.entry(key).and_modify(|total_size| *total_size += size).or_insert(size);
                })
            }
        );
    folders
}

fn calc_part1(input: &HashMap<String, usize>) -> usize {
    let mut res = 0usize;
    for (_, &size) in input {
        if size < 100_000usize {
            res += size;
        }
    }
    res
}

fn calc_part2(input: &HashMap<String, usize>) -> usize {
    let free_space = 70000000 - input.get("/").unwrap();
    let minimal_delete_size = 30000000 - free_space;

    input
        .iter()
        .filter_map(|(_, &size)| if size > minimal_delete_size { Some(size) } else { None })
        .min()
        .unwrap()
}

#[test]
fn test() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    let test = parse_input(&input);
    assert_eq!(calc_part1(&test), 95437usize)
}

#[test]
fn test2() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    let test = parse_input(&input);
    assert_eq!(calc_part2(&test), 24933642usize)
}
