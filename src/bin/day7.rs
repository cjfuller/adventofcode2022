use std::collections::HashMap;
use std::path::{Path, PathBuf};

use adventofcode2022::read_input_lines;

#[derive(Debug)]
struct File {
    parent: PathBuf,
    _filename: String,
    size: usize,
}

impl File {
    fn from_str(s: &str, curr_path: &Path) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();
        File {
            parent: curr_path.to_path_buf(),
            _filename: parts[1].to_string(),
            size: parts[0].parse().unwrap(),
        }
    }
}

fn process_ls(lines: &[String], curr_path: &Path, output: &mut Vec<File>) {
    if let Some(l) = lines.first() {
        if l.starts_with('$') {
            return process_input_lines(lines, curr_path, output);
        }
        if l.starts_with("dir ") {
            return process_ls(&lines[1..lines.len()], curr_path, output);
        }
        output.push(File::from_str(l, curr_path));
        process_ls(&lines[1..lines.len()], curr_path, output);
    }
}

fn process_input_lines(lines: &[String], curr_path: &Path, output: &mut Vec<File>) {
    if let Some(l) = lines.first() {
        if !l.starts_with('$') {
            panic!("Found non-command in command context: {l}");
        }
        if l.starts_with("$ cd ") {
            let dirname = &l["$ cd ".len()..l.len()];
            if dirname == ".." {
                return process_input_lines(
                    &lines[1..lines.len()],
                    curr_path.parent().unwrap(),
                    output,
                );
            } else {
                return process_input_lines(
                    &lines[1..lines.len()],
                    &curr_path.join(dirname),
                    output,
                );
            }
        }
        if l.starts_with("$ ls") {
            return process_ls(&lines[1..lines.len()], curr_path, output);
        }
        panic!("Unknown command: {l}");
    }
}

fn process_input() -> Vec<File> {
    let lines = read_input_lines(7, false);
    let mut files = vec![];
    process_input_lines(&lines, Path::new("/"), &mut files);
    files
}

fn calc_sizes_by_dir(files: &[File]) -> HashMap<&Path, usize> {
    let mut sizes_by_dir: HashMap<&Path, usize> = HashMap::new();
    for file in files.iter() {
        let mut p: &Path = &file.parent;
        loop {
            *sizes_by_dir.entry(p).or_insert(0) += file.size;
            if p == Path::new("/") {
                break;
            }
            p = p.parent().unwrap();
        }
    }
    sizes_by_dir
}

fn part1() {
    let files = process_input();
    let result: usize = calc_sizes_by_dir(&files)
        .values()
        .filter(|it| **it <= 100000)
        .sum();
    println!("Part 1: {result}");
}

fn part2() {
    let files = process_input();
    let dir_sizes = calc_sizes_by_dir(&files);
    let total_file_size = dir_sizes[Path::new("/")];
    let space_needed = 30000000 - (70000000 - total_file_size);
    let mut best_dir = Path::new("/");
    let mut best_size = total_file_size;
    for (dir, size) in dir_sizes.iter() {
        if *size >= space_needed && *size < best_size {
            best_dir = *dir;
            best_size = *size;
        }
    }
    println!("Part 2: {best_dir:?} -> {best_size}");
}

fn main() {
    part1();
    part2();
}
