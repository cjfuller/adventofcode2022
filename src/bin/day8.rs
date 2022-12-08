use adventofcode2022::read_input_lines;

type Forest = Vec<Vec<u8>>;

fn parse_forest() -> Forest {
    let mut output = vec![];
    for line in read_input_lines(8, false) {
        let mut curr = vec![];
        for digit in line.chars() {
            curr.push(digit.to_digit(10).unwrap() as u8)
        }
        output.push(curr);
    }
    output
}

fn count_visible(forest: &Forest) -> u32 {
    let mut visible_count: u32 = 0;

    for (row, row_vec) in forest.iter().enumerate() {
        for (col, height) in row_vec.iter().enumerate() {
            if row == 0 || forest.iter().take(row).all(|it| it[col] < *height) {
                visible_count += 1;
                continue;
            }
            if row == forest.len() - 1 || forest.iter().skip(row + 1).all(|it| it[col] < *height) {
                visible_count += 1;
                continue;
            }
            if col == 0 || row_vec.iter().take(col).all(|it| *it < *height) {
                visible_count += 1;
                continue;
            }
            if col == row_vec.len() - 1 || row_vec.iter().skip(col + 1).all(|it| *it < *height) {
                visible_count += 1;
                continue;
            }
        }
    }
    visible_count
}

fn best_scenic_score(forest: &Forest) -> u32 {
    let mut best_score: u32 = 0;

    for (row, row_vec) in forest.iter().enumerate() {
        for (col, height) in row_vec.iter().enumerate() {
            let mut up_score: u32 = 0;
            for tree in forest.iter().take(row).map(|it| it[col]).rev() {
                up_score += 1;
                if tree >= *height {
                    break;
                }
            }

            let mut down_score: u32 = 0;
            for tree in forest.iter().skip(row + 1).map(|it| it[col]) {
                down_score += 1;
                if tree >= *height {
                    break;
                }
            }

            let mut left_score: u32 = 0;
            for tree in row_vec.iter().take(col).rev().copied() {
                left_score += 1;
                if tree >= *height {
                    break;
                }
            }

            let mut right_score: u32 = 0;
            for tree in row_vec.iter().skip(col + 1).copied() {
                right_score += 1;
                if tree >= *height {
                    break;
                }
            }

            let total_score = up_score * down_score * left_score * right_score;

            best_score = std::cmp::max(best_score, total_score);
        }
    }
    best_score
}

fn part1() {
    let forest = parse_forest();
    let result = count_visible(&forest);
    println!("Part 1: {result}");
}

fn part2() {
    let forest = parse_forest();
    let result = best_scenic_score(&forest);
    println!("Part 2: {result}");
}

fn main() {
    part1();
    part2();
}
