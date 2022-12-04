pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = include_str!("../../inputs/input4.txt");

    let input_numbers = parse_input(contents);

    let part1 = calc_part1(&input_numbers);

    println!("result: {}", part1);

    let part2 = calc_part2(&    input_numbers);

    println!("result: {}", part2);

    Ok(())
}

fn parse_input(contents: &str) -> Vec<Vec<Vec<usize>>> {
    contents
        .lines()
        .map(|line|
            line
                .split(",")
                .map(|area|
                    area
                        .split("-")
                        .map(|x|
                            x.parse::<usize>().ok().unwrap()).collect()).collect())
        .collect()
}

fn calc_part2(input: &[Vec<Vec<usize>>]) -> usize {
    input.iter().filter(|line|
        // if the first area starts lower
        if line[0][0] < line[1][0] {
            // then the second area must start before or at the same spot as the end of the first
            line[1][0] <= line[0][1]
        }
            // if the second area starts lower
        else if line[0][0] > line[1][0]{
            // then the first area must start befor or at the end of the second one
            line[0][0] <= line[1][1]
        }
        else {
            // if they start at the same spot, the end does not matter
            true
        }

    ).count()
}

fn calc_part1(input: &[Vec<Vec<usize>>]) -> usize {
    input.iter().filter(|line|
        // if the first area starts lower
        if line[0][0] < line[1][0] {
            // then the first area must end later or at the same place
            line[0][1] >= line[1][1]
        }
        // if the second area starts lower
        else if line[0][0] > line[1][0]{
            // then the second area must end later or at the same place
            line[0][1] <= line[1][1]
        }
        else {
            // if they start at the same spot, the end does not matter
            true
        }

    ).count()
}

#[test]
fn test() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    let test = parse_input(&input);
    assert_eq!(calc_part1(&test), 2usize)
}


#[test]
fn test2() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    let test = parse_input(&input);
    assert_eq!(calc_part2(&test), 4usize)
}