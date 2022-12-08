use std::borrow::{Borrow, BorrowMut};
use adventlib;

fn main() {
    adventlib::input_helpers::print_puzzle_header(5);
    //adventlib::measure_execution_time_ns(part1, 10000);
    run(true);
}

#[derive(Clone)]
struct File {
    name: String,
    size: u32,
}

impl File
{
    fn new(name: &str, size: u32) -> File
    {
        File{
            name: String::from(name),
            size
        }
    }
}

#[derive(Clone)]
struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<File>
}

impl Directory
{
    fn new(name: &str) -> Directory
    {
        Directory {
            name: String::from(name),
            directories: Vec::new(),
            files: Vec::new()
        }
    }

    fn add_directory(&mut self, directory: Directory)
    {
        self.directories.push(directory);
    }

    fn add_file(&mut self, file: File)
    {
        self.files.push(file);
    }
}

fn run(do_print: bool)
{
    let input = include_str!("../input.txt");

    let mut directories: Vec<Directory> = Vec::new();
    let mut root = Directory::new("/");
    let mut cwd = root;
    let mut parent: Option<Directory> = None;

    for line in input.lines()
    {
        if line == "$ cd .."
        {
            cwd = parent.unwrap();
            parent
        }
        else if line.contains("$ cd")
        {
            let new_dir_name = &line[5..];
            let mut new_dir = Directory::new(new_dir_name);
            cwd.add_directory(new_dir);
            parent = Some(cwd);
            cwd = &mut new_dir;
        }
    }

    if do_print{println!("Part 1 - Start Sequence found at index:");}
}
