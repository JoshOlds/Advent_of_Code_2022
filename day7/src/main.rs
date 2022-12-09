use std::rc::Rc;
use std::cell::RefCell;
use adventlib;

fn main() {
    adventlib::input_helpers::print_puzzle_header(7);
    adventlib::measure_execution_time_us(run, 10000);
    //run(true);
}

// Create a File struct that stores name and size. Derive clone to allow for .clone() calls
#[derive(Clone)]
struct File {
    pub name: String,
    pub size: u32,
}

impl File
{
    // Constructor for a new File
    fn new(name: &str, size: u32) -> File
    {
        File {
            name: String::from(name),
            size,
        }
    }
}

// Directory Struct. Has a pointer to a parent, and a collection of child directories and files, as well as a tracked size
// Size is updated any time a new file is added. This is recursive up the directory tree
#[derive(Clone)]
struct Directory {
    pub name: String,
    // Rc is reference counting type, like a shared_ptr in C++.
    // RefCell is used to allow us to have multiple mutable references to an object that is borrow checked at Runtime
    // This is slow, and verbose to type, but the only way to have a double-linked list structure like we have here
    pub parent: Option<Rc<RefCell<Directory>>>,
    pub directories: Vec<Rc<RefCell<Directory>>>,
    pub files: Vec<File>,
    pub size: u32,
}

impl Directory
{
    // Directory constructor
    fn new(name: &str, parent: Option<Rc<RefCell<Directory>>>) -> Directory
    {
        Directory {
            name: String::from(name),
            parent,
            directories: Vec::new(),
            files: Vec::new(),
            size: 0,
        }
    }

    fn add_directory(&mut self, directory: Rc<RefCell<Directory>>)
    {
        self.directories.push(directory);
    }

    // Adds a file to the directory. Walks up the directory tree and adjusts the size of all parent directories
    fn add_file(&mut self, name: &str, size: u32)
    {
        self.files.push(File::new(name, size));
        self.size += size; // Update our size

        // Get our parent reference
        let mut parent = self.parent.clone();
        loop {
            match parent // check if we have a parent, break if at the top of the tree
            {
                None => break,
                Some(_) => {}
            }
            parent.clone().unwrap().borrow_mut().size += size; // Update parent size
            parent = parent.clone().unwrap().borrow_mut().parent.clone(); // Parent pointer is now the parent of the current parent...
        }
    }

    // Recursive function that gathers references to all children directories into a flat vector
    fn get_all_subdirectories(&self) -> Vec<Rc<RefCell<Directory>>>
    {
        let mut subdirs: Vec<Rc<RefCell<Directory>>> = Vec::new();
        for child in self.directories.clone()
        {
            subdirs.push(child.clone()); // Add this child to the subdir list
            let mut recursive_subdirs = child.borrow_mut().get_all_subdirectories(); // recursive call on the child, keep walking down the directories
            subdirs.append(&mut recursive_subdirs); // Append result of the recursive call
        }
        return subdirs;
    }
}

fn run(do_print: bool)
{
    let input = include_str!("../input.txt");

    // Start with a Heap allocated root directory
    let mut cwd: Rc<RefCell<Directory>> = Rc::new(RefCell::new(Directory::new("/", None)));
    let root = cwd.clone(); // Keep it around for later

    for line in input.lines()
    {
        // Skip root line and ls line
        if line.contains("$ cd /") || line == "$ ls"
        {
            continue;
        }
        if line == "$ cd .."
        {
            // If we are traversing up, set cwd to its own parent
            let parent_dir = cwd.borrow_mut().parent.clone().unwrap();
            cwd = parent_dir;
        } else if line.contains("$ cd")
        {
            // If we are changing directories, get the destination directory name, find it in the list of cwd subdirectories, and set cwd to the found reference
            let dest_dir_name = &line[5..];
            let dirs = cwd.borrow_mut().clone().directories;
            let dest_dir = dirs.iter().find(
                |dir| dir.borrow_mut().name == dest_dir_name
            ).unwrap();
            cwd = dest_dir.clone();
        } else if line.contains("dir")
        {
            // If creating a new directory, heap allocate and add to cwd's subdirectory list
            let new_dir_name = &line[4..];
            let new_dir = Rc::new(RefCell::new(Directory::new(new_dir_name, Some(cwd.clone()))));
            cwd.borrow_mut().add_directory(new_dir.clone());
        } else {
            // Else we must be adding a file, split out the values and add a file to cwd
            let mut vals = line.split_whitespace();
            let size: u32 = vals.next().unwrap().parse::<u32>().unwrap();
            let name = vals.next().unwrap();
            cwd.borrow_mut().add_file(name, size);
        }
    }

    //println!("Test size: {}, root size: {}", test_size, root.borrow_mut().size); // Nice test to print and check directories
    // Get a flat list of all subdirs
    let all_subdirectories = root.borrow_mut().get_all_subdirectories();

    // Some maths given the puzzle directions
    let root_size: u32 = root.borrow_mut().size;
    let free_size: u32 = 70000000 - root_size;
    let required_size = 30000000 - free_size;

    // Track sum and best candidate for part 1 & 2
    let mut sum: u32 = 0;
    let mut best_candidate_size: u32 = root_size;

    // walk the subdirs
    for subdir in all_subdirectories.clone()
    {
        let size = subdir.borrow_mut().size.clone();
        if size <= 100000 { sum += size; }
        if size < best_candidate_size && size >= required_size { best_candidate_size = size; }
    }

    if do_print { println!("Part 1 - Sum of all directories <= 100,000: {}", sum); }
    if do_print { println!("Part 2 - Best candidate for deletion is of size: {}", best_candidate_size); }
}
