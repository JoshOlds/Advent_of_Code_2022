use adventlib;

fn main() {
    adventlib::input_helpers::print_puzzle_header(4);
    adventlib::measure_execution_time_us(run, 10000);
    //run(true);
}

fn run(do_print: bool)
{
    let input = include_str!("../input.txt");

    // assignments stored as a vector containing pairs of tuples
    let mut assignments: Vec<((usize, usize), (usize, usize))> = Vec::with_capacity(1000);

    for line in input.lines()
    {
        let mut assignments_iter = line.split(",");
        let mut halves = assignments_iter.next().unwrap().split("-"); // First of the two assignments, split on the '-' to get the lower and upper values
        let first: usize = halves.next().unwrap().parse::<usize>().unwrap();
        let second: usize = halves.next().unwrap().parse::<usize>().unwrap();
        halves = assignments_iter.next().unwrap().split("-"); // Advance iterator to second of the two assignments
        let third: usize = halves.next().unwrap().parse::<usize>().unwrap();
        let fourth: usize = halves.next().unwrap().parse::<usize>().unwrap();
        assignments.push(((first, second), (third, fourth)));
    }

    // Part 1
    let mut count: usize = 0;
    for assignment in assignments.iter()
    {
        if fully_contains(assignment) { count += 1; }
    }

    if do_print { println!("Part 1 - {} assignments fully contain another.", count) }

    // Part 2
    count = 0;
    for assignment in assignments.iter()
    {
        if check_any_overlap(assignment) { count += 1; }
    }
    if do_print { println!("Part 2 - {} assignments have any overlap.", count) }
}

fn fully_contains(in_tuple: &((usize, usize), (usize, usize))) -> bool
{
    if in_tuple.0.0 <= in_tuple.1.0
    {
        if in_tuple.0.1 >= in_tuple.1.1
        {
            return true;
        }
    }
    if in_tuple.1.0 <= in_tuple.0.0
    {
        if in_tuple.1.1 >= in_tuple.0.1
        {
            return true;
        }
    }
    false
}

fn check_any_overlap(in_tuple: &((usize, usize), (usize, usize))) -> bool
{
    if in_tuple.0.1 < in_tuple.1.0
    {
        return false;
    }
    if in_tuple.1.1 < in_tuple.0.0
    {
        return false;
    }
    true
}