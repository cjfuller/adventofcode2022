use adventofcode2022::read_input_lines;

// (row, col) indexes of start and end and grid
fn parse_input() -> ((usize, usize), (usize, usize), Vec<Vec<u8>>) {
    let lines = read_input_lines(12, false);
    let mut output = vec![];
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);
    for (r, line) in lines.into_iter().enumerate() {
        let mut curr = vec![];
        for (c, char) in line.chars().enumerate() {
            match char {
                'S' => {
                    curr.push(0);
                    start = (r, c);
                }
                'E' => {
                    curr.push(25);
                    end = (r, c)
                }
                other => curr.push(other as u8 - b'a'),
            }
        }
        output.push(curr);
    }
    assert!(start.0 < usize::MAX);
    assert!(end.0 < usize::MAX);
    (start, end, output)
}

fn make_empty_grid(input_grid: &[Vec<u8>]) -> Vec<Vec<Option<u32>>> {
    let mut output = vec![];
    for row in input_grid {
        let mut output_row = vec![];
        for _ in 0..row.len() {
            output_row.push(None);
        }
        output.push(output_row);
    }
    output
}

fn update_distance_grid_at(
    ri: usize,
    ci: usize,
    distance_grid: &mut [Vec<Option<u32>>],
    grid: &[Vec<u8>],
) {
    let mut best_dist: Option<u32> = None;
    for rni in (std::cmp::max(0, ri as i64 - 1) as usize)..=std::cmp::min(grid.len() - 1, ri + 1) {
        if rni == ri {
            continue;
        }
        if let Some(d) = distance_grid[rni][ci] {
            let height_diff = grid[rni][ci] as i64 - grid[ri][ci] as i64;
            if height_diff <= 1 {
                best_dist = Some(std::cmp::min(best_dist.unwrap_or(u32::MAX), d + 1));
            }
        }
    }
    for cni in
        (std::cmp::max(0, ci as i64 - 1) as usize)..=std::cmp::min(grid[ri].len() - 1, ci + 1)
    {
        if cni == ci {
            continue;
        }
        if let Some(d) = distance_grid[ri][cni] {
            let height_diff = grid[ri][cni] as i64 - grid[ri][ci] as i64;
            if height_diff <= 1 {
                best_dist = Some(std::cmp::min(best_dist.unwrap_or(u32::MAX), d + 1));
            }
        }
    }
    distance_grid[ri][ci] = best_dist;
}

fn part1_2() {
    let (start, end, grid) = parse_input();
    let mut distance_grid = make_empty_grid(&grid);
    let mut min_to_0_elev: Option<u32> = None;
    distance_grid[end.0][end.1] = Some(0);
    loop {
        for (ri, row) in grid.iter().enumerate() {
            for (ci, _) in row.iter().enumerate() {
                if distance_grid[ri][ci].is_some() {
                    continue;
                }
                update_distance_grid_at(ri, ci, &mut distance_grid, &grid);
                if distance_grid[ri][ci].is_some() && grid[ri][ci] == 0 {
                    min_to_0_elev = Some(std::cmp::min(
                        distance_grid[ri][ci].unwrap(),
                        min_to_0_elev.unwrap_or(u32::MAX),
                    ));
                }
            }
        }
        if distance_grid[start.0][start.1].is_some() {
            break;
        }
    }

    println!("Part 1: {}", distance_grid[start.0][start.1].unwrap());
    println!("Part 2: {}", min_to_0_elev.unwrap());
}

fn main() {
    part1_2();
}
