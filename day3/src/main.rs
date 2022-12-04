use adventlib;

fn main() {
    adventlib::input_helpers::print_puzzle_header(3);
    adventlib::measure_execution_time_us(run, 10000);
}

fn run(do_print: bool)
{
    let input = include_str!("../input.txt");

    // Rucksacks are stored as a vector of tuples (rucksack_a, rucksack_b)
    let mut rucksacks: Vec<(&str, &str)> = Vec::with_capacity(300); // input is 300 lines

    // Split out the rucksack halves
    for line in input.lines()
    {
        let half = line.len() / 2;
        let rucksack_a = &line[..half];
        let rucksack_b = &line[half..];
        rucksacks.push((rucksack_a, rucksack_b));
    }

    // Find matches and sum values
    let mut total: usize = 0;
    for (rucksack_a, rucksack_b) in rucksacks
    {
        let match_character = find_match(rucksack_a, rucksack_b).unwrap();
        total += get_priority_value(&match_character);
    }

    if do_print{println!("Part 1 - Sum of priorities: {}", total)}

    // Part 2
    total = 0;
    let mut line_iter = input.lines();
    loop
    {
        // Grab three lines per loop
        let a = match line_iter.next() // Use match here to break loop when at end of iterator
        {
            Some(x) => x,
            None => break
        };
        let b = line_iter.next().unwrap();
        let c = line_iter.next().unwrap();
        let match_character = find_match_triple(a, b, c).unwrap();
        total += get_priority_value(&match_character);
    }

    if do_print{println!("Part 2 - Sum of priorities: {}", total)}
}

// Iterate over characters of first string and check second for matches
fn find_match(a: &str, b: &str) -> Option<char>
{
    for c in a.chars()
    {
        if b.contains(c)
        {
            return Some(c);
        }
    }
    // If no match found
    None
}
// Iterate over characters of first string and check for matches in second and third
fn find_match_triple(a: &str, b: &str, c: &str) -> Option<char>
{
    for x in a.chars()
    {
        if b.contains(x)
        {
            if c.contains(x)
            {
                return Some(x);
            }
        }
    }
    None
}

fn get_priority_value(c: &char) -> usize
{
    // Convert from ASCII to Advent values
    return if c.is_uppercase()
    {
        *c as usize - 38
    } else {
        *c as usize - 96
    } as usize;
}