use adventlib;

fn main() {
    adventlib::input_helpers::print_puzzle_header(1);
    adventlib::measure_execution_time_us(run, 10000);
}

fn run(do_print: bool)
{
    let input = include_str!("../input.txt");

    // Vector of Vectors of calorie values (initialize vectors bigger than needed to speed up execution time)
    let mut elf_vector: Vec<Vec<usize>> = Vec::with_capacity(1000);

    let mut calorie_vector: Vec<usize> = Vec::with_capacity(100);
    for calorie_val in input.lines()
    {
        if calorie_val.is_empty() // If we are at a blank line, the previous elf is complete
        {
            elf_vector.push(calorie_vector.clone());
            calorie_vector.clear();
        } else {
            calorie_vector.push(calorie_val.parse::<usize>().unwrap()); // Convert to usize and accumulate in a calorie vector
        }
    }

    // Create a new vector of sums for the elf values
    let mut elf_sum_vector: Vec<usize> = elf_vector.iter()
        .map(|cal_vec| cal_vec.iter().sum())
        .collect();

    // Find the highest calorie value elf
    let mut max_ref = elf_sum_vector.iter()
        .max()
        .unwrap();

    // Get index of max value - used for deleting later
    let index = elf_sum_vector.iter().position(|element| element == max_ref).unwrap();

    if do_print
    {
        println!("Part 1: The maximum calorie count elf is Elf #{}, and he is carrying {} calories", index, max_ref);
    }

    // Part 2
    // Max is currently a reference, so let's clone it to a usize so we can add to it
    let mut max_val = max_ref.clone();

    // Remove previous max elf
    elf_sum_vector.remove(index);

    // Recalculate max
    max_ref = elf_sum_vector.iter()
        .max()
        .unwrap();
    let index = elf_sum_vector.iter().position(|element| element == max_ref).unwrap();
    max_val = max_val + max_ref;

    // One more time... (time to make a function?)
    elf_sum_vector.remove(index);
    max_ref = elf_sum_vector.iter()
        .max()
        .unwrap();
    max_val = max_val + max_ref;

    if do_print
    {
        println!("Part 2: The three elves with the most food are carrying {} calories.", max_val);
    }
}